#include <iostream>
#include <string>
#include <algorithm>
#include <cmath>
int main (void) {
     std::string str;
     char x;
     std::cin >> str;
     int T = 0;
     int C = 0;
     int G = 0;
     for (int i = 0; i < str.size(); i++) {
        char x = str.at(i);
        if (x == 'T') 
            T++;
        else if (x == 'C') 
            C++;
        else 
            G++;
     }
     
    int min_tmp = std::min(T, C);
    int min = std::min(G, min_tmp);

    std::cout << pow(T,2)+std::pow(G,2)+std::pow(C,2)+(min*7) << std::endl;
    return 0;
}
