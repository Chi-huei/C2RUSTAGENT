#include <stdio.h>
#include <stdlib.h>

// Calculate factorial recursively
int factorial(int n) {
    if (n <= 1) return 1;
    return n * factorial(n - 1);
}

// Calculate fibonacci iteratively
int fibonacci(int n) {
    if (n <= 1) return n;
    int a = 0, b = 1, temp;
    for (int i = 2; i <= n; i++) {
        temp = a + b;
        a = b;
        b = temp;
    }
    return b;
}

int main() {
    printf("Factorial and Fibonacci Calculator\n");
    printf("===================================\n\n");
    
    for (int i = 0; i <= 10; i++) {
        printf("factorial(%2d) = %7d    fibonacci(%2d) = %3d\n", 
               i, factorial(i), i, fibonacci(i));
    }
    
    return 0;
}
