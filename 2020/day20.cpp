#include <chrono>
#include <cmath>
#include <cstdint>
#include <cstdlib>
#include <functional>
#include <map>
#include <range/v3/action/remove_if.hpp>
#include <range/v3/algorithm/all_of.hpp>
#include <range/v3/algorithm/count_if.hpp>
#include <range/v3/algorithm/find_if.hpp>
#include <range/v3/algorithm/minmax.hpp>
#include <range/v3/all.hpp>
#include <range/v3/numeric/accumulate.hpp>
#include <range/v3/range/conversion.hpp>
#include <range/v3/view/enumerate.hpp>
#include <range/v3/view/filter.hpp>
#include <range/v3/view/split.hpp>
#include <range/v3/view/transform.hpp>
#include <set>
#include <string_view>
#include <unordered_map>
#include <unordered_set>

#include "common.h"
#include "fmt/core.h"
#include "sdlpp.hpp"

namespace aoc2020 {
namespace {
using namespace std::literals::string_view_literals;

struct Tile {
	std::vector<std::vector<bool>> rows;
	std::int16_t top() const {
		return ranges::accumulate(
			rows.front() | ranges::views::enumerate |
				ranges::views::transform([](const auto& index_value) {
					auto [index, value] = index_value;
					return (int)value << index;
				}),
			0);
	}
	std::int16_t bottom() const {
		return ranges::accumulate(
			rows.back() | ranges::views::enumerate |
				ranges::views::transform([](const auto& index_value) {
					auto [index, value] = index_value;
					return (int)value << index;
				}),
			0);
	}
	std::int16_t left() const {
		return ranges::accumulate(
			rows | ranges::views::enumerate |
				ranges::views::transform([](const auto& index_row) {
					const auto& [index, row] = index_row;
					return (int)row.front() << index;
				}),
			0);
	}
	std::int16_t right() const {
		return ranges::accumulate(
			rows | ranges::views::enumerate |
				ranges::views::transform([](const auto& index_row) {
					const auto& [index, row] = index_row;
					return (int)row.back() << index;
				}),
			0);
	}
	template <typename Pred>
	Tile rotation_when(Pred&& pred) const {
		Tile tile = *this;
		for (bool fx : {false, true}) {
			for (bool fy : {false, true}) {
				for (int row : ranges::views::iota(0, (int)rows.size())) {
					int that_row = fx ? (int)rows.size() - 1 - row : row;
					for (int col : ranges::views::iota(0, (int)rows.size())) {
						int that_col = fy ? (int)rows.size() - 1 - col : col;
						tile.rows[that_row][that_col] = rows[row][col];
					}
				}
				if (pred(tile)) {
					return tile;
				}
				tile = *this;
				for (int row : ranges::views::iota(0, (int)rows.size())) {
					int that_row = fx ? (int)rows.size() - 1 - row : row;
					for (int col : ranges::views::iota(0, (int)rows.size())) {
						int that_col = fy ? (int)rows.size() - 1 - col : col;
						tile.rows[that_col][that_row] = rows[row][col];
					}
				}
				if (pred(tile)) {
					return tile;
				}
				tile = *this;
			}
		}
		abort();
	}
};
std::int16_t flip_edge(std::int16_t x) {
	std::int16_t other = 0;
	for ([[maybe_unused]] int i : ranges::views::iota(0, 10)) {
		other <<= 1;
		other |= x & 1;
		x >>= 1;
	}
	return other;
	// fmt::print("{:0>10b} -> {:0>10b}\n", x, other);
}
std::int16_t normalize_edge(std::int16_t x) {
	return std::min(x, flip_edge(x));
}

struct Solver : AbstractSolver {
	Solver() {
		if (visual_enabled()) m_vis.emplace(800, 800);
	}
	bool supports_visual() const override { return true; }
	int default_visual_speed() const override { return 10; }
	virtual Visualizer* visualizer() override { return &*m_vis; }

	mutable std::optional<Visualizer> m_vis;
	std::unordered_map<int, Tile> m_input;
	void parse(std::string_view input) override {
		m_input =
			input | ranges::views::split("\n\n"sv) | to_string_view() |
			ranges::views::transform([](std::string_view tile_str) {
				auto full_range =
					tile_str | ranges::views::split('\n') | to_string_view();
				int name = ints<int>(ranges::front(full_range)).front();
				Tile tile;
				tile.rows =
					full_range | ranges::views::drop(1) |
					ranges::views::transform([](std::string_view line_str) {
						return line_str | ranges::views::transform([](char c) {
								   return c == '#';
							   }) |
				               ranges::to_vector;
					}) |
					ranges::to_vector;
				return std::pair{name, tile};
			}) |
			ranges::to<std::unordered_map<int, Tile>>();
	}
	std::unordered_map<int /* tile */, std::vector<std::int16_t>> find_edges()
		const {
		std::unordered_map<int /* tile */, std::vector<std::int16_t>> edges;
		for (const auto& [name, tile] : m_input) {
			auto& result = edges[name];
			result = {
				tile.top(),
				tile.bottom(),
				tile.left(),
				tile.right(),
			};
			for (auto& x : result) {
				x = normalize_edge(x);
			}
		}
		return edges;
	}
	void part1(std::ostream& ostr) const override {
		std::unordered_map<int /* tile */, std::vector<std::int16_t>> edges =
			find_edges();
		std::unordered_map<std::int16_t, int> counts;
		for (const auto& [tile, ed] : edges) {
			for (auto e : ed) {
				counts[e]++;
			}
		}
		ostr << ranges::accumulate(
			edges | ranges::views::filter([&](const auto& tile_edges) {
				const auto& [tile, ed] = tile_edges;
				return ranges::count_if(ed, [&](std::int16_t e) {
						   return counts.at(e) == 1;
					   }) == 2;
			}),
			std::int64_t{1}, std::multiplies<>(), [](const auto& tile_edges) {
				const auto& [tile, ed] = tile_edges;
				return tile;
			});
	}
	void part2(std::ostream& ostr) const override {
		std::unordered_map<int /* tile */, std::vector<std::int16_t>> edges =
			find_edges();
		std::unordered_map<std::int16_t, std::vector<int /* tile */>> by_edge;
		for (const auto& [tile, ed] : edges) {
			for (auto e : ed) {
				by_edge[e].push_back(tile);
			}
		}
		std::optional<DrawState> draw_state;
		if (visual_enabled()) {
			draw_state.emplace(m_vis->m_renderer, m_input);
		}
		auto [corner_tile, corner_edges] =
			*ranges::find_if(edges, [&](const auto& tile_edges) {
				const auto& [tile, ed] = tile_edges;
				return ranges::count_if(ed, [&](std::int16_t e) {
						   return by_edge.at(e).size() == 1;
					   }) == 2;
			});
		corner_edges |= ranges::actions::remove_if(
			[&](std::int16_t e) { return by_edge.at(e).size() == 1; });
		std::set<std::pair<int /* row */, int /* col */>> image;
		auto add_to_image = [&](const Tile& tile, int row, int col) {
			row *= 8;
			col *= 8;
			for (int r : ranges::views::iota(0, 8)) {
				for (int c : ranges::views::iota(0, 8)) {
					if (tile.rows[r + 1][c + 1]) {
						image.insert(std::pair{row + r, col + c});
					}
				}
			}
		};
		Tile corner_rotated = [&, &corner_edges = corner_edges,
		                       &corner_tile = corner_tile] {
			std::int16_t edge_to_right = corner_edges.front();
			std::int16_t edge_to_right_flipped = flip_edge(edge_to_right);
			std::int16_t edge_to_down = corner_edges.back();
			std::int16_t edge_to_down_flipped = flip_edge(edge_to_down);
			return m_input.at(corner_tile).rotation_when([&](const Tile& t) {
				auto bottom = t.bottom();
				auto right = t.right();
				return (bottom == edge_to_down ||
				        bottom == edge_to_down_flipped) &&
				       (right == edge_to_right ||
				        right == edge_to_right_flipped);
			});
		}();
		auto go_down = [&](Tile current_t, int current_tile, int col) {
			int row = 0;
			while (true) {
				add_to_image(current_t, row, col);
				if (visual_enabled()) {
					draw(*draw_state, current_tile, std::pair{row, col},
					     current_t);
				}
				std::int16_t edge_to_down = current_t.bottom();
				auto& variants = by_edge.at(normalize_edge(edge_to_down));
				auto next_iter = ranges::find_if(
					variants, [&](int tile) { return tile != current_tile; });
				if (next_iter == variants.end()) break;
				int next_tile = *next_iter;
				Tile next_t = m_input.at(next_tile).rotation_when(
					[&](const Tile& t) { return t.top() == edge_to_down; });
				row++;
				current_tile = std::move(next_tile);
				current_t = next_t;
			}
		};
		auto go_right = [&](Tile current_t, int current_tile) {
			int col = 0;
			while (true) {
				go_down(current_t, current_tile, col);
				std::int16_t edge_to_right = current_t.right();
				auto& variants = by_edge.at(normalize_edge(edge_to_right));
				auto next_iter = ranges::find_if(
					variants, [&](int tile) { return tile != current_tile; });
				if (next_iter == variants.end()) break;
				int next_tile = *next_iter;
				Tile next_t = m_input.at(next_tile).rotation_when(
					[&](const Tile& t) { return t.left() == edge_to_right; });
				col++;
				current_tile = std::move(next_tile);
				current_t = next_t;
			}
		};
		go_right(corner_rotated, corner_tile);
		auto [min_row, max_row] =
			ranges::minmax(image | ranges::views::transform([](const auto& p) {
							   return std::get<0>(p);
						   }));
		auto [min_col, max_col] =
			ranges::minmax(image | ranges::views::transform([](const auto& p) {
							   return std::get<1>(p);
						   }));
		Tile image2d;
		image2d.rows =
			ranges::views::iota(min_row, max_row + 1) |
			ranges::views::transform(
				[&, min_col = min_col, max_col = max_col](int row) {
					return ranges::views::iota(min_col, max_col + 1) |
			               ranges::views::transform([&](int col) {
							   return image.contains(std::pair{row, col});
						   }) |
			               ranges::to_vector;
				}) |
			ranges::to_vector;
		std::vector<std::string_view> monster_pic = {
			"                  # "sv,
			"#    ##    ##    ###"sv,
			" #  #  #  #  #  #   "sv,
		};
		std::vector<std::pair<int, int>> monster_shape;
		for (auto [row, line] : monster_pic | ranges::views::enumerate) {
			for (auto [index, c] : line | ranges::views::enumerate) {
				if (c == '#') monster_shape.push_back(std::pair{row, index});
			}
		}
		int monsters = 0;
		image2d.rotation_when([&](const Tile& t) -> bool {
			draw2_prepare(*draw_state, t);
			monsters = 0;
			for (int row = 0; row < t.rows.size() - monster_pic.size() + 1;
			     row++) {
				for (int col = 0;
				     col < t.rows[row].size() - monster_pic[0].size() + 1;
				     col++) {
					bool found_monster =
						ranges::all_of(monster_shape, [&](const auto& p) {
							return t.rows[row + p.first][col + p.second];
						});
					if (found_monster) {
						monsters++;
					}
					if (visual_enabled() &&
					    (found_monster || visual_speed() < std::rand() % 100)) {
						draw2(*draw_state, std::pair{row, col}, found_monster);
					}
				}
			}
			return monsters;
		});
		draw2(*draw_state, std::nullopt, false);
		ostr << (image.size() - monsters * monster_shape.size());
	}

	static sdl::Surface render_text(TTF_Font* font, const std::string& text,
	                                const SDL_Color& color) {
		SDL_Surface* surface =
			TTF_RenderUTF8_Blended(font, text.c_str(), color);
		return sdl::Surface(surface);
	}

	struct DrawState {
		explicit DrawState(sdl::Renderer& r, const auto& input)
			: m_input(input),
			  font(open_font(12)),
			  hashpic(
				  render_text(font.get(), "#", SDL_Color{255, 255, 255, 0})),
			  dotpic(render_text(font.get(), "·", SDL_Color{255, 255, 255, 0})),
			  monster(r.get(), open_sprite("monster").get()),
			  pattern(r.get(), open_sprite("monster-pattern").get()) {
			int counter = 0;
			int side = input.size() / 4;
			for (const auto& [tilenum, data] : input) {
				sdl::Surface pic(0, 100, 100, 32, 0, 0, 0, 0);
				pic.fillRect(nullptr, SDL_MapRGB(pic.get()->format, 0, 0, 255));
				SDL_Rect rect{.x = 0, .y = 0, .w = 10, .h = 10};
				for (const auto& [y, row] :
				     data.rows | ranges::views::enumerate) {
					rect.y = y * 10;
					for (const auto& [x, b] : row | ranges::views::enumerate) {
						rect.x = x * 10 + (b ? 1 : 4);
						(b ? hashpic : dotpic).blit(nullptr, pic.get(), &rect);
					}
				}
				const int width = 5;
				SDL_Rect hedge{.x = 0, .y = 0, .w = 100, .h = width};
				pic.fillRect(&hedge, SDL_MapRGB(pic.get()->format, 0, 0, 0));
				hedge.y = 100 - width;
				pic.fillRect(&hedge, SDL_MapRGB(pic.get()->format, 0, 0, 0));
				SDL_Rect vedge{.x = 0, .y = 0, .w = width, .h = 100};
				pic.fillRect(&vedge, SDL_MapRGB(pic.get()->format, 0, 0, 0));
				vedge.x = 100 - width;
				pic.fillRect(&vedge, SDL_MapRGB(pic.get()->format, 0, 0, 0));
				tiles.emplace(tilenum, sdl::Texture(r.get(), pic.get()));
				// tilepics.emplace(tilenum, std::move(pic));
				float angle = data.bottom() ^ data.top();
				angle -= 500;
				int offset = 50 * (data.left() ^ data.right()) / 2500;
				const int single = 750 / side;
				if (counter < side) {
					initial_location[tilenum] =
						std::tuple{counter * single, offset, angle};
				} else if (counter < 2 * side) {
					initial_location[tilenum] = std::tuple{
						750 - offset, (counter - side) * single, angle};
				} else if (counter < 3 * side) {
					initial_location[tilenum] =
						std::tuple{(counter - 2 * side) * single + 50,
					               750 - offset, angle};
				} else {
					initial_location[tilenum] = std::tuple{
						offset, (counter - 3 * side) * single + 50, angle};
				}
				counter++;
			}
		}
		void background(sdl::Renderer& R) {
			R.setDrawColor(128, 128, 128, 255);
			R.clear();
			SDL_Rect field{
				.x = 95 - 36, .y = 95 - 36, .w = 610 + 72, .h = 610 + 72};
			R.setDrawColor(0, 0, 0, 255);
			R.fillRect(&field);
			R.setDrawColor(128, 128, 128, 255);
			field.x += 5;
			field.y += 5;
			field.w -= 10;
			field.h -= 10;
			R.fillRect(&field);
		}
		const std::unordered_map<int, Tile>& m_input;
		std::unique_ptr<TTF_Font, decltype(&TTF_CloseFont)> font;
		sdl::Surface hashpic, dotpic;
		std::unordered_map<int, std::tuple<int, int, float>> initial_location;
		std::unordered_map<int, sdl::Texture> tiles;
		// std::unordered_map<int, sdl::Surface> tilepics;
		std::unordered_map<int, std::tuple<int, int, float, SDL_RendererFlip>>
			arranged_tiles;
		std::vector<std::pair<int, int>> monsters;
		sdl::Texture monster, pattern;
		std::optional<sdl::Texture> bigmap;
	};
	void draw(DrawState& state, int current_tile, std::pair<int, int> position,
	          const Tile& rotated_tile) const {
		const auto& [new_loc, which] = [&] {
			const Tile& original_tile = m_input.at(current_tile);
			float angle;
			SDL_RendererFlip flip;
			std::int16_t ortop = original_tile.top();
			std::int16_t ortop_flipped = flip_edge(ortop);
			if (ortop == rotated_tile.top()) {
				angle = 0;
				flip = SDL_RendererFlip(0);
			} else if (ortop_flipped == rotated_tile.top()) {
				angle = 0;
				flip = SDL_RendererFlip::SDL_FLIP_HORIZONTAL;
			} else if (ortop == rotated_tile.right()) {
				angle = 90;
				flip = SDL_RendererFlip(0);
			} else if (ortop_flipped == rotated_tile.right()) {
				angle = 90;
				flip = SDL_RendererFlip::SDL_FLIP_HORIZONTAL;
			} else if (ortop == rotated_tile.left()) {
				angle = 270;
				flip = SDL_RendererFlip::SDL_FLIP_HORIZONTAL;
			} else if (ortop_flipped == rotated_tile.left()) {
				angle = 270;
				flip = SDL_RendererFlip(0);
			} else if (ortop == rotated_tile.bottom()) {
				angle = 180;
				flip = SDL_RendererFlip::SDL_FLIP_HORIZONTAL;
			} else if (ortop_flipped == rotated_tile.bottom()) {
				angle = 180;
				flip = SDL_RendererFlip(0);
			} else {
				abort();
			}
			return std::pair{
				state.arranged_tiles
					.insert(std::pair{current_tile,
			                          std::tuple{position.first,
			                                     position.second, angle, flip}})
					.first->second,
				00};
		}();

		auto& R = m_vis->m_renderer;
		const int tile_size = 672 / std::round(std::sqrt(m_input.size()));
		int steps = [&, &new_loc = new_loc]() -> int {
			float speed = visual_speed() + 1;
			auto [startx, starty, startangle] =
				state.initial_location.at(current_tile);
			auto [row, col, endangle, flip] = new_loc;
			int deltax = 100 - 36 + col * tile_size - startx;
			int deltay = 100 - 36 + row * tile_size - starty;
			return std::sqrtf(deltax * deltax + deltay * deltay) / speed;
		}();
		for (int i : ranges::views::iota(0, steps)) {
			if (!visual_enabled()) return;
			state.background(R);

			for (auto& [tile, tex] : state.tiles) {
				if (tile == current_tile) {
				} else if (auto it = state.arranged_tiles.find(tile);
				           it != state.arranged_tiles.end()) {
					auto [row, col, angle, flip] = it->second;
					SDL_Rect rect{.x = 100 - 36 + col * tile_size,
					              .y = 100 - 36 + row * tile_size,
					              .w = tile_size,
					              .h = tile_size};
					SDL_Rect from{.x = 10, .y = 10, .w = 80, .h = 80};
					R.copyEx(tex.get(), &from, &rect, angle, nullptr, flip);
					bool more_to_right =
						row == 0 &&
						(col == position.second ||
					     (col == position.second - 1 && position.first == 0));
					if (more_to_right) {
						if (position.first == 0) {
							R.setDrawColor(255, 0, 0, 255);
						} else {
							R.setDrawColor(0, 0, 0, 255);
						}
						SDL_Rect edge = rect;
						edge.x += tile_size;
						edge.w = 3;
						R.fillRect(&edge);
					}
					bool more_to_down =
						col == position.second && row == position.first - 1;
					if (more_to_down) {
						R.setDrawColor(255, 0, 0, 255);
						SDL_Rect edge = rect;
						edge.y += tile_size;
						edge.h = 3;
						R.fillRect(&edge);
					}
				}
			}
			for (auto& [tile, tex] : state.tiles) {
				if (tile == current_tile) {
				} else if (state.arranged_tiles.contains(tile)) {
				} else {
					auto [x, y, angle] = state.initial_location.at(tile);
					SDL_Rect rect{.x = x, .y = y, .w = 50, .h = 50};
					R.copyEx(tex.get(), nullptr, &rect, angle, nullptr,
					         SDL_RendererFlip(0));
				}
			}
			{
				auto [startx, starty, startangle] =
					state.initial_location.at(current_tile);
				auto [row, col, endangle, flip] = new_loc;
				bool flipping = std::abs(i - steps / 2) < steps / 5;
				SDL_Rect rect{
					.x = startx +
				         i * (100 - 36 + col * tile_size - startx) / steps,
					.y = starty +
				         i * (100 - 36 + row * tile_size - starty) / steps,
					.w = 50,
					.h = flipping ? 50 * std::abs(i - steps / 2) * 5 / steps
				                  : 50};
				R.copyEx(state.tiles.at(current_tile).get(), nullptr, &rect,
				         startangle + i * (endangle - startangle) / steps,
				         nullptr, i < steps / 2 ? SDL_RendererFlip(0) : flip);
			}
			R.present();
			yield(std::chrono::milliseconds{0});
		}
	}
	void draw2_prepare(DrawState& state, const Tile& tile) const {
		sdl::Surface pic(0, 960, 960, 32, 0, 0, 0, 0);
		pic.fillRect(nullptr, SDL_MapRGB(pic.get()->format, 0, 0, 255));
		for (const auto& [y, row] : tile.rows | ranges::views::enumerate) {
			SDL_Rect rect{.x = 0, .y = (int)y * 10, .w = 10, .h = 10};
			for (const auto& [x, b] : row | ranges::views::enumerate) {
				rect.x = x * 10 + (b ? 1 : 4);
				(b ? state.hashpic : state.dotpic)
					.blit(nullptr, pic.get(), &rect);
			}
		}
		state.bigmap.emplace(m_vis->m_renderer.get(), pic.get());
	}
	void draw2(DrawState& state, std::optional<std::pair<int, int>> pos,
	           bool found) const {
		if (found) state.monsters.push_back(*pos);
		auto& R = m_vis->m_renderer;
		state.background(R);
		SDL_Rect rect{.x = 100 - 36, .y = 100 - 36, .w = 672, .h = 672};
		R.copy(state.bigmap->get(), nullptr, &rect);
		for (const auto& [row, col] : state.monsters) {
			SDL_Rect mon{.x = 100 - 36 + 672 * col / 96,
			             .y = 100 - 36 + 672 * row / 96,
			             .w = 672 * 20 / 96,
			             .h = 672 * 3 / 96};
			R.copy(state.monster.get(), nullptr, &mon);
		}
		if (pos.has_value()) {
			SDL_Rect mon{.x = 100 - 36 + 672 * pos->second / 96,
			             .y = 100 - 36 + 672 * pos->first / 96,
			             .w = 672 * 20 / 96,
			             .h = 672 * 3 / 96};
			R.copy(state.pattern.get(), nullptr, &mon);
		}
		R.present();
		yield(std::chrono::milliseconds{1});
	}
};
}  // namespace

std::unique_ptr<AbstractSolver> AbstractSolver::Create() {
	return std::make_unique<Solver>();
}
}  // namespace aoc2020
