#include <iostream>
#include <string>
#include <unordered_map>
using namespace std;
int main (void) {
    
    int n;
    int counter = 1;
    cin >> n;
    unordered_map<string, int> m;


    for (auto i = 0; i < n; i++) {
        string uni;
        string team; 
        cin >> uni;
        cin >> team;


        m.insert({uni, 0});
        
        m[uni] += 1;

        if (m[uni] == 1) {
            cout << uni << " " << team << "\n";
            counter++;
        }

        if (counter >= 13) {
            break;
        }


    }
    return 0;

}
