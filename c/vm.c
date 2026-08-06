#include "./vm.h"
#include <stddef.h>
#include <stdint.h>
#include <stdio.h>

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
  while (vm->pc > vm->data.len) {
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
  vm->pc++;
  if (vm->code.len <= vm->pc) {
    // TODO
  }

  switch (vm->code.ptr[vm->pc]) {
  case PUSH0: {
    push_stack(&vm->stack, 0);
  };
  case PUSH1: {
  };
  case PUSH2: {
  };
  case PUSH3: {
  };
  case PUSH4: {
  };
  case PUSH5: {
  };
  case PUSH6: {
  };
  case PUSH7: {
  };
  case PUSH8: {
  };
  case DUP: {
  };
  case DUP0: {
  };
  case SWAP: {
  };
  case SWAP0: {
  };
  case POP: {
  };
  case ALLOC: {
  };
  case WRITE1: {
  };
  case WRITE2: {
  };
  case WRITE3: {
  };
  case WRITE4: {
  };
  case WRITE5: {
  };
  case WRITE6: {
  };
  case WRITE7: {
  };
  case WRITE8: {
  };
  case READ1: {
  };
  case READ2: {
  };
  case READ3: {
  };
  case READ4: {
  };
  case READ5: {
  };
  case READ6: {
  };
  case READ7: {
  };
  case READ8: {
  };
  case DREAD1: {
  };
  case DREAD2: {
  };
  case DREAD3: {
  };
  case DREAD4: {
  };
  case DREAD5: {
  };
  case DREAD6: {
  };
  case DREAD7: {
  };
  case DREAD8: {
  };
  case DCOPY: {
  };
  case DLEN: {
  };
  case ADD: {
  };
  case SUB: {
  };
  case MUL: {
  };
  case DIV: {
  };
  case EXP: {
  };
  case MOD: {
  };
  case EQ: {
  };
  case NEQ: {
  };
  case LT: {
  };
  case GT: {
  };
  case NOT: {
  };
  case SHL: {
  };
  case SHR: {
  };
  case NEG: {
  };
  case OR: {
  };
  case XOR: {
  };
  case AND: {
  };
  case JUMP: {
  };
  case JNZ: {
  };
  case CALL: {
  };
  case EXIT: {
  };
  case TRAP: {
  };
  }

  return (StepResult){.tag = STEP_RESULT_STEPPED};
}
