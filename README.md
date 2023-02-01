# ubcc

A toy C CodeGenerator implemented by Rust.

## Usage

```sh
# first time
make

# build and testing
make build
make e2e
```

## Able to compile

### int literals

```c
int main() {
    return 0;
}
```

```c
int main() {
    return 42;
}
```

### binary expressions

```c
int main() {
    return 5 + 20 - 4;
}
```

```c
int main() {
    return 5 * (9 - 6);
}
```

```c
int main() {
    return 5 + 6 * 7;
}
```

```c
int main() {
    return (3 + 5) / 2;
}
```

### compare

```c
int main() {
    return 1 > 0;
}

```

```c
int main() {
    return 1 < 1;
}

```

```c
int main() {
    return 1 > 2;
}

```

```c
int main() {
    return 1 >= 2;
}

```

```c
int main() {
    return 0 != 1;
}

```

```c
int main() {
    return 2 <= 1;
}

```

```c
int main() {
    return 42 == 42;
}

```

```c
int main() {
    return 0 <= 1;
}

```

```c
int main() {
    return 0 < 1;
}

```

```c
int main() {
    return 1 <= 1;
}

```

```c
int main() {
    return 1 >= 0;
}

```

```c
int main() {
    return 42 != 42;
}

```

```c
int main() {
    return 2 < 1;
}

```

```c
int main() {
    return 1 > 1;
}

```

```c
int main() {
    return 0 == 1;
}

```

```c
int main() {
    return 1 >= 1;
}

```

### variables

```c
int main() {
    int a = 10;
    a = a + 2;
    return a;
}
```

```c
int main() {
    int a = 2;
    int z = 5;
    int c = a + z;
    return c;
}
```

```c
int main() {
    int foo = 2;
    int z = 5;
    return foo + z;
}
```

```c
int main() {
    int a = 2;
    return a + 2;
}
```

### pointer

```c
int main() {
    int a[2];
    *a = 1;
    *(a + 1) = 2;
    int *p;
    p = a;
    return *p + *(p + 1);
}
```

```c
int main() {
    int a[2];
    *a = 1;
    return *a;
}
```

```c
int main() {
    int x = 100;
    int a = 200;
    int b = 300;
    int *p = &x;
    p = p + 2;
    return *p;
}
```

```c
int main() {
    int x = 3;
    int *y = &x;
    return *y;
}
```

```c
int main() {
    int x = 100;
    int a = 200;

    int *p = &x;
    p = p + 1;
    return *p;
}
```

```c
int main() {
    int x = 100;
    int a = 200;

    int *p = &a;
    p = p - 1;

    return *p;
}
```

```c
int main() {
    int x = 0;
    int *y = &x;
    *y = 3;
    return x;
}
```

```c
int main() {
    int x = 100;
    int a = 200;

    int *p = &a;
    p = p - 1;

    return *p;
}
```

### collections

```c
int main() {
    int a[2] = {1, 2};
    return a[0];
}
```

```c
int main() {
    int a[2];
    *a = 1;
    return a[0];
}
```

```c
int main() {
    int a[10] = {1, 2, 3, 4, 5, 6, 7, 8, 9, 10};
    return a[9];
}
```

```c
int main() {
    int a[2];
    a[0] = 1;
    return a[0];
}
```

```c
int main() {
    int a[2];
    *a = 1;
    return *a;
}
```

```c
int main() {
    int a[2];
    a[0] = 1;
    a[1] = 2;
    return a[1];
}
```

```c
int main() {
    int a[2];
    a[0] = 1;
    return *a;
}
```

```c
int main() {
    char a[3] = "abc";
    return 0;
}
```

```c
int main() {
    char *a = "ABC";
    return *a;
}
```

```c
int main() {
    char *a = "ABCDEF";
    return a[5];
}
```

### branches

```c
int main() {
    int foo = 10;
    int z = 0;
    if (foo / 2 == 2)
        z = 50;
    else
        z = 100;
    return foo + z;
}
```

```c
int main() {
    int foo = 4;
    int z = 5;
    if (foo / 2 == 2) z = 50;
    return foo + z;
}
```

```c
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
```

### loop

```c
int main() {
    int i = 0;
    while (i < 10) i = i + 1;
    return i;
}
```

```c
int main() {
    int i = 0;
    for (i = 1; i < 10; i = i + 2) {
        i = i - 1;
    }
    return i;
}
```

### function

```c
int foo(int i) {
    return i;
}

int main() {
    int a = foo(10);
    return 10;
}
```

### builtin function

```c
int main() {
    int x = 0;
    return sizeof(x);
}
```

### comments

```c
int main() {
    /*
     * comment
     */
    return 0;
}
```

```c
int main() {
    // comment
    return 0;
}
```
