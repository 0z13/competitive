#include <iostream>

using namespace std;


int f(int x) {
    int snd = x / 10;
    auto third = snd / 10;
    return (x % 10) * 100 + (snd % 10) * 10 + (third % 10) * 1;
}



int main(void) {
    
    int x, y;
    int tmpx, tmpy;
    cin >> x >> y;
    tmpx = x;
    tmpy = y;
    while (tmpx != 0) {
        if (tmpx % 10 > tmpy % 10) {
            cout << f(x) << endl;
            break;
        }
        else if (tmpx % 10 < tmpy % 10) {
            cout << f(y) << endl;;
            break;
        }
        else { 
            tmpx /= 10;
            tmpy /= 10;
        }
    }
    return 0;
}


