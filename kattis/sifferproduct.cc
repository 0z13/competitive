#include <iostream>

using namespace std;

int f(int x) {
    int sum = 1;
    while (x != 0) {
        if (x % 10 != 0)
            sum *= (x % 10);
        x /= 10;
    }
    return sum;
}
int main (void) {
    int x;
    cin >> x;

    while (!(f(x) == x)) {
        x = f(x);
    }
    cout << x << endl;
    return 0;
}
