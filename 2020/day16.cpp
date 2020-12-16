#include <cstdint>
#include <map>
#include <range/v3/all.hpp>
#include <range/v3/range/conversion.hpp>
#include <range/v3/view/transform.hpp>
#include <string_view>
#include <unordered_map>
#include <unordered_set>

#include "common.h"

namespace aoc2020 {
namespace {
using namespace std::literals::string_view_literals;

struct Ticket {
	std::vector<int> numbers;
};

struct Rule {
	std::vector<std::pair<int, int>> ranges;

	bool match(int num) const {
		return ranges::any_of(ranges, [&](const auto& p) {
			return p.first <= num && num <= p.second;
		});
	}
};

struct Solver : AbstractSolver {
	std::unordered_map<std::string_view, Rule> rules;
	Ticket my_ticket;
	std::vector<Ticket> nearby_tickets;

	void parse(std::string_view input) override {
		auto input_range =
			input | ranges::views::split("\n\n"sv) | to_string_view();
		auto input_range_iter = input_range.begin();
		rules = *input_range_iter++ | ranges::views::split('\n') |
		        to_string_view() |
		        ranges::views::transform([](std::string_view line) {
					auto colon = line.find_first_of(':');
					std::string_view name = line.substr(0, colon);
					auto ii = ints<int>(line.substr(colon));
					auto ranges =
						ii | ranges::views::chunk(2) |
						ranges::views::transform([](const auto& r) {
							return std::pair{ranges::front(r), ranges::back(r)};
						}) |
						ranges::to_vector;
					return std::pair{name, Rule{ranges}};
				}) |
		        ranges::to<std::unordered_map<std::string_view, Rule>>();
		my_ticket.numbers = ints<int>(*input_range_iter++);
		nearby_tickets =
			*input_range_iter++ | ranges::views::split('\n') |
			to_string_view() | ranges::views::drop(1) |
			ranges::views::transform(ints<int>) |
			ranges::views::transform([](const auto& t) { return Ticket{t}; }) |
			ranges::to_vector;
	}
	bool match_any_rule(int num) const {
		return ranges::any_of(rules | ranges::views::values,
		                      [&](const auto& r) { return r.match(num); });
	}
	void part1(std::ostream& ostr) const override {
		ostr << ranges::accumulate(
			nearby_tickets | ranges::views::for_each([](const auto& t) {
				return ranges::yield_from(ranges::views::all(t.numbers));
			}) | ranges::views::filter([&](int num) {
				return !match_any_rule(num);
			}),
			0);
	}
	void part2(std::ostream& ostr) const override {
		std::vector<Ticket> valid_tickets =
			ranges::views::concat(
				ranges::yield(my_ticket),
				nearby_tickets | ranges::views::filter([&](const auto& t) {
					return ranges::all_of(t.numbers, [&](int num) {
						return match_any_rule(num);
					});
				})) |
			ranges::to_vector;
		std::unordered_map<std::string_view, std::unordered_set<int>>
			possibilities =
				rules |
				ranges::views::transform(
					[&](const std::pair<std::string_view, Rule>& name_rule) {
						const auto& [name, rule] = name_rule;
						auto possible_fields =
							ranges::views::iota(0,
			                                    (int)my_ticket.numbers.size()) |
							ranges::views::filter([&, &rule = rule](int field) {
								return ranges::all_of(
									valid_tickets, [&](const Ticket& t) {
										return rule.match(t.numbers[field]);
									});
							}) |
							ranges::to<std::unordered_set<int>>();
						return std::pair{name, possible_fields};
					}) |
				ranges::to<std::unordered_map<std::string_view,
		                                      std::unordered_set<int>>>();
		std::int64_t result = 1;
		int result_count = 0;
		while (true) {
			auto it = ranges::find_if(possibilities, [](const auto& x) {
				return x.second.size() == 1;
			});
			if (it == possibilities.end()) return;
			std::string_view name = it->first;
			int position = *it->second.begin();
			if (name.starts_with("departure "sv)) {
				result_count++;
				result *= my_ticket.numbers[position];
				if (result_count == 6) {
					ostr << result;
					return;
				}
			}
			possibilities.erase(it);
			for (auto& v : possibilities | ranges::views::values) {
				v.erase(position);
			}
		}
	}
};
}  // namespace

std::unique_ptr<AbstractSolver> AbstractSolver::Create() {
	return std::make_unique<Solver>();
}
}  // namespace aoc2020
