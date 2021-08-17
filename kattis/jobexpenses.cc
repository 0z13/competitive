#include <iostream>
#include <algorithm>

using namespace std;
int main (void) {
    int N;
    cin >> N;
    int sum = 0;
    for (int i = 0; i < N; i++) {
        int n;
        cin >> n;

        if (abs(n) != n) {
            sum += abs(n);
        }
    }
    cout << sum << endl;;
    return 0;
}
