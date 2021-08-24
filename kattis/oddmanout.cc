#include <iostream>
#include <unordered_map>
using namespace std;
int main (void) {
    int N; 
    int counter = 1;
    cin >> N;
    unordered_map<string, int> m;
    for (auto i = 0; i < N; i++) {
        int n;
        cin >> n;
        for (auto j = 0; j < n; j++) {
            string x;
            cin >> x;
            m.insert({x, 0});
            m[x]++;
        }

        for (auto const& ii : m) {
            if (ii.second != 2) {
                cout << "Case #" << counter << ": " << ii.first << endl;
                counter++;
            }
        }
        m.clear();
    }
}
