int main() {
    int x = 0;
    for (int i = 0; i < 10; i++) {
        for (int j = 0; j < 10; j++) {
            x++;
        }
    }

    do {
        x--;
    } while (x > 0);

    return 0;
}
