#include <iostream>
#include <iomanip>
#include <cctype>
#include <string>

int main (void) {
    std::string xs;
    std::cin >> xs;

    int x, y;
    int len;
    int ws = 0;
    int up = 0;
    int lower = 0;
    int symb = 0;

    len = xs.length();

    for (int i = len; i > 0; i--) {
        char tmpChar;
        tmpChar = xs.at(i - 1);
        if (tmpChar == '_') 
            ws++;
        else if (std::isupper(tmpChar)) 
            up++;
        else if (std::islower(tmpChar))
            lower++;
        else 
            symb++;
    }

    std::cout << std::setprecision(8);
    std::cout << (double(ws) / len) << "\n";
    std::cout << (double(lower) / len) << "\n";
    std::cout << (double(up) / len) << "\n";
    std::cout << (double(symb) / len) << "\n";
    return 0;

}
