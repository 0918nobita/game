cmake_minimum_required(VERSION 3.0)

project(Game CXX)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_FLAGS "-O2 -Wall -Wextra")

find_package(Vulkan REQUIRED)
find_package(glfw3 3.3 REQUIRED)
find_package(GTest REQUIRED)

add_executable(Game main.cpp)
include_directories(SYSTEM ${Vulkan_INCLUDE_DIRS})
target_link_libraries(Game ${Vulkan_LIBRARIES} glfw)

add_executable(Test test.cpp)
target_link_libraries(Test PRIVATE GTest::GTest GTest::Main)
