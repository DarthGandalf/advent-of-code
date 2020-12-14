#include <charconv>
#include <cstdint>
#include <range/v3/all.hpp>
#include <range/v3/numeric/accumulate.hpp>
#include <range/v3/view/enumerate.hpp>
#include <range/v3/view/transform.hpp>
#include <string_view>
#include <unordered_map>
#include <variant>

#include "common.h"
#include "ctre.hpp"

namespace aoc2020 {
namespace {

template <class... Ts>
struct overload : Ts... {
	using Ts::operator()...;
};
template <class... Ts>
overload(Ts...) -> overload<Ts...>;

// https://stackoverflow.com/questions/5279051/how-can-i-create-cartesian-product-of-vector-of-vectors
auto cart_product(const auto& v) {
	std::vector<std::vector<int>> s = {{}};
	for (const auto& u : v) {
		std::vector<std::vector<int>> r;
		for (const auto& x : s) {
			for (const auto& y : u) {
				r.push_back(x);
				r.back().push_back(y);
			}
		}
		s = move(r);
	}
	return s;
}

struct Solver : AbstractSolver {
	struct Mask {
		std::string_view str;
	};
	struct Set {
		std::int64_t addr;
		std::int64_t value;
	};
	std::vector<std::variant<Mask, Set>> m_data;
	void parse(std::string_view input) override {
		using namespace ctre::literals;
		m_data =
			input | ranges::views::split('\n') | to_string_view() |
			ranges::views::transform(
				[](std::string_view line) -> std::variant<Mask, Set> {
					if (auto m = "mask = (.+)"_ctre.match(line)) {
						return Mask{.str = m.get<1>()};
					}
					if (auto m = R"(mem\[(\d+)] = (\d+))"_ctre.match(line)) {
						std::int64_t addr, value;
						std::from_chars(m.get<1>().begin(), m.get<1>().end(),
				                        addr);
						std::from_chars(m.get<2>().begin(), m.get<2>().end(),
				                        value);
						return Set{.addr = addr, .value = value};
					}
					abort();
				}) |
			ranges::to<std::vector<std::variant<Mask, Set>>>();
	}
	void part1(std::ostream& ostr) const override {
		std::unordered_map<std::int64_t, std::int64_t> mem;
		std::int64_t ones = 0;
		std::int64_t zeroes = 0;
		for (const auto& cmd : m_data) {
			std::visit(overload{[&](const Mask& m) {
									ones = 0;
									zeroes = 0;
									for (char c : m.str) {
										ones <<= 1;
										zeroes <<= 1;
										switch (c) {
											case '1':
												ones |= 1;
												break;
											case '0':
												zeroes |= 1;
												break;
										}
									}
								},
			                    [&](const Set& s) {
									mem[s.addr] = (s.value & ~zeroes) | ones;
								}},
			           cmd);
		}
		ostr << ranges::accumulate(mem | ranges::views::values,
		                           std::int64_t{0});
	}
	void part2(std::ostream& ostr) const override {
		std::unordered_map<std::int64_t, std::int64_t> mem;
		Mask mask;
		for (const auto& cmd : m_data) {
			std::visit(
				overload{[&](const Mask& m) { mask = m; },
			             [&](const Set& s) {
							 std::vector<std::vector<int>> bits =
								 mask.str | ranges::views::enumerate |
								 ranges::views::transform([&](const auto& r) {
									 const auto& [index, c] = r;
									 switch (c) {
										 case '0':
											 return std::vector{int(
												 1 & (s.addr >> (35 - index)))};
										 case '1':
											 return std::vector{1};
										 case 'X':
											 return std::vector{0, 1};
									 }
									 abort();
								 }) |
								 ranges::to_vector;
							 for (const auto& addr_bits : cart_product(bits)) {
								 std::int64_t addr = 0;
								 for (const auto& [index, bit] :
					                  addr_bits | ranges::views::enumerate) {
									 addr <<= 1;
									 addr |= bit;
								 }
								 mem[addr] = s.value;
							 }
						 }},
				cmd);
		}
		ostr << ranges::accumulate(mem | ranges::views::values,
		                           std::int64_t{0});
	}
};
}  // namespace

std::unique_ptr<AbstractSolver> AbstractSolver::Create() {
	return std::make_unique<Solver>();
}
}  // namespace aoc2020
