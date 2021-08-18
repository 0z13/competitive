#include <iostream>
using namespace std;

int main (void) {
    int N;
    cin >> N;

    for (auto i = 0; i < N; i++) {
        string x;
        cin >> x;
        cout << x.size() << "\n";
    }
    return 0;
}
