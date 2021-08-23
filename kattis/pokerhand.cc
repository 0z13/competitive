#include <iostream>
#include <vector>
#include <unordered_map>
using namespace std;
int main (void) {
    string a, b, c, d, e;
    cin >> a >> b >> c >> d >> e;
    vector<string> xs = {a,b,c,d,e};
    unordered_map<char, int> m;


    for (auto i = 0; i < 5; i++) {
        auto curr = xs[i];
        m.insert({curr.at(0), 0});
        m[curr.at(0)]++;
   }
    int mx = 0; 
    for (const auto&n : m) {
        mx = max(mx, n.second); 
    }
    
    cout << mx << endl;
    return 0;
}
