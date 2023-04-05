#include <stdio.h>
#include "rustmod.h"

int main(void)
{
    printf("riot: RIOT_BOARD: %s\n", RIOT_BOARD);
    printf("riot: RIOT_MCU: %s\n", RIOT_MCU);

    printf("riot: before calling rustmod\n");
    start();
    printf("riot: after calling rustmod\n");

    return 0;
}
