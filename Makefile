BINS := game test
CPPFLAGS := -std=c++17 -O2 -Wall -Wextra
UNAME := $(shell uname)

all: $(BINS)

game: main.cpp
ifeq ($(UNAME), Darwin)
	g++ -stdlib=libc++ $(CPPFLAGS) -o game \
		main.cpp \
		-isystem ${VULKAN_SDK}/include \
		-lc++ -lglfw \
		-L${VULKAN_SDK}/lib -lvulkan
else
ifeq ($(UNAME), Linux)
	g++ $(CPPFLAGS) -pthread -o game \
		main.cpp \
		-isystem ${VULKAN_SDK}/include \
		-lglfw3 -ldl -lX11 \
		-L${VULKAN_SDK}/lib -lvulkan
endif
endif

test: test.cpp
ifeq ($(UNAME), Darwin)
	g++ -stdlib=libc++ $(CPPFLAGS) -o test test.cpp \
		-lc++ -lgtest -lgtest_main
else
ifeq ($(UNAME), Linux)
	g++ $(CPPFLAGS) -pthread -o test test.cpp \
		-lgtest -lgtest_main
endif
endif

.PHONY: clean
clean:
	rm -rf $(BINS)
