void rustmod_start(
        void (*)(uint32_t),
        void (*)(uint32_t),
        void (*)(uint32_t, void (*)(void *), void *, void **),
        void (*)(char *, char * /* WIP */));