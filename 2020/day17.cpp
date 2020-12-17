#include <fmt/core.h>

#include <range/v3/algorithm/minmax.hpp>
#include <range/v3/all.hpp>
#include <range/v3/view/cartesian_product.hpp>
#include <range/v3/view/concat.hpp>
#include <range/v3/view/enumerate.hpp>
#include <range/v3/view/for_each.hpp>
#include <range/v3/view/transform.hpp>
#include <set>
#include <tuple>
#include <unordered_map>
#include <unordered_set>

#include "common.h"

namespace aoc2020 {
namespace {

using State = std::set<std::tuple<int, int, int, int>>;

struct Solver : AbstractSolver {
	State m_input;
	void parse(std::string_view input) override {
		m_input = input | ranges::views::split('\n') |
		          ranges::views::enumerate |
		          ranges::views::for_each([](const auto& y_line_range) {
					  const auto& [y, line_range] = y_line_range;
					  return ranges::yield_from(
						  line_range | ranges::views::enumerate |
						  ranges::views::filter([](const auto& x_c) {
							  auto [x, c] = x_c;
							  return c == '#';
						  }) |
						  ranges::views::transform([y = y](const auto& x_c) {
							  auto [x, c] = x_c;
							  return std::tuple{x, y, 0, 0};
						  }));
				  }) |
		          ranges::to<State>();
	}
	void part1(std::ostream& ostr) const override {
		State state = m_input;
		for (int i : ranges::views::iota(0, 6)) {
			auto [min_x, max_x] = ranges::minmax(
				state | ranges::views::transform(
							[](const auto& t) { return std::get<0>(t); }));
			auto [min_y, max_y] = ranges::minmax(
				state | ranges::views::transform(
							[](const auto& t) { return std::get<1>(t); }));
			auto [min_z, max_z] = ranges::minmax(
				state | ranges::views::transform(
							[](const auto& t) { return std::get<2>(t); }));
			State next = ranges::views::cartesian_product(
							 ranges::views::iota(min_x - 1, max_x + 2),
							 ranges::views::iota(min_y - 1, max_y + 2),
							 ranges::views::iota(min_z - 1, max_z + 2),
							 ranges::yield(0)) |
			             ranges::views::filter([&](const auto& t) {
							 const auto [x, y, z, u] = t;
							 const int neighbours = ranges::count_if(
								 ranges::views::cartesian_product(
									 ranges::views::iota(x - 1, x + 2),
									 ranges::views::iota(y - 1, y + 2),
									 ranges::views::iota(z - 1, z + 2),
									 ranges::yield(0)),
								 [&, x = x, y = y, z = z](const auto& tt) {
									 const auto [xx, yy, zz, uu] = tt;
									 return (x != xx || y != yy || z != zz) &&
					                        state.contains(tt);
								 });
							 if (state.contains(t)) {
								 return neighbours == 2 || neighbours == 3;
							 } else {
								 return neighbours == 3;
							 }
						 }) |
			             ranges::to<State>();
			state = std::move(next);
		}
		ostr << state.size();
	}
	void part2(std::ostream& ostr) const override {
		State state = m_input;
		for (int i : ranges::views::iota(0, 6)) {
			auto [min_x, max_x] = ranges::minmax(
				state | ranges::views::transform(
							[](const auto& t) { return std::get<0>(t); }));
			auto [min_y, max_y] = ranges::minmax(
				state | ranges::views::transform(
							[](const auto& t) { return std::get<1>(t); }));
			auto [min_z, max_z] = ranges::minmax(
				state | ranges::views::transform(
							[](const auto& t) { return std::get<2>(t); }));
			auto [min_u, max_u] = ranges::minmax(
				state | ranges::views::transform(
							[](const auto& t) { return std::get<3>(t); }));
			State next =
				ranges::views::cartesian_product(
					ranges::views::iota(min_x - 1, max_x + 2),
					ranges::views::iota(min_y - 1, max_y + 2),
					ranges::views::iota(min_z - 1, max_z + 2),
					ranges::views::iota(min_u - 1, max_u + 2)) |
				ranges::views::filter([&](const auto& t) {
					const auto [x, y, z, u] = t;
					const int neighbours = ranges::count_if(
						ranges::views::cartesian_product(
							ranges::views::iota(x - 1, x + 2),
							ranges::views::iota(y - 1, y + 2),
							ranges::views::iota(z - 1, z + 2),
							ranges::views::iota(u - 1, u + 2)),
						[&, x = x, y = y, z = z, u = u](const auto& tt) {
							const auto [xx, yy, zz, uu] = tt;
							return (x != xx || y != yy || z != zz || u != uu) &&
					               state.contains(tt);
						});
					if (state.contains(t)) {
						return neighbours == 2 || neighbours == 3;
					} else {
						return neighbours == 3;
					}
				}) |
				ranges::to<State>();
			state = std::move(next);
		}
		ostr << state.size();
	}
};
}  // namespace

std::unique_ptr<AbstractSolver> AbstractSolver::Create() {
	return std::make_unique<Solver>();
}
}  // namespace aoc2020
