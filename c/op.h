#include <stdint.h>

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
};

inline uint64_t op_add(uint64_t a, uint64_t b) { return a + b; }

inline uint64_t op_sub(uint64_t a, uint64_t b) { return a - b; }

inline uint64_t op_mul(uint64_t a, uint64_t b) { return a * b; }

inline uint64_t op_div(uint64_t a, uint64_t b) { return a / b; }

inline uint64_t op_not(uint64_t a) { return a == 0; }

inline uint64_t op_gt(uint64_t a, uint64_t b) { return a > b; }

inline uint64_t op_lt(uint64_t a, uint64_t b) { return a < b; }

inline uint64_t op_neq(uint64_t a, uint64_t b) { return a != b; }

inline uint64_t op_eq(uint64_t a, uint64_t b) { return a == b; }

inline uint64_t op_mod(uint64_t a, uint64_t b) { return a % b; }

inline uint64_t op_and(uint64_t a, uint64_t b) { return a & b; }

inline uint64_t op_xor(uint64_t a, uint64_t b) { return a ^ b; }

inline uint64_t op_or(uint64_t a, uint64_t b) { return a | b; }

inline uint64_t op_neg(uint64_t a) { return ~a; }

inline uint64_t op_expmod(uint64_t a, uint64_t b) {
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

inline uint64_t op_shr(uint64_t a, uint64_t shift) {
  // return std.math.shr(u64, a, shift);
  return a >> shift;
}

inline uint64_t op_shl(uint64_t a, uint64_t shift) {
  // return std.math.shl(u64, a, shift);
  return a << shift;
}
