#include <stdio.h>
#include <dirent.h>

int main(int argc, char** argv) {
  fprintf(stderr, "readdir example\n");
  DIR* dir;
  struct dirent* entry;

  dir = opendir("src");
  while ((entry = readdir(dir)) != NULL) {
    printf("%s\n", entry->d_name);
  }

  closedir(dir);
  return 0;
}
