//#include <stdio.h>

void reverse_array(int* arr, int n) {
  for (int i = 0; i < (n >> 1); i++) {
    int temp = arr[i];
    arr[i] = arr[n - 1 - i];
    arr[n - 1 - i] = temp;
  }
}
