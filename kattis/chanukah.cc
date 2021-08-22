#include <iostream>
using namespace std;
int main(void) {
    
    int N;
    cin >> N;

    for (auto i = 0; i < N; i++) {
        int ln, n;
        cin >> ln >> n;
        cout << ln << " " << ((n*n + n*1)/2) + n << endl;
    }
    return 0;
}
