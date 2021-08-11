#include <iostream>
#include <vector>
#include <cstdlib>
using namespace std;
int main (void) {
    int N;
    cin >> N;
    vector<int> xs;
    int in;
    int counter = 0;
    for (auto i = 0; i < N; i++) {
        cin >> in; 
        xs.push_back(in);
    }


    for  (auto i : xs) {
        if (i != abs(i)) {
            counter++;
        }
    }
    cout << counter;
    return 0;
}
