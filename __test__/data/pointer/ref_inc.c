int main() {
    int x = 100;
    int a = 200;

    int *p = &x;
    p = p + 1;
    return *p;
}