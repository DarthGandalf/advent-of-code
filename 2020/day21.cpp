#include <fmt/core.h>

#include <map>
#include <range/v3/algorithm/count_if.hpp>
#include <range/v3/algorithm/for_each.hpp>
#include <range/v3/all.hpp>
#include <range/v3/view/for_each.hpp>
#include <range/v3/view/set_algorithm.hpp>
#include <set>
#include <unordered_map>
#include <unordered_set>

#include "common.h"
#include "ctre.hpp"

namespace aoc2020 {
namespace {
using namespace std::literals::string_view_literals;

struct Solver : AbstractSolver {
	std::vector<std::pair<std::vector<std::string_view> /* foods */,
	                      std::vector<std::string_view> /* allergens */>>
		m_input;
	void parse(std::string_view input) override {
		m_input = input | ranges::views::split('\n') | to_string_view() |
		          ranges::views::transform([](std::string_view line) {
					  using namespace ctre::literals;
					  auto m = R"((.+) \(contains (.+)\))"_ctre.match(line);
					  std::vector<std::string_view> foods =
						  m.get<1>() | ranges::views::split(' ') |
						  to_string_view() | ranges::to_vector;
					  std::vector<std::string_view> allergens =
						  m.get<2>() | ranges::views::split(", "sv) |
						  to_string_view() | ranges::to_vector;
					  return std::pair{foods, allergens};
				  }) |
		          ranges::to_vector;
	}
	std::unordered_map<std::string_view, std::set<std::string_view>>
	group_by_allergen() const {
		std::unordered_map<std::string_view, std::set<std::string_view>>
			by_allergen;
		for (const auto& [ff, aa] : m_input) {
			std::set<std::string_view> foods(ff.begin(), ff.end());
			for (const auto& a : aa) {
				if (auto it = by_allergen.find(a); it != by_allergen.end()) {
					auto res =
						ranges::views::set_intersection(it->second, foods) |
						ranges::to<std::set<std::string_view>>();
					it->second = std::move(res);
				} else {
					by_allergen[a] = foods;
				}
			}
		}
		return by_allergen;
	}
	void part1(std::ostream& ostr) const override {
		std::unordered_map<std::string_view, std::set<std::string_view>>
			by_allergen = group_by_allergen();
		std::unordered_set<std::string_view> possibly_bad_food =
			by_allergen | ranges::views::values |
			ranges::views::for_each([](const auto& ff) {
				return ranges::yield_from(ranges::views::all(ff));
			}) |
			ranges::to<std::unordered_set<std::string_view>>();
		ostr << ranges::count_if(
			m_input | ranges::views::keys |
				ranges::views::for_each([](const auto& ff) {
					return ranges::yield_from(ranges::views::all(ff));
				}),
			[&](const auto& f) { return !possibly_bad_food.contains(f); });
	}
	void part2(std::ostream& ostr) const override {
		std::unordered_map<std::string_view, std::set<std::string_view>>
			by_allergen = group_by_allergen();
		std::map<std::string_view, std::string_view> result;
		while (true) {
			auto it = ranges::find_if(by_allergen, [](const auto& x) {
				return x.second.size() == 1;
			});
			if (it == by_allergen.end()) break;
			std::string_view allergen = it->first;
			std::string_view food = *it->second.begin();
			result[allergen] = food;
			by_allergen.erase(it);
			for (auto& v : by_allergen | ranges::views::values) {
				v.erase(food);
			}
		}
		for (const auto& [index, x] :
		     result | ranges::views::values | ranges::views::enumerate) {
			if (index) ostr << ',';
			ostr << x;
		}
	}
};
}  // namespace

std::unique_ptr<AbstractSolver> AbstractSolver::Create() {
	return std::make_unique<Solver>();
}
}  // namespace aoc2020
