/* For LLVM, you need some runtime functions to produce ouput.  Use
   these and include them in final compilation with clang. */

#include <stdio.h>

void _print_int(int x) {
  printf("Out: %i\n", x);
}

void _print_float(double x) {
  printf("Out: %lf\n", x);
}

void _print_bool(int x) {
  if (x) {
    printf("Out: true\n");
  } else {
    printf("Out: false\n");
  }
}

void _print_char(char c) {
  printf("%c", c);
  fflush(stdout);
}
