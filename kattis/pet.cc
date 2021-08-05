#include <iostream>
using namespace std;
int main(void) {
    int max = 0;
    int x, y, z, v;
    int res = 0;
    for (int i = 0; i < 5; i++) {
        cin >> x >> y >> z >> v;
        int sum = x + y + z + v;
        if (sum > max) {
            res = i+1;
            max = sum;
        }
    }
    cout << res << " " << max << endl;
    return 0;
}
