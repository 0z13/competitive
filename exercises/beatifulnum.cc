#include <bits/stdc++.h>

using namespace std;

int main() {
  int n; 
  cin >> n;
  
  if (n == 1) {
    cout << n;
    return 0; 
  }
  
  if (n==2 || n==3) {
    cout << "NO SOLUTION";
    return 0;
  }
  // 2 4 1 3
  // 2 4 6 1 3 5
  for (int i = 1; i <= n; i++ ) {
    if (i % 2 == 0) {
      cout << i << " ";
    }
  }
  
  for (int i = 1; i <= n; i++ ) {
    if (i % 2 != 0) {
      cout << i << " ";
    }
  }
}
