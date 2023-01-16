#!/bin/bash
UBCC=./target/x86_64-unknown-linux-musl/debug/ubcc
assert() {
  expected="$1"
  input="$2"

  ${UBCC} "$input" >tmp.s
  cc -o tmp tmp.s
  ./tmp
  actual="$?"

  if [ "$actual" = "$expected" ]; then
    echo "$input => $actual"
  else
    echo "$input => $expected expected, but got $actual"
    exit 1
  fi
}

assert_with_link() {
  file_name="$1"
  expected="$2"
  input="$3"

  ${UBCC} "$input" >tmp.s
  cc -c tmp.o tmp.s
  cc -c "$1".o "$1".c
  cc -o tmp tmp.o "$1".o
  ./tmp
  actual="$?"

  if [ "$actual" = "$expected" ]; then
    echo "$input => $actual"
  else
    echo "$input => $expected expected, but got $actual"
    exit 1
  fi
}

assert 0 "return 0;"
assert 42 "return 42;"

assert 21 "return 5 + 20 - 4;"
assert 47 'return 5 + 6 * 7;'

assert 15 'return 5 * (9 - 6);'
assert 4 'return (3 + 5) / 2;'

assert 0 'return 0 == 1;'
assert 1 'return 42 == 42;'
assert 1 'return 0 != 1;'
assert 0 'return 42 != 42;'

assert 1 'return 0 < 1;'
assert 0 'return 1 < 1;'
assert 0 'return 2 < 1;'
assert 1 'return 0 <= 1;'
assert 1 'return 1 <= 1;'
assert 0 'return 2 <= 1;'

assert 1 'return 1 > 0;'
assert 0 'return 1 > 1;'
assert 0 'return 1 > 2;'
assert 1 'return 1 >= 0;'
assert 1 'return 1 >= 1;'
assert 0 'return 1 >= 2;'

assert 4 'a = 2; return a + 2;'
assert 7 "\
  a = 2;\
  z = 5;\
  c = a + z;\
  return c;\
"
assert 7 "\
  foo = 2;\
  z = 5;\
  return foo + z;\
"

assert 54 "\
  foo = 4;\
  z = 5;\
  if (foo / 2 == 2) z = 50;\
  return foo + z;\
"

assert 110 "\
  foo = 10;\
  if (foo / 2 == 2) z = 50; else z = 100;\
  return foo + z;\
"

assert 10 "\
  i = 0;\
  while (i < 10) i = i + 1;\
  return i;\
"

assert 10 "\
  for (i = 0; i < 10; i = i + 1) 0;\
  return i;\
"

assert 10 "\
  for (i = 1; i < 10; i = i + 2) {\
    i = i - 1;\
  }\
  return i;\
"

assert_with_link "foo" 0 "foo(); return 0;"

# assert 55 '
#   sum(m, n) {
#     acc = 0;
#     for (i = m; i <= n; i = i + 1)
#       acc = acc + i;
#     return acc;
#   }

#   main() {
#     return sum(1, 10); // 55を返す
#   }
# '
