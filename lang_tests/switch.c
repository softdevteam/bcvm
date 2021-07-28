// Compiler: 
// Runtime:
//    stdout: 0
//            1
//            2
//            default
void main() {
    int i0 = 0;
    switch (i0) {
        case 0 : 
            printf("0");
            break;
        case 1 : 
            printf("1");
            break;
        case 2 : 
            printf("2");
            break; 
        default : 
            printf("default");
    }

    func2();

    int i1 = 1;
    switch (i1) {
        case 0 : 
            printf("0");
            break;
        case 1 : 
            printf("1");
            break;
        case 2 : 
            printf("2");
            break; 
        default : 
            printf("default");
    }

    switch (2) {
        case 0 : 
            printf("0");
            break;
        case 1 : 
            printf("1");
            break;
        case 2 : 
            printf("2");
            break; 
        default : 
            printf("default");
    }

    switch (3) {
        case 0 : 
            printf("0");
            break;
        case 1 : 
            printf("1");
            break;
        case 2 : 
            printf("2");
            break; 
        default : 
            printf("default");
    }
}

int func2() {
    return 6;
}
