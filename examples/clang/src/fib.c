int fib(int n) {
    int a = 0, b = 1, t;
    for (int i = 0; i < n; i++) {
        t = a;
        a = b;
        b = t + b;
    }
    return a;
}

int main() {
    int n = 10;
    while (n > 0) {
        fib(n);
        n--;
    }
    return 0;
}
