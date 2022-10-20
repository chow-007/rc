#include <stdio.h>
#include <stdint.h>
#include <stdlib.h>
#include <unistd.h>

void print_sum(uint32_t a, uint32_t b);

uint32_t addtwo(uint32_t a, uint32_t b);

void print_str(const char *s);

char *change_str(char *s);

char *generate_str(void);

void free_str(char *s);

int main(void)
{
   print_sum(1, 2);

   uint32_t n = addtwo(2, 3);
   printf("print in c, sum is: %d\n", n);

   char *name = "zhangsan";
   print_str(name);

   // for (;;)
   // {
   //    // sleep(1);

   //    char *addr = "beijing";
   //    char *new_addr = change_str(addr);
   //    // printf("new_addr: %s\n", new_addr);
   //    free(new_addr); // c free可以释放内存
   //    // free_str(new_addr); // rust free_str也可以释放内存
   // }

     for( ; ; )
      {
         // sleep(1);

         char *s = generate_str();
         // printf("generate_str: %s\n", s );
         free(s);
         // free_str(s);
      }
   return 0;
}