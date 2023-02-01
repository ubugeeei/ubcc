#!/bin/bash
UBCC=target/x86_64-unknown-linux-musl/debug/core
TEST_DATA_DIR=__test__/data

assert() {
  expected="$1"
  input="$2"

  ${UBCC} "$input" >target/main.s
  cc -o target/a.out target/main.s -no-pie
  ./target/a.out
  actual="$?"

  if [ "$actual" = "$expected" ]; then
    echo "$input => $actual"
  else
    echo "$input => $expected expected, but got $actual"
    exit 1
  fi
}

assert 0 "${TEST_DATA_DIR}/expr/single_int_lit.c"
assert 42 "${TEST_DATA_DIR}/expr/multi_int_lit.c"
assert 21 "${TEST_DATA_DIR}/expr/add_sub.c"
assert 47 "${TEST_DATA_DIR}/expr/mul.c"
assert 15 "${TEST_DATA_DIR}/expr/grouped.c"
assert 4 "${TEST_DATA_DIR}/expr/grouped2.c"
assert 12 "${TEST_DATA_DIR}/expr/assign.c"

assert 1 "${TEST_DATA_DIR}/comp/equivalence2.c"
assert 0 "${TEST_DATA_DIR}/comp/equivalence.c"
assert 1 "${TEST_DATA_DIR}/comp/inequivalence.c"
assert 0 "${TEST_DATA_DIR}/comp/inequivalence2.c"
assert 1 "${TEST_DATA_DIR}/comp/lt.c"
assert 0 "${TEST_DATA_DIR}/comp/lt2.c"
assert 0 "${TEST_DATA_DIR}/comp/lt3.c"
assert 1 "${TEST_DATA_DIR}/comp/lte.c"
assert 1 "${TEST_DATA_DIR}/comp/lte2.c"
assert 0 "${TEST_DATA_DIR}/comp/lte3.c"
assert 1 "${TEST_DATA_DIR}/comp/gt.c"
assert 0 "${TEST_DATA_DIR}/comp/gt2.c"
assert 0 "${TEST_DATA_DIR}/comp/gt3.c"
assert 1 "${TEST_DATA_DIR}/comp/gte.c"
assert 1 "${TEST_DATA_DIR}/comp/gte2.c"
assert 0 "${TEST_DATA_DIR}/comp/gte3.c"

assert 4 "${TEST_DATA_DIR}/declare/var.c"
assert 7 "${TEST_DATA_DIR}/declare/var2.c"
assert 7 "${TEST_DATA_DIR}/declare/var3.c"
assert 10 "${TEST_DATA_DIR}/declare/func.c"
assert 1 "${TEST_DATA_DIR}/declare/array/deref.c"
# assert 3 "${TEST_DATA_DIR}/declare/array/deref2.c"  # FIXME: this is not working
assert 1 "${TEST_DATA_DIR}/declare/array/deref3.c"
assert 1 "${TEST_DATA_DIR}/declare/array/index.c"
assert 2 "${TEST_DATA_DIR}/declare/array/index2.c"
assert 1 "${TEST_DATA_DIR}/declare/array/init.c"
assert 10 "${TEST_DATA_DIR}/declare/array/init2.c"
assert 0 "${TEST_DATA_DIR}/declare/string/init.c"
assert 65 "${TEST_DATA_DIR}/declare/string/head.c"
assert 70 "${TEST_DATA_DIR}/declare/string/index.c"

assert 54 "${TEST_DATA_DIR}/branch/if.c"
assert 110 "${TEST_DATA_DIR}/branch/if2.c"
assert 150 "${TEST_DATA_DIR}/branch/if3.c"

assert 10 "${TEST_DATA_DIR}/loop/while.c"
assert 10 "${TEST_DATA_DIR}/loop/for.c"

assert 3 "${TEST_DATA_DIR}/pointer/ref.c"
assert 3 "${TEST_DATA_DIR}/pointer/deref_assign.c"
assert 200 "${TEST_DATA_DIR}/pointer/ref_inc.c"
# assert 200 "${TEST_DATA_DIR}/pointer/ref_inc2.c"  # FIXME: this is not working
assert 100 "${TEST_DATA_DIR}/pointer/ref_dec.c"

assert 8 "${TEST_DATA_DIR}/builtin/sizeof.c"
assert 0 "${TEST_DATA_DIR}/comment/line.c"
assert 0 "${TEST_DATA_DIR}/comment/block.c"
