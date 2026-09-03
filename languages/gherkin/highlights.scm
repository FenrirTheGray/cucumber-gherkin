[(feature_kw) (background_kw) (scenario_kw) (scenario_outline_kw) (rule_kw) (examples_kw)] @keyword

(context) @title

(description) @comment.doc

(given_group (_ (_ [(given_kw) (and_kw) (but_kw) "* "] @keyword.given)))
(when_group (_ (_ [(when_kw) (and_kw) (but_kw) "* "] @function.when)))
(then_group (_ (_ [(then_kw) (and_kw) (but_kw) "* "] @type.then)))

(step_param) @variable.special

(tag) @attribute

(table_head_row (table_col (table_cell) @property))
"|" @punctuation.special

(doc_string) @string
(media_type) @label

"language" @keyword
(language_name) @string.special
(invalid_language_name) @comment.error

":" @punctuation.delimiter
(comment) @comment
