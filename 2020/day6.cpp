#include <iterator>
#include <range/v3/algorithm/set_algorithm.hpp>
#include <range/v3/all.hpp>
#include <range/v3/numeric/accumulate.hpp>
#include <range/v3/view/set_algorithm.hpp>
#include <range/v3/view/transform.hpp>
#include <set>
#include <unordered_map>
#include <unordered_set>

#include "common.h"

namespace aoc2020 {
namespace {
using namespace std::literals::string_view_literals;

struct Solver : AbstractSolver {
	std::string_view m_input;
	void parse(std::string_view input) override { m_input = input; }
	void part1(std::ostream& ostr) const override {
		ostr << ranges::accumulate(
			m_input | ranges::views::split("\n\n"sv) |
				ranges::views::transform([](auto&& group_str) {
					std::unordered_set<char> answers =
						group_str | ranges::to<std::unordered_set<char>>();
					answers.erase('\n');
					return answers.size();
				}),
			0);
	}
	void part2(std::ostream& ostr) const override {
		ostr << ranges::accumulate(
			m_input | ranges::views::trim([](char c) { return c == '\n'; }) |
				ranges::views::split("\n\n"sv) |
				ranges::views::transform([](auto&& group_str) {
					std::unordered_map<char, int> counts;
					for (char c : group_str) {
						counts[c]++;
					}
					int people = counts['\n'] + 1;
					counts.erase('\n');
					return ranges::count_if(
						counts, [&](auto&& kv) { return kv.second == people; });
				}),
			0);
	}
	void part2_v2(std::ostream& ostr) const {
		ostr << ranges::accumulate(
			m_input | ranges::views::split("\n\n"sv) |
				ranges::views::transform([](auto&& group_str) {
					auto sets =
						group_str | ranges::views::split('\n') |
						ranges::views::transform([](auto&& line_str) {
							return line_str | ranges::to<std::set<char>>();
						});
					return ranges::accumulate(
							   sets, *sets.begin(),
							   [](auto&& set1, auto&& set2) {
								   return ranges::views::set_intersection(
											  set1, set2) |
										  ranges::to<std::set<char>>();
							   })
						.size();
				}),
			0);
	}
};
}  // namespace

std::unique_ptr<AbstractSolver> AbstractSolver::Create() {
	return std::make_unique<Solver>();
}
}  // namespace aoc2020
   // 3189 too low
