// Compiler: 
// Runtime:
//    stdout: int args: -1, 0, 1
//            double args: 1.5, 0.8
//            float args: 4, 2
void main() {
   int i1 = -3;
   int i2 = 2;
   int i3 = i1 / i2;
   int i4 = 44 / -91;
   unsigned int i5 = 3;
   unsigned int i6 = i5/2;
   
   double d1 = 3;
   double d2 = 2;
   double d3= d1 / d2;
   double d4 = 4.0 / 5;

   float f1 = 8.0;
   float f2 = 2.0;
   float f3 = f1 / f2;
   float f4 = 4.0 / 2.0;

   printf("int args: %i, %i, %i", i3, i4, i6);
   printf("double args: %f, %f", d3, d4);
   printf("float args: %f, %f", f3, f4);
}
