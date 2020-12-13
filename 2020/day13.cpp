#include <charconv>
#include <cstdint>
#include <numeric>
#include <range/v3/algorithm/find_if.hpp>
#include <range/v3/all.hpp>
#include <range/v3/view/enumerate.hpp>
#include <range/v3/view/iota.hpp>
#include <range/v3/view/transform.hpp>
#include <string_view>

#include "common.h"

namespace aoc2020 {
namespace {

struct Solver : AbstractSolver {
	int m_starttime;
	std::vector<int> m_offsets;
	std::vector<int> m_buses;
	void parse(std::string_view input) override {
		auto _n = input.find_first_of('\n');
		std::from_chars(input.begin(), input.begin() + _n, m_starttime);
		for (auto [offset, value] :
		     input.substr(_n + 1) | ranges::views::split(',') |
		         to_string_view() | ranges::views::enumerate |
		         ranges::views::filter([](auto&& element) {
					 auto [index, value] = element;
					 return value.front() != 'x';
				 }) |
		         ranges::views::transform([](auto&& element) {
					 auto [offset, str] = element;
					 int value;
					 std::from_chars(str.begin(), str.end(), value);
					 return std::pair{offset, value};
				 })) {
			m_offsets.push_back(offset);
			m_buses.push_back(value);
		}
	}
	void part1(std::ostream& ostr) const override {
		for (int now : ranges::views::iota(m_starttime)) {
			for (int bus : m_buses) {
				if ((now % bus) == 0) {
					ostr << bus * (now - m_starttime);
					return;
				}
			}
		}
	}

	static auto stepped_iota(std::int64_t start, std::int64_t step) {
		return ranges::views::iota(0) |
			   ranges::views::transform(
				   [=](std::int64_t x) { return x * step + start; });
	}

	void part2(std::ostream& ostr) const override {
		std::int64_t step = 1;
		auto times = stepped_iota(0, step);
		for (auto [bus, offset] : ranges::views::zip(m_buses, m_offsets)) {
			std::int64_t first = *ranges::find_if(
				times, [bus = bus, offset = offset](std::int64_t time) {
					return (time + offset) % bus == 0;
				});
			step = std::lcm(step, bus);
			times = stepped_iota(first, step);
		}
		ostr << *times.begin();
	}

	void part2x(std::ostream& ostr) const {
		const std::vector<int>& a = m_buses;
		std::vector<int> r = ranges::views::zip(a, m_offsets) |
							 ranges::views::transform([](const auto& ao) {
								 int r = ao.first - ao.second;
								 while (r < 0) r += ao.first;
								 return r % ao.first;
							 }) |
							 ranges::to<std::vector<int>>();
		std::vector<std::vector<int>> rij(a.size());
		for (int i : ranges::views::iota(0, (int)a.size())) {
			rij[i].resize(a.size());
			for (int j : ranges::views::iota(0, (int)a.size())) {
				if (i == j) continue;
				// ai inv mod aj
				int m = a[j];
				int x = 1;
				int y = 0;
				int z = a[i];
				while (z > 1) {
					int q = z / m;
					int t = m;
					m = z % m;
					z = t;

					int u = y;
					y = x - q * y;
					x = u;
				}

				if (x < 0) x += a[j];
				rij[i][j] = x;
			}
		}
		std::vector<std::int64_t> x;
		for (int i : ranges::views::iota(0, (int)a.size())) {
			std::int64_t xi = r[i];
			for (int j : ranges::views::iota(0, i)) {
				xi -= x[j];
				while (xi < 0) xi += a[i];
				xi *= rij[j][i];
				xi %= a[i];
			}
			x.push_back(xi);
		}
		std::int64_t coef = 1;
		std::int64_t result = 0;
		for (int i : ranges::views::iota(0, (int)a.size())) {
			result += x[i] * coef;
			coef *= a[i];
		}
		ostr << result;
	}
};
}  // namespace

std::unique_ptr<AbstractSolver> AbstractSolver::Create() {
	return std::make_unique<Solver>();
}
}  // namespace aoc2020
