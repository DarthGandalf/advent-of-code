#pragma once

#include <string>
#include <span>
#include <vector>
#include <string_view>
#include <optional>
#include "sdlpp.hpp"

namespace aoc2020 {
	// Call this during long loops from time to time
	void yield();

	struct TtfInit {
		TtfInit();
		~TtfInit();
	};

	struct Visualizer {
		Visualizer(int width, int height);

		TtfInit m_ttf;
		sdl::Init m_sdl;
		sdl::Window m_window;
		sdl::Renderer m_renderer;
		sdl::Surface m_surface;
	};

	class Solver {
	public:
		Solver();

		void part1(std::string_view input);
		void part2(std::string_view input);
	private:
		std::optional<Visualizer> m_vis;
	};

	sdl::Surface open_sprite(std::string_view filename);

	class SurfaceLock {
	public:
		explicit SurfaceLock(sdl::Surface& s) : m_s(s) {
			if (SDL_MUSTLOCK(s.get()))
				s.lock();
		}
		~SurfaceLock() {
			if (SDL_MUSTLOCK(m_s.get()))
				m_s.unlock();
		}
	private:
		sdl::Surface& m_s;
	};

	std::vector<int> ints(std::string_view input);

	template<typename It>
	constexpr auto make_span(It begin, It end) {
		return std::span<std::remove_pointer_t<typename It::pointer>>(&(*begin), std::distance(begin, end));
	}
}
