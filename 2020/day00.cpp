#include <iostream>
#include <string>
#include <vector>
#include <range/v3/all.hpp>
#include "common.h"

static void drawRandomPixels(sdl::Renderer& renderer, sdl::Surface& surface, sdl::Texture& tex) {
	{
		aoc2020::SurfaceLock lock(surface);

		Uint8* pixels = reinterpret_cast<Uint8*>(surface.get()->pixels);
		
		for (int i=0; i < 200*300*4; i++) {
			char randomByte = rand() % 255;
			pixels[i] = randomByte;
		}
	}

	sdl::Texture screenTexture(renderer.get(), surface.get());

	renderer.clear();
	renderer.copy(screenTexture.get(), nullptr, nullptr);

	SDL_Rect dest;
	dest.h = 30;
	dest.w = 30;
	dest.x = 100;
	dest.y = 100;
	renderer.copy(tex.get(), nullptr, &dest);
	renderer.present();
}

namespace aoc2020 {
	Solver::Solver() {
		m_vis.emplace(300, 200);
	}

	void Solver::part1(std::string_view x) {
		sdl::Surface image = open_sprite("ball");
		sdl::Texture tex(m_vis->m_renderer.get(), image.get());
		std::cout << x << std::endl;
		for (int i = 0; i < 100; ++i) {
			drawRandomPixels(m_vis->m_renderer, m_vis->m_surface, tex);
			yield();
		}
	}
	void Solver::part2(std::string_view x) {
		std::cout << x << std::endl;
	}
}
