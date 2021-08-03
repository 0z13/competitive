#include <stdio.h>
#include <math.h>

unsigned int binary_dsum(unsigned int value) {
    
    int sum = 0;
    
    while (value > 0) {
        sum += value & 1;
        value >>= 1;
    }
    return sum;
}


// Exercise 1.2.1
void funtable(void) {
    double table[] = {0.0, 0.1, 0.2, 0.3};
    printf("sqrt       exp\n");
    for (int i = 0; i < 4; i++) {
        float x = table[i];
        printf("%f   %f\n", sqrt(x), exp(x));
    }
}




int main(void) {
    funtable();
   printf("%d", binary_dsum(3));
}



