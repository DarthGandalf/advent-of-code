#include <charconv>
#include <range/v3/all.hpp>
#include <range/v3/view/subrange.hpp>
#include <regex>
#include <unordered_map>

#include "common.h"

namespace aoc2020 {
namespace {

struct Solver : AbstractSolver {
	std::vector<std::unordered_map<std::string, std::string>> m_data;
	void parse(std::string_view input) override {
		// TODO: use CTRE or whatever instead of std::regex. This string
		// conversion doesn't make any sense here.
		std::string inp(input);
		std::regex split_re("\n\n");
		std::regex split2_re("\\s");
		m_data =
			ranges::subrange(std::sregex_token_iterator(inp.begin(), inp.end(),
		                                                split_re, -1),
		                     std::sregex_token_iterator()) |
			ranges::views::transform([&](const std::string& s) {
				return ranges::subrange(std::sregex_token_iterator(
											s.begin(), s.end(), split2_re, -1),
			                            std::sregex_token_iterator()) |
					   ranges::views::transform([](const std::string& kv) {
						   auto middle = kv.find(':');
						   return std::pair(kv.substr(0, middle),
				                            kv.substr(middle + 1));
					   }) |
					   ranges::to<
						   std::unordered_map<std::string, std::string>>();
			}) |
			ranges::to<
				std::vector<std::unordered_map<std::string, std::string>>>();
	}
	void part1(std::ostream& ostr) const override {
		ostr << ranges::count_if(m_data, [](const auto& pass) {
			// TODO: range from init list without vector?
			return ranges::all_of(
				std::vector<std::string>{"byr", "iyr", "eyr", "hgt", "hcl",
			                             "ecl", "pid"},
				[&](std::string_view z) {
					// TODO: map with heterogenuous lookup?
					return pass.contains(std::string(z));
				});
		});
	}
	void part2(std::ostream& ostr) const override {
		using namespace std::literals::string_view_literals;
		std::regex hcl_re("#[0-9a-f]{6}");
		std::regex ecl_re("amb|blu|brn|gry|grn|hzl|oth");
		std::regex pid_re("[0-9]{9}");
		ostr << ranges::count_if(m_data, [&](const auto& pass) {
			try {
				for (auto&& [year, min, max] :
				     {std::tuple{pass.at("byr"), 1920, 2002},
				      std::tuple{pass.at("iyr"), 2010, 2020},
				      std::tuple{pass.at("eyr"), 2020, 2030}}) {
					if (year.length() != 4) return false;
					int y = 0;
					std::from_chars(year.data(), year.data() + year.length(),
					                y);
					if (y < min || y > max) return false;
				}
				std::string_view hgt = pass.at("hgt");
				int hgti = 0;
				auto [hgtn, hgte] =
					std::from_chars(hgt.begin(), hgt.end(), hgti);
				if (hgte != std::errc{}) return false;
				if (hgtn == "cm"sv) {
					if (hgti < 150 || hgti > 193) return false;
				} else if (hgtn == "in"sv) {
					if (hgti < 59 || hgti > 76) return false;
				} else {
					return false;
				}
				if (!std::regex_match(pass.at("hcl"), hcl_re)) return false;
				if (!std::regex_match(pass.at("ecl"), ecl_re)) return false;
				if (!std::regex_match(pass.at("pid"), pid_re)) return false;
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
