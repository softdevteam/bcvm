// Compiler: 
// Runtime:
//    stdout: 1
//            noargs
//            1 arg: 1
//            2 args: 1 2
//            string arg: string
int main() {
   int a = 1;
   printf("%i", a);
   printf("noargs");
   printf("1 arg: %i", 1);
   printf("2 args: %i %i", 1, 2);
   printf("string arg: %s", "string");
   return 0;
}
