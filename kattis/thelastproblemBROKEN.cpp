#include <iostream>
#include <string>
using namespace std;
int main (void) {
  cout << "Thank you, ";
  string x = "";
  string y = "";
  while (cin >> x) {
    y += x;
    y += ", ";
  }
  cout << y;
  cout << "and farewell!";
}
