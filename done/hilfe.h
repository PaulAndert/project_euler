#ifndef def
#define def

long gen1(int nc, int ec); // eins schreiben
long gen0(int nc, int ec); // null schreiben

#endif

long gen1(int nc, int ec){ // eins schreiben
    //printf("0 NC: %d, EC: %d\n",nc,ec);
    long ret = 0;
    if(nc > 0){
        ret += gen0(nc-1,ec);
        if(ec > 0){
            ret += gen1(nc,ec-1);
        }
    }else if(ec > 0){
        ret += gen1(nc,ec-1);
    }else{
        //printf("HIT\n");
        return 1;
    }
    //printf("R: %d\n",ret);
    return ret;
}

long gen0(int nc, int ec){ // null schreiben
    //printf("1 NC: %d, EC: %d\n",nc,ec);
    long ret = 0;
    if(nc > 0){
        ret += gen0(nc-1,ec);
        if(ec > 0){
            ret += gen1(nc,ec-1);
        }
    }else if(ec > 0){
        ret += gen1(nc,ec-1);
    }else{
        //printf("HIT\n");
        return 1;
    }
    //printf("R: %d\n",ret);
    return ret;
}