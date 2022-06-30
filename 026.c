#include <stdio.h>
#include <math.h>
#include <string.h>

int check(double bruch){
    char num[1000]; 
    char arr[1000];
    int arrl = 1, cnt = 0;
    for(int i = 0; i < 1000; i++) num[i] = 0;
    sprintf(num , "%f", bruch);

    arr[0] = num[2];
    for(int i = 3; i < strlen(num); i++){


        for(int j = 0; j < arrl; j++){
            if(num[i] == arr[j]){

                int interncnt = 0;
                for(int k = 1; k < arrl; k++){
                    if(num[i+k] == arr[j+k]){
                        interncnt++;
                    }
                }
                if(interncnt == arrl){
                    cnt = arrl;
                }

            }else{
                arr[arrl++] = num[i];
                break;
            }
        }


    }

    printf("Char: ");
    for(int i = 0; i < strlen(num); i++){
        printf("%c",num[i]);
        
    }
    printf("\nL: %lu\n",strlen(num));
    
    return cnt;//length of sequencene 
}

int main(){

    int lastvalue = 0;
    for(int i = 2; i < 10; i++){
        printf("New---------\n");
        double b = 1. / i;
        printf("B: %f\n",b);
        printf("C: %d\n",check(b));
        // if(check(b) > lastvalue){

        // }
    }
    



    return 0;
}