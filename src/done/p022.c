#include <stdio.h>
#include <math.h>
#include <string.h>

void display(char array[][30], int x){
  for(int i=0; i<x; i++){
    printf("%s ", array[i]);
  }
  printf("\n");
}

int main(){
    FILE* p_datei = fopen("resources/022_names.txt", "rt");
    if(p_datei == NULL) return -1;
    char buffer[50000];
    int no = fread(buffer, sizeof(char), 50000, p_datei);
    fclose(p_datei);

    char array[6000][30];
    int x = 0, y = 0, z = 0;

    for(int i = 0; i < no; i++){
        if(buffer[i] >= 65 && buffer[i] <= 90){
            array[x][y++] = buffer[i];
        }else if(buffer[i] == 44){
            x++;
            y = 0;
        }
    }
    x++;
    
    char temp[30];
    for(int i = 0; i < x; i++){
        for(int j = 0; j < (x - 1 - i); j++){
            if(strcmp(array[j], array[j+1]) > 0){
                //swap array[j] and array[j+1]
                strcpy(temp, array[j]);
                strcpy(array[j], array[j+1]);
                strcpy(array[j+1], temp);
            }
        }
    }

    int cnt = 0;
    for(int i = 0; i < x; i++){
        int namecnt = 0;
        for(int j = 0; j < 30; j++){
            if(array[i][j] >= 65 && array[i][j] <= 90){
                namecnt += (array[i][j] - 64);
            }else j = 30;
        }
        cnt += (namecnt * (i+1));
    }     
    printf("CNT: %d\n",cnt);   
    return 0;
}