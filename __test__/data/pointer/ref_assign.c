int main() {
    int x = 0;
    int *y = &x;
    *y = 3;
    return x;
}