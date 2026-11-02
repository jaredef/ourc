/* freestanding stdio for the rzrs seed -- fd-based, no FILE type; rzcpp expands the stream names */
#define stdin 0
#define stdout 1
#define stderr 2
int printf();
int fprintf();
int fwrite();
int fread();
