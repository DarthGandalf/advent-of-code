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
	solver->parse(R"(class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12)");
	solver->parse(R"(class: 0-1 or 4-19
row: 0-5 or 8-19
seat: 0-13 or 16-19

your ticket:
11,12,13

nearby tickets:
3,9,18
15,1,5
5,14,9)");
	solver->part2(std::cout);
}
