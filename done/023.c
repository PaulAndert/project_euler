#include <stdio.h>
#include <math.h>
#include <string.h>

int abundant(int wert){
    int divisor = 0;
    for(int i = 1; i < wert; i++){
        if(wert % i == 0){
            divisor += i;
        }
    }
    if(divisor > wert) return 1;
    else return 0;
}
int main(){
    int list[30000];
    int a = 0;

    for(int i = 1; i < 30000; i++) if(abundant(i) == 1) list[a++] = i;

    int array[28123];
    for(int i = 0; i < 28123; i++) array[i] = 1;

    for(int i = 0; i < a; i++){
        for(int j = 0; j < a; j++){
            int b = list[i] + list[j] ;
            if(b <= 28123) array[b] = 0;
            else j = 28123;
        }
    }
    int wert = 0;
    for(int i = 0; i < 28123; i++) if(array[i] == 1) wert += i;

    printf("W: %d\n",wert);
    return 0;
}