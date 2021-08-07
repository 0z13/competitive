#include <iostream>
#include <unordered_map>
#include <string>
using namespace std;
int main (void) {

    int n;
    cin >> n;
    
    for (auto i = 0; i< n; i++) {
        int tst;
        cin >> tst;
        int counter = 0;
        unordered_map<string, int> xs;
        for (auto i = 0; i < tst; i++) {
            string x;
            cin >> x;
            xs.insert({x, 0});
            xs[x]++;
            if (xs[x] ==   1) 
                counter++;
        }
        cout << counter << endl;
    }

}
