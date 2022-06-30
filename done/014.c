#include <stdio.h>

int r = 0;

void Collatz(long i){
    r++;
    if(i == 1){ }
    else if(i % 2 == 0){ Collatz(i/2); }
    else if(i % 2 == 1){ Collatz(3*i+1); }
}

int main(){
    int s = 10;
    long bei = 13;
    for(long i = 13; i < 1000000; i++){
        r = 0;
        Collatz(i);
        if(r >= s){
            s = r;
            bei = i;
        }
    }
    printf("S: %d, bei %ld\n",s,bei);
}