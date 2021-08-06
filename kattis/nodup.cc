#include <iostream>
#include <unordered_map>
#include <string>

using namespace std;

int main(void) {
    unordered_map<string, int> m;
    string inp;
    while (cin >> inp) {
        m.insert(make_pair(inp, 0));
        m[inp] += 1;
        if (m[inp] > 1) {
            cout << "no" << endl;
            return 0;
        }
    }

    cout << "yes" << endl;
    return 0;
}
