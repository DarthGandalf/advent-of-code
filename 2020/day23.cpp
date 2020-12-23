#include <cstdint>
#include <list>
#include <range/v3/algorithm/minmax.hpp>
#include <range/v3/all.hpp>
#include <range/v3/view/filter.hpp>
#include <range/v3/view/transform.hpp>

#include "common.h"
#include "fmt/core.h"

namespace aoc2020 {
namespace {

struct Solver : AbstractSolver {
	std::string_view m_input;
	void parse(std::string_view input) override { m_input = input; }
	void part1(std::ostream& ostr) const override {
		std::deque game(m_input.begin(), m_input.end());
		while (game.back() == '\n') game.pop_back();
		const auto [min, max] = ranges::minmax(game);
		const auto minusone = [&min = min, &max = max](char c) {
			return c == min ? max : c - 1;
		};
		for ([[maybe_unused]] int i : ranges::views::iota(0, 100)) {
			const char current = game.front();
			game.pop_front();
			const std::array<char, 3> sub = {game[0], game[1], game[2]};
			game.pop_front();
			game.pop_front();
			game.pop_front();
			auto it = [&]() {
				char prev = minusone(current);
				while (true) {
					auto it = ranges::find(game, prev);
					if (it == game.end()) {
						prev = minusone(prev);
					} else {
						return it;
					}
				}
			}();
			game.insert(it + 1, sub.begin(), sub.end());
			game.push_back(current);
		}
		while (game.front() != '1') {
			const char current = game.front();
			game.push_back(current);
			game.pop_front();
		}
		game.pop_front();
		for (char c : game) {
			ostr << c;
		}
	}
	void part2(std::ostream& ostr) const override {
		constexpr int N = 1000000;
		auto input_range =
			m_input | ranges::views::filter([](char c) { return c != '\n'; }) |
			ranges::views::transform([](char c) { return c - '0'; });
		std::list<int> game(input_range.begin(), input_range.end());
		auto [min, max] = ranges::minmax(game);
		{
			auto R = ranges::views::iota(max + 1, N + 1);
			game.insert(game.end(), R.begin(), R.end());
		}
		const auto minusone = [&min = min](int c) {
			return c == min ? N : c - 1;
		};
		std::vector<std::list<int>::iterator> where;
		where.resize(N);
		for (auto it = game.begin(); it != game.end(); ++it) {
			where[*it - 1] = it;
		}
		for ([[maybe_unused]] int i : ranges::views::iota(0, 10000000)) {
			const int current = game.front();
			game.pop_front();
			std::array<int, 3> sub;
			sub[0] = game.front();
			game.pop_front();
			sub[1] = game.front();
			game.pop_front();
			sub[2] = game.front();
			game.pop_front();
			auto it = [&]() {
				int prev = minusone(current);
				while (prev == sub[0] || prev == sub[1] || prev == sub[2]) {
					prev = minusone(prev);
				}
				return where[prev - 1];
			}();
			++it;
			for (int e = 0; e < 3; ++e) {
				where[sub[e] - 1] = game.insert(it, sub[e]);
			}
			where[current - 1] = game.insert(game.end(), current);
		}
		while (game.front() != 1) {
			const char current = game.front();
			game.push_back(current);
			game.pop_front();
		}
		game.pop_front();
		std::int64_t a = game.front();
		game.pop_front();
		std::int64_t b = game.front();
		ostr << (a * b);
	}
};
}  // namespace

std::unique_ptr<AbstractSolver> AbstractSolver::Create() {
	return std::make_unique<Solver>();
}
}  // namespace aoc2020
