#include <stdio.h>
#include "rustmod.h"

int main(void)
{
    int input = 4;

    printf("riot: before calling rustmod\n");
    int output = square(input);
    printf("riot: after calling rustmod\n");

    printf("riot: square(%d) -> %d\n", input, output);

    return 0;
}
