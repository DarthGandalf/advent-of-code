#include "common.h"
#include "sdlpp.hpp"
#include <SDL_ttf.h>

namespace aoc2020 {
	void yield() {}

	sdl::Surface open_sprite(std::string_view filename) {
		TTF_Font* font = TTF_OpenFont("/usr/share/fonts/hack/Hack-Regular.ttf", 30);
		SDL_Surface* surface = TTF_RenderText_Shaded(font, std::string(filename).c_str(), SDL_Color{255, 255, 255, 255}, SDL_Color{0, 0, 0, 0});
		TTF_CloseFont(font);
		return sdl::Surface(surface);
	}
}

int main() {
	aoc2020::Solver solver;
	solver.part2(R"(
	1721
979
366
299
675
1456)");
}
