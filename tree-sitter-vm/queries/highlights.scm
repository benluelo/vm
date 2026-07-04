[
  "def"
] @keyword.function

[
  "if"
  "else"
  "break"
  "continue"
] @keyword.control.conditional

[
  "loop"
] @keyword.control.repeat

[
  "<-"
  "->"
] @operator

[
  "("
  ")"
  "{"
  "}"
] @punctuation.bracket

[
  ","
] @punctuation.delimiter

"..." @operator

(mir_comment) @comment
(asm_comment) @comment

(number) @constant.numeric.integer
(byte_literal) @constant.numeric.integer

(mir_label) @label
(asm_label) @label
(asm_label_reference) @label

(mir_definition
  name: (identifier) @function)

(mir_parameter) @variable.parameter
(mir_return) @variable.parameter

(mir_assignment
  left: (identifier) @variable)

(mir_call
  function: (identifier) @function)

(mir_call
  function: (mir_builtin) @function.builtin)

(asm_push_opcode) @keyword.operator
(asm_opcode) @keyword.operator
(asm_push_label
  opcode: "pushl" @keyword.operator)
