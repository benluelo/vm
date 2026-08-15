#include <stdbool.h>
#include <stdckdint.h>
#include <stddef.h>
#include <stdint.h>
#include <stdio.h>
#include <string.h>

#include "./vm.h"

#ifdef DEBUG
#define debug(...) printf(__VA_ARGS__)
#else
#define debug(...)
#endif

#define likely(x) __builtin_expect(!!(x), 1)
#define unlikely(x) __builtin_expect(!!(x), 0)
// #define likely(x) x
// #define unlikely(x) x

#define bail(err) return err
// #include <assert.h>
// #define bail(err) assert(err)

#define ensure_stack(n)                                                        \
  {                                                                            \
    if (unlikely(vm->stack.len < n)) {                                         \
      bail(VM_ERR_STACK_EMPTY);                                                \
    }                                                                          \
  }

#define ensure_memory(ptr, n)                                                  \
  {                                                                            \
    if (unlikely(vm->memory.size < (ptr + n))) {                               \
      bail(VM_ERR_SEGFAULT);                                                   \
    }                                                                          \
  }

#define ensure_data(ptr, n)                                                    \
  {                                                                            \
    if (unlikely(vm->data.len < (ptr + n))) {                                  \
      bail(VM_ERR_SEGFAULT);                                                   \
    }                                                                          \
  }

#define ensure_code(n)                                                         \
  {                                                                            \
    if (unlikely(vm->code.len < (vm->pc + n))) {                               \
      bail(VM_ERR_EOF);                                                        \
    }                                                                          \
  }

#define binop(op)                                                              \
  {                                                                            \
    size_t len = vm->stack.len;                                                \
                                                                               \
    if (unlikely(len < 2)) {                                                   \
      bail(VM_ERR_STACK_EMPTY);                                                \
    }                                                                          \
                                                                               \
    uint64_t lhs = vm->stack.data[len - 2];                                    \
    uint64_t rhs = vm->stack.data[len - 1];                                    \
                                                                               \
    vm->stack.len -= 1;                                                        \
    vm->stack.data[len - 2] = op(lhs, rhs);                                    \
    break;                                                                     \
  }

#define try(expr)                                                              \
  {                                                                            \
    VmResult res = expr;                                                       \
    if (unlikely(res != VM_OK)) {                                              \
      bail(res);                                                               \
    }                                                                          \
  }

#define push_n(n)                                                              \
  {                                                                            \
    debug("PUSH%d\n", n);                                                      \
    ensure_code(n);                                                            \
    try(push_stack(&vm->stack, u64_from_bytes(n, vm->code.ptr + vm->pc)));     \
    vm->pc += n;                                                               \
  }

#define read_n(n)                                                              \
  {                                                                            \
    debug("READ%d\n", n);                                                      \
    ensure_stack(1);                                                           \
    size_t *top = &vm->stack.data[vm->stack.len - 1];                          \
    uint64_t ptr = *top;                                                       \
    ensure_memory(ptr, n);                                                     \
    uint64_t res = u64_from_bytes(n, vm->memory.data + ptr);                   \
    *top = res;                                                                \
  }

#define dread_n(n)                                                             \
  {                                                                            \
    debug("DREAD%d\n", n);                                                     \
    ensure_stack(1);                                                           \
    uint64_t *top = &vm->stack.data[vm->stack.len - 1];                        \
    uint64_t ptr = *top;                                                       \
    ensure_data(ptr, n);                                                       \
    uint64_t res = u64_from_bytes(n, vm->data.ptr + ptr);                      \
    *top = res;                                                                \
  }

#define write_n(n)                                                             \
  {                                                                            \
    debug("WRITE%d\n", n);                                                     \
    ensure_stack(2);                                                           \
    uint64_t value = vm->stack.data[vm->stack.len - 1];                        \
    size_t ptr = vm->stack.data[vm->stack.len - 2];                            \
    ensure_memory(ptr, n);                                                     \
    write_u64(n, value, vm->memory.data + ptr);                                \
    vm->stack.len -= 2;                                                        \
  }

#define try_add(res, val, n)                                                   \
  {                                                                            \
    if (unlikely(ckd_add(res, val, n))) {                                      \
      bail(VM_ERR_INVALID_STACK_IDX);                                          \
    };                                                                         \
  }

typedef union u64 {
  uint64_t n;
  uint8_t bz[8];
} u64;

inline uint64_t u64_from_bytes(size_t n, const uint8_t *arr) {
  u64 out = {0};
  memcpy(out.bz, arr, n);
  // debug("n: %zu, value: %02lx\n", n, out.n);
  // TODO: only do this if target endianness is LE
  // out.n = __builtin_bswap64(out.n) >> (8 * (8 - n));
#if BYTE_ORDER == __LITTLE_ENDIAN
  out.n = htobe64(out.n) >> (8 * (8 - n));
#endif
  return out.n;
}

inline void write_u64(size_t n, uint64_t value, uint8_t *buffer) {
  u64 out;
  // TODO: only do this if target endianness is LE
#if BYTE_ORDER == __LITTLE_ENDIAN
  out.n = be64toh(value << (8 * (8 - n)));
#endif
  memcpy(buffer, out.bz, n);
  debug("WRITE_U64: n: %zu, value: %016lx, out: %016lx\n", n, value, out.n);
}

// inline void write_u64(size_t n, uint64_t value, uint8_t *buffer) {
//   for (size_t i = n; i > 0; i--) {
//     uint8_t byte = (uint8_t)(value & 0x00000000000000FF);
//     buffer[i - 1] = byte;
//     value >>= 8;
//   }
// }

VmResult push_stack(Stack *stack, uint64_t value) {
  if (unlikely(stack->capacity == 0)) {
    uint64_t *ptr = (uint64_t *)malloc(4 * 8);
    if (unlikely(ptr == NULL)) {
      return VM_ERR_OUT_OF_MEMORY;
    }
    stack->data = ptr;
    stack->capacity = 4;
  } else if (unlikely(stack->len == stack->capacity)) {
    uint64_t *ptr = (uint64_t *)realloc(stack->data, stack->capacity * 2 * 8);
    if (unlikely(ptr == NULL)) {
      return VM_ERR_OUT_OF_MEMORY;
    }
    stack->data = ptr;
    stack->capacity *= 2;
  }

  stack->data[stack->len] = value;
  stack->len++;

  return VM_OK;
}

VmResult alloc_memory(Memory *memory, size_t additional) {
  uint8_t *ptr = (uint8_t *)realloc(memory->data, memory->size + additional);
  if (unlikely(ptr == NULL)) {
    return VM_ERR_OUT_OF_MEMORY;
  }
  memory->data = ptr;

  memset(ptr + memory->size, 0, additional);

  memory->size += additional;

  return VM_OK;
}

VmResult pop_stack(Stack *stack, uint64_t *value) {
  if (unlikely(stack->len == 0)) {
    return VM_ERR_STACK_EMPTY;
  }

  stack->len--;
  *value = stack->data[stack->len];

  return VM_OK;
}

inline VmResult step_vm(Vm *vm) {
  if (unlikely(vm->pc >= vm->code.len)) {
    return VM_STEP_RESULT_EOF;
  }

  uint8_t op = vm->code.ptr[vm->pc];

  debug("pc: %zu, op: %02x\n", vm->pc, op);
  debug("stack (%ld): [ ", vm->stack.len);
  // for (int i = 0; i < vm->stack.len; i++) {
  //   debug("%016lx ", vm->stack.data[i]);
  // }
  debug("]\n");
  debug("memory (%ld): ", vm->memory.size);
  // for (int i = 0; i < vm->memory.size; i++) {
  //   debug("%02x", vm->memory.data[i]);
  // }
  debug("\n");

  vm->pc++;

  switch (op) {
  case PUSH0:
    try(push_stack(&vm->stack, 0));
    break;
  case PUSH1:
    push_n(1);
    break;
  case PUSH2:
    push_n(2);
    break;
  case PUSH3:
    push_n(3);
    break;
  case PUSH4:
    push_n(4);
    break;
  case PUSH5:
    push_n(5);
    break;
  case PUSH6:
    push_n(6);
    break;
  case PUSH7:
    push_n(7);
    break;
  case PUSH8:
    push_n(8);
    break;
  case DUP: {
    // debug("DUP\n");
    ensure_stack(1);
    uint64_t *idx = &vm->stack.data[vm->stack.len - 1];
    uint64_t stack_idx;
    try_add(&stack_idx, *idx, 1);
    // debug("idx: %lu\n", *idx);
    ensure_stack(stack_idx);
    *idx = vm->stack.data[(vm->stack.len - 1) - stack_idx];
    break;
  };
  case DUP0: {
    // debug("DUP0\n");
    ensure_stack(1);
    try(push_stack(&vm->stack, vm->stack.data[vm->stack.len - 1]));
    break;
  };
  case SWAP: {
    // debug("SWAP\n");
    uint64_t idx = 0;
    try(pop_stack(&vm->stack, &idx));
    try_add(&idx, idx, 1);
    size_t len = vm->stack.len;
    if (unlikely(len < idx)) {
      bail(VM_ERR_INVALID_STACK_IDX);
    }
    size_t a_idx = len - 1;
    size_t b_idx = a_idx - idx;
    uint64_t a = vm->stack.data[a_idx];
    vm->stack.data[a_idx] = vm->stack.data[b_idx];
    vm->stack.data[b_idx] = a;
    break;
  };
  case SWAP0: {
    // debug("SWAP0\n");
    ensure_stack(2);
    uint64_t a = vm->stack.data[vm->stack.len - 1];
    vm->stack.data[vm->stack.len - 1] = vm->stack.data[vm->stack.len - 2];
    vm->stack.data[vm->stack.len - 2] = a;
    break;
  };
  case POP: {
    // debug("POP\n");
    ensure_stack(1);
    vm->stack.len--;
    break;
  };
  case ALLOC: {
    // debug("ALLOC\n");
    uint64_t value = 0;
    try(pop_stack(&vm->stack, &value));
    // debug("%lu\n", value);
    try(alloc_memory(&vm->memory, value));
    break;
  };

  case WRITE1:
    write_n(1);
    break;
  case WRITE2:
    write_n(2);
    break;
  case WRITE3:
    write_n(3);
    break;
  case WRITE4:
    write_n(4);
    break;
  case WRITE5:
    write_n(5);
    break;
  case WRITE6:
    write_n(6);
    break;
  case WRITE7:
    write_n(7);
    break;
  case WRITE8:
    write_n(8);
    break;

  case READ1:
    read_n(1);
    break;
  case READ2:
    read_n(2);
    break;
  case READ3:
    read_n(3);
    break;
  case READ4:
    read_n(4);
    break;
  case READ5:
    read_n(5);
    break;
  case READ6:
    read_n(6);
    break;
  case READ7:
    read_n(7);
    break;
  case READ8:
    read_n(8);
    break;

  case DREAD1:
    dread_n(1);
    break;
  case DREAD2:
    dread_n(2);
    break;
  case DREAD3:
    dread_n(3);
    break;
  case DREAD4:
    dread_n(4);
    break;
  case DREAD5:
    dread_n(5);
    break;
  case DREAD6:
    dread_n(6);
    break;
  case DREAD7:
    dread_n(7);
    break;
  case DREAD8:
    dread_n(8);
    break;

  case DCOPY: {
    // debug("DCOPY\n");
    if (unlikely(vm->stack.len < 3)) {
      bail(VM_ERR_STACK_EMPTY);
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
    try(push_stack(&vm->stack, vm->data.len));
    break;
  };
  case ADD:
    binop(op_add);
  case SUB:
    binop(op_sub);
  case MUL:
    binop(op_mul);
  case DIV: {
    size_t len = vm->stack.len;
    if (unlikely(len < 2)) {
      bail(VM_ERR_STACK_EMPTY);
    }
    uint64_t lhs = vm->stack.data[len - 2];
    uint64_t rhs = vm->stack.data[len - 1];
    vm->stack.len -= 1;
    if (unlikely(rhs == 0)) {
      bail(VM_ERR_DIVIDE_BY_ZERO);
    }
    vm->stack.data[len - 2] = op_div(lhs, rhs);
    break;
  };
  case EXP:
    binop(op_expmod);
  case MOD: {
    size_t len = vm->stack.len;
    if (unlikely(len < 2)) {
      bail(VM_ERR_STACK_EMPTY);
    }
    uint64_t lhs = vm->stack.data[len - 2];
    uint64_t rhs = vm->stack.data[len - 1];
    vm->stack.len -= 1;
    if (unlikely(rhs == 0)) {
      bail(VM_ERR_DIVIDE_BY_ZERO);
    }
    vm->stack.data[len - 2] = op_mod(lhs, rhs);
    break;
  };
  case EQ:
    binop(op_eq);
  case NEQ:
    binop(op_neq);
  case LT:
    binop(op_lt);
  case GT:
    binop(op_gt);
  case NOT: {
    // debug("NOT\n");
    ensure_stack(1);
    uint64_t *a = &vm->stack.data[vm->stack.len - 1];
    *a = op_not(*a);
    break;
  };
  case SHL:
    binop(op_shl);
  case SHR:
    binop(op_shr);
  case NEG: {
    // debug("NEG\n");
    ensure_stack(1);
    uint64_t *a = &vm->stack.data[vm->stack.len - 1];
    *a = op_neg(*a);
    break;
  };
  case OR:
    binop(op_or);
  case XOR:
    binop(op_xor);
  case AND:
    binop(op_and);
  case JUMP: {
    // debug("JUMP\n");
    try(pop_stack(&vm->stack, &vm->pc));
    break;
  };
  case JNZ: {
    // debug("JNZ\n");

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
    // debug("CALL\n");

    ensure_stack(1);

    uint64_t *top = &vm->stack.data[vm->stack.len - 1];
    uint64_t address = *top;
    *top = vm->pc;
    vm->pc = address;
    break;
  };
  case EXIT: {
    // debug("EXIT\n");
    ensure_stack(2);

    // debug("memory: ");
    // for (int i; i < vm->memory.size; i++) {
    //   // debug("%02x", vm->memory.data[i]);
    // }
    // debug("\n");

    uint64_t len = vm->stack.data[vm->stack.len - 1];
    uint64_t ptr = vm->stack.data[vm->stack.len - 2];
    vm->stack.len -= 2;

    vm->out.exit = new_fat(vm->memory.data + ptr, len);
    return VM_STEP_RESULT_EXIT;
  };
  case TRAP: {
    debug("TRAP\n");
    ensure_stack(1);
    vm->stack.len--;
    vm->out.trap = vm->stack.data[vm->stack.len];
    return VM_STEP_RESULT_TRAP;
  };
  default:
    debug("unknown op %02x\n", op);
    bail(VM_ERR_UNKNOWN_OP);
  }

  return VM_OK;
}

VmResult run_vm(Vm *vm) {
  int cycles = 0;
  while (likely(vm->pc < vm->code.len)) {
    // debug("\n[%d] ", cycles);
    VmResult res = step_vm(vm);
    cycles++;

    // switch (__builtin_expect(res.tag, STEP_RESULT_STEPPED)) {
    if (res) {
      return res;
    } else {
      continue;
    }
  }

  return VM_OK;
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
      .out = {0},
  };
}

Fat new_fat(uint8_t const *ptr, size_t len) {
  return (Fat){.len = len, .ptr = ptr};
}

// ENTRYPOINT

int readFile(char *path, uint8_t **out, size_t *size) {
  FILE *infile;

  infile = fopen(path, "r");

  if (infile == NULL)
    return 1;

  fseek(infile, 0L, SEEK_END);
  *size = ftell(infile);
  // debug("file size: %lu\n", *size);

  fseek(infile, 0L, SEEK_SET);

  *out = (uint8_t *)calloc(*size, sizeof(uint8_t));

  if (out == NULL)
    return 1;

  unsigned long res = fread(*out, sizeof(char), *size, infile);
  if (res != *size) {
    // debug("only read %lu/%zu bytes\n", res, *size);
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
    // debug("unable to read code: %d\n", code_res);
    return code_res;
  }

  // debug("code size: %zu\n", code_len);

  uint8_t *data;
  size_t data_len;
  int data_res = readFile(argv[2], &data, &data_len);
  if (data_res != 0) {
    // debug("unable to read data: %d\n", data_res);
    return data_res;
  }

  // debug("data size: %zu\n", data_len);

  Vm vm = new_vm(new_fat(code, code_len), new_fat(data, data_len));

  VmResult res = run_vm(&vm);

  switch (__builtin_expect(res, VM_OK)) {
  case VM_OK: {
    fprintf(stdout, "done\n");
    break;
  }
  case VM_STEP_RESULT_EOF: {
    fprintf(stdout, "eof\n");
    break;
  }
  case VM_STEP_RESULT_TRAP: {
    fprintf(stdout, "trap %zu\n", vm.out.trap);
    break;
  }
  case VM_STEP_RESULT_EXIT: {
    fprintf(stdout, "exit ");
    for (size_t i = 0; i < vm.out.exit.len; i++) {
      fprintf(stdout, "%02x", vm.out.exit.ptr[i]);
    }
    fprintf(stdout, "\n");
    break;
  }
  default:
    fprintf(stdout, "error: %d", res);
    break;
  }
}
