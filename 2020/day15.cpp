#include <range/v3/all.hpp>
#include <range/v3/view/enumerate.hpp>
#include <unordered_map>

#include "common.h"

namespace aoc2020 {
namespace {

struct Solver : AbstractSolver {
	std::vector<int> m_input;
	void parse(std::string_view input) override { m_input = ints<int>(input); }
	int solve(int N) const {
		std::vector<int> prev;
		int last = m_input.back();
		for (auto [index, num] : m_input | ranges::views::enumerate) {
			while (prev.size() <= num) prev.push_back(-1);
			prev[num] = index;
		}
		for (int i : ranges::views::iota((int)m_input.size(), N)) {
			while (prev.size() <= last) prev.push_back(-1);
			int next;
			if (prev[last] == -1) {
				next = 0;
			} else {
				next = i - prev[last] - 1;
			}
			prev[last] = i - 1;
			last = next;
		}
		return last;
	}
	void part1(std::ostream& ostr) const override { ostr << solve(2020); }
	void part2(std::ostream& ostr) const override { ostr << solve(30000000); }
};
}  // namespace

std::unique_ptr<AbstractSolver> AbstractSolver::Create() {
	return std::make_unique<Solver>();
}
}  // namespace aoc2020
