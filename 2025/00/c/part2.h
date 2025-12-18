#include <stdio.h>

void run(FILE* input_file) {
    puts(__FILE_NAME__);
    char c;
    while((c = getc(input_file)) != EOF) {
        putchar(c);
    }
}
