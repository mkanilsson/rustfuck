#include <stdio.h>

char get_character() {
  int character = getchar();
  int tmp = character;

  while (tmp != '\n')
    tmp = getchar();

  return character;
}

char memory[30000] = {0};

int main() {
  char *ptr = memory;
