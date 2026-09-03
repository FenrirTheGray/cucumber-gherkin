# Cucumber Gherkin for Zed

Gherkin (`.feature`) support for [Zed](https://zed.dev). Syntax highlighting comes from a
tree-sitter grammar, step recognition from
[`@cucumber/language-server`](https://github.com/cucumber/language-server).

What you get:

- Given, When and Then each have their own colour. `And`, `But` and `*` use the colour of the step
  they follow, so an `And` after a `When` looks like a `When`.
- Highlighting for tags, Scenario Outline `<placeholders>`, data tables, doc strings and
  `# language:` headers. A doc string with a media type (` ```json `, ` ```xml `, ...) is
  highlighted as that language.
- Feature, Rule, Background, Scenario and Examples show up in the outline panel and breadcrumbs.
- From the language server: go to step definition (F12), `Undefined step` warnings, step
  completion, formatting, and a code action that writes a step definition snippet for an
  undefined step. With semantic tokens on, arguments of matched steps are highlighted too.

## Install

This extension isn't in the Zed extension registry. Install it from this repo as a dev extension:

```sh
git clone https://github.com/FenrirTheGray/cucumber-gherkin.git
```

In Zed, run `zed: install dev extension` from the command palette (or click Install Dev Extension
on the Extensions page) and pick the cloned folder. Zed builds the extension and the Gherkin
grammar on the first install. You need Rust installed through rustup; with Rust from Homebrew or
Nix you have to add the `wasm32-wasip2` target yourself.

To update, `git pull` and run `zed: rebuild dev extension`.

The language server is installed from npm the first time you open a `.feature` file and updated
to the latest version when Zed restarts. If `cucumber-language-server` is on your `PATH`, that one
is used instead.

## Configuration

All settings go in Zed's `settings.json`, either your user settings or a project's
`.zed/settings.json`.

### Feature and step definition paths

The server's default globs (`features/**/*.feature`, `features/**/*.ts` and so on) rarely match a
real project, so you'll almost always want to set your own. The `settings` object is passed to the
server unchanged:

```jsonc
"lsp": {
  "cucumber": {
    "settings": {
      "cucumber": {
        "features": ["apps/*/cucumber/**/*.feature"],
        "glue": ["apps/*/cucumber/**/*.steps.ts"],
        "parameterTypes": [{ "name": "color", "regexp": "red|green|blue" }],
        "snippetTemplates": {}
      }
    }
  }
}
```

`parameterTypes` lists custom parameter types so steps that use them still match.
`snippetTemplates` overrides the step definition snippet per glue language (`java`, `tsx`,
`c_sharp`, `php`, `python`, `ruby`, `rust`, `javascript`, `go`, `scala`). Both are optional.

### Turning the language server off

```jsonc
"languages": { "Gherkin": { "language_servers": ["!cucumber", "..."] } }
```

### Highlighting matched arguments

Semantic tokens are off by default in Zed. Turn them on for Gherkin, then run
`editor: restart language server`:

```jsonc
"languages": { "Gherkin": { "semantic_tokens": "combined" } }
```

The extension only styles the `parameter` token (an argument of a step that matched a step
definition). The server's other tokens are switched off so they don't paint over the
Given/When/Then colours. These defaults are in `languages/gherkin/semantic_token_rules.json`, and
you can override them under `global_lsp_settings.semantic_token_rules`.

### Colours

Every highlight is a theme style, so you can change any of them per theme with
`theme_overrides`:

```jsonc
"theme_overrides": {
  "One Dark": {
    "syntax": {
      "keyword.given": { "color": "#c678dd" },
      "function.when": { "color": "#61afef" },
      "type.then": { "color": "#e5c07b" },
      "title": { "font_weight": 700 },
      "attribute": { "color": "#56b6c2" },
      "variable.special": { "color": "#d19a66" }
    }
  }
}
```

| Style | Used for |
| --- | --- |
| `keyword` | Feature, Background, Scenario, Scenario Outline, Rule, Examples, `language` |
| `title` | the name after those keywords |
| `comment.doc` | free-text description lines under a Feature, Scenario, etc. |
| `keyword.given`, `function.when`, `type.then` | step keywords; And, But and `*` use the one before them |
| `variable.special` | `<placeholder>` |
| `variable.parameter` | matched step argument (semantic tokens only) |
| `attribute` | `@tag` |
| `property` | Examples table header cells |
| `punctuation.special` | table pipes |
| `punctuation.delimiter` | the `:` after a keyword |
| `string`, `label` | doc string, and its media type |
| `string.special`, `comment.error` | `# language:` value, and an unknown one |
| `comment` | `# comments` |

## Troubleshooting

If F12 does nothing or every step is marked undefined, open `dev: open language server logs` and
pick the Cucumber server. On start it prints how many feature files, glue files and step
definitions it found, along with the globs it searched. If those are the default globs, your
`lsp.cucumber.settings` block isn't reaching the server; check that it's nested under a
`cucumber` key as shown above.

## Why the extension doesn't use the server's own bin

`bin/cucumber-language-server.cjs` expects the language service's wasm files at
`node_modules/@cucumber/language-server/node_modules/@cucumber/language-service/dist`. npm hoists
that package one level up, so the path doesn't exist, every glue file fails to parse, and the
server finds zero step definitions without any visible error. The extension writes a small
`server.cjs` next to the install that resolves the real path and starts the server from there.

A global install (`npm i -g @cucumber/language-server`) doesn't hoist dependencies, so its bin
works fine. That's why a server on `PATH` is used as is.

## License

Licensed under the GNU GPLv3 or any later version (SPDX: `GPL-3.0-or-later`). See [LICENSE](LICENSE).
