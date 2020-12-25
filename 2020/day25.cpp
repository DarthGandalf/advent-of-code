#include <fmt/core.h>

#include <cstdint>
#include <range/v3/all.hpp>
#include <range/v3/numeric/accumulate.hpp>
#include <range/v3/view/repeat.hpp>

#include "common.h"

namespace aoc2020 {
namespace {

constexpr std::int64_t M = 20201227;

std::int64_t multiply(std::int64_t x, std::int64_t y) { return (x * y) % M; }

std::int64_t transform(std::int64_t subject, std::int64_t loop) {
	return ranges::accumulate(
		ranges::views::repeat(subject) | ranges::views::take(loop),
		std::int64_t{1}, multiply);
}

std::int64_t decrypt(std::int64_t key) {
	std::int64_t num = 1;
	for (std::int64_t loop : ranges::views::iota(0, M)) {
		if (num == key) {
			return loop;
		}
		num = multiply(num, 7);
	}
	abort();
}

struct Solver : AbstractSolver {
	std::int64_t m_a, m_b;
	void parse(std::string_view input) override {
		std::vector in = ints<std::int64_t>(input);
		m_a = in.front();
		m_b = in.back();
	}
	void part1(std::ostream& ostr) const override {
		auto keya = decrypt(m_a);
		ostr << transform(m_b, keya);
	}
	void part2(std::ostream& ostr) const override {}
};
}  // namespace

std::unique_ptr<AbstractSolver> AbstractSolver::Create() {
	return std::make_unique<Solver>();
}
}  // namespace aoc2020
