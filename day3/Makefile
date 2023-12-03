.PHONY: all test clean

all: sol 

sol: sol.c sol.h run.c
	$(CC) -o sol sol.c run.c

test: _test
	./_test

_test: sol.c sol.h test.c
	$(CC) -g -o _test sol.c test.c

clean:
	rm -f _test sol
