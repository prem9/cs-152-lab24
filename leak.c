#include <stdlib.h>
#include <stdio.h>

int* zero_out_negatives(int a[], int len) {
  int *res = malloc(sizeof(int) * len);
  //int res[len];
  for (int i=0; i<len; i++) {
    if (a[i] < 0) res[i] = 0;
    else res[i] = a[i];
  }
  return res;
}

int main(int argc, char** argv) {
  int nums[] = {0, 12, 5, -42, 9, 7, -18, 0};
  int len = 8;
  int *no_negs = zero_out_negatives(nums, len);
  for (int i=0; i<len; i++) {
    printf("%d ", no_negs[i]);
  }
  printf("\n");
  // Forgot free(no_negs);
}


