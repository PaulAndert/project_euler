#include <stdio.h>
#include <math.h>
#include <string.h>

int leap(int i){
    if (i % 4 == 0 && i % 100 != 0 || i % 400 == 0 ){
        return 1;
    }else{
        return 0;
    }
}

int main(){
    int daynum = 2;
    int suncnt = 0;
    for(int i = 0; i < 99; i++){
        for(int j = 1; j <= 12; j++){
            if(j == 1 || j == 3 || j == 5 || j == 7 || j == 8 || j == 10 || j == 12){
                daynum += 31;
            }else if(j == 2){
                if(leap(i) == 1){
                    daynum += 29;
                }else{
                    daynum += 28;
                }
            }else if(j == 4 || j == 6 || j == 9 || j == 11){
                daynum += 30;
            }
            if(daynum % 7 == 0){
                suncnt++;
            }
        }
    }
    printf("SC: %d\n",suncnt);
    return 0;
}
// Januar, februar, märz
// april, mai, juni
// juli, august, september
// oktober, november, dezember