#include <iostream>

using namespace std;
int main (void) {
    int N;
    cin >> N;
    int sum = 0; 
    for (int i = 0; i < N; i++) {
        int x; 
        cin >> x;

        for (int j = 0; j < x; j++) {
            int n; 
            cin >> n;
            sum += n;
        }
        cout << (sum - (x - 1)) << endl;
        sum = 0;
    }
    return 0;
}
