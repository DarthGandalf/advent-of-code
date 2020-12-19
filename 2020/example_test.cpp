#include <SDL_ttf.h>

#include <chrono>
#include <iostream>

#include "common.h"
#include "sdlpp.hpp"

namespace aoc2020 {
void yield(std::chrono::milliseconds delay) {}
bool visual_enabled() { return false; }
std::chrono::milliseconds visual_delay() { return std::chrono::milliseconds(0); }

sdl::Surface open_sprite(std::string_view filename) {
	TTF_Font* font = TTF_OpenFont("/usr/share/fonts/hack/Hack-Regular.ttf", 30);
	SDL_Surface* surface = TTF_RenderText_Shaded(
		font, std::string(filename).c_str(), SDL_Color{255, 255, 255, 255},
		SDL_Color{0, 0, 0, 0});
	TTF_CloseFont(font);
	return sdl::Surface(surface);
}
}  // namespace aoc2020

int main() {
	auto solver = aoc2020::AbstractSolver::Create();
	solver->parse(R"(0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: "a"
5: "b"

ababbb
bababa
abbbab
aaabbb
aaaabbb)");

	solver->parse(R"(42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: "a"
11: 42 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: "b"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1

abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
bbabbbbaabaabba
babbbbaabbbbbabbbbbbaabaaabaaa
aaabbbbbbaaaabaababaabababbabaaabbababababaaa
bbbbbbbaaaabbbbaaabbabaaa
bbbababbbbaaaaaaaabbababaaababaabab
ababaaaaaabaaab
ababaaaaabbbaba
baabbaaaabbaaaababbaababb
abbbbabbbbaaaababbbbbbaaaababb
aaaaabbaabaaaaababaa
aaaabbaaaabbaaa
aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
babaaabbbaaabaababbaabababaaab
aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba)");

	solver->part2(std::cout);
}
