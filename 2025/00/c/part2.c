#include <stdio.h>

int main(int argc, char** argv) {
    char* file_path = argc >= 2 ? argv[1] : "../input.txt";

    FILE* input_file = fopen(argv[1], "r");

    if(input_file == NULL) {
        perror("failed to open input file");
        return 1;
    }

    char c;
    while((c = getc(input_file))) {
        putchar(c);
    }

    fclose(input_file);

    return 0;
}
