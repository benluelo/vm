#include <assert.h>
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#include <stdio.h>

#include "./vm.h"

VM_ERR push_stack(Stack *stack, uint64_t value) {
  if (stack->len == stack->capacity) {
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

VM_ERR pop_stack(Stack *stack, uint64_t value) {
  if (stack->len == 0) {
    return VM_ERR_STACK_EMPTY;
  }

  stack->len--;

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

  fseek(infile, 0L, SEEK_SET);

  *out = (uint8_t *)calloc(*size, sizeof(uint8_t));

  if (out == NULL)
    return 1;

  unsigned long res = fread(*out, sizeof(char), *size, infile);
  if (res != *size)
    return 2;

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
    return code_res;
  }

  uint8_t *data;
  size_t data_len;
  int data_res = readFile(argv[1], &data, &data_len);
  if (data_res != 0) {
    return data_res;
  }

  Vm vm = {
      .code = {.ptr = code, .len = code_len},
      .data = {.ptr = data, .len = data_len},
      .pc = 0,
      .memory = {.data = 0, .capacity = 0, .len = 0},
      .stack = {.data = 0, .capacity = 0, .len = 0},
  };

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
    for (int i = 0; i < res.data.exit.len; i++) {
      fprintf(stdout, "%02X", res.data.exit.ptr[i]);
    }
    fprintf(stdout, "\n");
    break;
  }
  }
}

RunResult run_vm(Vm *vm) {
  while (vm->pc < vm->data.len) {
    StepResult res = step_vm(vm);

    switch (res.tag) {
    case STEP_RESULT_STEPPED:
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
  if (vm->code.len <= vm->pc) {
    // TODO
  }

  uint8_t op = vm->code.ptr[vm->pc];

  printf("pc: %d, op: %02x\n", vm->pc, op);

  vm->pc++;

  switch (op) {
  case PUSH0: {
    push_stack(&vm->stack, 0);
    break;
  };
  case PUSH1: {
    printf("PUSH1\n");
    push_stack(&vm->stack, vm->code.ptr[vm->pc]);
    vm->pc += 1;
    break;
  };
  case PUSH2: {
    printf("PUSH2\n");
    assert(false);
    break;
  };
  case PUSH3: {
    printf("PUSH3\n");
    assert(false);
    break;
  };
  case PUSH4: {
    printf("PUSH4\n");
    assert(false);
    break;
  };
  case PUSH5: {
    printf("PUSH5\n");
    assert(false);
    break;
  };
  case PUSH6: {
    printf("PUSH6\n");
    assert(false);
    break;
  };
  case PUSH7: {
    printf("PUSH7\n");
    assert(false);
    break;
  };
  case PUSH8: {
    printf("PUSH8\n");
    assert(false);
    break;
  };
  case DUP: {
    printf("DUP\n");
    assert(false);
    break;
  };
  case DUP0: {
    printf("DUP0\n");
    assert(false);
    break;
  };
  case SWAP: {
    printf("SWAP\n");
    assert(false);
    break;
  };
  case SWAP0: {
    printf("SWAP0\n");
    assert(false);
    break;
  };
  case POP: {
    printf("POP\n");
    assert(false);
    break;
  };
  case ALLOC: {
    printf("ALLOC\n");
    assert(false);
    break;
  };
  case WRITE1: {
    printf("WRITE1\n");
    assert(false);
    break;
  };
  case WRITE2: {
    printf("WRITE2\n");
    assert(false);
    break;
  };
  case WRITE3: {
    printf("WRITE3\n");
    assert(false);
    break;
  };
  case WRITE4: {
    printf("WRITE4\n");
    assert(false);
    break;
  };
  case WRITE5: {
    printf("WRITE5\n");
    assert(false);
    break;
  };
  case WRITE6: {
    printf("WRITE6\n");
    assert(false);
    break;
  };
  case WRITE7: {
    printf("WRITE7\n");
    assert(false);
    break;
  };
  case WRITE8: {
    printf("WRITE8\n");
    assert(false);
    break;
  };
  case READ1: {
    printf("READ1\n");
    assert(false);
    break;
  };
  case READ2: {
    printf("READ2\n");
    assert(false);
    break;
  };
  case READ3: {
    printf("READ3\n");
    assert(false);
    break;
  };
  case READ4: {
    printf("READ4\n");
    assert(false);
    break;
  };
  case READ5: {
    printf("READ5\n");
    assert(false);
    break;
  };
  case READ6: {
    printf("READ6\n");
    assert(false);
    break;
  };
  case READ7: {
    printf("READ7\n");
    assert(false);
    break;
  };
  case READ8: {
    printf("READ8\n");
    assert(false);
    break;
  };
  case DREAD1: {
    printf("DREAD1\n");
    assert(false);
    break;
  };
  case DREAD2: {
    printf("DREAD2\n");
    assert(false);
    break;
  };
  case DREAD3: {
    printf("DREAD3\n");
    assert(false);
    break;
  };
  case DREAD4: {
    printf("DREAD4\n");
    assert(false);
    break;
  };
  case DREAD5: {
    printf("DREAD5\n");
    assert(false);
    break;
  };
  case DREAD6: {
    printf("DREAD6\n");
    assert(false);
    break;
  };
  case DREAD7: {
    printf("DREAD7\n");
    assert(false);
    break;
  };
  case DREAD8: {
    printf("DREAD8\n");
    assert(false);
    break;
  };
  case DCOPY: {
    printf("DCOPY\n");
    assert(false);
    break;
  };
  case DLEN: {
    printf("DLEN\n");
    assert(false);
    break;
  };
  case ADD: {
    printf("ADD\n");
    assert(false);
    break;
  };
  case SUB: {
    printf("SUB\n");
    assert(false);
    break;
  };
  case MUL: {
    printf("MUL\n");
    assert(false);
    break;
  };
  case DIV: {
    printf("DIV\n");
    assert(false);
    break;
  };
  case EXP: {
    printf("EXP\n");
    assert(false);
    break;
  };
  case MOD: {
    printf("MOD\n");
    assert(false);
    break;
  };
  case EQ: {
    printf("EQ\n");
    assert(false);
    break;
  };
  case NEQ: {
    printf("NEQ\n");
    assert(false);
    break;
  };
  case LT: {
    printf("LT\n");
    assert(false);
    break;
  };
  case GT: {
    printf("GT\n");
    assert(false);
    break;
  };
  case NOT: {
    printf("NOT\n");
    assert(false);
    break;
  };
  case SHL: {
    printf("SHL\n");
    assert(false);
    break;
  };
  case SHR: {
    printf("SHR\n");
    assert(false);
    break;
  };
  case NEG: {
    printf("NEG\n");
    assert(false);
    break;
  };
  case OR: {
    printf("OR\n");
    assert(false);
    break;
  };
  case XOR: {
    printf("XOR\n");
    assert(false);
    break;
  };
  case AND: {
    printf("AND\n");
    assert(false);
    break;
  };
  case JUMP: {
    printf("JUMP\n");
    assert(false);
    break;
  };
  case JNZ: {
    printf("JNZ\n");
    assert(false);
    break;
  };
  case CALL: {
    printf("CALL\n");
    assert(false);
    break;
  };
  case EXIT: {
    printf("EXIT\n");
    assert(false);
    break;
  };
  case TRAP: {
    printf("TRAP\n");
    assert(false);
    break;
  };
  default: assert(false);
  }

  return (StepResult){.tag = STEP_RESULT_STEPPED};
}
