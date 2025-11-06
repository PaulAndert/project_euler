#include <stdio.h>

int main(){
    for(int i = 1; i < 500000000; i++){
        int cnt = 0;
        for(int j = 1; j <= 20; j++) if(i % j == 0) cnt++;
        if(cnt == 20){
            printf("C: %d\n",i);
            return 0;
        }
    }
}