#include <stddef.h>

typedef struct Neighbor2 Neighbor2;

typedef struct Neighbor3 Neighbor3;

typedef struct MLMatrixFFI {
  double *data;
  unsigned long long len;
} MLMatrixFFI;









void slot_matrix(const double *input, unsigned long long len, uint8_t slot);

struct MLMatrixFFI *fetch_matrix(uint8_t slot);

void free_matrix_ffi(struct MLMatrixFFI *ptr);

struct MLMatrixFFI *process_matrix_ffi(const double *input, unsigned long long len);
