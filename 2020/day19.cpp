#include <queue>
#include <range/v3/algorithm/any_of.hpp>
#include <range/v3/all.hpp>
#include <range/v3/range/conversion.hpp>
#include <string_view>
#include <unordered_map>
#include <unordered_set>
#include <variant>

#include "common.h"
#include "fmt/core.h"

namespace aoc2020 {
namespace {
using namespace std::literals::string_view_literals;

struct Branch;

struct Rule {
	std::vector<Branch> branches;
};

using Token = std::variant<std::string_view, char>;

struct Branch {
	std::vector<Token> tokens;
};

struct QueueElement {
	std::string_view str;
	std::string_view rule;
};

struct Grammar {
	std::unordered_map<std::string_view, Rule> rules;
	// Returns list of substrings *after* the matched substring.
	std::unordered_set<std::string_view> match(std::string_view s,
	                                           std::string_view rule) const {
		if (s.empty()) return {};
		std::unordered_set<std::string_view> result;
		for (const Branch& branch : rules.at(rule).branches) {
			std::unordered_set<std::string_view> qq = {s};
			for (const Token& t : branch.tokens) {
				std::unordered_set<std::string_view> next;
				for (std::string_view attempt : qq) {
					if (std::holds_alternative<char>(t)) {
						if (attempt.empty() ||
						    attempt[0] != std::get<char>(t)) {
							break;
						} else {
							next.insert(attempt.substr(1));
						}
					} else {
						auto aftermatches =
							match(attempt, std::get<std::string_view>(t));
						for (std::string_view after : aftermatches) {
							next.insert(after);
						}
					}
				}
				qq = std::move(next);
			}
			result.insert(qq.begin(), qq.end());
		}
		return result;
	}
	bool check(std::string_view s, std::string_view rule) const {
		return ranges::any_of(match(s, rule),
		                      [](const auto& rest) { return rest.empty(); });
	}
};

struct Solver : AbstractSolver {
	Grammar m_grammar;
	std::vector<std::string_view> m_strings;
	void parse(std::string_view input) override {
		auto input_range =
			input | ranges::views::split("\n\n"sv) | to_string_view();
		auto input_iter = input_range.begin();
		m_grammar.rules =
			*input_iter++ | ranges::views::split('\n') | to_string_view() |
			ranges::views::transform([](std::string_view line) {
				auto colon_range =
					line | ranges::views::split(": "sv) | to_string_view();
				auto colon_iter = colon_range.begin();
				std::string_view name = *colon_iter++;
				Rule rule;
				rule.branches =
					*colon_iter | ranges::views::split(" | "sv) |
					to_string_view() |
					ranges::views::transform([](std::string_view branch_str) {
						Branch branch;
						branch.tokens =
							branch_str | ranges::views::split(' ') |
							to_string_view() |
							ranges::views::transform(
								[](std::string_view token_str) -> Token {
									if (token_str.starts_with('"'))
										return token_str[1];
									return token_str;
								}) |
							ranges::to_vector;
						return branch;
					}) |
					ranges::to_vector;
				return std::pair{name, rule};
			}) |
			ranges::to<std::unordered_map<std::string_view, Rule>>();
		m_strings = *input_iter++ | ranges::views::split('\n') |
		            to_string_view() | ranges::to_vector;
	}
	void part1(std::ostream& ostr) const override {
		ostr << ranges::count_if(m_strings, [&](std::string_view str) {
			return m_grammar.check(str, "0"sv);
		});
	}
	void part2(std::ostream& ostr) const override {
		Grammar grammar = m_grammar;
		grammar.rules["8"sv].branches.push_back(
			Branch{.tokens = std::vector<Token>{"42"sv, "8"sv}});
		grammar.rules["11"sv].branches.push_back(
			Branch{.tokens = std::vector<Token>{"42"sv, "11"sv, "31"sv}});
		ostr << ranges::count_if(m_strings, [&](std::string_view str) {
			return grammar.check(str, "0"sv);
		});
	}
};
}  // namespace

std::unique_ptr<AbstractSolver> AbstractSolver::Create() {
	return std::make_unique<Solver>();
}
}  // namespace aoc2020

// 216 too low
