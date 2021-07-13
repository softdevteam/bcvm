// Compiler: 
// Runtime:
//    stdout: 6
//            1
//            0
void main() {
    int i1 = 6;
    long i2 = (long) i1;
    printf("%i", i2);

    int i3 = 2;
    int b1 = i3 == i3;
    if (b1){
        printf("%i", b1);
    }
    
    int b2 = i3==i1;
    if (!b2){
        printf("%i", b2);
    }
}
