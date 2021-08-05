#include <vector>
#include <iostream>
using namespace std;
int main(void) {
    int X, N;
    vector<int> xs;
    cin >> X >> N;
    for (int i = 0; i < N; i++) {
       int x;
       cin >> x;
       xs.push_back(x);
    }
    int total = 0;
    for (auto i: xs) {
        total += i; 
    }

    cout << ((X*(N+1))-total) << endl;
    return 0;
}
