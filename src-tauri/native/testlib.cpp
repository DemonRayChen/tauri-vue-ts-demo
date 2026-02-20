#include "include/testlib.h"

#include <cstring>
#include <string>

extern "C"
{

    int add_numbers(int a, int b)
    {
        return a + b;
    }

    char *process_string(const char *input)
    {
        if (!input)
        {
            input = "";
        }

        std::string result = std::string("Processed: ") + input;

        char *output = new char[result.size() + 1];
        std::strcpy(output, result.c_str());
        return output;
    }

    void free_string(char *ptr)
    {
        delete[] ptr;
    }

} // extern "C"