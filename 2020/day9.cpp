#include <cstdint>
#include <range/v3/algorithm/find_if.hpp>
#include <range/v3/all.hpp>
#include <range/v3/view/cartesian_product.hpp>
#include <range/v3/view/enumerate.hpp>
#include <range/v3/view/sliding.hpp>
#include <range/v3/view/take_exactly.hpp>

#include "common.h"

namespace aoc2020 {
namespace {

constexpr static int N = 25;

struct Solver : AbstractSolver {
	std::vector<std::int64_t> m_numbers;
	void parse(std::string_view input) override {
		m_numbers = ints<std::int64_t>(input);
	}
	void part1(std::ostream& ostr) const override { ostr << part1_algo(); }
	std::int64_t part1_algo() const {
		auto slider = m_numbers | ranges::views::sliding(N + 1);
		if (auto result = ranges::find_if_not(
				slider,
				[](const auto& subrange) {
					auto last = *(subrange | ranges::views::reverse).begin();
					// enumed+filter by index for the case if numbers in input
			        // are not unique. Filtering by value would leave the
			        // possibility for a number to be summed with itself.
					auto enumed = subrange | ranges::views::take_exactly(N) |
								  ranges::views::enumerate;
					return ranges::any_of(
						ranges::views::cartesian_product(enumed, enumed) |
							ranges::views::filter([](const auto& en2) {
								const auto& [left_index, left_value] =
									std::get<0>(en2);
								const auto& [right_index, right_value] =
									std::get<1>(en2);
								return left_index != right_index;
							}) |
							ranges::views::transform([](const auto& en2) {
								return std::pair{std::get<0>(en2).second,
				                                 std::get<1>(en2).second};
							}),
						[&](const auto& p) {
							return p.first + p.second == last;
						});
				});
		    result != slider.end()) {
			return *(*result | ranges::views::reverse).begin();
		}
		return 0;
	}
	void part2(std::ostream& ostr) const override {
		auto needle = part1_algo();
		int min_index = 0;
		int max_index = 1;
		std::int64_t current = m_numbers[0] + m_numbers[1];
		while (true) {
			if (current == needle) {
				auto result = ranges::minmax(
					ranges::subrange(m_numbers.begin() += min_index,
				                     m_numbers.begin() += max_index + 1));
				ostr << (result.min + result.max);
				return;
			}
			if (current < needle) {
				current += m_numbers[++max_index];
			} else {
				current -= m_numbers[min_index++];
			}
		}
	}
};
}  // namespace

std::unique_ptr<AbstractSolver> AbstractSolver::Create() {
	return std::make_unique<Solver>();
}
}  // namespace aoc2020
