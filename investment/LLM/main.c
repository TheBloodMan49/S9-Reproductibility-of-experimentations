// banker_investment.c
// Compile: gcc -O2 -std=c11 banker_investment.c -lm
// Run: ./a.out        (uses n = 50)
// or:  ./a.out 50

#include <stdio.h>
#include <stdlib.h>
#include <math.h>

int main(int argc, char *argv[]) {
    int n = 50; // default years
    if (argc > 1) n = atoi(argv[1]);
    if (n < 0) { fprintf(stderr, "n must be >= 0\n"); return 1; }

    long double e = expl(1.0L);

    // Method 1: iterate recurrence A_i = (A_{i-1} - 1) * i
    long double A = e;
    for (int i = 1; i <= n; ++i) {
        A = (A - 1.0L) * (long double)i;
    }
    long double final_rec = A - 1.0L;

    // Method 2: closed form final = n! * (e - sum_{k=0}^{n-1} 1/k!) - 1
    long double sum = 0.0L;
    long double term = 1.0L; // term = 1/k!
    for (int k = 0; k <= n-1; ++k) {
        sum += term;
        term /= (long double)(k + 1); // next term becomes 1/(k+1)!
    }
    // compute n!
    long double fact = 1.0L;
    for (int i = 1; i <= n; ++i) fact *= (long double)i;

    long double final_closed = fact * (e - sum) - 1.0L;

    printf("Years n = %d\n", n);
    printf("Start: e = %.18Lf\n", e);
    printf("Final (recurrence) : %.30Lf\n", final_rec);
    printf("Final (closed form) : %.30Lf\n", final_closed);
    return 0;
}
