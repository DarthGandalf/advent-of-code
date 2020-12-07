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
		std::unordered_map<std::string_view, std::int64_t> results;
		std::set<std::string_view> todo =
			m_data | ranges::views::keys |
			ranges::to<std::set<std::string_view>>();
		while (true) {
			if (auto i = results.find(init_bag); i != results.end()) {
				ostr << i->second - 1;
				return;
			}
			std::set<std::string_view> done;
			for (const auto& x : todo) {
				const auto& list = m_data.at(x);
				try {
					std::int64_t r = ranges::accumulate(
						list | ranges::views::transform(
								   [&](const auto& bag) -> std::int64_t {
									   return bag.first *
											  results.at(bag.second);
								   }),
						1);
					done.insert(x);
					results[x] = r;
				} catch (const std::out_of_range&) {
				}
			}
			todo = ranges::views::set_difference(todo, done) |
				   ranges::to<std::set<std::string_view>>();
		}
	}
};
}  // namespace

std::unique_ptr<AbstractSolver> AbstractSolver::Create() {
	return std::make_unique<Solver>();
}
}  // namespace aoc2020
