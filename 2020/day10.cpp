#include <range/v3/all.hpp>

#include "common.h"

namespace aoc2020 {
namespace {

struct Solver : AbstractSolver {
	std::vector<int> m_data;
	void parse(std::string_view input) override {
		m_data = ints<int>(input);
		ranges::sort(m_data);
	}
	void part1(std::ostream& ostr) const override {
		std::vector<int> freq = {1, 0, 1};
		for (int diff : m_data | ranges::views::sliding(2) |
		                    ranges::views::transform([](const auto& r) {
								return ranges::back(r) - ranges::front(r);
							})) {
			freq.at(diff - 1)++;
		}
		ostr << freq[0] * freq[2];
	}
	void part2(std::ostream& ostr) const override {
		std::vector<std::int64_t> upto;
		upto.resize(m_data.back() + 3);
		upto[2] = 1;
		for (int a : m_data) {
			upto.at(a + 2) = upto.at(a - 1) + upto[a] + upto[a + 1];
		}
		ostr << upto.back();
	}
};
}  // namespace

std::unique_ptr<AbstractSolver> AbstractSolver::Create() {
	return std::make_unique<Solver>();
}
}  // namespace aoc2020
