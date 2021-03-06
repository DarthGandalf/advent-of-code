#include <fmt/core.h>

#include <charconv>
#include <cmrc/cmrc.hpp>
#include <cstdlib>
#include <filesystem>
#include <fstream>
#include <iostream>
#include <memory>
CMRC_DECLARE(inputs);
#include <SDL2/SDL_image.h>

#include "common.h"

int main(int argc, char* argv[]) {
	auto solver = aoc2020::AbstractSolver::Create();

	// Use the correct input. All of them are compiled in.
	std::filesystem::path self(argv[0]);
	auto fs = cmrc::inputs::get_filesystem();
	auto file = fs.open("input/2020/" + self.stem().string() + ".txt");
	std::string_view input(file.begin(), file.end() - file.begin());
	solver->parse(input);
	std::cout << "part1: ";
	solver->part1(std::cout);
	std::cout << "\npart2: ";
	solver->part2(std::cout);
	std::cout << '\n';
}

namespace aoc2020 {
void yield(std::chrono::milliseconds delay) { SDL_Delay(delay.count()); }

sdl::Surface open_sprite(std::string_view filename) {
	auto fs = cmrc::inputs::get_filesystem();
	auto file = fs.open("sprites/" + std::string(filename) + ".png");
	std::string_view data(file.begin(), file.end() - file.begin());
	SDL_Surface* result =
		IMG_Load_RW(SDL_RWFromConstMem(data.data(), data.length()), 1);
	if (!result) {
		throw sdl::Error(fmt::format("IMG_Load_RW(): {}", IMG_GetError()));
	}
	return sdl::Surface(result);
}

std::unique_ptr<TTF_Font, decltype(&TTF_CloseFont)> open_font(int size) {
	auto fs = cmrc::inputs::get_filesystem();
	auto file = fs.open("SynchronizerNbpRegular-Zgpz.ttf");
	std::string_view data(file.begin(), file.end() - file.begin());
	TTF_Font* result =
		TTF_OpenFontRW(SDL_RWFromConstMem(data.data(), data.length()), 1, size);
	if (!result) {
		throw sdl::Error(fmt::format("TTF_OpenFontRW(): {}", TTF_GetError()));
	}
	return std::unique_ptr<TTF_Font, decltype(&TTF_CloseFont)>(result,
	                                                           &TTF_CloseFont);
}

bool visual_enabled() {
	char* e = std::getenv("VISUAL_SPEED");
	if (!e) return true;
	if (*e == '-') return false;
	return true;
}
int visual_speed() {
	using namespace std::chrono_literals;
	char* e = std::getenv("VISUAL_SPEED");
	if (!e) return 90;
	if (*e == '-') return 0;
	std::string_view s(e);
	int i;
	std::from_chars(s.begin(), s.end(), i);
	return i;
}
std::string get_renderer_param() { return ""; }
}  // namespace aoc2020
