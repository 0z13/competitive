#include <iostream>

using namespace std;
int main (void) {
    string x;
    cin >> x;
    int len = x.size();
    int splitter = len / 3;
    string fst = x.substr(0, splitter); 
    string snd = x.substr(splitter, splitter); 
    string thrd = x.substr(splitter*2, splitter); 
    if (fst.compare(snd) == 0 || fst.compare(thrd) == 0)
        cout << fst << endl;
    else 
        cout << snd << endl; 
    return 0;
}
