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
	solver->parse(R"(16
10
15
5
1
11
7
19
6
12
4)");
if (true||false)	solver->parse(R"(28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3)");
	solver->part2(std::cout);
}
