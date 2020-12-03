#include <fstream>
#include <iostream>
#include <memory>

#include <emscripten.h>
#include <emscripten/html5.h>
#include <emscripten/val.h>
#include <SDL2/SDL_image.h>
#include <sstream>

#include "common.h"

using emscripten::val;

static std::string get_input() {
	return val::global("document").call<val>("getElementById", val("input"))["value"].as<std::string>();
}

static EM_BOOL run_clicked(int eventType, const EmscriptenMouseEvent* mouseEvent, void* userData) {
	aoc2020::AbstractSolver* solver = reinterpret_cast<aoc2020::AbstractSolver*>(userData);
	auto document = val::global("document");
	auto button = document.call<val>("getElementById", val("run"));
	button.set("disabled", "disabled");
	auto out1 = document.call<val>("getElementById", val("output1"));
	out1.set("value", "");
	auto out2 = document.call<val>("getElementById", val("output2"));
	out2.set("value", "");
	auto input = get_input();
	std::ostringstream str1, str2;
	solver->part1(input, str1);
	out1.set("value", str1.str());
	solver->part2(input, str2);
	out2.set("value", str2.str());
	button.set("disabled", "");
	return true;
}

int main(int argc, char* argv[]) {
	auto solver = aoc2020::AbstractSolver::Create();
	auto document = val::global("document");

	{
		std::ifstream f("input.txt");
		std::string str(std::istreambuf_iterator<char>{f}, {});
		document.call<val>("getElementById", val("input")).set("value", str);
	}
	document.call<val>("getElementById", val("output1")).set("value", "");
	document.call<val>("getElementById", val("output2")).set("value", "");
	document.call<val>("getElementById", val("run")).set("disabled", "");
	emscripten_set_click_callback("#run", solver.get(), false, &run_clicked);

	// Just prevent solver from being destructed
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
