#include <stdio.h>
#include <math.h>
#include <string.h>

int glohigh = 0;

void schritt(int tiefe, int davor, int route, int num[15][15]){
    // printf("S:: T: %d, R: %d, D: %d\n",tiefe, route, davor);
    if(tiefe < 15){
        // printf("S1: ");
        // printf("Add1 %d ", num[tiefe][route]);
        schritt(tiefe+1, davor + num[tiefe][route], route + 1, num);
        // printf("S0: ");
        // printf("Add0 %d ", num[tiefe][0]);
        schritt(tiefe+1, davor + num[tiefe][route-1], route + 0, num);
    }else{
        // printf("END: T: %d, R: %d, D: %d\n\n",tiefe, route, davor);
        if(davor > glohigh){
            
            glohigh = davor;
        }
    }
}

int main(){
    int num[15][15] = {
                    {75},
                    {95, 64},
                    {17, 47, 82},
                    {18, 35, 87, 10},
                    {20, 4, 82, 47, 65},
                    {19, 1, 23, 75, 3, 34},
                    {88, 2, 77, 73, 7, 63, 67},
                    {99, 65, 4, 28, 06, 16, 70, 92},
                    {41, 41, 26, 56, 83, 40, 80, 70, 33},
                    {41, 48, 72, 33, 47, 32, 37, 16, 94, 29},
                    {53, 71, 44, 65, 25, 43, 91, 52, 97, 51, 14},
                    {70, 11, 33, 28, 77, 73, 17, 78, 39, 68, 17, 57},
                    {91, 71, 52, 38, 17, 14, 91, 43, 58, 50, 27, 29, 48},
                    {63, 66, 4, 68, 89, 53, 67, 30, 73, 16, 69, 87, 40, 31},
                    {4, 62, 98, 27, 23, 9, 70, 98, 73, 93, 38, 53, 60, 4, 23}
                };
    schritt(0, 0, 0, num);

    printf("G: %d\n",glohigh);
    return 0;
}