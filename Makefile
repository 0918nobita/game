BINS := bin/compile_time bin/test
UNAME := $(shell uname)
CPPFLAGS := -O2 -Wall -Wextra
ifeq ($(UNAME), Darwin)
CPPFLAGS += -std=c++2a -stdlib=libc++
DYLIB := libNovelGameCpp.dylib
else
CPPFLAGS += -std=c++20
DYLIB := libNovelGameCpp.so
endif

.PHONY: build
build: $(DYLIB) $(BINS) src/fsharp/NovelGameFs.fsproj src/fsharp/Program.fs
	dotnet build src/fsharp

libNovelGameCpp.dylib: src/cpp/main.cpp
	g++ $(CPPFLAGS) -fPIC -shared -o libNovelGameCpp.dylib src/cpp/main.cpp \
		-isystem ${VULKAN_SDK}/include \
		-lglfw -L${VULKAN_SDK}/lib -lvulkan

libNovelGameCpp.so: src/cpp/main.cpp
	g++ $(CPPFLAGS) -fPIC -shared -o libNovelGameCpp.so src/cpp/main.cpp \
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

.PHONY: run
run: $(DYLIB) src/fsharp/NovelGameFs.fsproj src/fsharp/Program.fs save_data.sqlite3
	dotnet run -p src/fsharp

save_data.sqlite3: InitDB.fsx
	dotnet fsi InitDB.fsx

.PHONY: test
test: bin/test
	./bin/test

.PHONY: lint
lint:
	clang-format --dry-run --Werror ./src/*.cpp
	cpplint --recursive --verbose 5 ./src

.PHONY: format
format:
	clang-format --Werror -i ./src/*.cpp

.PHONY: clean
clean:
	rm -rf bin libNovelGameCpp.so libNovelGameCpp.dylib save_data.sqlite3
