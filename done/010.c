#include <stdio.h>
#include <math.h>

int isprim(int i){
    int cnt = 0;
    for(int j = 1; j <= i; j++) if(i % j == 0) cnt++;
    if(cnt == 2) return 1;
    else return 0;
}

int main(){
    double sum = 328;
    for(int i = 51; i < 2000000; i += 2){
        if(i % 3 != 0) if(i % 5 != 0) if(i % 7 != 0) if(i % 11 != 0) if(i % 13 != 0) if(i % 17 != 0) if(i % 19 != 0) if(i % 23 != 0) if(i % 29 != 0) if(i % 31 != 0) if(i % 37 != 0) if(i % 41 != 0) if(i % 43 != 0) if(i % 47 != 0) if(isprim(i) == 1) sum += (double) i;
    }
    printf("P: %f\n",sum);

    return 0;
}