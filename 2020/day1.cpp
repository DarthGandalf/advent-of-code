#include <algorithm>
#include <cctype>
#include <ios>
#include <iterator>
#include <range/v3/all.hpp>
#include <string>
#include <system_error>

#include "common.h"

namespace aoc2020 {
namespace {
struct Solver : AbstractSolver {
	mutable std::vector<int> m_numbers;

	void parse(std::string_view input) override { m_numbers = ints<int>(input); }

	void part1(std::ostream& ostr) const override {
		auto middle = std::partition(m_numbers.begin(), m_numbers.end(),
		                             [](int i) { return i < 1010; });
		std::span<int> small = make_span(m_numbers.begin(), middle);
		std::span<int> big = make_span(middle, m_numbers.end());
		big |= ranges::actions::sort;
		for (int a : small) {
			if (std::binary_search(big.begin(), big.end(), 2020 - a)) {
				ostr << (a * (2020 - a));
				return;
			}
		}
		ostr << "not found";
	}

	void part2(std::ostream& ostr) const override {
		m_numbers |= ranges::actions::sort;
		for (auto it_1 = m_numbers.begin(); it_1 + 2 != m_numbers.end();
		     ++it_1) {
			for (auto it_2 = it_1 + 1; it_2 + 1 != m_numbers.end(); ++it_2) {
				int another = 2020 - *it_1 - *it_2;
				if (std::binary_search(it_2 + 1, m_numbers.end(), another)) {
					ostr << (*it_1 * *it_2 * another);
					return;
				}
			}
		}
		ostr << "not found";
	}
};
}  // namespace

std::unique_ptr<AbstractSolver> AbstractSolver::Create() {
	return std::make_unique<Solver>();
}
}  // namespace aoc2020
