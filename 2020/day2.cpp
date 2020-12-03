#include <charconv>
#include <memory>
#include <range/v3/all.hpp>
#include <range/v3/iterator/operations.hpp>
#include <regex>
#include <string>

#include "common.h"

namespace aoc2020 {
namespace {
static std::regex re("(\\d+)-(\\d+) (.): (.+)");
template <typename Pred>
static bool handleline(const std::string& line, Pred&& pred) {
	std::smatch m;
	std::regex_match(line, m, re);
	int min, max;
	std::from_chars(&*m[1].first, &*m[1].second, min);
	std::from_chars(&*m[2].first, &*m[2].second, max);
	char c = *m[3].first;
	std::string_view pass(&*m[4].first, m[4].length());
	return pred(min, max, c, pass);
}

static bool goodpass(const std::string& line) {
	return handleline(
		line, [](int min, int max, char c, std::string_view pass) {
			int got = ranges::distance(
				pass | ranges::views::filter([c](char x) { return x == c; }));
			return min <= got && got <= max;
		});
}

static bool goodpass2(const std::string& line) {
	return handleline(line,
	                  [](int one, int two, char c, std::string_view pass) {
						  return (pass[one - 1] == c) ^ (pass[two - 1] == c);
					  });
}

struct Solver : AbstractSolver {
	std::string_view m_input;
	void parse(std::string_view input) override { m_input = input; }
	void part1(std::ostream& ostr) override {
		ostr << ranges::distance(m_input | ranges::views::split('\n') |
		                         ranges::views::filter(goodpass));
	}
	void part2(std::ostream& ostr) override {
		ostr << ranges::distance(m_input | ranges::views::split('\n') |
		                         ranges::views::filter(goodpass2));
	}
};
}  // namespace

std::unique_ptr<AbstractSolver> AbstractSolver::Create() {
	return std::make_unique<Solver>();
}
}  // namespace aoc2020
