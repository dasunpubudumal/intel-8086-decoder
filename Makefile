rust-build:
	cargo build

cpp-build:
	g++ -std=c++20 cpp-impl/main.cpp -o main
