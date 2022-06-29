#include <stdio.h>
#include "rustmod.h"

int main(void)
{
    int input = 4;

    printf("riot: RIOT_BOARD: %s\n", RIOT_BOARD);
    printf("riot: RIOT_MCU: %s\n", RIOT_MCU);

    printf("riot: before calling rustmod\n");
    int output = square(input);
    printf("riot: after calling rustmod\n");

    printf("riot: square(%d) -> %d\n", input, output);

    return 0;
}
