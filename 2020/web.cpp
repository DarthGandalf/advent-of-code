#include <SDL2/SDL_image.h>
#include <emscripten.h>
#include <emscripten/html5.h>
#include <emscripten/val.h>
#include <fmt/core.h>

#include <fstream>
#include <iostream>
#include <memory>
#include <sstream>

#include "common.h"

using emscripten::val;

static EM_BOOL run_clicked(int eventType,
                           const EmscriptenMouseEvent* mouseEvent,
                           void* userData) {
	aoc2020::AbstractSolver* solver =
		reinterpret_cast<aoc2020::AbstractSolver*>(userData);
	auto vue = val::global("aocvue");
	vue.call<void>("setInProgress", true);
	vue.call<void>("setOutput1", val(""));
	vue.call<void>("setOutput2", val(""));
	std::string input = vue.call<std::string>("getInput");
	solver->parse(input);
	std::ostringstream str1, str2;
	solver->part1(str1);
	vue.call<void>("setOutput1", str1.str());
	solver->part2(str2);
	vue.call<void>("setOutput2", str2.str());
	vue.call<void>("setInProgress", false);
	if (solver->supports_visual()) {
		vue.call<void>("finishVisual", false);
	}
	return true;
}

int main(int argc, char* argv[]) {
	auto vue = val::global("aocvue");
	vue.call<void>("setLoaded");

	auto solver = aoc2020::AbstractSolver::Create();

	if (solver->supports_visual()) {
		vue.call<void>("supportVisual");
		vue.call<void>("setVisualSpeed", solver->default_visual_speed());
	}
	{
		std::ifstream f("input.txt");
		std::string str(std::istreambuf_iterator<char>{f}, {});
		vue.call<void>("setInput", str);
	}
	vue.call<void>("setOutput1", val(""));
	vue.call<void>("setOutput2", val(""));
	emscripten_set_click_callback("#run", solver.get(), false, &run_clicked);

	// Just prevent solver from being destructed
	emscripten_set_main_loop(
		+[] {}, /* fps = */ 1, /* simulate_infinite_loop = */ 1);
}

namespace aoc2020 {
void yield(std::chrono::milliseconds delay) { emscripten_sleep(delay.count()); }
bool visual_enabled() {
	return val::global("aocvue").call<bool>("visualEnabled");
}
int visual_speed() {
	return val::global("aocvue").call<int>("getVisualSpeed");
}

sdl::Surface open_sprite(std::string_view filename) {
	SDL_Surface* result =
		IMG_Load(("sprites/" + std::string(filename) + ".png").c_str());
	if (!result) {
		throw sdl::Error(fmt::format("IMG_Load(): {}", IMG_GetError()));
	}
	return sdl::Surface(result);
}

std::unique_ptr<TTF_Font, decltype(&TTF_CloseFont)> open_font(int size) {
	TTF_Font* result = TTF_OpenFont("SynchronizerNbpRegular-Zgpz.ttf", size);
	if (!result) {
		throw sdl::Error(fmt::format("TTF_OpenFont(): {}", TTF_GetError()));
	}
	return std::unique_ptr<TTF_Font, decltype(&TTF_CloseFont)>(result,
	                                                           &TTF_CloseFont);
}

}  // namespace aoc2020
