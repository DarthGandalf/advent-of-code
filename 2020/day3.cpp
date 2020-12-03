#include <iostream>
#include <range/v3/algorithm/count_if.hpp>
#include <range/v3/all.hpp>
#include <range/v3/numeric/iota.hpp>
#include <range/v3/view/transform.hpp>

#include "common.h"

namespace aoc2020 {
namespace {

struct Map {
	std::vector<std::vector<bool>> m_rows;

	bool tree(int row, int column) const {
		if (row < 0 || column < 0 || row >= m_rows.size()) {
			return false;
		}
		auto& rrow = m_rows[row];
		int ccolumn = column % rrow.size();
		return rrow[ccolumn];
	}

	int height() const { return m_rows.size(); }
};

struct Solver : AbstractSolver {
	Solver() {
		if (visual_enabled()) m_vis.emplace(512, 512);
	}

	bool supports_visual() const override {
		return true;
	}

	mutable std::optional<Visualizer> m_vis;
	Map m_map;

	void parse(std::string_view input) override {
		m_map.m_rows = input | ranges::views::split('\n') | to_string_view() |
					   ranges::views::transform([](std::string_view line) {
						   return line | ranges::views::transform([](char c) {
									  return c == '#';
								  }) |
								  ranges::to<std::vector<bool>>();
					   }) |
					   ranges::to<std::vector<std::vector<bool>>>();
	}

	void part1(std::ostream& ostr) const override {
		ostr << ranges::count_if(
			ranges::views::iota(0, m_map.height()),
			[&](int row) { return m_map.tree(row, row * 3); });
	}

	std::int64_t trees(int xoff, int yoff) const {
		draw(xoff, yoff);
		return ranges::count_if(
			ranges::views::iota(0, m_map.height()) |
				ranges::views::stride(yoff),
			[&](int row) { return m_map.tree(row, row * xoff / yoff); });
	}

	void part2(std::ostream& ostr) const override {
		ostr << trees(1, 1) * trees(3, 1) * trees(5, 1) * trees(7, 1) *
					trees(1, 2);
	}

	void draw(int xoff, int yoff) const {
		if (!visual_enabled()) return;

		sdl::Texture pine(m_vis->m_renderer.get(),
		                  open_sprite("pinetree").get());
		sdl::Texture pine2(m_vis->m_renderer.get(),
		                   open_sprite("pinetree-dead").get());
		sdl::Texture crash(m_vis->m_renderer.get(), open_sprite("crash").get());
		sdl::Texture toboggan(m_vis->m_renderer.get(),
		                      open_sprite("toboggan").get());
		m_vis->m_renderer.setDrawColor(255, 255, 255, 255);
		int current_x = 0;
		int current_y = 0;
		while (current_y < m_map.height()) {
			current_x += xoff;
			current_y += yoff;
			for (int i = 0; i < 4; ++i) {
				float subx = i * xoff / 4.0f;
				float suby = i * yoff / 4.0f;
				m_vis->m_renderer.clear();
				SDL_Rect center;
				center.h = 13;
				center.w = 12;
				center.x = 200;
				center.y = 200;
				for (int y :
				     ranges::views::iota(current_y - 20, current_y + 30)) {
					for (int x :
					     ranges::views::iota(current_x - 20, current_x + 30)) {
						if (!m_map.tree(y, x)) continue;
						SDL_Rect dest;
						dest.h = 13;
						dest.w = 12;
						dest.x = center.x + (x - current_x - subx) * 12;
						dest.y = center.y + (y - current_y - suby) * 13;
						if (x < current_x && y < current_y &&
						    (current_x - x) * yoff == (current_y - y) * xoff) {
							m_vis->m_renderer.copy(pine2.get(), nullptr, &dest);
						} else {
							m_vis->m_renderer.copy(pine.get(), nullptr, &dest);
						}
					}
				}
				m_vis->m_renderer.copy(
					m_map.tree(current_y, current_x) && i == 3 ? crash.get()
															   : toboggan.get(),
					nullptr, &center);
				m_vis->m_renderer.present();
				yield(visual_delay());
			}
			if (m_map.tree(current_y, current_x)) {
				yield(visual_delay() * 4);
			}
			if (!visual_enabled()) return;
		}
	}
};
}  // namespace

std::unique_ptr<AbstractSolver> AbstractSolver::Create() {
	return std::make_unique<Solver>();
}
}  // namespace aoc2020
