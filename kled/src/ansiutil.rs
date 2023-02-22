// Copied from:
// http://athena.csus.edu/~gordonvs/180/colorAnsi.html
void textcolor(attr: u8, fg: u8, bg: u8) {
#ifndef NO_ANSI
  char command[13];
  std::sprintf(command, "%c[%d;%d;%dm", 0x1B, attr, fg + 30, bg + 40);
  printf("%s", command);
#endif
}

void textattr(int attr) {
#ifndef NO_ANSI
  char command[13];
  std::sprintf(command, "%c[%dm", 0x1B, attr);
  printf("%s", command);
#endif
}

void textfg(int fg) {
#ifndef NO_ANSI
  char command[13];
  std::sprintf(command, "%c[%dm", 0x1B, fg + 30);
  printf("%s", command);
#endif
}

void textfg_256(int color) {
#ifndef NO_ANSI
  printf("%c[38;5;%dm", 0x1B, color);
#endif
}

void textbg(int bg) {
#ifndef NO_ANSI
  char command[13];
  std::sprintf(command, "%c[%dm", 0x1B, bg + 40);
  printf("%s", command);
#endif
}

void textbg_256(int color) {
#ifndef NO_ANSI
  printf("%c[48;5;%dm", 0x1B, color);
#endif
}

void resettext() {
#ifndef NO_ANSI
  printf("%c[0;;m", 0x1B);
#endif
}

void printGooglyHeader(const char* text[], int len, const char* ch, int termWidth) {
  int pad = (termWidth - len);
  for (int r = 0; r < 4; ++r){
    for (int i = 3; i < pad / 2; ++i) {
      printf(".");
    }
    printf("   ");
    for (int idx = 0; idx < len; idx += 3) {
      textfg(googleColors[(idx / 3) % 6]);
      printf("%s", ((text[r] + idx)[0] == ' ') ? " " : ch);
      printf("%s", ((text[r] + idx)[1] == ' ') ? " " : ch);
      printf("%s", ((text[r] + idx)[2] == ' ') ? " " : ch);
      resettext();
    }
    printf("   ");
    for (int i = 3; i < (pad-1)/2 + 1; ++i) {
      printf(".");
    }
    printf("\n");
  }
}

void printHeader(const char* text[], int len, int fg0, int fg1, const char* ch, int termWidth) {
  int pad = (termWidth - len);
  for (int r = 0; r < 4; ++r){
    for (int i = 3; i < pad / 2; ++i) {
      printf(".");
    }
    printf("   ");
    for (int idx = 0; idx < len; idx += 3) {
      textattr((idx & 1) ? fg1 : fg0);
      printf("%s", ((text[r] + idx)[0] == ' ') ? " " : ch);
      printf("%s", ((text[r] + idx)[1] == ' ') ? " " : ch);
      printf("%s", ((text[r] + idx)[2] == ' ') ? " " : ch);
      resettext();
    }
    printf("   ");
    for (int i = 3; i < (pad-1)/2 + 1; ++i) {
      printf(".");
    }
    printf("\n");
  }
}
