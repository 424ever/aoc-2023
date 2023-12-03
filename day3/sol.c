#include <ctype.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "sol.h"

#define MAXHITS 9
#define MAXGEARS 1000

struct gearhit {
  unsigned gearrow;
  unsigned gearcol;
  unsigned hits[MAXHITS];
  unsigned nhits;
};
static struct gearhit gearhits[MAXGEARS];
static unsigned ngearhits;
static struct {
  unsigned row;
  unsigned col;
} lasthit;

typedef int (*acceptfn)(int);

static int issymbol(int c) { return c != '.' && !isdigit(c); }
static int isasterisk(int c) { return c == '*'; }
/* stores the last hit in a global */
static unsigned count_adjacent_chars(unsigned linec, const char **lines,
                                     unsigned line, unsigned col, unsigned len,
                                     acceptfn f) {
  unsigned i, j;
  unsigned slen;
  unsigned count;
  unsigned minrow, maxrow, mincol, maxcol;

  count = 0;
  minrow = line == 0 ? 0 : line - 1;
  maxrow = line == (linec - 1) ? linec - 1 : line + 1;
  for (i = minrow; i <= maxrow; ++i) {
    slen = strlen(lines[i]);
    mincol = col == 0 ? 0 : col - 1;
    maxcol = col + len >= slen - 1 ? slen - 1 : col + len;
    for (j = mincol; j <= maxcol; ++j)
      if (f(lines[i][j])) {
        ++count;
        lasthit.row = i;
        lasthit.col = j;
      }
  }
  return count;
}

/* read a number from lines[l], starting at *iptr, such that when finished
 * line[*iptr] is after the last digit character, if the number does not border
 */
static unsigned parsenum(unsigned linec, const char **lines, unsigned i,
                         unsigned *jptr, acceptfn f, unsigned *hitcount) {
  unsigned num;
  unsigned len;
  unsigned start;

  *hitcount = 0;
  num = 0;
  len = 0;
  start = *jptr;
  while (isdigit(lines[i][*jptr])) {
    num *= 10;
    num += lines[i][*jptr] - '0';
    ++len;
    ++*jptr;
  }

  *hitcount = count_adjacent_chars(linec, lines, i, start, len, f);

  if (*hitcount > 0)
    return num;
  else
    return 0;
}

struct gearhit *find_gearhit(unsigned row, unsigned col) {
  unsigned i;
  for (i = 0; i < ngearhits; ++i) {
    if (gearhits[i].gearrow == row && gearhits[i].gearcol == col)
      return gearhits + i;
  }
  return NULL;
}

static void record_gearhit(unsigned row, unsigned col, unsigned num) {
  struct gearhit *hit;
  /* check if we already have an entry for this gear */
  hit = find_gearhit(row, col);
  if (!hit) {
    if (ngearhits == MAXGEARS) {
      fprintf(stderr, "max number of gears registered");
      abort();
    }
    hit = gearhits + ngearhits;
    hit->gearrow = row;
    hit->gearcol = col;
    hit->nhits = 0;
    ++ngearhits;
  }
  /* check there aren't too many hits for this already */
  if (hit->nhits == MAXHITS) {
    fprintf(stderr, "max number of hits for gear at %u/%u already saved", row,
            col);
    abort();
  }
  /* record */
  hit->hits[hit->nhits++] = num;
}

unsigned part1(unsigned int linec, const char **lines) {
  unsigned sum;
  unsigned i, j;
  char c;
  unsigned _;

  for (i = 0; i < linec; ++i) {
    for (j = 0; j < strlen(lines[i]); ++j) {
      c = lines[i][j];
      if (isdigit(c))
        sum += parsenum(linec, lines, i, &j, issymbol, &_);
    }
  }

  return sum;
}

unsigned part2(unsigned linec, const char **lines) {
  unsigned curnum;
  unsigned sum;
  unsigned prod;
  unsigned i, j;
  char c;
  unsigned hitcount;
  struct gearhit hit;

  for (i = 0; i < linec; ++i) {
    for (j = 0; j < strlen(lines[i]); ++j) {
      c = lines[i][j];
      if (isdigit(c)) {
        curnum = parsenum(linec, lines, i, &j, isasterisk, &hitcount);
        if (hitcount > 1) {
          fprintf(stderr, "more than 1 gear for number %u\n", curnum);
          abort();
        } else if (hitcount == 1) {
          record_gearhit(lasthit.row, lasthit.col, curnum);
        }
      }
    }
  }

  sum = 0;
  for (i = 0; i < ngearhits; ++i) {
    hit = gearhits[i];

    if (hit.nhits == 2) {
      prod = 1;
      for (j = 0; j < hit.nhits; ++j)
        prod *= hit.hits[j];
      sum += prod;
    }
  }

  return sum;
}
