#include <range/v3/all.hpp>
#include <range/v3/view/transform.hpp>

#include "common.h"

namespace aoc2020 {
namespace {

struct Solver : AbstractSolver {
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
		for (const auto& [letter, num] : m_input) {
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
					for (int i = 0; i < num ; i += 90) {
						int wx2 = wy;
						int wy2 = -wx;
						wx = wx2;
						wy = wy2;
					}
					break;
				case 'R':
					for (int i = 0; i < num ; i += 90) {
						int wx2 = -wy;
						int wy2 = wx;
						wx = wx2;
						wy = wy2;
					}
			}
		}
		ostr << std::abs(x) + std::abs(y);
	}
};
}  // namespace

std::unique_ptr<AbstractSolver> AbstractSolver::Create() {
	return std::make_unique<Solver>();
}
}  // namespace aoc2020

// 817 too low
