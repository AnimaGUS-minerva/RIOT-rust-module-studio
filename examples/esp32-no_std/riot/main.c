#include <stdio.h>
#include "rustmod.h"

int main(void)
{
    printf("riot: hello\n");

    int input = 21;

    printf("riot: before calling rustmod\n");
    int output = double_input(input);
    printf("riot: after calling rustmod\n");

    printf("riot: %d * 2 = %d\n", input, output);

    printf("riot: done\n");

    return 0;
}
