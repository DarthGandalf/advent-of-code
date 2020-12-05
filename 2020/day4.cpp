#include <charconv>
#include <iostream>
#include <range/v3/all.hpp>
#include <range/v3/view/subrange.hpp>
#include <unordered_map>

#include "common.h"
#include "ctre.hpp"

namespace aoc2020 {
namespace {
using namespace std::literals::string_view_literals;

struct Solver : AbstractSolver {
	std::vector<std::unordered_map<std::string_view, std::string_view>> m_data;
	void parse(std::string_view input) override {
		m_data = input | ranges::views::split("\n\n"sv) | to_string_view() |
				 ranges::views::transform([](std::string_view s) {
					 return s | ranges::views::split_when([](char c) {
								return c == ' ' || c == '\n';
							}) |
							to_string_view() |
							ranges::views::transform([](std::string_view kv) {
								auto middle = kv.find(':');
								return std::pair(kv.substr(0, middle),
				                                 kv.substr(middle + 1));
							}) |
							ranges::to<std::unordered_map<std::string_view,
			                                              std::string_view>>();
				 }) |
				 ranges::to<std::vector<
					 std::unordered_map<std::string_view, std::string_view>>>();
	}
	void part1(std::ostream& ostr) const override {
		ostr << ranges::count_if(m_data, [](const auto& pass) {
			return ranges::all_of(
				std::initializer_list<std::string_view>{
					"byr"sv, "iyr"sv, "eyr"sv, "hgt"sv, "hcl"sv, "ecl"sv,
					"pid"sv},
				[&](std::string_view z) { return pass.contains(z); });
		});
	}
	void part2(std::ostream& ostr) const override {
		ostr << ranges::count_if(m_data, [](const auto& pass) {
			try {
				for (auto&& [year, min, max] :
				     {std::tuple{pass.at("byr"sv), 1920, 2002},
				      std::tuple{pass.at("iyr"sv), 2010, 2020},
				      std::tuple{pass.at("eyr"sv), 2020, 2030}}) {
					if (year.length() != 4) return false;
					int y = 0;
					std::from_chars(year.data(), year.data() + year.length(),
					                y);
					if (y < min || y > max) return false;
				}
				std::string_view hgt = pass.at("hgt"sv);
				int hgti = 0;
				auto [hgtn, hgte] =
					std::from_chars(hgt.begin(), hgt.end(), hgti);
				if (hgte != std::errc{}) return false;
				std::string_view hgtu(hgtn, ranges::distance(hgtn, hgt.end()));
				if (hgtu == "cm"sv) {
					if (hgti < 150 || hgti > 193) return false;
				} else if (hgtu == "in"sv) {
					if (hgti < 59 || hgti > 76) return false;
				} else {
					return false;
				}
				using namespace ctre::literals;
				if (!"#[0-9a-f]{6}"_ctre.match(pass.at("hcl"sv))) return false;
				if (!"amb|blu|brn|gry|grn|hzl|oth"_ctre.match(pass.at("ecl"sv)))
					return false;
				if (!"[0-9]{9}"_ctre.match(pass.at("pid"sv))) return false;
				return true;
			} catch (const std::out_of_range&) {
				return false;
			}
		});
	}
};
}  // namespace

std::unique_ptr<AbstractSolver> AbstractSolver::Create() {
	return std::make_unique<Solver>();
}
}  // namespace aoc2020
