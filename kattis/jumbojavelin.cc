#include <iostream>
using namespace std;
int main (void) {
    int n;    
    cin >> n;
    int sum = 0;

    for (auto i = 0; i < n; i++) {
        int x;
        cin >> x;
        sum += (x - 1);
    }
    cout << (sum + 1) << endl; 
    return 0;
}
