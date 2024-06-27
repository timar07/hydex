#include <stdio.h>
#include <stdint.h>
#include <stdlib.h>

extern char *md_to_html_generate(const char *c);
void md_to_html_free(char *c) {
    free(c);
}

int main(void) {
    const char* markdown = "# Hello from C!\nThis is *Markdown*.";

    char* html = md_to_html_generate(markdown);

    if (html != NULL) {
        printf("%s\n", html);
        md_to_html_free(html);
    } else {
        fprintf(stderr, "Markdown compilation failed\n");
        return 1;
    }

    return 0;

}
