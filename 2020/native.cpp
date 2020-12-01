#include <fstream>
#include <iostream>
#include <memory>

#include <filesystem>
#include <cmrc/cmrc.hpp>
CMRC_DECLARE(inputs);
#include <SDL2/SDL_image.h>

#include "common.h"

int main(int argc, char* argv[]) {
	aoc2020::Solver solver;

	// Use the correct input. All of them are compiled in.
	std::filesystem::path self(argv[0]);
	auto fs = cmrc::inputs::get_filesystem();
	auto file = fs.open("input/2020/" + self.stem().string() + ".txt");
	std::string_view input(file.begin(), file.end() - file.begin());
	std::cout << "part1: ";
	solver.part1(input);
	std::cout << "\npart2: ";
	solver.part2(input);
	std::cout << '\n';
}

namespace aoc2020 {
	void yield() {}

	sdl::Surface open_sprite(std::string_view filename) {
		auto fs = cmrc::inputs::get_filesystem();
		auto file = fs.open("sprites/" + std::string(filename) + ".png");
		std::string_view data(file.begin(), file.end() - file.begin());
		SDL_Surface* result = IMG_Load_RW(SDL_RWFromConstMem(data.data(), data.length()), 1);
		if (!result) {
			std::ostringstream strm;
			strm << "IMG_Load_RW(): " << IMG_GetError();
			throw sdl::Error(strm.str());
		}
		return sdl::Surface(result);
	}
}
