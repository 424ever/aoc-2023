#include <stdio.h>
#include <stdlib.h>

#include "sol.h"

int has_adjacent_symbol(unsigned linec, const char **lines, unsigned line,
                        unsigned col);

int main(void) {
  unsigned i, j;
  unsigned sol;
  const char *lines[] = {"467..114..", "...*......", "..35..633.", "......#...",
                         "617*......", ".....+.58.", "..592.....", "......755.",
                         "...$.*....", ".664.598.."};

  sol = part1(10, lines);
  if (sol != 4361) {
    printf("test 1 failed (%u).\n", sol);
    exit(1);
  }
  printf("test 1 passed.\n");

  sol = part2(10, lines);
  if (sol != 467835) {
    printf("test 2 failed (%u).\n", sol);
    exit(1);
  }
  printf("test 2 passed.\n");
}
