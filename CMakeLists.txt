# CMakeList.txt : Top-level CMake project file, do global configuration
# and include sub-projects here.
#
cmake_minimum_required (VERSION 3.19)

project ("ppmdu_2")
set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED 20)
set(ppmdu_2_DEPS_DIRNAME "deps")

add_compile_options("-w")
add_compile_options("-Wno-register")

# Build included dependencies
add_subdirectory("${ppmdu_2_DEPS_DIRNAME}")
# Build main libraries
add_subdirectory ("ppmdu_2")

# Build apps
add_subdirectory ("ppmdu_audioutil")
#add_subdirectory ("ppmdu_gfxcrunch")
#add_subdirectory ("ppmdu_packfile_util")
#add_subdirectory ("ppmdu_pxutil")
#add_subdirectory ("ppmdu_statsutil")