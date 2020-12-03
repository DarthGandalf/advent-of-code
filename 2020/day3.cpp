#include <range/v3/algorithm/count_if.hpp>
#include <range/v3/all.hpp>
#include <range/v3/view/transform.hpp>

#include "common.h"

namespace aoc2020 {
namespace {

struct Map {
	std::vector<std::vector<bool>> m_rows;

	bool tree(int row, int column) {
		auto& rrow = m_rows[row];
		int ccolumn = column % rrow.size();
		return rrow[ccolumn];
	}

	int height() { return m_rows.size(); }
};

struct Solver : AbstractSolver {
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

	void part1(std::ostream& ostr) override {
		ostr << ranges::count_if(
			ranges::views::iota(0, m_map.height()),
			[&](int row) { return m_map.tree(row, row * 3); });
	}

	long int trees(int xoff, int yoff) {
		return ranges::count_if(
			ranges::views::iota(0, m_map.height()) |
				ranges::views::stride(yoff),
			[&](int row) { return m_map.tree(row, row * xoff / yoff); });
	}

	void part2(std::ostream& ostr) override {
		ostr << trees(1, 1) * trees(3, 1) * trees(5, 1) * trees(7, 1) *
					trees(1, 2);
	}
};
}  // namespace

std::unique_ptr<AbstractSolver> AbstractSolver::Create() {
	return std::make_unique<Solver>();
}
}  // namespace aoc2020
