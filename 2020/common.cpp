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
	std::string renparam = get_renderer_param();
	if (!renparam.empty()) {
		SDL_SetHint(SDL_HINT_RENDER_DRIVER, renparam.c_str());
	}
	SDL_Window* window;
	SDL_Renderer* renderer;
	if (SDL_CreateWindowAndRenderer(width, height, 0, &window, &renderer) < 0) {
		throw sdl::Error(
			fmt::format("SDL_CreateWindowAndRenderer(): {}", SDL_GetError()));
	}
	SDL_RendererInfo info{};
	int info_res = SDL_GetRendererInfo(renderer, &info);
	fmt::print("Renderer ({}: {}): name={} flags={} max tex {}x{}\n", info_res, SDL_GetError(), info.name, info.flags, info.max_texture_width, info.max_texture_height);
	for (int i = 0; i < info.num_texture_formats; ++i) {
		fmt::print("Texture format {}: {}\n", i, info.texture_formats[i]);
	}
	m_window = sdl::Window(window);
	m_renderer = sdl::Renderer(renderer);
	SDL_EventState(SDL_MOUSEWHEEL, SDL_IGNORE);
	SDL_EventState(SDL_FINGERDOWN, SDL_IGNORE);
}

TtfInit::TtfInit() { TTF_Init(); }
TtfInit::~TtfInit() { TTF_Quit(); }

}  // namespace aoc2020
