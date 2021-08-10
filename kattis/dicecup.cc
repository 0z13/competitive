#include <iostream>
#include <unordered_map>
#include <vector>
#include <algorithm>

using namespace std;

vector<int> max_in_m(unordered_map<int,int> m) {
    int max = 0;
    vector<int> xs;
    for (const auto& n : m) {
        if (n.second > max) {
            max = n.second;
        }
    }

    for (const auto& n : m) {
        if (n.second == max) {
            xs.push_back(n.first);
        }
    }
    return xs;
}

int main (void) {
    int n, M;
    cin >> n >> M;
    unordered_map<int,int> m;
    int sum = 0;
    for (auto i = 1; i <= n; i++) {
        for (auto j = 1; j <= M; j++) {
            sum = i + j;
            m.insert({sum, 0});
            m[sum]++;
        }
    }

    auto xs = max_in_m(m);
    sort(xs.begin(), xs.end());
    

    for (const auto x: xs) {
        cout << x << "\n";
    }
    return 0;
}
