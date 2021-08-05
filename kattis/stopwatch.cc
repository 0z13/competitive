#include <vector>
#include <iostream>
using namespace std;
int main(void) {
    int n;
    cin >> n;

    if (n % 2 != 0) {
        cout << "still running"  << endl;
        return 0;
    }
    int total = 0;
    vector<int> xs; 
    
    for (int i = 0; i < n; i++) {
       int x; 
       cin >> x;
       xs.push_back(x);
    }



    int prev = xs[0];
    for (auto i = 1; i <= n; i++) {
        if (i % 2 != 0) {
            total += xs[i] - prev;
        } else {
            prev = xs[i];
        }
    }
    cout << total << endl;
    return 0;
}
