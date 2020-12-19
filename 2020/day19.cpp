#include <chrono>
#include <range/v3/all.hpp>

#include "common.h"
#include "fmt/core.h"
#include "peglib.h"

#ifdef __EMSCRIPTEN__
#include <emscripten/val.h>
#else
#include "subprocess.hpp"
#endif

namespace aoc2020 {
namespace {
using namespace std::literals::string_view_literals;

struct Solver : AbstractSolver {
	std::string_view m_rules;
	std::string_view m_strings;
	void parse(std::string_view input) override {
		auto input_range =
			input | ranges::views::split("\n\n"sv) | to_string_view();
		auto input_iter = input_range.begin();
		m_rules = *input_iter++;
		m_strings = *input_iter++;
	}
	void part1(std::ostream& ostr) const override {
		std::string fixed_rules = "root <- x0\n";
		for (std::string_view line :
		     m_rules | ranges::views::split('\n') | to_string_view()) {
			auto range = line | ranges::views::split(' ') | to_string_view();
			auto name = ranges::front(range);
			fixed_rules +=
				fmt::format("x{} <-", name.substr(0, name.size() - 1));
			for (std::string_view token : range | ranges::views::drop(1)) {
				switch (token[0]) {
					case '|':
						fixed_rules += " /";
						break;
					case '"':
						fixed_rules += fmt::format(" {}", token);
						break;
					default:
						fixed_rules += fmt::format(" x{}", token);
				}
			}
			fixed_rules += '\n';
		}
		peg::parser parser;
		/*parser.log = [&](size_t line, size_t col, const std::string& msg) {
		    ostr << line << ":" << col << ": " << msg << "\n";
		};*/
		if (!parser.load_grammar(fixed_rules.c_str())) {
			ostr << "can't parse grammar";
			return;
		}
		ostr << ranges::count_if(
			m_strings | ranges::views::split('\n') | to_string_view(),
			[&](std::string_view line) {
				return parser.parse(std::string(line).c_str());
			});
	}
	void part2(std::ostream& ostr) const override {
		std::string program =
			"use v6;\n"
			"use MONKEY-SEE-NO-EVAL;\n"
			"grammar G {\n"
			"regex TOP { <x0> }\n"
			"regex x8 { <x42> | <x42> <x8> }\n"
			"regex x11 { <x42> <x31> | <x42> <x11> <x31> }\n";
		for (std::string_view line :
		     m_rules | ranges::views::split('\n') | to_string_view()) {
			auto range = line | ranges::views::split(' ') | to_string_view();
			auto name = ranges::front(range);
			if (name == "8:"sv || name == "11:"sv) continue;
			program +=
				fmt::format("regex x{} ", name.substr(0, name.size() - 1));
			program += '{';
			for (std::string_view token : range | ranges::views::drop(1)) {
				switch (token[0]) {
					case '|':
						program += " |";
						break;
					case '"':
						program += fmt::format(" {}", token);
						break;
					default:
						program += fmt::format(" <x{}>", token);
				}
			}
			program += " }\n";
		}
		program += "}\n my @input = (\n";
		for (auto s :
		     m_strings | ranges::views::split('\n') | to_string_view()) {
			program += "'" + std::string(s) + "',\n";
		}
		program +=
			");\n"
			"my $x = 0; for @input -> $item { $x++ if G.parse($item); }\n";
#ifdef __EMSCRIPTEN__
		using emscripten::val;
		auto field = val::global("document").call<val>("getElementById", val("day19output"));
		field.set("value", "");
		program += R"(EVAL(:lang<JavaScript>, "document.getElementById('day19output').value = '$x';"))";
		val::global("evalP6")(program);
		while (true) {
			yield(std::chrono::milliseconds{1});
			std::string result = field["value"].as<std::string>();
			if (!result.empty()) {
				ostr << result;
				return;
			}
		}
#else
		program += "print $x;\n";
		auto process =
			subprocess::Popen({"raku"}, subprocess::input{subprocess::PIPE},
		                      subprocess::output{subprocess::PIPE});
		auto res = process.communicate(program.data(), program.size());
		ostr << std::string_view(res.first.buf.data(), res.first.length);
#endif
		return;
	}
};
}  // namespace

std::unique_ptr<AbstractSolver> AbstractSolver::Create() {
	return std::make_unique<Solver>();
}
}  // namespace aoc2020

// 216 too low
