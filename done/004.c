#include <stdio.h>

int ispalindrom(int pal){
    if(pal % 10 == pal / 100000){
        if(pal / 10 % 10 == pal / 10000 % 10){
            if(pal / 100 % 10 == pal / 1000 % 10){
                return 1;
            }else return 0;
        }else return 0;
    }else return 0;
}

int main(){
    int c = 0;
    for(int i = 999; i > 0; i--){
        for(int j = 999; j > 0; j--){
            if(ispalindrom(i*j) == 1){
                if(c < i*j) c = i*j;
            }
        }
    }
    printf("C: %d\n",c);
    return 0;
}