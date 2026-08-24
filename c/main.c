#include "./vm.c"

int readFile(char *path, uint8_t **out, size_t *size) {
  FILE *infile;

  infile = fopen(path, "r");

  if (infile == NULL) {
    return 1;
  }

  fseek(infile, 0L, SEEK_END);
  *size = ftell(infile);
  // debug("file size: %lu\n", *size);

  fseek(infile, 0L, SEEK_SET);

  *out = (uint8_t *)calloc(*size, sizeof(uint8_t));

  if (out == NULL) {
    fclose(infile);
    return 1;
  }

  unsigned long res = fread(*out, sizeof(char), *size, infile);
  if (res != *size) {
    free(*out);
    fclose(infile);
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
    free(code);
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

  free((void *)vm.code.ptr);
  free((void *)vm.data.ptr);
  free(vm.memory.data);
  free(vm.stack.data);
}
