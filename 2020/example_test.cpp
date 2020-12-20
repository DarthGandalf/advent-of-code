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
	solver->parse(R"(Tile 2311:
..##.#..#.
##..#.....
#...##..#.
####.#...#
##.##.###.
##...#.###
.#.#.#..##
..#....#..
###...#.#.
..###..###

Tile 1951:
#.##...##.
#.####...#
.....#..##
#...######
.##.#....#
.###.#####
###.##.##.
.###....#.
..#.#..#.#
#...##.#..

Tile 1171:
####...##.
#..##.#..#
##.#..#.#.
.###.####.
..###.####
.##....##.
.#...####.
#.##.####.
####..#...
.....##...

Tile 1427:
###.##.#..
.#..#.##..
.#.##.#..#
#.#.#.##.#
....#...##
...##..##.
...#.#####
.#.####.#.
..#..###.#
..##.#..#.

Tile 1489:
##.#.#....
..##...#..
.##..##...
..#...#...
#####...#.
#..#.#.#.#
...#.#.#..
##.#...##.
..##.##.##
###.##.#..

Tile 2473:
#....####.
#..#.##...
#.##..#...
######.#.#
.#...#.#.#
.#########
.###.#..#.
########.#
##...##.#.
..###.#.#.

Tile 2971:
..#.#....#
#...###...
#.#.###...
##.##..#..
.#####..##
.#..####.#
#..#.#..#.
..####.###
..#.#.###.
...#.#.#.#

Tile 2729:
...#.#.#.#
####.#....
..#.#.....
....#..#.#
.##..##.#.
.#.####...
####.#.#..
##.####...
##..#.##..
#.##...##.

Tile 3079:
#.#.#####.
.#..######
..#.......
######....
####.#..#.
.#...#.##.
#.#####.##
..#.###...
..#.......
..#.###...)");

	solver->part2(std::cout);
}
