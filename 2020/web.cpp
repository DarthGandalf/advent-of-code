#include <fstream>
#include <iostream>
#include <memory>

#include <emscripten.h>
#include <emscripten/html5.h>
#include <emscripten/val.h>
#include <SDL2/SDL_image.h>

#include "common.h"

using emscripten::val;

static std::string get_input() {
	using emscripten::val;
	return val::global("document").call<val>("getElementById", val("input"))["value"].as<std::string>();
}

static EM_BOOL run_clicked(int eventType, const EmscriptenMouseEvent* mouseEvent, void* userData) {
	aoc2020::Solver* solver = reinterpret_cast<aoc2020::Solver*>(userData);
	val::global("document").call<val>("getElementById", val("run")).set("disabled", "disabled");
	solver->part1(get_input());
	solver->part2(get_input());
	val::global("document").call<val>("getElementById", val("run")).set("disabled", "");
	return true;
}

int main(int argc, char* argv[]) {
	aoc2020::Solver solver;

	{
		std::ifstream f("input.txt");
		std::string str(std::istreambuf_iterator<char>{f}, {});
		val::global("document").call<val>("getElementById", val("input")).set("value", str);
	}
	val::global("document").call<val>("getElementById", val("run")).set("disabled", "");
	emscripten_set_click_callback("#run", &solver, false, &run_clicked);

	// Just prevent solver to be destructed
	emscripten_set_main_loop(+[] {}, /* fps = */ 1, /* simulate_infinite_loop = */ 1);
}

namespace aoc2020 {
	void yield() {
		emscripten_sleep(0);
	}

	sdl::Surface open_sprite(std::string_view filename) {
		SDL_Surface* result = IMG_Load(("sprites/" + std::string(filename) + ".png").c_str());
		if (!result) {
			std::ostringstream strm;
			strm << "IMG_Load(): " << IMG_GetError();
			throw sdl::Error(strm.str());
		}
		return sdl::Surface(result);
	}
}
