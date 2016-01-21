// https://icpc.baylor.edu/worldfinals/problems/icpc2015.pdf
// Problem 1

#include <iostream>
#include <cmath>
#include <iomanip>
using namespace std;

static double last_max = 0;
static double last_min = 10000;
static double last_diff = 0;
static double total_max = 0;

int main() {
     int p, a, b, c, d, n;
     cin >> p;
     cin >> a;
     cin >> b;
     cin >> c;
     cin >> d;
     cin >> n;

     for (int i = 1; i <= n; i++) {
         double value = p * (sin(a * i + b) + cos(c * i + d) + 2);
         total_max = max(total_max, value);
         double diff = total_max - value;
         if (diff > last_diff) {
             last_min = value;
             last_diff = diff;
         }
     }

     cout << std::setprecision(20) << last_diff;
     cout << "\n";
     return 0;
}
