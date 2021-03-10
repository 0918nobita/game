BINS := bin/compile_time bin/game bin/test
UNAME := $(shell uname)
CPPFLAGS := -O2 -Wall -Wextra
ifeq ($(UNAME), Darwin)
CPPFLAGS += -std=c++2a -stdlib=libc++
else
CPPFLAGS += -std=c++20
endif

all: $(BINS)

obj/save_data.o: src/save_data.cpp src/save_data.hpp
	mkdir -p obj
	g++ $(CPPFLAGS) -c src/save_data.cpp -o obj/save_data.o

bin/game: src/main.cpp obj/save_data.o
	mkdir -p bin
ifeq ($(UNAME), Darwin)
	g++ $(CPPFLAGS) -o bin/game src/main.cpp obj/save_data.o \
		-isystem ${VULKAN_SDK}/include \
		-lc++ -lglfw \
		-L${VULKAN_SDK}/lib -lvulkan
else
ifeq ($(UNAME), Linux)
	g++ $(CPPFLAGS) -pthread -o bin/game src/main.cpp obj/save_data.o \
		-isystem ${VULKAN_SDK}/include \
		`pkg-config --libs glfw3` -ldl -lX11 \
		-L${VULKAN_SDK}/lib -lvulkan
endif
endif

bin/compile_time: src/compile_time.cpp
	mkdir -p bin
	g++ $(CPPFLAGS) -o bin/compile_time src/compile_time.cpp

bin/test: src/test.cpp
	mkdir -p bin
ifeq ($(UNAME), Darwin)
	g++ $(CPPFLAGS) -o bin/test src/test.cpp \
		-lc++ -lgtest -lgtest_main
else
ifeq ($(UNAME), Linux)
	g++ $(CPPFLAGS) -pthread -o bin/test src/test.cpp \
		-lgtest -lgtest_main
endif
endif

.PHONY: lint
lint:
	clang-format --dry-run --Werror ./src/*.cpp ./src/*.hpp
	cpplint --recursive --verbose 5 \
		./src

.PHONY: format
format:
	clang-format --Werror -i ./src/*.cpp ./src/*.hpp

.PHONY: clean
clean:
	rm -rf bin obj save.data
