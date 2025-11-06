#include <stdio.h>
#include <math.h>

int main(){
    int a = 0;
    for(int i = 1; i <= 100; i++){
        a += pow(i,2);
    }
    int b = (100 * (100 + 1)) / 2;
    b = pow(b,2);
    printf("Dif: %d\n",b-a);
}