// 112A - Petya and Strings
//
// Ref: https://codeforces.com/problemset/problem/112/A

#include <bits/stdc++.h>
#include <cstdlib>
using namespace std;

int main() {
  ios::sync_with_stdio(false);
  cin.tie(nullptr);

  bool done = false;
  string x, y;
  cin >> x >> y;
  for (size_t i = 0; i < x.length(); i++) {
    char xc = x[i];
    char yc = y[i];
    if (xc >= 97)
      xc = xc - 32;
    if (yc >= 97)
      yc = yc - 32;

    if (xc < yc) {
      cout << "-1";
      done = true;
      break;
    } else if (xc > yc) {
      cout << "1";
      done = true;
      break;
    }
  }
  if (done == false) {
    cout << "0";
  }
  return 0;
}
