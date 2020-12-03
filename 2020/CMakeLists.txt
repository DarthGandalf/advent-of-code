cmake_minimum_required(VERSION 3.17)
project(AoC2020 LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED yes)
list(APPEND CMAKE_MODULE_PATH "${PROJECT_SOURCE_DIR}/cmake")
enable_testing()
list(APPEND CMAKE_CTEST_ARGUMENTS "--output-on-failure")
set(index_list "")

function(aoc_executable name)
	add_library(${name}_lib ${name}.cpp)
	add_executable(${name} empty.cpp)
	target_link_libraries(${name}_lib)
	target_link_libraries(${name} common ${name}_lib common)
	if(EMSCRIPTEN)
		target_link_libraries(${name} "--preload-file ${CMAKE_SOURCE_DIR}/input/2020/${name}.txt@input.txt")
		target_link_libraries(${name} "--preload-file ${CMAKE_SOURCE_DIR}/sprites@sprites")
		configure_file(day.html ${name}.html)
		target_compile_options(${name}_lib PUBLIC "-sUSE_SDL=2")
		target_compile_options(${name}_lib PUBLIC "-sUSE_SDL_TTF=2")
		target_compile_options(${name}_lib PUBLIC "-sUSE_SDL_IMAGE=2")
		set(index_list "${index_list}\n<li><a href=${name}.html>${name}</a></li>" PARENT_SCOPE)
	else()
		target_link_libraries(${name}_lib SDL2::SDL2
			PkgConfig::SDL2IMAGE
			PkgConfig::SDL2TTF)
		add_test(${name}_test bash -c "diff -u <(cat ${CMAKE_SOURCE_DIR}/answers/${name}.txt) <(${CMAKE_BINARY_DIR}/${name})")
	endif()
endfunction()

add_library(common common.cpp)

if(EMSCRIPTEN)
	include_directories(${CMAKE_SOURCE_DIR})
	target_sources(common PRIVATE web.cpp)
	target_link_libraries(common "-s USE_SDL=2")
	target_link_libraries(common "-s USE_SDL_TTF=2")
	target_link_libraries(common "-s USE_SDL_IMAGE=2")
	target_link_libraries(common "-s SDL2_IMAGE_FORMATS='[\"png\"]'")
	target_link_libraries(common "-s ASYNCIFY")
	target_link_libraries(common "--bind")
	target_compile_options(common PUBLIC "-sUSE_SDL=2")
	target_compile_options(common PUBLIC "-sUSE_SDL_TTF=2")
	target_compile_options(common PUBLIC "-sUSE_SDL_IMAGE=2")

	# https://stackoverflow.com/questions/57358039/how-to-see-the-underlying-compiler-linker-command-line-with-cmake-nmake
	set(CMAKE_CXX_USE_RESPONSE_FILE_FOR_INCLUDES 0 FORCE)
	set(CMAKE_CXX_USE_RESPONSE_FILE_FOR_LIBRARIES 0 FORCE)
	set(CMAKE_CXX_USE_RESPONSE_FILE_FOR_OBJECTS 0 FORCE)
else()
	target_sources(common PRIVATE native.cpp)
	find_package(SDL2 REQUIRED)
	find_package(PkgConfig REQUIRED)
	pkg_check_modules(SDL2IMAGE REQUIRED IMPORTED_TARGET SDL2_image)
	pkg_check_modules(SDL2TTF REQUIRED IMPORTED_TARGET SDL2_ttf)
	target_link_libraries(common SDL2::SDL2
		PkgConfig::SDL2IMAGE
		PkgConfig::SDL2TTF)
	include(CMakeRC)
endif()

aoc_executable(day1)
aoc_executable(day2)
aoc_executable(day3)
set(last_day 3)

if(EMSCRIPTEN)
	configure_file(index.html index.html)
else()
	file(GLOB input_files
		"${CMAKE_SOURCE_DIR}/sprites/*.png"
		"${CMAKE_SOURCE_DIR}/input/2020/*.txt")
	cmrc_add_resource_library(inputs ${input_files})
	target_link_libraries(common inputs)

	add_executable(example_test example_test.cpp common.cpp)
	target_link_libraries(example_test day${last_day}_lib)
endif()
