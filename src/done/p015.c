#include <stdio.h>
#include <math.h>
#include "hilfe.h"

long gen(int nc, int ec){
    //printf("NC: %d, EC: %d\n",nc,ec);
    long ret = 0;
    if(nc > 0){
        ret += gen0(nc-1,ec);
        if(ec > 0){
            ret += gen1(nc,ec-1);
        }
    }else if(ec > 0){
        ret += gen1(nc,ec-1);
    }else{
        return 1;
    }
    //printf("R: %d\n",ret);
    return ret;
}

int main(){
    int feld = 20; // 2x2
    int n = feld * 2;
    int ec = n/2, nc = n/2;

    long ret = gen(nc,ec);
    

    
    printf("A: %ld\n",ret);
}