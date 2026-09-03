use std::{env, fs, path::Path};
use zed::settings::LspSettings;
use zed_extension_api::{self as zed, Result};

const SERVER_BINARY: &str = "cucumber-language-server";
const PACKAGE_NAME: &str = "@cucumber/language-server";
const PACKAGE_MAIN: &str = "node_modules/@cucumber/language-server/dist/cjs/src/index.js";
const LAUNCHER: &str = "server.cjs";

/// Replacement for the package's bin, which resolves the language-service wasm grammars at a
/// nested node_modules path that npm hoisting breaks, so every glue file silently fails to parse
/// and no step definition is ever found. This launcher resolves them for real.
const LAUNCHER_SRC: &str = r#"const path = require('path')
const nm = path.join(__dirname, 'node_modules')
const { startStandaloneServer } = require(path.join(nm, '@cucumber/language-server/dist/cjs/src/wasm/startStandaloneServer'))
const { NodeFiles } = require(path.join(nm, '@cucumber/language-server/dist/cjs/src/node/NodeFiles'))
const wasmBasePath = path.join(path.dirname(require.resolve(path.join(nm, '@cucumber/language-service'))), '..', '..')
startStandaloneServer(wasmBasePath, (rootUri) => new NodeFiles(rootUri))
"#;

struct CucumberExtension {
    did_find_server: bool,
}

impl CucumberExtension {
    fn server_exists(&self) -> bool {
        Path::new(PACKAGE_MAIN).is_file()
    }

    fn ensure_server(&mut self, id: &zed::LanguageServerId) -> Result<()> {
        if self.did_find_server && self.server_exists() {
            return Ok(());
        }
        zed::set_language_server_installation_status(
            id,
            &zed::LanguageServerInstallationStatus::CheckingForUpdate,
        );
        let latest = zed::npm_package_latest_version(PACKAGE_NAME)?;
        let installed = zed::npm_package_installed_version(PACKAGE_NAME)?;
        if !self.server_exists() || installed.as_deref() != Some(latest.as_str()) {
            zed::set_language_server_installation_status(
                id,
                &zed::LanguageServerInstallationStatus::Downloading,
            );
            if let Err(error) = zed::npm_install_package(PACKAGE_NAME, &latest) {
                if !self.server_exists() {
                    return Err(error);
                }
            }
        }
        fs::write(LAUNCHER, LAUNCHER_SRC).map_err(|e| format!("failed to write {LAUNCHER}: {e}"))?;
        self.did_find_server = true;
        Ok(())
    }
}

impl zed::Extension for CucumberExtension {
    fn new() -> Self {
        Self { did_find_server: false }
    }

    /// Uses a globally installed server when one is on `PATH` (npm -g nests its deps, so its own
    /// bin works). Otherwise installs the npm package and runs it through [`LAUNCHER_SRC`], passed
    /// by absolute path because the server process does not run in the extension work dir.
    fn language_server_command(
        &mut self,
        id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        if let Some(command) = worktree.which(SERVER_BINARY) {
            return Ok(zed::Command { command, args: vec!["--stdio".into()], env: vec![] });
        }
        self.ensure_server(id)?;
        let launcher = env::current_dir()
            .map_err(|e| e.to_string())?
            .join(LAUNCHER)
            .to_string_lossy()
            .to_string();
        Ok(zed::Command {
            command: zed::node_binary_path()?,
            args: vec![launcher, "--stdio".into()],
            env: vec![],
        })
    }

    /// Hands `lsp.cucumber.settings` from settings.json to the server verbatim. The server asks
    /// for section `cucumber`, so users write `{ "cucumber": { "features": [...], "glue": [...] } }`.
    fn language_server_workspace_configuration(
        &mut self,
        _id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<Option<zed::serde_json::Value>> {
        Ok(LspSettings::for_worktree("cucumber", worktree).ok().and_then(|s| s.settings))
    }
}

zed::register_extension!(CucumberExtension);
