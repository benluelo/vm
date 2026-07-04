/**
 * @file Tree-sitter grammar for benluelo/vm MIR and assembly sources
 * @author kernelnomad
 * @license MIT
 */

/// <reference types="tree-sitter-cli/dsl" />
// @ts-check

const MIR_BUILTINS = [
  "add",
  "mul",
  "sub",
  "div",
  "exp",
  "mod",
  "eq",
  "lt",
  "gt",
  "shl",
  "shr",
  "or",
  "xor",
  "and",
  "not",
  "neg",
  "alloc",
  "write1",
  "write2",
  "write3",
  "write4",
  "write5",
  "write6",
  "write7",
  "write8",
  "read1",
  "read2",
  "read3",
  "read4",
  "read5",
  "read6",
  "read7",
  "read8",
  "dread1",
  "dread2",
  "dread3",
  "dread4",
  "dread5",
  "dread6",
  "dread7",
  "dread8",
  "dcopy",
  "dlen",
  "exit",
  "trap",
];

const ASM_NO_ARG_OPS = [
  "push0",
  "dup",
  "swap",
  "pop",
  "alloc",
  "write1",
  "write2",
  "write3",
  "write4",
  "write5",
  "write6",
  "write7",
  "write8",
  "read1",
  "read2",
  "read3",
  "read4",
  "read5",
  "read6",
  "read7",
  "read8",
  "dread1",
  "dread2",
  "dread3",
  "dread4",
  "dread5",
  "dread6",
  "dread7",
  "dread8",
  "dcopy",
  "dlen",
  "add",
  "sub",
  "mul",
  "div",
  "exp",
  "mod",
  "eq",
  "neq",
  "lt",
  "gt",
  "not",
  "shl",
  "shr",
  "neg",
  "or",
  "xor",
  "and",
  "jump",
  "jnz",
  "call",
  "exit",
  "trap",
];

const ASM_PUSH_OPS = [
  "push1",
  "push2",
  "push3",
  "push4",
  "push5",
  "push6",
  "push7",
  "push8",
];

module.exports = grammar({
  name: "vm",

  word: $ => $.identifier,

  extras: $ => [
    /\s/,
    $.mir_comment,
    $.asm_comment,
  ],

  supertypes: $ => [
    $.mir_statement,
    $.mir_expression,
    $.asm_item,
    $.asm_instruction,
  ],

  rules: {
    source_file: $ => choice(
      $.mir_program,
      $.asm_program,
    ),

    mir_program: $ => repeat1($.mir_statement),

    asm_program: $ => repeat1($.asm_item),

    mir_statement: $ => choice(
      $.mir_definition,
      $.mir_loop,
      $.mir_break,
      $.mir_continue,
      $.mir_if,
      $.mir_assignment,
      $.mir_expression_statement,
    ),

    mir_definition: $ => seq(
      "def",
      field("name", $.identifier),
      "(",
      field("parameters", commaSep($.mir_parameter)),
      ")",
      optional(seq("->", field("returns", commaSep1($.mir_return)))),
      field("body", $.mir_block),
    ),

    mir_parameter: $ => $.identifier,

    mir_return: $ => $.identifier,

    mir_loop: $ => seq(
      "loop",
      field("label", $.mir_label),
      field("body", $.mir_block),
    ),

    mir_break: $ => seq(
      "break",
      field("label", $.mir_label),
    ),

    mir_continue: $ => seq(
      "continue",
      field("label", $.mir_label),
    ),

    mir_if: $ => seq(
      "if",
      field("condition", $.mir_expression),
      field("consequence", $.mir_block),
      optional($.mir_else_clause),
    ),

    mir_else_clause: $ => seq(
      "else",
      field("alternative", choice($.mir_if, $.mir_block)),
    ),

    mir_assignment: $ => seq(
      field("left", commaSep1($.identifier)),
      "<-",
      field("right", $.mir_expression),
    ),

    mir_expression_statement: $ => $.mir_expression,

    mir_expression: $ => choice(
      $.mir_call,
      $.identifier,
      $.number,
    ),

    mir_call: $ => seq(
      optional(field("spread", "...")),
      field("function", choice($.mir_builtin, $.identifier)),
      "(",
      field("arguments", commaSep($.mir_expression)),
      ")",
    ),

    mir_builtin: _ => choice(...MIR_BUILTINS),

    mir_block: $ => seq(
      "{",
      repeat($.mir_statement),
      "}",
    ),

    mir_label: _ => token(seq(":", /[A-Za-z_][A-Za-z0-9_]*/)),

    asm_item: $ => choice(
      $.asm_label,
      $.asm_instruction,
    ),

    asm_label: _ => token(seq(":", /[A-Za-z_][A-Za-z0-9_]*/)),

    asm_instruction: $ => choice(
      $.asm_push,
      $.asm_push_label,
      $.asm_simple_instruction,
    ),

    asm_push: $ => seq(
      field("opcode", $.asm_push_opcode),
      field("value", $.byte_literal),
    ),

    asm_push_label: $ => seq(
      field("opcode", "pushl"),
      field("target", $.asm_label_reference),
    ),

    asm_simple_instruction: $ => field("opcode", $.asm_opcode),

    asm_push_opcode: _ => choice(...ASM_PUSH_OPS),

    asm_opcode: _ => choice(...ASM_NO_ARG_OPS),

    asm_label_reference: _ => token(seq("@", /[A-Za-z_][A-Za-z0-9_]*/)),

    identifier: _ => /[A-Za-z_][A-Za-z0-9_]*/,

    number: _ => token(choice(
      /0x[0-9A-Fa-f]+/,
      /0b[01]+/,
      /[0-9]+/,
    )),

    byte_literal: _ => token(/0x[0-9A-Fa-f]+/),

    mir_comment: _ => token(seq("#", /.*/)),

    asm_comment: _ => token(seq(";", /.*/)),
  },
});

function commaSep1(rule) {
  return seq(rule, repeat(seq(",", rule)), optional(","));
}

function commaSep(rule) {
  return optional(commaSep1(rule));
}
