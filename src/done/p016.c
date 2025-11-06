#include <stdio.h>
#include <math.h>
#include <string.h>

int main(){
    char num[1000]; 
    double a = pow(2,1000);

    sprintf(num , "%f", a);

    int cnt = 0;

    for(int i = 0; i < strlen(num); i++){
        if(num[i] <= 57 && num[i] >= 48) cnt += num[i] - '0';
    }
    printf("Cnt: %d\n",cnt);
}