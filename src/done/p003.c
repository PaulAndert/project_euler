#include <stdio.h>

int isprim(int i){
    int cnt = 0;
    for(int j = 1; j <= i; j++) if(i % j == 0) cnt++;
    if(cnt == 2) return 1;
    else return 0;
}

int main(){
    long z = 600851475143;
    int p = 0, s = 0;
    int prim[2000];
    int sequenz[50];
    for(int i = 2; i < 8000; i++) if(isprim(i) == 1) prim[p++] = i;

    for(int i = 0; i < 1000; i++){
        if(z % prim[i] == 0){
            z = z / prim[i];
            sequenz[s++] = prim[i];
            i = 0;
        }else if(i == 999 || z <= prim[i]){
            i = 1000;
        }
    }
    printf("Seq: %d\n",sequenz[s-1]);
    return 0;
}