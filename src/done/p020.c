#include <stdio.h>
#include <math.h>
#include <string.h>

int main(){
    char num[1000]; 
    double a = 1;
    
    for(int i = 1; i <= 100; i++) a = a * i;

    sprintf(num , "%f", a);

    int cnt = 0;

    // Weil C double harte rundungsfehler hat muss man die zahl mit einem richtigen rechner berechnen
    char b[200] = "93326215443944152681699238856266700490715968264381621468592963895217599993229915608941463976156518286253697920827223758251185210916864000000000000000000000000";

    for(int i = 0; i < strlen(b); i++){
        if(b[i] <= 57 && b[i] >= 48) cnt += b[i] - '0';
    }

    printf("Cnt: %d\n",cnt);    
    return 0;
}