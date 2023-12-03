#include <err.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "sol.h"

#define MAXLINES 200

int main(int argc, char *argv[]) {
  char *lines[MAXLINES];
  size_t nlines;
  size_t i;
  size_t linelen;
  FILE *f;

  if (argc < 2) {
    err(1, "not enough arguments");
  }

  f = fopen(argv[1], "r");
  if (!f) {
    perror("fopen");
    exit(2);
  }

  for (i = 0; i < MAXLINES; ++i)
    lines[i] = NULL;

  nlines = 0;
  while (getline(&lines[nlines], &linelen, f) != EOF) {
    /* remove \n */
    lines[nlines][strlen(lines[nlines]) - 1] = '\0';

    ++nlines;
    if (nlines == MAXLINES) {
      err(1, "not enough lines allocated");
    }
  }

  printf("%u\n", part1(nlines, (const char **)lines));
  printf("%u\n", part2(nlines, (const char **)lines));

  for (i = 0; i < nlines; ++i)
    free(lines[i]);
  fclose(f);
}
