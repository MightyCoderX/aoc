#include <stdio.h>

#define STR(x) #x
#define XSTR(x) STR(x)

#ifndef PART
#define PART 1
#endif /* ifndef PART */

#if PART == 1
#include "part1.h"
#elif PART == 2
#include "part2.h"
#else
#pragma message "Invalid PART macro value: " XSTR(PART)
#error
#endif /* if PART == part1 */

int main(int argc, char** argv) {
    char* file_path = argc >= 2 ? argv[1] : "../input.txt";

    FILE* input_file = fopen(file_path, "r");

    if(input_file == NULL) {
        perror("failed to open input file");
        return 1;
    }

    run(input_file);

    fclose(input_file);

    return 0;
}
