#include <iostream>
using namespace std;

bool hashard(int n) {
   int sum = 0;
   int N = n;
   while (N > 0) {
       sum += N % 10;
       N = N / 10;
   }
   return (n % sum == 0);
}


int main (void) {
   int sum = 0;

   int N;
   cin >> N;
   while(!hashard(N)) {
       N++;
   };
   cout << N << endl;
   return 0;


}
