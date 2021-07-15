// Compiler: 
// Runtime:
//    stdout: int args: -1, 44, 1
void main() {
   int i1 = -3;
   int i2 = 2;
   int i3 = i1 % i2;
   int i4 = 44 % -91;
   int i5 = 3;
   unsigned int i6 = i5 % i2;
   printf("int args: %i, %i, %i", i3, i4, i6);
}
