#include <stdio.h>
#include <math.h>
#include <string.h>

int globalcnt = 0;
int array[10];

void schritt(int array[10], int stelle, int left[10]){
    int saveleft = left;
    for(int i = stelle; i < 10; i++){
        array[stelle] = left[i];
        for(int j = i; j < 10; j++){
            left[i] = left[i+1];
        }
        schritt(array, stelle+1, left);
    }



}

int main(){

    int left[10] = {0,1,2,3,4,5,6,7,8,9};
    int array[10];

    schritt(array, 0, left);
    
    return 0;
}