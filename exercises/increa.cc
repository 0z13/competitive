#include <bits/stdc++.h> 
using namespace std;

#define ll long long

int main(void) {
  
  int N;
  vector<int> xs;
  cin >> N;  
  for (int i =0; i < N; ++i) {
    int n;
    cin >> n;
    xs.push_back(n);
  }

  long long res =0;
  for (int i = 0; i < xs.size() - 1; i++) {
    if (xs[i+1] < xs[i]) {
      res += (xs[i] - xs[i+1]);
      xs[i+1] = xs[i];
    }
  }
  cout << res << "\n";

}
