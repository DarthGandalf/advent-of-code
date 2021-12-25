#include <iostream>
#include <vector>
#include <stdint.h>

void process_digit(int w, int& z, int aux1, int aux2, int aux3) {
  int x = z % 26;
  z /= aux1;
  x += aux2;

  if (x == w) {
    z += w + aux3;
  } else {
    z *= 26;
  }
  std::cout << z << ' ';

//  z *= (25 * cond + 1);
//  z += (w + aux3) * cond;
}


template <typename F>
int alu(F&& input) {
  int x = 0, y = 0, z = 0, w = 0;
//  process_digit(z, 1, 11, 3);
  w = input();
  x *= 0;
  x += z;
  x %= 26;
  z /= 1; // aux1
  x += 11; // aux2
  x = x == w ? 1 : 0;
  x = x == 0 ? 1 : 0;
  y *= 0;
  y += 25;
  y *= x;
  y += 1;
  z *= y;
  y *= 0;
  y += w;
  y += 3; // aux3
  y *= x;
  z += y;

  //process_digit(z, 1, 14, 7);
  w = input();
  x *= 0;
  x += z;
  x %= 26;
  z /= 1;
  x += 14;
  x = x == w ? 1 : 0;
  x = x == 0 ? 1 : 0;
  y *= 0;
  y += 25;
  y *= x;
  y += 1;
  z *= y;
  y *= 0;
  y += w;
  y += 7;
  y *= x;
  z += y;

  //process_digit(z, 1, 13, 1);
  w = input();
  x *= 0;
  x += z;
  x %= 26;
  z /= 1;
  x += 13;
  x = x == w ? 1 : 0;
  x = x == 0 ? 1 : 0;
  y *= 0;
  y += 25;
  y *= x;
  y += 1;
  z *= y;
  y *= 0;
  y += w;
  y += 1;
  y *= x;
  z += y;

  //process_digit(z, 26, -4, 6);
  w = input();
  x *= 0;
  x += z;
  x %= 26;
  z /= 26;
  x += -4;
  x = x == w ? 1 : 0;
  x = x == 0 ? 1 : 0;
  y *= 0;
  y += 25;
  y *= x;
  y += 1;
  z *= y;
  y *= 0;
  y += w;
  y += 6;
  y *= x;
  z += y;

  //process_digit(z, 1, 11, 14);
  w = input();
  x *= 0;
  x += z;
  x %= 26;
  z /= 1;
  x += 11;
  x = x == w ? 1 : 0;
  x = x == 0 ? 1 : 0;
  y *= 0;
  y += 25;
  y *= x;
  y += 1;
  z *= y;
  y *= 0;
  y += w;
  y += 14;
  y *= x;
  z += y;

  //process_digit(z, 1, 10, 7);
  w = input();
  x *= 0;
  x += z;
  x %= 26;
  z /= 1;
  x += 10;
  x = x == w ? 1 : 0;
  x = x == 0 ? 1 : 0;
  y *= 0;
  y += 25;
  y *= x;
  y += 1;
  z *= y;
  y *= 0;
  y += w;
  y += 7;
  y *= x;
  z += y;

  //process_digit(z, 26, -4, 9);
  w = input();
  x *= 0;
  x += z;
  x %= 26;
  z /= 26;
  x += -4;
  x = x == w ? 1 : 0;
  x = x == 0 ? 1 : 0;
  y *= 0;
  y += 25;
  y *= x;
  y += 1;
  z *= y;
  y *= 0;
  y += w;
  y += 9;
  y *= x;
  z += y;

  //process_digit(z, 26, -12, 9);
  w = input();
  x *= 0;
  x += z;
  x %= 26;
  z /= 26;
  x += -12;
  x = x == w ? 1 : 0;
  x = x == 0 ? 1 : 0;
  y *= 0;
  y += 25;
  y *= x;
  y += 1;
  z *= y;
  y *= 0;
  y += w;
  y += 9;
  y *= x;
  z += y;

  //process_digit(z, 1, 10, 6);
  w = input();
  x *= 0;
  x += z;
  x %= 26;
  z /= 1;
  x += 10;
  x = x == w ? 1 : 0;
  x = x == 0 ? 1 : 0;
  y *= 0;
  y += 25;
  y *= x;
  y += 1;
  z *= y;
  y *= 0;
  y += w;
  y += 6;
  y *= x;
  z += y;

  //process_digit(z, 26, -11, 4);
  w = input();
  x *= 0;
  x += z;
  x %= 26;
  z /= 26;
  x += -11;
  x = x == w ? 1 : 0;
  x = x == 0 ? 1 : 0;
  y *= 0;
  y += 25;
  y *= x;
  y += 1;
  z *= y;
  y *= 0;
  y += w;
  y += 4;
  y *= x;
  z += y;

  //process_digit(z, 1, 12, 0);
  w = input();
  x *= 0;
  x += z;
  x %= 26;
  z /= 1;
  x += 12;
  x = x == w ? 1 : 0;
  x = x == 0 ? 1 : 0;
  y *= 0;
  y += 25;
  y *= x;
  y += 1;
  z *= y;
  y *= 0;
  y += w;
  y += 0;
  y *= x;
  z += y;

  //process_digit(z, 26, -1, 7);
  w = input();
  x *= 0;
  x += z;
  x %= 26;
  z /= 26;
  x += -1;
  x = x == w ? 1 : 0;
  x = x == 0 ? 1 : 0;
  y *= 0;
  y += 25;
  y *= x;
  y += 1;
  z *= y;
  y *= 0;
  y += w;
  y += 7;
  y *= x;
  z += y;

  //process_digit(z, 26, 0, 12);
  w = input();
  x *= 0;
  x += z;
  x %= 26;
  z /= 26;
  x += 0;
  x = x == w ? 1 : 0;
  x = x == 0 ? 1 : 0;
  y *= 0;
  y += 25;
  y *= x;
  y += 1;
  z *= y;
  y *= 0;
  y += w;
  y += 12;
  y *= x;
  z += y;

  //process_digit(z, 26, -11, 1);
  w = input();
  x *= 0;
  x += z;
  x %= 26;
  z /= 26;
  x += -11;
  x = x == w ? 1 : 0;
  x = x == 0 ? 1 : 0;
  y *= 0;
  y += 25;
  y *= x;
  y += 1;
  z *= y;
  y *= 0;
  y += w;
  y += 1;
  y *= x;
  z += y;

  return z;
}

bool test(int64_t number) {
  static std::vector<uint8_t> digits;
  digits.clear();
  while (number > 0) {
    if (number % 10 == 0) return false;
    digits.push_back(number % 10);
    number /= 10;
  }
  if (digits.size() != 14) throw "wrong num of digits";
  auto next = [&] {
    uint8_t result = digits.back();
    digits.pop_back();
    return result;
  };
  int z = alu(next);
/*  process_digit(next(), z, 1, 11, 3);
  process_digit(next(), z, 1, 14, 7);
  process_digit(next(), z, 1, 13, 1);
  process_digit(next(), z, 26, -4, 6);
  process_digit(next(), z, 1, 11, 14);
  process_digit(next(), z, 1, 10, 7);
  process_digit(next(), z, 26, -4, 9);
  process_digit(next(), z, 26, -12, 9);
  process_digit(next(), z, 1, 10, 6);
  process_digit(next(), z, 26, -11, 4);
  process_digit(next(), z, 1, 12, 0);
  process_digit(next(), z, 26, -1, 7);
  process_digit(next(), z, 26, 0, 12);
  process_digit(next(), z, 26, -11, 1);*/
  return z == 0;
}

int main() {
  int steps = 0;
  for (int64_t n = 99999999999999; n >= 11111111111111; --n) {
    if (test(n)) {
      std::cout << "Success! " << n << std::endl;
      return 0;
    }
    steps++;
    steps%=10000000;
    if (steps == 0) {
      std::cout << "progress: " << n << std::endl;
    }
  }
  return 0;
}
