#include <iostream>

using namespace std;

void helper (int x) {
    
    switch(x) {

        case 0: 
            cout << "Monday\n";
        case 1:
            cout << "Tuesday\n";
        case 2:
            cout << "Wednesday\n";
        case 3:
            cout << "Thursday\n";
        case 4:
            cout << "Friday\n";
        case 5:
            cout << "Saturday\n";
        case 6:
            cout << "Sunday";

    }
}


int main (void) {


    


    for (auto i = 0; i < 365; i++) {
        for (auto j = 0; j < 7; j++) {
            helper(i);
        }
    }
}
