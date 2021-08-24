#include <iostream>
using namespace std;

int main (void) {
    int N;
    cin >> N;

    for (auto i = 0; i < N; i++) {
        int stops;
        cin >> stops;
        double p = 0;
        for (int j=0;j<stops;j++) {
            p = (p + 0.5) * 2;
        }
        cout << (int)p << endl;
        p = 0;
    }
}
