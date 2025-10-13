#include <stdio.h>
#include <stdlib.h>
#include <math.h>
int main() {
    int N = 10000000, count = 0;
    for (int i = 0; i < N; i++) {
        double x = ((double)rand()/RAND_MAX - 0.5) * 1e20;
        double y = ((double)rand()/RAND_MAX - 0.5) * 1e20;
        double z = ((double)rand()/RAND_MAX - 0.5) * 1e20;
        if ((x + (y + z)) == ((x + y) + z))
            count++;
    }
    printf("%f%%\n", 100.0 * count / N);
}
