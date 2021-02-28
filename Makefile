BINS := game load save test
CPPFLAGS := -std=c++17 -O2 -Wall -Wextra
UNAME := $(shell uname)

all: $(BINS)

game: main.cpp
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

test: test.cpp
ifeq ($(UNAME), Darwin)
	g++ -stdlib=libc++ $(CPPFLAGS) -o $@ $? \
		-lc++ -lgtest -lgtest_main
else
ifeq ($(UNAME), Linux)
	g++ $(CPPFLAGS) -pthread -o $@ $? \
		-lgtest -lgtest_main
endif
endif

save_data.pb.cc: save_data.pb.h
save_data.pb.h: save_data.proto
	protoc --cpp_out=./ $?

load: load.cpp save_data.pb.cc save_data.pb.h
	g++ -pthread -o $@ $? -lprotobuf

save: save.cpp save_data.pb.cc save_data.pb.h
	g++ -pthread -o $@ $? -lprotobuf

.PHONY: clean
clean:
	rm -rf $(BINS) *.pb.cc *.pb.h save.data
