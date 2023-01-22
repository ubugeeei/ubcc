#!/bin/bash
UBCC=./target/x86_64-unknown-linux-musl/debug/ubcc
assert() {
  expected="$1"
  input="$2"

  ${UBCC} "$input" >target/tmp.s
  cc -o target/tmp target/tmp.s
  ./target/tmp
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

  ${UBCC} "$input" >target/tmp.s
  cc -c target/tmp.s -o target/tmp.o
  cc -c lib/"$1".c -o target/"$1".o
  cc -o target/tmp target/tmp.o target/"$1".o
  ./target/tmp
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
    a = 2;
    return a + 2;
  }
"
assert 7 "\
  int main() {
    a = 2;
    z = 5;
    c = a + z;
    return c;
  }
"
assert 7 "\
  int main() {
    foo = 2;
    z = 5;
    return foo + z;
  }
"

assert 54 "\
  int main() {
    foo = 4;
    z = 5;
    if (foo / 2 == 2) z = 50;
    return foo + z;
  }
"

assert 110 "\
  int main() {
    foo = 10;
    if (foo / 2 == 2) z = 50; else z = 100;
    return foo + z;
  }
"

assert 10 "\
  int main() {
    i = 0;
    while (i < 10) i = i + 1;
    return i;
  }
"

assert 10 "\
  int main() {
    for (i = 0; i < 10; i = i + 1) {}
    return i;
  }
"

assert 10 "\
  int main() {
    for (i = 1; i < 10; i = i + 2) {
      i = i - 1;
    }
    return i;
  }
"

assert_with_link "foo" 0 "\
  int main() {
    foo();
    return 0;
  }
"

assert_with_link "bar" 0 "\
  int main() {
    bar(1, 2);
    return 0;
  }
"

assert 10 "\
  int foo(i) {
    return i;
  }
  int main() {
    a = foo(10);
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
