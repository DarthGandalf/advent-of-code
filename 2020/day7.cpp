#include <charconv>
#include <cstdint>
#include <queue>
#include <range/v3/algorithm/all_of.hpp>
#include <range/v3/algorithm/set_algorithm.hpp>
#include <range/v3/all.hpp>
#include <range/v3/numeric/accumulate.hpp>
#include <range/v3/view/transform.hpp>
#include <set>
#include <stdexcept>
#include <string_view>
#include <unordered_map>
#include <unordered_set>

#include "common.h"
#include "ctre.hpp"

namespace aoc2020 {
namespace {
using namespace std::literals::string_view_literals;
using namespace ctre::literals;
static constexpr std::string_view init_bag = "shiny gold"sv;

struct Solver : AbstractSolver {
	std::unordered_map<std::string_view,
	                   std::vector<std::pair<int, std::string_view>>>
		m_data;
	void parse(std::string_view input) override {
		m_data =
			input | ranges::views::split('\n') | to_string_view() |
			ranges::views::transform([](std::string_view line) {
				auto m = R"((\w+ \w+) bags contain (.+))"_ctre.match(line);
				std::string_view this_bag = m.get<1>();
				std::string_view contains = m.get<2>();
				if (contains[0] == 'n') {
					return std::pair{
						this_bag,
						std::vector<std::pair<int, std::string_view>>{}};
				}
				return std::pair{
					this_bag,
					contains | ranges::views::split(", "sv) | to_string_view() |
						ranges::views::transform(
							[](std::string_view bags_info) {
								auto m2 = R"((\d+) (\w+ \w+) bag.*)"_ctre.match(
									bags_info);
								int num;
								std::from_chars(m2.get<1>().begin(),
				                                m2.get<1>().end(), num);
								return std::pair{num, m2.get<2>()};
							}) |
						ranges::to<
							std::vector<std::pair<int, std::string_view>>>()};
			}) |
			ranges::to<std::unordered_map<
				std::string_view,
				std::vector<std::pair<int, std::string_view>>>>();
	}
	void part1(std::ostream& ostr) const override {
		std::unordered_map<std::string_view, std::vector<std::string_view>>
			reverse;
		for (const auto& [outer, list] : m_data) {
			for (const auto& [num, inner] : list) {
				reverse[inner].push_back(outer);
			}
		}
		std::unordered_set<std::string_view> vis = {init_bag};
		std::queue<std::string_view> q;
		q.push(init_bag);
		while (!q.empty()) {
			const std::string_view current = q.front();
			q.pop();
			for (const auto& next : reverse[current]) {
				if (!vis.contains(next)) {
					vis.insert(next);
					q.push(next);
				}
			}
		}
		ostr << vis.size() - 1;
	}
	void part2(std::ostream& ostr) const override {
		std::unordered_map<std::string_view,
		                   std::unordered_set<std::string_view>>
			reverse;
		std::unordered_map<std::string_view, int> dependnum;
		std::unordered_set<std::string_view> ready;
		for (const auto& [outer, list] : m_data) {
			for (const auto& [num, inner] : list) {
				reverse[inner].insert(outer);
			}
			dependnum[outer] = list.size();
			if (list.empty()) {
				ready.insert(outer);
			}
		}
		std::unordered_map<std::string_view, std::int64_t> results;

		while (!ready.empty()) {
			std::string_view current = *ready.begin();
			ready.erase(ready.begin());
			std::int64_t r = ranges::accumulate(
				m_data.at(current) | ranges::views::transform(
										 [&](const auto& bag) -> std::int64_t {
											 return bag.first *
													results.at(bag.second);
										 }),
				1);
			if (current == init_bag) {
				ostr << r - 1;
				return;
			}
			results[current] = r;
			for (std::string_view above : reverse[current]) {
				auto& there = dependnum[above];
				there--;
				if (there == 0) {
					ready.insert(above);
				}
			}
		}
	}
};
}  // namespace

std::unique_ptr<AbstractSolver> AbstractSolver::Create() {
	return std::make_unique<Solver>();
}
}  // namespace aoc2020
