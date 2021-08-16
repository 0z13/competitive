#include <iostream>
#include <string>
using namespace std;
int main (void) {

    string x;
    cin >> x;
    int out = 1;
    for (auto ch: x) {
        if (ch == 'A') {
            if (out == 1) {
                out = 2;
            } else if (out == 2) {
                out = 1;
            }
        }
        if (ch == 'B') {
            if (out == 2) {
                out = 3;
            } else if (out == 3) {
                out = 2;
            }
        }
        if (ch == 'C') {
            if (out == 1) {
                out = 3;
            } else if (out == 3) {
                out = 1;
            }
        }
    }
    cout << out << endl;
    return out;
}
