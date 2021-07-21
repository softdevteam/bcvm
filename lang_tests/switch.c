// Compiler: 
// Runtime:
//    stdout: 1
//            2
//            default
void main() {
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
