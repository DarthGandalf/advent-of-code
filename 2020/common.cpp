#include "common.h"

#include <SDL_ttf.h>
#include <fmt/core.h>

#include "sdlpp.hpp"

namespace aoc2020 {
Visualizer::Visualizer(int width, int height)
	: m_sdl(SDL_INIT_VIDEO),
	  m_window(static_cast<SDL_Window*>(nullptr)),
	  m_renderer(static_cast<SDL_Renderer*>(nullptr)),
	  m_surface(0, width, height, 32, 0, 0, 0, 0) {
	SDL_SetHint(SDL_HINT_EMSCRIPTEN_KEYBOARD_ELEMENT, "#canvas");
	SDL_Window* window;
	SDL_Renderer* renderer;
	if (SDL_CreateWindowAndRenderer(width, height, 0, &window, &renderer) < 0) {
		throw sdl::Error(
			fmt::format("SDL_CreateWindowAndRenderer(): {}", SDL_GetError()));
	}
	m_window = sdl::Window(window);
	m_renderer = sdl::Renderer(renderer);
}

TtfInit::TtfInit() { TTF_Init(); }
TtfInit::~TtfInit() { TTF_Quit(); }

}  // namespace aoc2020
