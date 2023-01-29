int main() {
    int x = 100;
    int a = 200;

    int *p = &a;
    p = p - 1;

    return *p;
}