#!/bin/bash
UBCC=./target/x86_64-unknown-linux-musl/debug/ubcc
assert() {
  expected="$1"
  input="$2"

  ${UBCC} "$input" > tmp.s
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

assert 0 0
assert 42 42

assert 21 "5+20-4"
assert 47 '5+6*7'

assert 15 '5*(9-6)'
assert 4 '(3+5)/2'

assert 0 '0==1'
assert 1 '42==42'
assert 1 '0!=1'
assert 0 '42!=42'

assert 1 '0<1'
assert 0 '1<1'
assert 0 '2<1'
assert 1 '0<=1'
assert 1 '1<=1'
assert 0 '2<=1'

assert 1 '1>0'
assert 0 '1>1'
assert 0 '1>2'
assert 1 '1>=0'
assert 1 '1>=1'
assert 0 '1>=2'

assert 55 '
  sum(m, n) {
    acc = 0;
    for (i = m; i <= n; i = i + 1)
      acc = acc + i;
    return acc;
  }

  main() {
    return sum(1, 10); // 55を返す
  }
'
