#include <assert.h>
#include <stddef.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "./op.h"

typedef struct Stack {
  size_t len;
  size_t capacity;
  uint64_t *data;
} Stack;

typedef enum VmResult : int8_t {
  VM_OK = 0,
  /// Out of memory.
  VM_ERR_OUT_OF_MEMORY = -1,
  /// Attempted to pop off of an empty stack.
  VM_ERR_STACK_EMPTY = -2,
  /// Attempted to read a stack index that doesn't exist.
  VM_ERR_INVALID_STACK_IDX = -3,
  /// Attempted to read past the max allocated memory address.
  VM_ERR_SEGFAULT = -4,
  /// Unexpected EOF when executing code.
  VM_ERR_EOF = -5,
  /// Attempted to divide by zero.
  VM_ERR_DIVIDE_BY_ZERO = -6,
  /// Invalid stack value for operation.
  VM_ERR_INVALID_STACK_VALUE = -7,
  VM_ERR_UNKNOWN_OP = -8,

  VM_STEP_RESULT_EOF = 1,
  VM_STEP_RESULT_TRAP = 2,
  VM_STEP_RESULT_EXIT = 3,
} VmResult;

typedef struct Memory {
  size_t size;
  uint8_t *data;
} Memory;

typedef struct Fat {
  const uint8_t *ptr;
  size_t len;
} Fat;

typedef struct Vm {
  Fat code;
  Fat data;
  Stack stack;
  Memory memory;
  size_t pc;
  union {
    uint64_t trap;
    Fat exit;
  } out;
} Vm;

VmResult step_vm(Vm *vm);

VmResult run_vm(Vm *vm);

Vm new_vm(Fat code, Fat data);

Fat new_fat(uint8_t const *ptr, size_t len);
