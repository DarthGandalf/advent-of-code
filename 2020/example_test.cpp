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
	solver->parse(R"(35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576)");
	solver->part2(std::cout);
}
