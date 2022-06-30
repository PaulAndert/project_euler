#include <stdio.h>
#include <math.h>

int isprim(int i){
    int cnt = 0;
    for(int j = 1; j <= i; j++) if(i % j == 0) cnt++;
    if(cnt == 2) return 1;
    else return 0;
}

int main(){
    int cnt = 0;
    for(int i = 1; i <= 1000000; i++){
        if(isprim(i) == 1){
            cnt++;
        }
        if(cnt == 10001){
            printf("P: %d\n",i);
            return 0;
        }
    }
    return 0;
}