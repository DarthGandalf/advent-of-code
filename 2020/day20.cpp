#include <cstdint>
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

#include "common.h"
#include "fmt/core.h"

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
				}
			}
			return monsters;
		});
		ostr << (image.size() - monsters * monster_shape.size());
	}
};
}  // namespace

std::unique_ptr<AbstractSolver> AbstractSolver::Create() {
	return std::make_unique<Solver>();
}
}  // namespace aoc2020
