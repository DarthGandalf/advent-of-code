#include <algorithm>
#include <range/v3/all.hpp>
#include <range/v3/view/sliding.hpp>
#include <range/v3/view/transform.hpp>
#include <set>

#include "common.h"

namespace aoc2020 {
namespace {

struct Solver : AbstractSolver {
	std::set<int> m_seats;
	void parse(std::string_view input) override {
		m_seats = input | ranges::views::split('\n') | to_string_view() |
				  ranges::views::transform([](std::string_view s) {
					  int x = 0;
					  for (char c : s) {
						  x *= 2;
						  if (c == 'B' || c == 'R') x++;
					  }
					  return x;
				  }) |
				  ranges::to<std::set<int>>();
	}
	void part1(std::ostream& ostr) const override { ostr << *m_seats.rbegin(); }
	void part2(std::ostream& ostr) const override {
		ostr << 1 + *(*(m_seats | ranges::views::sliding(2) |
		                ranges::views::filter([](auto&& r) {
							int a = *r.begin();
							int b = *std::next(r.begin());
							return a + 1 != b;
						})).begin())
						 .begin();
	}
};
}  // namespace

std::unique_ptr<AbstractSolver> AbstractSolver::Create() {
	return std::make_unique<Solver>();
}
}  // namespace aoc2020
