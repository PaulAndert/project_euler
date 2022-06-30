#include <stdio.h>
#include <math.h>

int zerlegen(long triangle){
    int divisor = 0;
    int n = triangle;
    for(int i = 1; i < n; i++){
        if(triangle % i == 0){
            divisor += 2;
            n = triangle / i;
        }
    }
    return divisor;
}

int main(){
    long triangle = 1;
    int c = 0;
    for(int i = 2; i < 1000000; i++){
        triangle += i;
        int ret = zerlegen(triangle);
        if(ret > 500){
            printf("%d: T: %ld\n",i,triangle);
            return 0;
        }
    }
    return 0;
}