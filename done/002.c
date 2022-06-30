#include <stdio.h>

int main(){
    int c = 0, fib1 = 0, fib2 = 1, temp;
    for(int i = 0; (i < 50) && (temp < 4000000); i++){
        temp = fib1 + fib2;
        if(temp % 2 == 0) c += temp;
        fib2 = fib1;
        fib1 = temp;
    }
    printf("C: %d\n",c);
}