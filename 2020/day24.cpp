#include <compare>
#include <initializer_list>
#include <range/v3/algorithm/count_if.hpp>
#include <range/v3/algorithm/minmax.hpp>
#include <range/v3/all.hpp>
#include <range/v3/range/conversion.hpp>
#include <range/v3/view/cartesian_product.hpp>
#include <range/v3/view/transform.hpp>
#include <set>
#include <string>

#include "common.h"
#include "fmt/core.h"

namespace aoc2020 {
namespace {
using namespace std::literals::string_view_literals;

struct Position {
	int row = 0;
	int col = 0;

	auto operator<=>(const Position& other) const = default;

	bool move(std::string_view dir) {
		if (dir[0] == 'e') {
			col++;
			return false;
		} else if (dir[0] == 'w') {
			col--;
			return false;
		} else if (dir == "se"sv) {
			col++;
			row++;
			return true;
		} else if (dir == "sw"sv) {
			row++;
			return true;
		} else if (dir == "nw"sv) {
			col--;
			row--;
			return true;
		} else if (dir == "ne"sv) {
			row--;
			return true;
		}
		return false;
	}
};

struct Solver : AbstractSolver {
	std::vector<std::string_view> m_input;
	void parse(std::string_view input) override {
		m_input = input | ranges::views::split('\n') | to_string_view() |
		          ranges::to_vector;
	}
	std::set<Position> solve1() const {
		std::set<Position> grid;
		for (auto line : m_input) {
			Position pos;
			bool skip = false;
			for (auto s : line | ranges::views::sliding(2) | to_string_view()) {
				if (skip) {
					skip = false;
					continue;
				}
				skip = pos.move(s);
			}
			if (!skip) {
				pos.move(std::string{line.back()});
			}
			if (grid.contains(pos)) {
				grid.erase(pos);
			} else {
				grid.insert(pos);
			}
		}
		return grid;
	}
	void part1(std::ostream& ostr) const override { ostr << solve1().size(); }
	void part2(std::ostream& ostr) const override {
		std::set<Position> grid = solve1();
		for ([[maybe_unused]] int i : ranges::views::iota(0, 100)) {
			auto [rowmin, rowmax] =
				ranges::minmax(grid | ranges::views::transform(
										  [](const auto& p) { return p.row; }));
			auto [colmin, colmax] =
				ranges::minmax(grid | ranges::views::transform(
										  [](const auto& p) { return p.col; }));
			auto grid2 =
				ranges::views::cartesian_product(
					ranges::views::iota(rowmin - 1, rowmax + 2),
					ranges::views::iota(colmin - 1, colmax + 2)) |
				ranges::views::transform([](const auto& z) {
					auto [row, col] = z;
					return Position{.row = row, .col = col};
				}) |
				ranges::views::filter([&](const Position& pos) {
					static std::array neigh = {
						std::pair{1, 0},  std::pair{1, 1},  std::pair{0, 1},
						std::pair{0, -1}, std::pair{-1, 0}, std::pair{-1, -1}};
					int count = ranges::count_if(
						neigh | ranges::views::transform([&](const auto& p) {
							const auto [drow, dcol] = p;
							return Position{.row = pos.row + drow,
					                        .col = pos.col + dcol};
						}),
						[&](const Position& p) { return grid.contains(p); });
					if (grid.contains(pos)) {
						return !(count == 0 || count > 2);
					} else {
						return count == 2;
					}
				}) |
				ranges::to<std::set<Position>>();
			grid = std::move(grid2);
		}
		ostr << grid.size();
	}
};
}  // namespace

std::unique_ptr<AbstractSolver> AbstractSolver::Create() {
	return std::make_unique<Solver>();
}
}  // namespace aoc2020
