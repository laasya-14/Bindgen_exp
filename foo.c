
#include <stdio.h>
typedef struct T {
    int x;
    int y;
} T;

T get_value(){
    T t;
    t.x = 1;
    t.y = 11;
    return t;
}

void print_value(T t){
    printf("%d %d\n", t.x, t.y);
}