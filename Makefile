BINS := bin/compile_time bin/game bin/test
UNAME := $(shell uname)
CPPFLAGS := -O2 -Wall -Wextra
ifeq ($(UNAME), Darwin)
CPPFLAGS += -std=c++2a -stdlib=libc++
else
CPPFLAGS += -std=c++20
endif

all: $(BINS)

obj/database.o: src/database.cpp src/database.hpp
	mkdir -p obj
	g++ $(CPPFLAGS) -c src/database.cpp -o obj/database.o

bin/game: src/main.cpp obj/database.o
	mkdir -p bin
ifeq ($(UNAME), Darwin)
	g++ $(CPPFLAGS) -o bin/game obj/database.o src/main.cpp \
		-isystem ${VULKAN_SDK}/include \
		-lc++ -lglfw -lsqlite3 \
		-L${VULKAN_SDK}/lib -lvulkan
else
ifeq ($(UNAME), Linux)
	g++ $(CPPFLAGS) -pthread -o bin/game obj/database.o src/main.cpp \
		-isystem ${VULKAN_SDK}/include \
		`pkg-config --libs glfw3` -ldl -lX11 -lsqlite3 \
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
	cpplint --recursive --verbose 5 ./src

.PHONY: format
format:
	clang-format --Werror -i ./src/*.cpp ./src/*.hpp

.PHONY: clean
clean:
	rm -rf bin obj save.data
