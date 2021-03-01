BINS := bin/game bin/test
CPPFLAGS := -std=c++17 -O2 -Wall -Wextra
UNAME := $(shell uname)

all: $(BINS)

obj/save_data.o: src/save_data.cpp src/save_data.hpp
	mkdir -p obj
	g++ $(CPPFLAGS) -c src/save_data.cpp -o obj/save_data.o

bin/game: src/main.cpp obj/save_data.o
	mkdir -p bin
ifeq ($(UNAME), Darwin)
	g++ -stdlib=libc++ $(CPPFLAGS) -o bin/game src/main.cpp obj/save_data.o \
		-isystem ${VULKAN_SDK}/include \
		-lc++ -lglfw \
		-L${VULKAN_SDK}/lib -lvulkan
else
ifeq ($(UNAME), Linux)
	g++ $(CPPFLAGS) -pthread -o bin/game src/main.cpp obj/save_data.o \
		-isystem ${VULKAN_SDK}/include \
		-lglfw3 -ldl -lX11 \
		-L${VULKAN_SDK}/lib -lvulkan
endif
endif

bin/test: src/test.cpp
	mkdir -p bin
ifeq ($(UNAME), Darwin)
	g++ -stdlib=libc++ $(CPPFLAGS) -o bin/test src/test.cpp \
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
