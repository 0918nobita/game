CC=g++
CPPFLAGS=-std=c++20 -O2 -Wall -Wextra -pthread
BINS=game test

all: $(BINS)

game: main.cpp
	$(CC) $(CPPFLAGS) -o game \
		main.cpp \
		-isystem ${VULKAN_SDK}/include \
		-lglfw3 -ldl -lX11 \
		-L${VULKAN_SDK}/lib -lvulkan

test: test.cpp
	$(CC) $(CPPFLAGS) -o test test.cpp -lgtest -lgtest_main

.PHONY: clean
clean:
	rm -rf $(BINS)
