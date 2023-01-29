#!/bin/bash
UBCC=./target/x86_64-unknown-linux-musl/debug/core
assert() {
  expected="$1"
  input="$2"

  ${UBCC} "$input" >target/main.s
  cc -o target/a.out target/main.s
  ./target/a.out
  actual="$?"

  if [ "$actual" = "$expected" ]; then
    echo "$input => $actual"
  else
    echo "$input => $expected expected, but got $actual"
    exit 1
  fi
}

assert 0 "\
  int main() {
    return 0;
  }
"
assert 42 "\
  int main() {
    return 42;
  }
"

assert 21 "\
  int main() {
    return 5 + 20 - 4;
  }"
assert 47 "\
  int main() {
    return 5 + 6 * 7;
  }
"

assert 15 "\
  int main() {
    return 5 * (9 - 6);
  }"
assert 4 "\
  int main() {
    return (3 + 5) / 2;
  }"

assert 0 "\
  int main() {
    return 0 == 1;
  }
"
assert 1 "\
  int main() {
    return 42 == 42;
  }
"
assert 1 "\
  int main() {
    return 0 != 1;
  }
"
assert 0 "\
  int main() {
    return 42 != 42;
  }
"

assert 1 "\
  int main() {
    return 0 < 1;
  }
"
assert 0 "\
  int main() {
    return 1 < 1;
  }
"
assert 0 "\
  int main() {
    return 2 < 1;
  }
"
assert 1 "\
  int main() {
    return 0 <= 1;
  }
"
assert 1 "\
  int main() {
    return 1 <= 1;
  }
"
assert 0 "\
  int main() {
    return 2 <= 1;
  }
"

assert 1 "\
  int main() {
    return 1 > 0;
  }
"
assert 0 "\
  int main() {
    return 1 > 1;
  }
"
assert 0 "\
  int main() {
    return 1 > 2;
  }
"
assert 1 "\
  int main() {
    return 1 >= 0;
  }
"
assert 1 "\
  int main() {
    return 1 >= 1;
  }
"
assert 0 "\
  int main() {
    return 1 >= 2;
  }
"

assert 4 "\
  int main() {
    int a = 2;
    return a + 2;
  }
"
assert 7 "\
  int main() {
    int a = 2;
    int z = 5;
    int c = a + z;
    return c;
  }
"
assert 7 "\
  int main() {
    int foo = 2;
    int z = 5;
    return foo + z;
  }
"

assert 54 "\
  int main() {
    int foo = 4;
    int z = 5;
    if (foo / 2 == 2) z = 50;
    return foo + z;
  }
"

assert 110 "\
  int main() {
    int foo = 10;
    int z = 0;
    if (foo / 2 == 2) z = 50; else z = 100;
    return foo + z;
  }
"

assert 150 "\
  int main() {
    int foo = 100;
    int z;
    if (foo / 2 == 50) {
      z = 50;
    } else {
      z = 100;
    }
    return foo + z;
  }
"

assert 10 "\
  int main() {
    int i = 0;
    while (i < 10) i = i + 1;
    return i;
  }
"

assert 10 "\
  int main() {
    int i = 0;
    for (i = 1; i < 10; i = i + 2) {
      i = i - 1;
    }
    return i;
  }
"

assert 10 "\
  int foo(int i) {
    return i;
  }
  int main() {
    int a = foo(10);
    return 10;
  }
"

assert 12 "\
  int main() {
    int a = 10;
    a = a + 2;
    return a;
  }
"

assert 3 "\
  int main() {
    int x = 3;
    int *y = &x;
    return *y;
  }
"

assert 3 "\
  int main() {
    int x = 0;
    int *y = &x;
    *y = 3;
    return x;
  }
"

assert 200 "\
  int main() {
    int x = 100;
    int a = 200;

    int *p = &x;
    p = p + 1;
    return *p;
  }
"

# FIXME: this is not working
# assert 300 "\
#   int main() {
#     int x = 100;
#     int a = 200;
#     int b = 300;
#     int *p = &x;
#     p = p + 2;
#     return *p;
#   }
# "

assert 100 "\
  int main() {
    int x = 100;
    int a = 200;

    int *p = &a;
    p = p - 1;

    return *p;
  }
"

assert 8 "\
  int main() {
    int x = 0;
    return sizeof(x);
  }
"

assert 1 "\
  int main() {
    int a[2];
    *a = 1;
    return *a;
  }
"

# FIXME: this is not working
# assert 3 "\
#   int main() {
#     int a[2];
#     *a = 1;
#     *(a + 1) = 2;
#     int *p;
#     p = a;
#     return *p + *(p + 1);
#   }
# "

assert 1 "\
  int main() {
    int a[2];
    a[0] = 1;
    return a[0];
  }
"

assert 2 "\
  int main() {
    int a[2];
    a[0] = 1;
    a[1] = 2;
    return a[1];
  }
"

assert 1 "\
  int main() {
    int a[2];
    a[0] = 1;
    return *a;
  }
"

assert 1 "\
  int main() {
    int a[2];
    *a = 1;
    return *a;
  }
"

assert 1 "\
  int main() {
    int a[2];
    *a = 1;
    return a[0];
  }
"

assert 1 "\
  int main() {
    int a[2] = { 1, 2 };
    return a[0];
  }
"

assert 10 "\
  int main() {
    int a[10] = { 1, 2, 3, 4, 5, 6, 7, 8, 9, 10 };
    return a[9];
  }
"

assert 0 "\
  int main() {
    // comment
    return 0;
  }
"

assert 0 "\
  int main() {
    /*
     * comment
     */
    return 0;
  }
"
