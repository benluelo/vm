#include <assert.h>
#include <stdbool.h>
#include <stdckdint.h>
#include <stddef.h>
#include <stdint.h>
#include <stdio.h>
#include <string.h>

#include "./vm.h"

#define ensure_stack(n)                                                        \
  {                                                                            \
    if (vm->stack.len < n) {                                                   \
      assert(VM_ERR_STACK_EMPTY);                                              \
    }                                                                          \
  }

#define ensure_memory(ptr, n)                                                  \
  {                                                                            \
    if (vm->memory.size < (ptr + n)) {                                         \
      assert(VM_ERR_SEGFAULT);                                                 \
    }                                                                          \
  }

#define ensure_data(ptr, n)                                                    \
  {                                                                            \
    if (vm->data.len < (ptr + n)) {                                            \
      assert(VM_ERR_SEGFAULT);                                                 \
    }                                                                          \
  }

#define ensure_code(n)                                                         \
  {                                                                            \
    if (vm->code.len < (vm->pc + n)) {                                         \
      assert(VM_ERR_EOF);                                                      \
    }                                                                          \
  }

#define binop(op)                                                              \
  {{size_t len = vm->stack.len;                                                \
                                                                               \
  if (len < 2) {                                                               \
    assert(VM_ERR_STACK_EMPTY);                                                \
  }                                                                            \
                                                                               \
  uint64_t lhs = vm->stack.data[len - 2];                                      \
  uint64_t rhs = vm->stack.data[len - 1];                                      \
                                                                               \
  vm->stack.len -= 1;                                                          \
  vm->stack.data[len - 2] = op(lhs, rhs);                                      \
  }                                                                            \
  }

#define try_binop(op)                                                          \
  {{size_t len = vm->stack.len;                                                \
                                                                               \
  if (len < 2) {                                                               \
    assert(VM_ERR_STACK_EMPTY);                                                \
  }                                                                            \
                                                                               \
  uint64_t lhs = vm->stack.data[len - 2];                                      \
  uint64_t rhs = vm->stack.data[len - 1];                                      \
                                                                               \
  vm->stack.len -= 1;                                                          \
  vm->stack.data[len - 2] = try(op(lhs, rhs));                                 \
  }                                                                            \
  }

#define try(e)                                                                 \
  {{VM_ERR res = e;                                                            \
  if (res != VM_ERR_OK) {                                                      \
    assert(res);                                                               \
  }                                                                            \
  }                                                                            \
  }

#define push_n(n)                                                              \
  ensure_code(n);                                                              \
  try(push_stack(&vm->stack, u64_from_bytes(n, vm->code.ptr + vm->pc)));       \
  vm->pc += n;

#define read_n(n)                                                              \
  ensure_stack(1);                                                             \
  size_t *top = &vm->stack.data[vm->stack.len - 1];                            \
  uint64_t ptr = *top;                                                         \
  ensure_memory(ptr, n);                                                       \
  uint64_t res = u64_from_bytes(n, vm->memory.data + ptr);                     \
  *top = res;

#define dread_n(n)                                                             \
  ensure_stack(1);                                                             \
  uint64_t *top = &vm->stack.data[vm->stack.len - 1];                          \
  uint64_t ptr = *top;                                                         \
  ensure_data(ptr, n);                                                         \
  uint64_t res = u64_from_bytes(n, vm->data.ptr + ptr);                        \
  *top = res;

#define write_n(n)                                                             \
  ensure_stack(2);                                                             \
  uint64_t value = vm->stack.data[vm->stack.len - 1];                          \
  size_t ptr = vm->stack.data[vm->stack.len - 2];                              \
  ensure_memory(ptr, n);                                                       \
  write_u64(n, value, vm->memory.data + ptr);                                  \
  vm->stack.len -= 2;

inline uint64_t u64_from_bytes(size_t n, const uint8_t *arr) {
  // printf("R: n: %zu\n", n);

  uint64_t out = 0;
  for (size_t i = 0; i < n; i++) {
    out += ((uint64_t)arr[i]) << (n - (i + 1)) * 8;
    // printf("R: i: %zu, value: %016lx\n", i, out);
  }
  return out;
}

inline void write_u64(size_t n, uint64_t value, uint8_t *buffer) {
  // printf("W: n: %zu, value: %016lx\n", n, value);

  for (size_t i = n; i > 0; i--) {
    uint8_t byte = (uint8_t)(value & 0x00000000000000FF);
    // printf("W: i: %zu, byte: %x, value: %016lx\n", i, byte, value);
    buffer[i - 1] = byte;
    value >>= 8;
  }
}

#define try_add(res, val, n)                                                   \
  {                                                                            \
    if (ckd_add(res, val, n)) {                                                \
      assert(VM_ERR_INVALID_STACK_IDX);                                        \
    };                                                                         \
  }

VM_ERR push_stack(Stack *stack, uint64_t value) {
  if (stack->capacity == 0) {
    uint64_t *ptr = (uint64_t *)malloc(4 * 8);
    if (ptr == NULL) {
      return VM_ERR_OUT_OF_MEMORY;
    }
    stack->data = ptr;
    stack->capacity = 4;
  } else if (stack->len == stack->capacity) {
    uint64_t *ptr = (uint64_t *)realloc(stack->data, stack->capacity * 2 * 8);
    if (ptr == NULL) {
      return VM_ERR_OUT_OF_MEMORY;
    }
    stack->data = ptr;
    stack->capacity *= 2;
  }

  stack->data[stack->len] = value;
  stack->len++;

  return VM_ERR_OK;
}

VM_ERR alloc_memory(Memory *memory, size_t additional) {
  uint8_t *ptr = (uint8_t *)realloc(memory->data, memory->size + additional);
  if (ptr == NULL) {
    return VM_ERR_OUT_OF_MEMORY;
  }
  memory->data = ptr;

  memset(ptr + memory->size, 0, additional);

  memory->size += additional;

  return VM_ERR_OK;
}

VM_ERR pop_stack(Stack *stack, uint64_t *value) {
  if (stack->len == 0) {
    return VM_ERR_STACK_EMPTY;
  }

  stack->len--;
  *value = stack->data[stack->len];

  return VM_ERR_OK;
}

// inline uint64_t u64_from_bytes(size_t n, uint8_t arr[n]) {
//     // return intCast((@Int(.unsigned, n * 8), &arr, .big));
// }

int readFile(char *path, uint8_t **out, size_t *size) {
  FILE *infile;

  infile = fopen(path, "r");

  if (infile == NULL)
    return 1;

  fseek(infile, 0L, SEEK_END);
  *size = ftell(infile);
  printf("file size: %lu\n", *size);

  fseek(infile, 0L, SEEK_SET);

  *out = (uint8_t *)calloc(*size, sizeof(uint8_t));

  if (out == NULL)
    return 1;

  unsigned long res = fread(*out, sizeof(char), *size, infile);
  if (res != *size) {
    printf("only read %lu/%zu bytes\n", res, *size);
    return 2;
  }
  fclose(infile);

  return 0;
}

int main(int argc, char *argv[]) {
  if (argc < 2) {
    fprintf(stderr, "missing argument\n");
    return 1;
  }

  uint8_t *code;
  size_t code_len;
  int code_res = readFile(argv[1], &code, &code_len);
  if (code_res != 0) {
    printf("unable to read code: %d\n", code_res);
    return code_res;
  }

  printf("code size: %zu\n", code_len);

  uint8_t *data;
  size_t data_len;
  int data_res = readFile(argv[2], &data, &data_len);
  if (data_res != 0) {
    printf("unable to read data: %d\n", data_res);
    return data_res;
  }

  printf("data size: %zu\n", data_len);

  Vm vm = new_vm(new_fat(code, code_len), new_fat(data, data_len));

  RunResult res = run_vm(&vm);

  switch (res.tag) {
  case RUN_RESULT_DONE: {
    fprintf(stdout, "done\n");
    break;
  }
  case RUN_RESULT_EOF: {
    fprintf(stdout, "eof\n");
    break;
  }
  case RUN_RESULT_TRAP: {
    fprintf(stdout, "trap %zu\n", res.data.trap);
    break;
  }
  case RUN_RESULT_EXIT: {
    fprintf(stdout, "exit ");
    for (size_t i = 0; i < res.data.exit.len; i++) {
      fprintf(stdout, "%02x", res.data.exit.ptr[i]);
    }
    fprintf(stdout, "\n");
    break;
  }
  }
}

RunResult run_vm(Vm *vm) {
  int cycles = 0;
  while (vm->pc < vm->code.len) {
    // printf("\n[%d] ", cycles);
    StepResult res = step_vm(vm);
    cycles++;

    switch (res.tag) {
    case STEP_RESULT_STEPPED:
      // printf("stepped\n");
      continue;
    case STEP_RESULT_EOF:
      return (RunResult){
          .tag = RUN_RESULT_EOF,
      };
    case STEP_RESULT_TRAP:
      return (RunResult){.tag = RUN_RESULT_TRAP,
                         .data = {.trap = res.data.trap}};
    case STEP_RESULT_EXIT:
      return (RunResult){.tag = RUN_RESULT_EXIT,
                         .data = {.exit = res.data.exit}};
    }
  }

  return (RunResult){
      .tag = RUN_RESULT_DONE,
  };
}

StepResult step_vm(Vm *vm) {
  if (vm->pc >= vm->code.len) {
    return (StepResult){.tag = STEP_RESULT_EOF};
  }

  uint8_t op = vm->code.ptr[vm->pc];

  // printf("pc: %zu, op: %02x\n", vm->pc, op);
  // printf("stack: [ ");
  // for (int i; i < vm->stack.len; i++) {
  //   printf("%016lx ", vm->stack.data[i]);
  // }
  // printf("]\n");
  // printf("memory: ");
  // for (int i; i < vm->memory.size; i++) {
  //   printf("%02x", vm->memory.data[i]);
  // }
  // printf("\n");

  vm->pc++;

  switch (op) {
  case PUSH0: {
    // printf("PUSH0\n");
    try(push_stack(&vm->stack, 0));
    break;
  };
  case PUSH1: {
    // printf("PUSH1\n");
    push_n(1);
    break;
  };
  case PUSH2: {
    // printf("PUSH2\n");
    push_n(2);
    break;
  };
  case PUSH3: {
    // printf("PUSH3\n");
    push_n(3);
    break;
  };
  case PUSH4: {
    // printf("PUSH4\n");
    push_n(4);
    break;
  };
  case PUSH5: {
    // printf("PUSH5\n");
    push_n(5);
    break;
  };
  case PUSH6: {
    // printf("PUSH6\n");
    push_n(6);
    break;
  };
  case PUSH7: {
    // printf("PUSH7\n");
    push_n(7);
    break;
  };
  case PUSH8: {
    // printf("PUSH8\n");
    push_n(8);
    break;
  };
  case DUP: {
    // printf("DUP\n");
    ensure_stack(1);
    uint64_t *idx = &vm->stack.data[vm->stack.len - 1];
    uint64_t stack_idx;
    try_add(&stack_idx, *idx, 1);
    // printf("idx: %d\n", *idx);
    ensure_stack(stack_idx);
    *idx = vm->stack.data[(vm->stack.len - 1) - stack_idx];
    break;
  };
  case DUP0: {
    // printf("DUP0\n");
    ensure_stack(1);
    try(push_stack(&vm->stack, vm->stack.data[vm->stack.len - 1]));
    break;
  };
  case SWAP: {
    // printf("SWAP\n");
    uint64_t idx = 0;
    try(pop_stack(&vm->stack, &idx));
    try_add(&idx, idx, 1);
    size_t len = vm->stack.len;
    if (len < idx) {
      assert(VM_ERR_INVALID_STACK_IDX);
    }
    size_t a_idx = len - 1;
    size_t b_idx = a_idx - idx;
    uint64_t a = vm->stack.data[a_idx];
    vm->stack.data[a_idx] = vm->stack.data[b_idx];
    vm->stack.data[b_idx] = a;
    break;
  };
  case SWAP0: {
    // printf("SWAP0\n");
    ensure_stack(2);
    uint64_t a = vm->stack.data[vm->stack.len - 1];
    vm->stack.data[vm->stack.len - 1] = vm->stack.data[vm->stack.len - 2];
    vm->stack.data[vm->stack.len - 2] = a;
    break;
  };
  case POP: {
    // printf("POP\n");
    ensure_stack(1);
    vm->stack.len--;
    break;
  };
  case ALLOC: {
    // printf("ALLOC\n");
    uint64_t value = 0;
    try(pop_stack(&vm->stack, &value));
    // printf("%lu\n", value);
    try(alloc_memory(&vm->memory, value));
    break;
  };
  case WRITE1: {
    // printf("WRITE1\n");
    write_n(1);
    break;
  };
  case WRITE2: {
    // printf("WRITE2\n");
    write_n(2);
    break;
  };
  case WRITE3: {
    // printf("WRITE3\n");
    write_n(3);
    break;
  };
  case WRITE4: {
    // printf("WRITE4\n");
    write_n(4);
    break;
  };
  case WRITE5: {
    // printf("WRITE5\n");
    write_n(5);
    break;
  };
  case WRITE6: {
    // printf("WRITE6\n");
    write_n(6);
    break;
  };
  case WRITE7: {
    // printf("WRITE7\n");
    write_n(7);
    break;
  };
  case WRITE8: {
    // printf("WRITE8\n");
    write_n(8);
    break;
  };
  case READ1: {
    // printf("READ1\n");
    read_n(1);
    break;
  };
  case READ2: {
    // printf("READ2\n");
    read_n(2);
    break;
  };
  case READ3: {
    // printf("READ3\n");
    read_n(3);
    break;
  };
  case READ4: {
    // printf("READ4\n");
    read_n(4);
    break;
  };
  case READ5: {
    // printf("READ5\n");
    read_n(5);
    break;
  };
  case READ6: {
    // printf("READ6\n");
    read_n(6);
    break;
  };
  case READ7: {
    // printf("READ7\n");
    read_n(7);
    break;
  };
  case READ8: {
    // printf("READ8\n");
    read_n(8);
    break;
  };
  case DREAD1: {
    // printf("DREAD1\n");
    dread_n(1);
    break;
  };
  case DREAD2: {
    // printf("DREAD2\n");
    dread_n(2);
    break;
  };
  case DREAD3: {
    // printf("DREAD3\n");
    dread_n(3);
    break;
  };
  case DREAD4: {
    // printf("DREAD4\n");
    dread_n(4);
    break;
  };
  case DREAD5: {
    // printf("DREAD5\n");
    dread_n(5);
    break;
  };
  case DREAD6: {
    // printf("DREAD6\n");
    dread_n(6);
    break;
  };
  case DREAD7: {
    // printf("DREAD7\n");
    dread_n(7);
    break;
  };
  case DREAD8: {
    // printf("DREAD8\n");
    dread_n(8);
    break;
  };
  case DCOPY: {
    // printf("DCOPY\n");
    if (vm->stack.len < 3) {
      assert(VM_ERR_STACK_EMPTY);
    }

    size_t len = vm->stack.data[vm->stack.len - 1];
    size_t dst = vm->stack.data[vm->stack.len - 2];
    size_t src = vm->stack.data[vm->stack.len - 3];

    ensure_data(src, len);
    ensure_memory(dst, len);

    vm->stack.len -= 3;

    memcpy(vm->memory.data + dst, vm->data.ptr + src, len);
    break;
  };
  case DLEN: {
    // printf("DLEN\n");
    try(push_stack(&vm->stack, vm->data.len));
    break;
  };
  case ADD: {
    // printf("ADD\n");
    binop(op_add);
    break;
  };
  case SUB: {
    // printf("SUB\n");
    binop(op_sub);
    break;
  };
  case MUL: {
    // printf("MUL\n");
    binop(op_mul);
    break;
  };
  case DIV: {
    // printf("DIV\n");
    size_t len = vm->stack.len;
    if (len < 2) {
      assert(VM_ERR_STACK_EMPTY);
    }
    uint64_t lhs = vm->stack.data[len - 2];
    uint64_t rhs = vm->stack.data[len - 1];
    vm->stack.len -= 1;
    if (rhs == 0) {
      assert(VM_ERR_DIVIDE_BY_ZERO);
    }
    vm->stack.data[len - 2] = op_div(lhs, rhs);
    break;
  };
  case EXP: {
    // printf("EXP\n");
    binop(op_expmod);
    break;
  };
  case MOD: {
    // printf("MOD\n");
    size_t len = vm->stack.len;
    if (len < 2) {
      assert(VM_ERR_STACK_EMPTY);
    }
    uint64_t lhs = vm->stack.data[len - 2];
    uint64_t rhs = vm->stack.data[len - 1];
    vm->stack.len -= 1;
    if (rhs == 0) {
      assert(VM_ERR_DIVIDE_BY_ZERO);
    }
    vm->stack.data[len - 2] = op_mod(lhs, rhs);
    break;
  };
  case EQ: {
    // printf("EQ\n");
    binop(op_eq);
    break;
  };
  case NEQ: {
    // printf("NEQ\n");
    binop(op_neq);
    break;
  };
  case LT: {
    // printf("LT\n");
    binop(op_lt);
    break;
  };
  case GT: {
    // printf("GT\n");
    binop(op_gt);
    break;
  };
  case NOT: {
    // printf("NOT\n");
    ensure_stack(1);
    uint64_t *a = &vm->stack.data[vm->stack.len - 1];
    *a = op_not(*a);
    break;
  };
  case SHL: {
    // printf("SHL\n");
    binop(op_shl);
    break;
  };
  case SHR: {
    // printf("SHR\n");
    binop(op_shr);
    break;
  };
  case NEG: {
    // printf("NEG\n");
    ensure_stack(1);
    uint64_t *a = &vm->stack.data[vm->stack.len - 1];
    *a = op_neg(*a);
    break;
  };
  case OR: {
    // printf("OR\n");
    binop(op_or);
    break;
  };
  case XOR: {
    // printf("XOR\n");
    binop(op_xor);
    break;
  };
  case AND: {
    // printf("AND\n");
    binop(op_and);
    break;
  };
  case JUMP: {
    // printf("JUMP\n");
    try(pop_stack(&vm->stack, &vm->pc));
    break;
  };
  case JNZ: {
    // printf("JNZ\n");

    ensure_stack(2);

    uint64_t dst = vm->stack.data[vm->stack.len - 1];
    uint64_t value = vm->stack.data[vm->stack.len - 2];

    vm->stack.len -= 2;

    if (value != 0) {
      vm->pc = dst;
    }
    break;
  };
  case CALL: {
    // printf("CALL\n");

    ensure_stack(1);

    uint64_t *top = &vm->stack.data[vm->stack.len - 1];
    uint64_t address = *top;
    *top = vm->pc;
    vm->pc = address;
    break;
  };
  case EXIT: {
    // printf("EXIT\n");
    ensure_stack(2);

    // printf("memory: ");
    // for (int i; i < vm->memory.size; i++) {
    //   printf("%02x", vm->memory.data[i]);
    // }
    // printf("\n");

    uint64_t len = vm->stack.data[vm->stack.len - 1];
    uint64_t ptr = vm->stack.data[vm->stack.len - 2];
    vm->stack.len -= 2;

    return (StepResult){.tag = STEP_RESULT_EXIT,
                        .data.exit = new_fat(vm->memory.data + ptr, len)};
  };
  case TRAP: {
    // printf("TRAP\n");
    assert(false);
    break;
  };
  default:
    // printf("unknown op %02x\n", op);
    assert(false);
  }

  return (StepResult){.tag = STEP_RESULT_STEPPED};
}

Vm new_vm(Fat code, Fat data) {
  return (Vm){
      .code = code,
      .data = data,
      .memory =
          {
              .data = 0,
              .size = 0,
          },
      .stack =
          {
              .data = 0,
              .capacity = 0,
              .len = 0,
          },
      .pc = 0,
  };
}

Fat new_fat(uint8_t const *ptr, size_t len) {
  return (Fat){.len = len, .ptr = ptr};
}
