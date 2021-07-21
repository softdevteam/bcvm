// Compiler: 
// Runtime:
//    stdout: 2
//            3
//            4 1.5 5
//            1
void main() {
    int i1 = func2();
    func4(3);
    func5(4, 1.5, 5);
    printf("%i", i1);
}

int func2() {
    int c1 = func3();
    printf("%i", c1);
    return 1;
}

int func3() {
    return 2;
}

int func4(int three) {
    printf("%i", three);
    return three;
}

int func5(int four, double f1, int five) {
    printf("%i %f %i", four, f1, five);
    return four;
}
