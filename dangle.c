#include <stdlib.h>
#include <stdio.h>

int* zero_out_negatives(int a[], int len) {
  int *res = malloc(sizeof(int) * len);
  for (int i=0; i<len; i++) {
    if (a[i] < 0) res[i] = 0;
    else res[i] = a[i];
  }
  return res;
}

void print_arr(int a[], int len) {
  for (int i=0; i<len; i++) {
    printf("%d ", a[i]);
  }
  printf("\n");
  free(a);
}

int main(int argc, char** argv) {
  int nums[] = {0, 12, 5, -42, 9, 7, -18, 0};
  int len = 8;
  int *no_negs = zero_out_negatives(nums, len);
  print_arr(no_negs, len);
  printf("The 4th elem was %d but now is %d\n", nums[3], no_negs[3]);
  print_arr(no_negs, len); // double free
}


