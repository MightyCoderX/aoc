CC := gcc
CFLAGS := -std=c99 -Wall -Wextra -pedantic -Wfatal-errors

part1: main.c
	${CC} ${CFLAGS} $^ -o $@ -DPART=1

part2: main.c
	${CC} ${CFLAGS} $^ -o $@ -DPART=2

.PHONY: clean
clean:
	rm part1 part2

