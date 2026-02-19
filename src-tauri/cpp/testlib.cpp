#include <string>
#include <cstring>

extern "C" {
    // 简单计算
    int add_numbers(int a, int b) {
        return a + b;
    }
    
    // 字符串处理（需要特别注意内存管理）
    char* process_string(const char* input) {
        std::string result = "Processed: " + std::string(input);
        char* output = new char[result.length() + 1];
        std::strcpy(output, result.c_str());
        return output;
    }
    
    // 释放 C++ 分配的内存
    void free_string(char* ptr) {
        delete[] ptr;
    }
}