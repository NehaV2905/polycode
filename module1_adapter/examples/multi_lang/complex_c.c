#include <stdio.h>

#define MAX(a, b) ((a) > (b) ? (a) : (b))
#define SQUARE(x) ((x) * (x))

typedef int (*operation_fn)(int, int);

int add(int a, int b) {
    return a + b;
}

int multiply(int a, int b) {
    return a * b;
}

int apply_operation(operation_fn op, int x, int y) {
    return op(x, y);
}

int main() {
    int result1 = add(5, 3);
    printf("%d\n", result1);
    
    int result2 = apply_operation(multiply, 4, 5);
    printf("%d\n", result2);
    
    int max_val = MAX(10, 20);
    printf("%d\n", max_val);
    
    int squared = SQUARE(5);
    printf("%d\n", squared);
    
    return 0;
}
