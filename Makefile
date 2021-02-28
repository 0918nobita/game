BINS := bin/game bin/test
CPPFLAGS := -std=c++17 -O2 -Wall -Wextra
UNAME := $(shell uname)

all: $(BINS)

bin/game: src/main.cpp src/save_data.cpp
	mkdir -p bin
ifeq ($(UNAME), Darwin)
	g++ -stdlib=libc++ $(CPPFLAGS) -o $@ $? \
		-isystem ${VULKAN_SDK}/include \
		-lc++ -lglfw \
		-L${VULKAN_SDK}/lib -lvulkan
else
ifeq ($(UNAME), Linux)
	g++ $(CPPFLAGS) -pthread -o $@ $? \
		-isystem ${VULKAN_SDK}/include \
		-lglfw3 -ldl -lX11 \
		-L${VULKAN_SDK}/lib -lvulkan
endif
endif

bin/test: src/test.cpp
	mkdir -p bin
ifeq ($(UNAME), Darwin)
	g++ -stdlib=libc++ $(CPPFLAGS) -o $@ $? \
		-lc++ -lgtest -lgtest_main
else
ifeq ($(UNAME), Linux)
	g++ $(CPPFLAGS) -pthread -o $@ $? \
		-lgtest -lgtest_main
endif
endif

.PHONY: lint
lint:
	clang-format --dry-run --Werror ./src/*.cpp
	cpplint --recursive --verbose 5 \
		./src

.PHONY: format
format:
	clang-format --Werror -i ./src/*.cpp

.PHONY: clean
clean:
	rm -rf bin save.data
