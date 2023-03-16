# intellij mangles indenting
# to manually fix indentation, run:
# sed -i 's/^    /\t/' Makefile

all: build test run

build:
	cargo build

run:
	cargo run

test:
	cargo test

pytest:
	python -m unittest solution_05.py