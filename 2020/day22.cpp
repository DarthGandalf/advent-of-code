#include <fmt/core.h>

#include <array>
#include <cstdint>
#include <deque>
#include <range/v3/all.hpp>
#include <range/v3/numeric/accumulate.hpp>
#include <range/v3/view/enumerate.hpp>
#include <range/v3/view/split.hpp>
#include <range/v3/view/transform.hpp>
#include <set>
#include <unordered_set>

#include "common.h"

namespace aoc2020 {
namespace {
using namespace std::literals::string_view_literals;

using Game = std::array<std::deque<int>, 2>;

struct Solver : AbstractSolver {
	Game m_input;
	void parse(std::string_view input) override {
		auto vec = input | ranges::views::split("\n\n"sv) | to_string_view() |
		           ranges::views::transform([](std::string_view str) {
					   auto i = ints<int>(str);
					   return i | ranges::views::drop(1) |
			                  ranges::to<std::deque<int>>();
				   }) |
		           ranges::to_vector;
		m_input[0] = std::move(vec[0]);
		m_input[1] = std::move(vec[1]);
	}
	void part1(std::ostream& ostr) const override {
		auto game = m_input;
		while (!game[0].empty() && !game[1].empty()) {
			std::array<int, 2> card;
			card[0] = game[0].front();
			card[1] = game[1].front();
			game[0].pop_front();
			game[1].pop_front();
			int winner = card[0] < card[1] ? 1 : 0;
			game[winner].push_back(card[winner]);
			game[winner].push_back(card[1 - winner]);
		}
		int winner = game[0].empty() ? 1 : 0;
		ostr << ranges::accumulate(
			game[winner] | ranges::views::reverse | ranges::views::enumerate |
				ranges::views::transform([](const auto& p) {
					auto [index, num] = p;
					return (index + 1) * num;
				}),
			std::int64_t{0});
	}
	std::pair<int /* winner */, Game> subgame(Game game) const {
		std::set<Game> seen;
		while (!game[0].empty() && !game[1].empty()) {
			if (seen.contains(game)) return std::pair{0, std::move(game)};
			seen.insert(game);
			std::array<int, 2> card;
			card[0] = game[0].front();
			card[1] = game[1].front();
			game[0].pop_front();
			game[1].pop_front();
			int winner;
			if (game[0].size() >= card[0] && game[1].size() >= card[1]) {
				Game sub;
				sub[0] = game[0] | ranges::views::take(card[0]) |
				         ranges::to<std::deque<int>>();
				sub[1] = game[1] | ranges::views::take(card[1]) |
				         ranges::to<std::deque<int>>();
				winner = subgame(std::move(sub)).first;
			} else {
				winner = card[0] < card[1] ? 1 : 0;
			}
			game[winner].push_back(card[winner]);
			game[winner].push_back(card[1 - winner]);
		}
		int winner = game[0].empty() ? 1 : 0;
		return std::pair{winner, std::move(game)};
	}
	void part2(std::ostream& ostr) const override {
		auto [winner, game] = subgame(m_input);
		ostr << ranges::accumulate(
			game[winner] | ranges::views::reverse | ranges::views::enumerate |
				ranges::views::transform([](const auto& p) {
					auto [index, num] = p;
					return (index + 1) * num;
				}),
			std::int64_t{0});
	}
};
}  // namespace

std::unique_ptr<AbstractSolver> AbstractSolver::Create() {
	return std::make_unique<Solver>();
}
}  // namespace aoc2020
