#include <stddef.h>

typedef struct MLMatrixFFI {
  double *data;
  unsigned long long len;
} MLMatrixFFI;

struct MLMatrixFFI *process_matrix_ffi(const double *input, unsigned long long len);

void free_matrix_ffi(struct MLMatrixFFI *ptr);
