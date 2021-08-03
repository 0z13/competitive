#include <stdio.h>


void geom_sum(void) {
    
    double sum = 1;

    for (double i = 2; i < 200; i++) {
        sum += 1/i;
        printf("adding 1/%f, with sum %f\n", i, sum);
    }
}

int main(void) {
    geom_sum();
    return 0;
}
