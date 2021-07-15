// Compiler: 
// Runtime:
//    stdout: int args: -1, -47
//            double args: 13.79, 10.558
//            float args: 10, 9
int main() {
   int i1 = -3;
   int i2 = 2;
   int i3 = i1 + i2;
   int i4 = 44 + -91;
 
   double d1 = 5.47; 
   double d2 = 8.32;
   double d3 = d1 + d2;
   double d4 = 4.768 + 5.79;

   float f1 = 8.0;
   float f2 = 2.0;
   float f3 = f1 + f2;
   float f4 = 4.0 + 5.0;

   printf("int args: %i, %i", i3, i4);
   printf("double args: %f, %f", d3, d4);
   printf("float args: %f, %f", f3, f4);
}
