#include <stdio.h>



int perfect_num(int x) {
    int acc = 0;

    for (int i = 1; i < x; i++) {
        if ((x % i) == 0) { 
            acc += i;
        }
    }
    if (acc == x)
        return 1;
    return 0;
}

void perfect_nums(void) {
    
    for (int i = 0; i <= 10000; i++) {
        if (perfect_num(i)) {
            printf("%d\n", i);
        }
    }
}

int main(void) {
    perfect_nums();
}
