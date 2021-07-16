// Compiler: 
// Runtime:
//    stdout: 2=1 is false
//            2 is two
//            2 doesn't equal 1
//            2 is bigger than 1
//            2 is bigger or equal to 1
//            2 is smaller than 3
//            2 is smaller or equal to 3
//            2 is bigger than 1
//            2 is bigger or equal to 1
//            2 is smaller than 3
//            2 is smaller or equal to 3
void main() {
    int i1 = 2;
    unsigned i2 = 2;
    int i3 = 1;
    int b1 = i1 == i3;

    if (!b1) {
        printf("2=1 is false");
    }

    if (i1 == 1) {
        printf("%i is one", i1);
    } else if (i1 == 2) {
        printf("%i is two", i1);
    } else if (i1 == 3) {
        printf("%i is three", i1);
    }
    else {
        printf("%i is neither one nor two nor three", i1);
    }

    if (i1 != 1) {
        printf("%i doesn't equal 1", i1);
    }

    if (i2 > 1) {
        printf("%i is bigger than 1", i1);
    }

    if (i2 >= 1) {
        printf("%i is bigger or equal to 1", i1);
    }
    if (i2 < 3) {
        printf("%i is smaller than 3", i1);
    }
    if (i2 <= 3) {
        printf("%i is smaller or equal to 3", i1);
    }
    if (i1 > 1) {
        printf("%i is bigger than 1", i1);
    }
    if (i1 >= 1) {
        printf("%i is bigger or equal to 1", i1);
    }
    if (i1 < 3) {
        printf("%i is smaller than 3", i1);
    }
    if (i1 <= 3) {
        printf("%i is smaller or equal to 3", i1);
    }
}
