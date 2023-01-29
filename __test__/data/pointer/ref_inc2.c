int main() {
    int x = 100;
    int a = 200;
    int b = 300;
    int *p = &x;
    p = p + 2;
    return *p;
}