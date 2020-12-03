#include <algorithm>
#include <cctype>
#include <ios>
#include <iterator>
#include <string>
#include <system_error>
#include <range/v3/all.hpp>
#include "common.h"

namespace aoc2020 {
	namespace{
	struct Solver : AbstractSolver {
	void part1(std::string_view input, std::ostream& ostr) override {
		std::vector<int> numbers = ints(input);
		auto middle = std::partition(numbers.begin(), numbers.end(), [](int i) { return i < 1010; });
		std::span<int> small = make_span(numbers.begin(), middle);
		std::span<int> big = make_span(middle, numbers.end());
		big |= ranges::actions::sort;
		for (int a : small) {
			if (std::binary_search(big.begin(), big.end(), 2020 - a)) {
				ostr << (a * (2020 -a));
				return;
			}
		}
		ostr << "not found";
	}

	void part2(std::string_view input, std::ostream& ostr) override {
		std::vector<int> numbers = ints(input);
		numbers |= ranges::actions::sort;
		for (auto it_1 = numbers.begin(); it_1 + 2 != numbers.end(); ++it_1) {
			for (auto it_2 = it_1 + 1; it_2 + 1!= numbers.end(); ++it_2) {
				int another = 2020 - *it_1 - *it_2;
				if (std::binary_search(it_2 + 1, numbers.end(), another)) {
					ostr << (*it_1 * *it_2 * another);
					return;
				}
			}
		}
		ostr << "not found";
	}
	
	};
	}

	std::unique_ptr<AbstractSolver> AbstractSolver::Create() {
		return std::make_unique<Solver>();
	}
}
