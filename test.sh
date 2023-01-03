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

assert 0 0
assert 42 42
assert 21 "5+20-4"

