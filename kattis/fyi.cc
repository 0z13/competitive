#include <iostream>

using namespace std;

int main(void) {
    string xs;
    cin >> xs;
    if (xs.substr(0, 3).compare("555") == 0) {
        cout << 1 << endl;
    } else {
        cout << 0 << endl;
    }
    return 0;
}

