#include <range/v3/all.hpp>

#include "common.h"

namespace aoc2020 {
namespace {

struct Solver : AbstractSolver {
	std::string_view m_input;
	void parse(std::string_view input) override { m_input = input; }
	void part1(std::ostream& ostr) const override {
	}
	void part2(std::ostream& ostr) const override {
	}
};
}  // namespace

std::unique_ptr<AbstractSolver> AbstractSolver::Create() {
	return std::make_unique<Solver>();
}
}  // namespace aoc2020
