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
	bool part2 = false;

	Cell cell(int row, int col) const {
		try {
			return rows.at(row).at(col);
		} catch (const std::out_of_range&) {
			return Cell::Floor;
		}
	}

	int occupied_near(int row, int col) const {
		int result = 0;
		for (int nrow : {row - 1, row, row + 1})
			for (int ncol : {col - 1, col, col + 1})
				if (nrow != row || ncol != col)
					if (cell(nrow, ncol) == Cell::Busy) ++result;
		return result;
	}

	int occupied_far(int row, int col) const {
		int result = 0;
		for (int drow : {-1, 0, 1})
			for (int dcol : {-1, 0, 1})
				if (drow || dcol) {
					int lrow = row;
					int lcol = col;
					Cell found = Cell::Floor;
					try {
						while (true) {
							lrow += drow;
							lcol += dcol;
							Cell here = rows.at(lrow).at(lcol);
							if (here == Cell::Floor) continue;
							found = here;
							break;
						}
					} catch (const std::out_of_range&) {
					}
					if (found == Cell::Busy) ++result;
				}
		return result;
	}

	Cell next_cell(int row, int col) const {
		Cell now = cell(row, col);
		if (part2) {
			switch (now) {
				case Cell::Floor:
					return Cell::Floor;
				case Cell::Empty:
					return occupied_far(row, col) == 0 ? Cell::Busy
													   : Cell::Empty;
				case Cell::Busy:
					return occupied_far(row, col) >= 5 ? Cell::Empty
													   : Cell::Busy;
			}
		} else {
			switch (now) {
				case Cell::Floor:
					return Cell::Floor;
				case Cell::Empty:
					return occupied_near(row, col) == 0 ? Cell::Busy
														: Cell::Empty;
				case Cell::Busy:
					return occupied_near(row, col) >= 4 ? Cell::Empty
														: Cell::Busy;
			}
		}
	}

	Map next_step() const {
		Map next = *this;
		for (int row = 0; row < rows.size(); ++row) {
			for (int col = 0; col < rows[row].size(); ++col) {
				next.rows[row][col] = next_cell(row, col);
			}
		}
		return next;
	}

	friend std::ostream& operator<<(std::ostream& ostr, const Map& map) {
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
	bool operator==(const Map& other) const = default;

	int num_busy() const {
		return ranges::count(
			rows | ranges::views::for_each([](const auto& row) {
				return ranges::yield_from(ranges::views::all(row));
			}),
			Cell::Busy);
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
		while (true) {
			Map next_map = map.next_step();
			if (map == next_map) {
				ostr << map.num_busy();
				return;
			}
			map = std::move(next_map);
		}
	}
	void part2(std::ostream& ostr) const override {
		Map map = m_map;
		map.part2 = true;
		while (true) {
			Map next_map = map.next_step();
			if (map == next_map) {
				ostr << map.num_busy();
				return;
			}
			map = std::move(next_map);
		}
	}
};
}  // namespace

std::unique_ptr<AbstractSolver> AbstractSolver::Create() {
	return std::make_unique<Solver>();
}
}  // namespace aoc2020
