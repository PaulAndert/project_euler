#include <stdio.h>
#include <math.h>
#include <string.h>

int one(int i){
    //printf("One: %d\n",i);
    switch(i){
        case 0: {
            printf("Zero");
            return 0; // nothing
        }
        case 1: {
            printf("One");
            return 3; // one
        }
        case 2: {
            printf("Two");
            return 3; // two
        }
        case 3: {
            printf("Three");
            return 5; // three
        }
        case 4: {
            printf("Four");
            return 4; // four
        }
        case 5: {
            printf("Five");
            return 4; // five
        }
        case 6: {
            printf("Six");
            return 3; // six
        }
        case 7: {
            printf("Seven");
            return 5; // seven
        }
        case 8: {
            printf("Eight");
            return 5; // eight
        }
        case 9: {
            printf("Nine");
            return 4; // nine
        }
    }
    return 0;
}
int two(int i){
    //printf("Two: %d\n",i);
    if(i < 10){
        return one(i);
    }else if(i < 20){
        switch(i){
            case 10: {
                printf("Ten");
                return 3; // ten
            }
            case 11: {
                printf("Eleven");
                return 6; // eleven
            }
            case 12: {
                printf("Twelve");
                return 6; // twelve
            }
            case 13: {
                printf("Thirteen");
                return 8; // thirteen
            }
            case 14: {
                printf("Fourteen");
                return 8; // fourteen
            }
            case 15: {
                printf("Fifteen");
                return 7; // fifteen
            }
            case 16: {
                printf("Sixteen");
                return 7; // sixteen
            }
            case 17: {
                printf("Seventeen");
                return 9; // seventeen
            }
            case 18: {
                printf("Eighteen");
                return 8; // eighteen
            }
            case 19: {
                printf("Nineteen");
                return 8; // nineteen
            }
        }
    }else{
        int m = i / 10;
        //printf("Z: %d\n",m);
        int n = i % 10;
        //printf("E: %d\n",n);
        switch(m){
            case 2: {
                printf("Twenty ");
                int a = one(n);
                return 6 + a; // twenty
            }
            case 3: {
                printf("Thirty ");
                int a = one(n);
                return 6 + a; // thirty
            }
            case 4: {
                printf("Forty ");
                int a = one(n);
                return 5 + a; // forty
            }
            case 5: {
                printf("Fifty ");
                int a = one(n);
                return 5 + a; // fifty
            }
            case 6: {
                printf("Sixty ");
                int a = one(n);
                return 5 + a; // sixty
            }
            case 7: {
                printf("Seventy ");
                int a = one(n);
                return 7 + a; // seventy
            }
            case 8: {
                printf("Eighty ");
                int a = one(n);
                return 6 + a; // eighty
            }
            case 9: {
                printf("Ninety ");
                int a = one(n);
                return 6 + a; // ninety
            }
        }
    }
    return 0;
}
int three(int i){
    int n = (i / 100) % 10;
    //printf("H: %d\n",n);
    int m = i % 100;
    //printf("R: %d\n",m);
    if(m == 0){
        int a = one(n);
        printf(" hundred");
        return a + 7;
    }else{
        int a = one(n);
        printf(" hundred and ");
        int b = two(m);
        return a + 10 + b;
    }
}

int main(){
    int cnt = 0;

    for(int i = 1; i < 1000; i++){
        //printf("\n%d: Cnt: %d\n",i,cnt);
        printf("%d: ",i);
        int cnta;
        if(i < 10){
            cnta = one(i);
        }else if(i < 100){
            cnta = two(i);
        }else{
            cnta = three(i);
        }
        printf(" : %d\n",cnta);
        cnt += cnta;
    }
    cnt += 11;// one thousand
    printf("Cnt: %d\n",cnt);
    return 0;
}