#pragma once

#include <SDL_ttf.h>

#include <charconv>
#include <chrono>
#include <memory>
#include <optional>
#include <ostream>
#include <range/v3/all.hpp>
#include <span>
#include <string>
#include <string_view>
#include <system_error>
#include <vector>

#include "sdlpp.hpp"

namespace aoc2020 {
// Call this during long loops from time to time
void yield(std::chrono::milliseconds delay);

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

class AbstractSolver {
  public:
	static std::unique_ptr<AbstractSolver> Create();
	virtual bool supports_visual() const { return false; }
	virtual int default_visual_speed() const { return 50; }
	virtual ~AbstractSolver() = default;
	virtual void parse(std::string_view input) = 0;
	virtual void part1(std::ostream& ostr) const = 0;
	virtual void part2(std::ostream& ostr) const = 0;
};

sdl::Surface open_sprite(std::string_view filename);
std::unique_ptr<TTF_Font, decltype(&TTF_CloseFont)> open_font(int size);

bool visual_enabled();
int visual_speed();

class SurfaceLock {
  public:
	explicit SurfaceLock(sdl::Surface& s) : m_s(s) {
		if (SDL_MUSTLOCK(s.get())) s.lock();
	}
	~SurfaceLock() {
		if (SDL_MUSTLOCK(m_s.get())) m_s.unlock();
	}

  private:
	sdl::Surface& m_s;
};

template <typename Int>
std::vector<Int> ints(std::string_view input) {
	std::vector<Int> numbers;
	const char* b = input.cbegin();
	const char* const e = input.cend();
	while (true) {
		if (b >= e) break;
		if (!std::isdigit(*b)) {
			++b;
			continue;
		}
		Int i{};
		auto [next, err] = std::from_chars(b, e, i);
		if (err != std::errc{}) {
			throw std::system_error(std::make_error_code(err));
		}
		b = next;
		numbers.push_back(i);
	}
	return numbers;
}

template <typename It>
constexpr auto make_span(It begin, It end) {
	return std::span<std::remove_pointer_t<typename It::pointer>>(
		&(*begin), std::distance(begin, end));
}

inline auto to_string_view() {
	return ranges::views::transform([](auto&& range) {
		return std::string_view(&*range.begin(), ranges::distance(range));
	});
}
}  // namespace aoc2020
