#include <cstdint>
#include <range/v3/all.hpp>
#include <range/v3/numeric/accumulate.hpp>
#include <range/v3/view/transform.hpp>
#include <stack>

#include "common.h"
#include "ctre.hpp"
#include "fmt/core.h"

namespace aoc2020 {
namespace {

struct Solver : AbstractSolver {
	static void apply_op(char op, std::int64_t& result, std::int64_t arg) {
		if (op == '+') {
			result += arg;
		} else {
			result *= arg;
		}
	}
	std::string_view m_input;
	void parse(std::string_view input) override { m_input = input; }
	void part1(std::ostream& ostr) const override {
		ostr << ranges::accumulate(
			m_input | ranges::views::split('\n') | to_string_view() |
				ranges::views::transform([](std::string_view line) {
					const char* where = line.begin();
					std::stack<std::int64_t> stack;
					std::stack<char> ops;
					stack.push(0);
					ops.push('+');
					while (where < line.end()) {
						switch (*where) {
							case ' ':
								where++;
								continue;
							case '+':
							case '*':
								ops.top() = *where;
								where++;
								continue;
							case '(':
								stack.push(0);
								ops.push('+');
								where++;
								continue;
							case ')': {
								std::int64_t num = stack.top();
								stack.pop();
								ops.pop();
								apply_op(ops.top(), stack.top(), num);
								where++;
								continue;
							}
							default: {
								std::int64_t num;
								auto [next, err] =
									std::from_chars(where, line.end(), num);
								apply_op(ops.top(), stack.top(), num);
								where = next;
								continue;
							}
						}
					}
					return stack.top();
				}),
			std::int64_t{0});
	}
	void part2(std::ostream& ostr) const override {
		ostr << ranges::accumulate(
			m_input | ranges::views::split('\n') | to_string_view() |
				ranges::views::transform([](std::string_view line) {
					const char* where = line.begin();
					std::stack<std::int64_t> stack;
					std::stack<char> ops;
					while (where < line.end()) {
						switch (*where) {
							case ' ':
								where++;
								break;
								;
							case '+':
							case '*':
								while (!ops.empty() && ops.top() != '(' &&
						               (ops.top() == '+' || *where == '*')) {
									std::int64_t num = stack.top();
									stack.pop();
									apply_op(ops.top(), stack.top(), num);
									ops.pop();
								}
								ops.push(*where);
								where++;
								break;
							case '(':
								ops.push(*where);
								where++;
								break;
							case ')':
								while (ops.top() != '(') {
									std::int64_t num = stack.top();
									stack.pop();
									apply_op(ops.top(), stack.top(), num);
									ops.pop();
								}
								ops.pop();
								where++;
								break;
							default: {
								std::int64_t num;
								auto [next, err] =
									std::from_chars(where, line.end(), num);
								stack.push(num);
								where = next;
							}
						}
					}
					while (!ops.empty()) {
						std::int64_t num = stack.top();
						stack.pop();
						apply_op(ops.top(), stack.top(), num);
						ops.pop();
					}
					return stack.top();
				}),
			std::int64_t{0});
	}
};
}  // namespace

std::unique_ptr<AbstractSolver> AbstractSolver::Create() {
	return std::make_unique<Solver>();
}
}  // namespace aoc2020
