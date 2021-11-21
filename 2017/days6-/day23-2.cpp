#include <stdint.h>
#include <iostream>

void term(int64_t h) {
	std::cout << h << std::endl;
	std::exit(0);
}

bool simple(int64_t n) {
	for (int64_t i = 2; i <= n / 2; ++i) {
		if (n % i == 0) {
			return false;
		}
	}
	return true;
}

int main() {
	int64_t a = 0, b = 0, c = 0, h = 0;
	a = 1;

	b = 99;
	c = b;
	if (a != 0) {
		b *= 100;
		b += 100000;
		c = b;
		c += 17000;
	}

	while (true) {
		/*for (int64_t d = 2; d < b; ++d) {
			for (int64_t e = 2; e < b; ++e) {
				if (d * e == b) {
					f = 0;
				}
			}
		}
		if (f == 0) {
			h += 1;
		}*/
		if (!simple(b)) {
			h += 1;
		}
		if (b == c) {
			term(h);
			return 0;
		}
		b += 17;
	}
}
