#include <iostream>

using namespace std;
int main (void) {
    int x, y;
    cin >> x >> y;

    if (x < y) {
        cout << x << " " << y << endl;
        return 0;
    }

    cout << y << " " << x << endl;
    return 0;
}
