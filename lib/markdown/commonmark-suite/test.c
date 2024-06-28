#include <stdio.h>
#include <stdint.h>
#include <stdlib.h>

extern char *md_to_html_generate(const char *c);
void md_to_html_free(char *c) {
    free(c);
}
