 #include<iostream>

using namespace std;

int main (void) {
  int ws, hs, hl, wl;

  cin >> ws >> hs >> hl >> wl;
  

  if (ws >= (hs+2)  && hs >= (wl+2)) 
    cout << "1" << endl;
  else 
    cout << "0" << endl;
}
