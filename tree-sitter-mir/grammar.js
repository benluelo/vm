/**
 * @file Mir grammar for tree-sitter
 * @author ben
 * @license MIT
 */

/// <reference types="tree-sitter-cli/dsl" />
// @ts-nocheck

module.exports = grammar({
  name: "mir",

  word: $ => $.ident,

  rules: {
    source_file: $ => nlSep($.statement),

    statement: $ => choice(
      $.def,
      $.expr,
      $.loop,
      $._break,
      $._continue,
      $._if,
      $.assignment,
    ),

    ident: $ => token(/[a-zA-Z]{1}[a-zA-Z0-9_]*/),

    loop: $ => seq("loop", $.label, $.block),
    _break: $ => seq("break", $.label),
    _continue: $ => seq("continue", $.label),
    _if: $ => seq("if", $.expr, $.block),
    assignment: $ => seq(commaSep1($.ident), "<-", $.expr),
    expr: $ => choice(
      seq(optional("..."), $.ident, '(', commaSep($.expr), ')'),
      $.ident,
      $.val
    ),
    def: $ => seq(
      "def",
      $.ident,
      '(',
      field("arg", commaSep($.ident)),
      ')',
      optional(seq("->", field("ret", commaSep($.ident)))),
      $.block,
    ),

    label: $ => seq(':', /* token.immediate */ ($.ident)),
    val: $ => token(choice(/0x[a-fA-F0-9]+/, /\d+/)),

    block: $ => seq('{', nlSep(choice($.statement, '\n')), '}')
  }
});

function commaSep1(rule) {
  return seq(rule, repeat(seq(',', rule)))
}

function commaSep(rule) {
  return optional(commaSep1(rule))
}

function nlSep1(rule) {
  return seq(rule, repeat(seq('\n', rule)))
}

function nlSep(rule) {
  return optional(nlSep1(rule))
}
