#include <range/v3/algorithm/count_if.hpp>
#include <range/v3/algorithm/transform.hpp>
#include <range/v3/all.hpp>
#include <range/v3/view/for_each.hpp>
#include <range/v3/view/transform.hpp>
#include <stdexcept>

#include "common.h"

namespace aoc2020 {
namespace {

enum class Cell {
	Floor,
	Empty,
	Busy,
};

struct Map {
	std::vector<std::vector<Cell>> rows;
	int part2 = 0;
	std::vector<std::vector<std::vector<std::pair<int, int>>>> deps;

	int occupied_for(int row, int col) const {
		int result = 0;
		for (auto [frow, fcol] : deps[row][col])
			if (rows[frow][fcol] == Cell::Busy) ++result;
		return result;
	}

	Cell next_cell(int row, int col) const {
		Cell now = rows[row][col];
		switch (now) {
			case Cell::Floor:
				return Cell::Floor;
			case Cell::Empty:
				return occupied_for(row, col) == 0 ? Cell::Busy : Cell::Empty;
			case Cell::Busy:
				return occupied_for(row, col) >= 4 + part2 ? Cell::Empty
				                                           : Cell::Busy;
		}
	}

	void next_step_out(std::vector<std::vector<Cell>>& result) const {
		for (int row = 0; row < rows.size(); ++row) {
			for (int col = 0; col < rows[row].size(); ++col) {
				result[row][col] = next_cell(row, col);
			}
		}
	}

	[[maybe_unused]] friend std::ostream& operator<<(std::ostream& ostr,
	                                                 const Map& map) {
		for (const auto& row : map.rows) {
			for (Cell cell : row) {
				switch (cell) {
					case Cell::Floor:
						ostr << '.';
						break;
					case Cell::Busy:
						ostr << '#';
						break;
					case Cell::Empty:
						ostr << 'L';
						break;
				}
			}
			ostr << '\n';
		}
		return ostr;
	}

	int num_busy() const {
		return ranges::count(
			rows | ranges::views::for_each([](const auto& row) {
				return ranges::yield_from(ranges::views::all(row));
			}),
			Cell::Busy);
	}

	int run() {
		std::vector<std::vector<Cell>> next = rows;
		while (true) {
			next_step_out(next);
			if (rows == next) {
				return num_busy();
			}
			std::swap(rows, next);
		}
	}
};

struct Solver : AbstractSolver {
	Map m_map;
	void parse(std::string_view input) override {
		m_map.rows = input | ranges::views::split('\n') |
		             ranges::views::transform([](auto&& line_range) {
						 return line_range |
			                    ranges::views::transform([](char c) {
									if (c == 'L') return Cell::Empty;
									return Cell::Floor;
								}) |
			                    ranges::to<std::vector<Cell>>();
					 }) |
		             ranges::to<std::vector<std::vector<Cell>>>();
	}
	void part1(std::ostream& ostr) const override {
		Map map = m_map;
		map.deps.resize(m_map.rows.size());
		for (int row = 0; row < m_map.rows.size(); ++row) {
			const auto& rrow = map.rows[row];
			auto& rdep = map.deps[row];
			rdep.resize(rrow.size());
			for (int col = 0; col < rrow.size(); ++col) {
				auto& deps = rdep[col];
				for (int nrow : {row - 1, row, row + 1})
					for (int ncol : {col - 1, col, col + 1})
						if (nrow != row || ncol != col) {
							try {
								if (map.rows.at(nrow).at(ncol) != Cell::Floor) {
									deps.push_back(std::pair{nrow, ncol});
								}
							} catch (const std::out_of_range&) {
							}
						}
			}
		}
		ostr << map.run();
	}
	void part2(std::ostream& ostr) const override {
		Map map = m_map;
		map.part2 = 1;
		map.deps.resize(m_map.rows.size());
		for (int row = 0; row < m_map.rows.size(); ++row) {
			const auto& rrow = map.rows[row];
			auto& rdep = map.deps[row];
			rdep.resize(rrow.size());
			for (int col = 0; col < rrow.size(); ++col) {
				if (rrow[col] == Cell::Floor) continue;
				auto& deps = rdep[col];
				for (int drow : {-1, 0, 1})
					for (int dcol : {-1, 0, 1})
						if (drow || dcol) {
							int lrow = row;
							int lcol = col;
							try {
								while (true) {
									lrow += drow;
									lcol += dcol;
									Cell here = m_map.rows.at(lrow).at(lcol);
									if (here == Cell::Floor) continue;
									deps.push_back(std::pair{lrow, lcol});
									break;
								}
							} catch (const std::out_of_range&) {
							}
						}
			}
		}
		ostr << map.run();
	}
};
}  // namespace

std::unique_ptr<AbstractSolver> AbstractSolver::Create() {
	return std::make_unique<Solver>();
}
}  // namespace aoc2020
