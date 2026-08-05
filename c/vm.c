#include <errno.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int readFile(char *path) {
  FILE *fp = fopen(path, "rb");
  if (fp == NULL) {
    fprintf(stderr, "cannot open %s: %s\n", path, strerror(errno));
    return 1;
  }

  long size;
  if (fseek(fp, 0, SEEK_END) < 0 || (size = ftell(fp)) < 0) {
    fprintf(stderr, "cannot determine file size %s: %s\n", path,
            strerror(errno));
    // should read the file and reallocate the buffer as needed
    fclose(fp);
    return 1;
  }

  char *dict = malloc(size + 1);
  if (dict == NULL) {
    fprintf(stderr, "cannot allocate memory: %s\n", strerror(errno));
    fclose(fp);
    return 1;
  }
  rewind(fp);
  size_t nread = fread(dict, 1, size, fp);
  if (nread != size) {
    fprintf(stderr, "only read %zu/%zu bytes\n", nread, size);
  }
  dict[nread] = '\0';

  free(dict);
  return 0;
}

int main(int argc, char *argv[]) {
  if (argc < 2) {
    fprintf(stderr, "missing argument\n");
    return 1;
  }

  int res_data = readFile(argv[1]);
  if (res_data != 0) {
    return res_data;
  }

  int res_input = readFile(argv[1]);
  if (res_input != 0) {
    return res_input;
  }
}

typedef struct Stack {
} Stack;
typedef struct Memory {
} Memory;

typedef struct Vm {
  uint8_t *const code;
  int code_len;
  uint8_t *data;
  int data_len;
  Stack stack;
  Memory memory;
  int pc;
} Vm;

typedef enum StepResultTag {
  STEP_RESULT_STEPPED,
  STEP_RESULT_EOF,
  STEP_RESULT_TRAP,
  STEP_RESULT_EXIT,
} StepResultTag;

typedef union StepResultData {
  uint64_t trap;
  uint8_t const *exit;
} StepResultData;

typedef enum RunResultTag {
  RUN_RESULT_DONE,
  RUN_RESULT_EOF,
  RUN_RESULT_TRAP,
  RUN_RESULT_EXIT,
} RunResultTag;

typedef union RunResultData {
  uint64_t trap;
  uint8_t const *exit;
} RunResultData;

typedef struct StepResult {
  StepResultTag tag;
  StepResultData data;
} StepResult;

typedef struct RunResult {
  RunResultTag tag;
  RunResultData data;
} RunResult;

enum Op {
  PUSH0 = 0x00,
  PUSH1 = 0x01,
  PUSH2 = 0x02,
  PUSH3 = 0x03,
  PUSH4 = 0x04,
  PUSH5 = 0x05,
  PUSH6 = 0x06,
  PUSH7 = 0x07,
  PUSH8 = 0x08,
  DUP = 0x09,
  DUP0 = 0x0a,
  SWAP = 0x0b,
  SWAP0 = 0x0c,
  POP = 0x0d,
  ALLOC = 0x20,
  WRITE1 = 0x21,
  WRITE2 = 0x22,
  WRITE3 = 0x23,
  WRITE4 = 0x24,
  WRITE5 = 0x25,
  WRITE6 = 0x26,
  WRITE7 = 0x27,
  WRITE8 = 0x28,
  READ1 = 0x29,
  READ2 = 0x2a,
  READ3 = 0x2b,
  READ4 = 0x2c,
  READ5 = 0x2d,
  READ6 = 0x2e,
  READ7 = 0x2f,
  READ8 = 0x30,
  DREAD1 = 0x31,
  DREAD2 = 0x32,
  DREAD3 = 0x33,
  DREAD4 = 0x34,
  DREAD5 = 0x35,
  DREAD6 = 0x36,
  DREAD7 = 0x37,
  DREAD8 = 0x38,
  DCOPY = 0x39,
  DLEN = 0x3a,
  ADD = 0x40,
  SUB = 0x41,
  MUL = 0x42,
  DIV = 0x43,
  EXP = 0x44,
  MOD = 0x45,
  EQ = 0x4a,
  NEQ = 0x4b,
  LT = 0x4c,
  GT = 0x4d,
  NOT = 0x4e,
  SHL = 0x4f,
  SHR = 0x50,
  NEG = 0x51,
  OR = 0x52,
  XOR = 0x53,
  AND = 0x54,
  JUMP = 0xa0,
  JNZ = 0xa1,
  CALL = 0xa2,
  EXIT = 0xa4,
  TRAP = 0xa5,
} Op;


StepResult step_vm(Vm *vm) {
  vm->pc++;
  if (vm->code_len <= vm->pc) {
    // TODO
  }

  switch (vm->code[vm->pc]) {
    case PUSH0: {};
    case PUSH1: {};
    case PUSH2: {};
    case PUSH3: {};
    case PUSH4: {};
    case PUSH5: {};
    case PUSH6: {};
    case PUSH7: {};
    case PUSH8: {};
    case DUP: {};
    case DUP0: {};
    case SWAP: {};
    case SWAP0: {};
    case POP: {};
    case ALLOC: {};
    case WRITE1: {};
    case WRITE2: {};
    case WRITE3: {};
    case WRITE4: {};
    case WRITE5: {};
    case WRITE6: {};
    case WRITE7: {};
    case WRITE8: {};
    case READ1: {};
    case READ2: {};
    case READ3: {};
    case READ4: {};
    case READ5: {};
    case READ6: {};
    case READ7: {};
    case READ8: {};
    case DREAD1: {};
    case DREAD2: {};
    case DREAD3: {};
    case DREAD4: {};
    case DREAD5: {};
    case DREAD6: {};
    case DREAD7: {};
    case DREAD8: {};
    case DCOPY: {};
    case DLEN: {};
    case ADD: {};
    case SUB: {};
    case MUL: {};
    case DIV: {};
    case EXP: {};
    case MOD: {};
    case EQ: {};
    case NEQ: {};
    case LT: {};
    case GT: {};
    case NOT: {};
    case SHL: {};
    case SHR: {};
    case NEG: {};
    case OR: {};
    case XOR: {};
    case AND: {};
    case JUMP: {};
    case JNZ: {};
    case CALL: {};
    case EXIT: {};
    case TRAP: {};
  }
}

RunResult run_vm(Vm *vm) {
  while (vm->pc > vm->data_len) {
    StepResult res = step_vm(vm);

    switch (res.tag) {
    case STEP_RESULT_STEPPED:
      continue;
    case STEP_RESULT_EOF:
      return (RunResult){
          .tag = RUN_RESULT_EOF,
      };
    case STEP_RESULT_TRAP:
      return (RunResult){.tag = RUN_RESULT_TRAP, .data = res.data.trap};
    case STEP_RESULT_EXIT:
      return (RunResult){.tag = RUN_RESULT_EXIT,
                         .data = (uint8_t const)*res.data.exit};
    }
  }

  return (RunResult){
      .tag = RUN_RESULT_DONE,
  };
}

uint64_t
op_add(uint64_t a, uint64_t b) {
  return a + b;
}

uint64_t op_sub(uint64_t a, uint64_t b) { return a - b; }

uint64_t op_mul(uint64_t a, uint64_t b) { return a * b; }

uint64_t op_div(uint64_t a, uint64_t b) {
  // if (b == 0) {
  //     @branchHint(.cold);
  //     return Error.DivideByZero;
  // } else {
  return a / b;
  // }
}

uint64_t op_not(uint64_t a) { return a == 0; }

uint64_t op_gt(uint64_t a, uint64_t b) { return a > b; }

uint64_t op_lt(uint64_t a, uint64_t b) { return a < b; }

uint64_t op_neq(uint64_t a, uint64_t b) { return a != b; }

uint64_t op_eq(uint64_t a, uint64_t b) { return a == b; }

uint64_t op_mod(uint64_t a, uint64_t b) {
  // if (b == 0) {
  //     @branchHint(.cold);
  //     return Error.DivideByZero;
  // } else {
  return a % b;
  // }
}

uint64_t op_and(uint64_t a, uint64_t b) { return a & b; }

uint64_t op_xor(uint64_t a, uint64_t b) { return a ^ b; }

uint64_t op_or(uint64_t a, uint64_t b) { return a | b; }

uint64_t op_neg(uint64_t a) { return ~a; }

uint64_t op_expmod(uint64_t a, uint64_t b) {
  if (b == 0) {
    return 1;
  }

  uint64_t acc = 1;
  __int128 base = (__int128)a;
  uint64_t exp = b;

  for (;;) {
    if ((exp & 1) == 1) {
      acc = (acc * base) % 0xFFFFFFFFFFFFFFFF;
      // since exp!=0, finally the exp must be 1.
      if (exp == 1) {
        return (uint64_t)acc;
      }
    }
    exp >>= 1;
    base = (base * base) % 0xFFFFFFFFFFFFFFFF;
  }
}

uint64_t op_shr(uint64_t a, uint64_t shift) {
  // return std.math.shr(u64, a, shift);
  return a >> shift;
}

uint64_t op_shl(uint64_t a, uint64_t shift) {
  // return std.math.shl(u64, a, shift);
  return a << shift;
}
