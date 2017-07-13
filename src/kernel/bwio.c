#include <kernel/bwio.h>

void bwputs(const char* str) {
	for (size_t i = 0; str[i] != '\0'; i ++)
		bwputc((unsigned char)str[i]);
}
