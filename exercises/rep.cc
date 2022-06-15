#include <iostream>
#include <bits/stdc++.h>
using namespace std;
#define ll long long
int main(void) {

  string s;
  cin >> s;
  char pri = '0';
  int iter = 0;
  int result = 1;
  for (auto c: s)  {
    if (c == pri) {
      iter++;
      result = max(result, iter);
    } else {
      pri = c;
      iter=1;
    }
  }
  cout << result << '\n';
}
