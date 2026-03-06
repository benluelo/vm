A very basic 64-bit stack-based VM, with a custom assembly language and low-level programming language that compiles to it.

The assembly is largely feature-complete, aside from some missing opcodes that I haven't implemented yet (mainly bitwise operators).

The MIR is also nearly feature complete:

- [x] Basic expressions
- [x] Control flow (if/else-if/else)
- [x] Loops with labelled break/continue
- [x] Function definitions
  - [x] Multiple return values and the spread operator to use them inline in one expression
- [ ] Access to all VM functionality (currently missing all the read/write/dread widths as I haven't gotten around to it yet)
- [ ] Inline assembly and/or naked functions

The goal with the MIR is to use it as a target for a higher level programming language in the future, to allow for an easier target than using the assembly directly.

A tree-sitter grammar for the MIR can be found at [./tree-sitter-mir](./tree-sitter-mir).
