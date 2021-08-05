#include <stdio.h>
#include <assert.h>

void sieve(void) {
    int binary_arr[10000];
    int res[100];
    int res_counter = 0;
    // initialize to 1 
    for (int i = 0; i < 10000; i++) {
        binary_arr[i] = 1;
    }
    
    for (int i = 0; i < 10000; i++) {
        if (binary_arr[i] == 1 && res_counter < 100) {
            int prim_num = (binary_arr[i] + 1);
            res[res_counter] = prim_num;
            printf("%d\n", prim_num);
            res_counter++;
            for (int j = prim_num; j < 10000; j=j+prim_num) {
                binary_arr[j] = 0; 
            }
        }
    }

    for (int i = 0; i < 100; i++) 
        printf("%d\n", res[i]);
}


int main(void) {
    sieve();
    return 0;
}
