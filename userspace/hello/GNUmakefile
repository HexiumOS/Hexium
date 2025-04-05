.PHONY: all clean

all: ./bin/main

./bin/main: ./bin/main.o
	ld ./bin/main.o -o ./bin/main

./bin/main.o: main.asm
	mkdir -p ./bin
	nasm -f elf64 main.asm -o ./bin/main.o

clean:
	rm -rf ./bin
