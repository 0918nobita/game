BINS := bin/compile_time bin/test
UNAME := $(shell uname)
CPPFLAGS := -O2 -Wall -Wextra
ifeq ($(UNAME), Darwin)
CPPFLAGS += -std=c++2a -stdlib=libc++
else
CPPFLAGS += -std=c++20
endif

.PHONY: build
build: lib/libgraphics.so $(BINS) src/NovelGame/NovelGame.fsproj src/NovelGame/Program.fs
	dotnet build src/NovelGame

lib/libgraphics.so: src/main.cpp
	mkdir -p lib
	g++ $(CPPFLAGS) -fPIC -shared -o lib/libgraphics.so src/main.cpp \
		-isystem ${VULKAN_SDK}/include \
		`pkg-config --libs glfw3` -ldl -lX11 \
		-L${VULKAN_SDK}/lib -lvulkan

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
	clang-format --dry-run --Werror ./src/*.cpp
	cpplint --recursive --verbose 5 ./src

.PHONY: format
format:
	clang-format --Werror -i ./src/*.cpp

.PHONY: clean
clean:
	rm -rf bin lib obj src/NovelGame/bin src/NovelGame/obj
