#include <vector>
#include <algorithm>
#include <iostream>

using namespace std;
int main (void) {
    vector<int> xs = {1,2,3,4,5};
    
    for (auto i :xs) {
        cout << i;
    }
    cout << endl;

    xs = for_each(begin(xs), end(xs), []
            (auto x) { return x^2; });

    for (auto i :xs) {
        cout << i;
    }
    cout << endl;
}
