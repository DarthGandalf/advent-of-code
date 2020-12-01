#include <charconv>
#include "common.h"
#include "sdlpp.hpp"
#include <SDL_ttf.h>
#include <system_error>

namespace aoc2020 {
	Visualizer::Visualizer(int width, int height) :
		m_sdl(SDL_INIT_VIDEO),
		m_window(static_cast<SDL_Window*>(nullptr)),
		m_renderer(static_cast<SDL_Renderer*>(nullptr)),
		m_surface(0, width, height, 32, 0, 0, 0, 0) {
		SDL_Window* window;
		SDL_Renderer* renderer;
		if (SDL_CreateWindowAndRenderer(width, height, 0, &window, &renderer) < 0) {
			std::ostringstream strm;
			strm << "SDL_CreateWindowAndRenderer(): " << SDL_GetError();
			throw sdl::Error(strm.str());
		}
		m_window = sdl::Window(window);
		m_renderer = sdl::Renderer(renderer);
	}

	TtfInit::TtfInit() { TTF_Init(); }
	TtfInit::~TtfInit() { TTF_Quit(); }

	std::vector<int> ints(std::string_view input) {
		std::vector<int> numbers;
		const char* b = input.cbegin();
		const char* const e = input.cend();
		while (true) {
			if (b >= e) break;
			if (!std::isdigit(*b)) {
				++b;
				continue;
			}
			int i;
			auto [next, err] = std::from_chars(b, e, i);
			if (err != std::errc{}) {
				throw std::system_error(std::make_error_code(err));
			}
			b = next;
			numbers.push_back(i);
		}
		return numbers;
	}
}
