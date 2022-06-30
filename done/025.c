#include <stdio.h>
#include <math.h>
#include <string.h>


int main(){

    int a[2000];
    for(int i = 0; i < 2000; i++) a[i] = 0;
    int b[2000];
    for(int i = 0; i < 2000; i++) b[i] = 0;
    b[0] = 1;
    int c[2000];

    int len[10000];
    len[0] = 1;
    len[1] = 1;
    printf("0: 0\n1: 1\n");
    for(int i = 2; i < 10000; i++){
        //printf("I: %d----\n",i);
        printf("%d: ",i);

        int über = 0;
        int increse = 0;
        for(int j = 0; j < len[i-1]+1; j++){

            int r = a[j] + b[j];
            //printf("R: %d mit Ü: %d\n",r,über);
            if(über == 1){
                //printf("Ü\n");
                r++;
                über = 0;
            }
            if(r > 9){
                //printf(">9\n");
                c[j] = r - 10;
                über = 1;
                if(j+1 == len[i-1]) increse++;
            }else if(r < 10){
                //printf("<10\n");
                c[j] = r;
            }
        }
        
        len[i] = len[i-1] + increse;

        // printf("L: %d\n",len[i]);

        // for(int j = len[i]-1; j >= 0; j--){
        //     printf("%d",c[j]);
        // }
        // printf("\n");

        if(len[i] == 1000) {
            printf("---->I: %d\n",i);
            i = 10000;
        }

        memcpy(a, b, sizeof(a));
        memcpy(b, c, sizeof(b));
        for(int i = 0; i < 2000; i++) c[i] = 0;
    }
    return 0;
}