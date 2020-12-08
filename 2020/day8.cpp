#include <charconv>
#include <cstdint>
#include <deque>
#include <range/v3/all.hpp>
#include <range/v3/view/transform.hpp>

#include "common.h"

namespace aoc2020 {
namespace {

enum class instruction_type {
	nop,
	acc,
	jmp,
};

struct instruction {
	instruction_type type;
	std::int64_t param;
};

struct VM {
	std::int64_t m_acc = 0;
	std::vector<instruction> m_program;
	std::vector<bool> m_was;
	int m_pc = 0;

	void step() {
		m_was[m_pc] = true;
		const auto& [t, p] = m_program[m_pc];
		switch (t) {
			case instruction_type::nop:
				++m_pc;
				break;
			case instruction_type::acc:
				++m_pc;
				m_acc += p;
				break;
			case instruction_type::jmp:
				m_pc += p;
				break;
		}
	}
};

struct Solver : AbstractSolver {
	std::vector<instruction> m_program;
	void parse(std::string_view input) override {
		m_program = input | ranges::views::split('\n') | to_string_view() |
					ranges::views::transform([](std::string_view line) {
						instruction_type t{};
						switch (line[0]) {
							case 'n':
								t = instruction_type::nop;
								break;
							case 'a':
								t = instruction_type::acc;
								break;
							case 'j':
								t = instruction_type::jmp;
								break;
						}
						std::int64_t p;
						std::from_chars(line.data() + 5, line.end(), p);
						if (line[4] == '-') p *= -1;
						return instruction{.type = t, .param = p};
					}) |
					ranges::to<std::vector<instruction>>();
	}
	void part1(std::ostream& ostr) const override {
		VM vm;
		vm.m_program = m_program;
		vm.m_was.resize(m_program.size());
		while (true) {
			if (vm.m_was[vm.m_pc]) {
				ostr << vm.m_acc;
				return;
			}
			vm.step();
		}
	}
	void part2(std::ostream& ostr) const override {
		std::deque<VM> vms;
		for (int i = 0; i < m_program.size(); ++i) {
			if (m_program[i].type != instruction_type::acc) {
				VM vm;
				vm.m_program = m_program;
				if (m_program[i].type == instruction_type::nop) {
					vm.m_program[i].type = instruction_type::jmp;
				} else {
					vm.m_program[i].type = instruction_type::nop;
				}
				vm.m_was.resize(m_program.size());
				vms.push_back(std::move(vm));
			}
		}
		while (true) {
			for (auto& vm : vms) {
				if (vm.m_pc < 0) continue;
				if (vm.m_pc >= m_program.size()) continue;
				if (vm.m_was[vm.m_pc]) continue;
				vm.step();
				if (vm.m_pc == m_program.size()) {
					ostr << vm.m_acc;
					return;
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
