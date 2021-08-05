#include <iostream>

int main (void) {
    int n;

    std::cin >> n;

    for (auto i = 1; i < n+1; i++) {
        std::cout << i << " Abracadabra\n";
    }
    std::cout << std::endl;
    
    return 0;

}
