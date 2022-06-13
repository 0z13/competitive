#include <iostream>
using namespace std;
int main(void) {
  
  long n;
  cin >> n;
  
  cout <<  n;
  while (n != 1) {
    if (n % 2 == 0 ) {
      n /= 2;
      cout << " " << n;
    } else {
      n *= 3;
      n++;
      cout << " " << n;
    }
  }
}