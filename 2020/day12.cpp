#include <fmt/core.h>

#include <cmath>
#include <range/v3/all.hpp>
#include <range/v3/view/enumerate.hpp>
#include <range/v3/view/transform.hpp>

#include "common.h"
#include "sdlpp.hpp"

namespace aoc2020 {
namespace {

struct Solver : AbstractSolver {
	Solver() {
		if (visual_enabled()) m_vis.emplace(512, 512);
	}
	bool supports_visual() const override { return true; }
	int default_visual_speed() const override { return 10; }
	virtual Visualizer* visualizer() override { return &*m_vis; }
	mutable std::optional<Visualizer> m_vis;

	std::vector<std::pair<char, int>> m_input;
	void parse(std::string_view input) override {
		m_input = input | ranges::views::split('\n') | to_string_view() |
				  ranges::views::transform([](std::string_view line) {
					  int i;
					  std::from_chars(line.begin() + 1, line.end(), i);
					  return std::pair{line.front(), i};
				  }) |
				  ranges::to<std::vector<std::pair<char, int>>>();
	}
	void part1(std::ostream& ostr) const override {
		int dirx = 1;
		int diry = 0;
		int x = 0;
		int y = 0;
		for (const auto& [letter, num] : m_input) {
			switch (letter) {
				case 'F':
					x += num * dirx;
					y += num * diry;
					break;
				case 'N':
					y -= num;
					break;
				case 'E':
					x += num;
					break;
				case 'W':
					x -= num;
					break;
				case 'S':
					y += num;
					break;
				case 'L':
					for (int i = 0; i < num; i += 90) {
						int diry2 = -dirx;
						int dirx2 = diry;
						dirx = dirx2;
						diry = diry2;
					}
					break;
				case 'R':
					for (int i = 0; i < num; i += 90) {
						int diry2 = dirx;
						int dirx2 = -diry;
						dirx = dirx2;
						diry = diry2;
					}
			}
		}
		ostr << std::abs(x) + std::abs(y);
	}
	void part2(std::ostream& ostr) const override {
		int wx = 10;
		int wy = -1;
		int x = 0;
		int y = 0;
		std::optional<DrawState> state;
		if (visual_enabled()) {
			state.emplace(m_vis->m_renderer, m_input);
		}
		for (const auto& [index, data] : m_input | ranges::views::enumerate) {
			const auto& [letter, num] = data;
			if (visual_enabled()) {
				draw(*state, index, x, y, wx, wy);
			}
			switch (letter) {
				case 'N':
					wy -= num;
					break;
				case 'S':
					wy += num;
					break;
				case 'E':
					wx += num;
					break;
				case 'W':
					wx -= num;
					break;
				case 'F':
					x += wx * num;
					y += wy * num;
					break;
				case 'L':
					for (int i = 0; i < num; i += 90) {
						int wx2 = wy;
						int wy2 = -wx;
						wx = wx2;
						wy = wy2;
						if (i + 90 < num && visual_enabled()) {
							draw(*state, index, x, y, wx, wy);
						}
					}
					break;
				case 'R':
					for (int i = 0; i < num; i += 90) {
						int wx2 = -wy;
						int wy2 = wx;
						wx = wx2;
						wy = wy2;
						if (i + 90 < num && visual_enabled()) {
							draw(*state, index, x, y, wx, wy);
						}
					}
			}
		}
		ostr << std::abs(x) + std::abs(y);
	}

	static sdl::Surface render_text(TTF_Font* font, const std::string& text,
	                                const SDL_Color& color) {
		SDL_Surface* surface =
			TTF_RenderText_Blended(font, text.c_str(), color);
		return sdl::Surface(surface);
	}

	struct DrawState {
		explicit DrawState(sdl::Renderer& r, const auto& input)
			: font(open_font(12)),
			  compass(r.get(), open_sprite("compass").get()),
			  ship(r.get(), open_sprite("ship").get()),
			  Ns(render_text(font.get(), "N", SDL_Color{255, 255, 255, 0})),
			  Nw{Ns.get()->w},
			  Nh{Ns.get()->h},
			  Nt(r.get(), Ns.get()) {
			commands.reserve(input.size());
			commands_size.reserve(input.size());
			for (const auto& [letter, num] : input) {
				sdl::Surface sfc =
					render_text(font.get(), fmt::format("{}{}", letter, num),
				                SDL_Color{255, 255, 255, 0});
				commands.emplace_back(r.get(), sfc.get());
				commands_size.emplace_back(sfc.get()->w, sfc.get()->h);
			}
		}
		std::unique_ptr<TTF_Font, decltype(&TTF_CloseFont)> font;
		sdl::Texture compass;
		sdl::Texture ship;
		sdl::Surface Ns;
		int Nw, Nh;
		sdl::Texture Nt;
		std::vector<std::pair<int, int>> path = {{0, 0}};
		std::vector<sdl::Texture> commands;
		std::vector<std::pair<int, int>> commands_size;
	};

	void draw(DrawState& state, int index, int x, int y, int wx, int wy) const {
		static const float pi_half = std::acosf(0.0);
		const auto& [letter, num] = m_input[index];
		float angle = std::atan2f(wx, -wy);
		int new_wx = wx;
		int new_wy = wy;
		int new_x = x;
		int new_y = y;
		float distance = 0;
		switch (letter) {
			case 'N':
				new_wy -= num;
				break;
			case 'S':
				new_wy += num;
				break;
			case 'E':
				new_wx += num;
				break;
			case 'W':
				new_wx -= num;
				break;
			case 'F':
				state.path.push_back(std::pair{x, y});
				new_x += wx * num;
				new_y += wy * num;
				distance = std::sqrtf(wx * wx + wy * wy) * num;
				break;
		}
		float new_angle = std::atan2f(new_wx, -new_wy);
		switch (letter) {
			case 'L':
				new_angle = angle - pi_half;
				break;
			case 'R':
				new_angle = angle + pi_half;
				break;
		}

		float speed = visual_speed() + 1;
		int steps =
			(int)std::max(std::abs(new_angle - angle) * 100, distance / 3) /
				speed +
			1;
		float step_x = float(new_x - x) / steps;
		float step_y = float(new_y - y) / steps;
		float step_angle = (new_angle - angle) / steps;

		std::vector<SDL_Point> points;
		points.resize(state.path.size() + 1);

		for (int step = 0; step < steps; ++step) {
			if (!visual_enabled()) return;
			float mid_x = x + step * step_x;
			float mid_y = y + step * step_y;
			float mid_angle = angle + step * step_angle;
			float si = std::sinf(mid_angle);
			float co = std::cosf(mid_angle);

			int window_x, window_y;
			m_vis->m_window.getSize(&window_x, &window_y);

			for (int i = 0; i < state.path.size(); ++i) {
				float dx = state.path[i].first - mid_x;
				float dy = state.path[i].second - mid_y;
				dx /= 10;
				dy /= 10;
				points[i].x = window_x / 2 + int(dx * co + dy * si);
				points[i].y = window_y / 2 + int(dy * co - dx * si);
			}

			points.back().x = window_x / 2;
			points.back().y = window_y / 2;

			m_vis->m_renderer.setDrawColor(0, 0, 255, 255);
			m_vis->m_renderer.clear();
			m_vis->m_renderer.setDrawColor(0, 255, 255, 255);
			m_vis->m_renderer.drawLines(points.data(), points.size());

			SDL_Rect ship{.x = window_x / 2 - 16,
			              .y = window_y / 2 - 16,
			              .w = 32,
			              .h = 32};
			m_vis->m_renderer.copy(state.ship.get(), nullptr, &ship);

			const int compass_size = 64;
			const int compass_center_x = window_x - 20 - compass_size / 2;
			const int compass_center_y = window_y - 20 - compass_size / 2;
			SDL_Rect compass{.x = compass_center_x - compass_size / 2,
			                 .y = compass_center_y - compass_size / 2,
			                 .w = 64,
			                 .h = 64};
			m_vis->m_renderer.copyEx(state.compass.get(), nullptr, &compass,
			                         -mid_angle * 90 / pi_half, nullptr,
			                         (SDL_RendererFlip)0);
			SDL_Rect N{.x = int(compass_center_x - 40 * si) - state.Nw / 2,
			           .y = int(compass_center_y - 40 * co) - state.Nh / 2,
			           .w = state.Nw,
			           .h = state.Nh};
			m_vis->m_renderer.copy(state.Nt.get(), nullptr, &N);

			for (int cmd = index - 15; cmd < index + 15; ++cmd) {
				SDL_Rect rect{.x = 10,
				              .y = (cmd - index) * 20 + window_y / 2,
				              .w = state.commands_size[cmd].first,
				              .h = state.commands_size[cmd].second};
				if (cmd < 0) continue;
				if (cmd >= state.commands.size()) continue;
				if (cmd == index) {
					sdl::Surface sfc = render_text(
						state.font.get(), fmt::format("{}{}", letter, num),
						SDL_Color{255, 0, 0, 0});
					sdl::Texture tex(m_vis->m_renderer.get(), sfc.get());
					m_vis->m_renderer.copy(tex.get(), nullptr, &rect);
				} else {
					m_vis->m_renderer.copy(state.commands[cmd].get(), nullptr,
					                       &rect);
				}
			}

			m_vis->m_renderer.present();
			yield(std::chrono::milliseconds(0));
		}
	}
};
}  // namespace

std::unique_ptr<AbstractSolver> AbstractSolver::Create() {
	return std::make_unique<Solver>();
}
}  // namespace aoc2020

// 817 too low
