#ifndef MYLIB_H
#define MYLIB_H

#ifdef __cplusplus
extern "C" {
#endif

int add_numbers(int a, int b);
char* process_string(const char* input);
void free_string(char* ptr);

#ifdef __cplusplus
}
#endif

#endif