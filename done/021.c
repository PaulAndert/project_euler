#include <stdio.h>
#include <math.h>
#include <string.h>

int zerlegen(int wert){
    int divisor = 0;
    for(int i = 1; i < wert; i++){
        if(wert % i == 0){
            divisor += i;
        }
    }
    return divisor;
}

int main(){
    int num[1000];
    int w = 0;
    for(int i = 1; i <= 10000; i++){
        int a = zerlegen(i);
        int b = zerlegen(a);
        if(i == b && i != a){
            w += i;
        }
    }  
    printf("W: %d\n",w);      
    return 0;
}