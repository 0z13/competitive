#include <iostream>
#include <vector>
#include <bits/stdc++.h>
using namespace std;
int main (void) {
  long x;
  long y;
  cin >> y;
  vector<long> xs;
  for (int i = 0; i < (y -1 ); i++) {
    cin >> x;
    xs.push_back(x);
  }
  sort(xs.begin(), xs.end());

  for (int i = 1; i <= y; i++ ) {
    if (i != xs[(i- 1)] || i > xs.size()) {
      cout << i;
      break;
    }
  }
}
