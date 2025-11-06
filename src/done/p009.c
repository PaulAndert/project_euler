#include <stdio.h>
#include <math.h>

int main(){
    int a = 0, b = 0, c = 0;
    for(int i = 1; i < 1000; i++){
        for(int j = 1; j < 1000; j++){
            for(int k = 1; k < 1000; k++){
                if(i + j + k == 1000){
                    if(pow(i,2) + pow(j,2) == pow(k,2)){
                        printf("A = %d, B = %d, C = %d\nProdukt = %d\n",i,j,k,i*j*k);
                    }
                }
            }
        }
    }


    return 0;
}