#ifndef INFO_MACOS_CPU
#define INFO_MACOS_CPU
#include <stddef.h>
#include <stdint.h>
struct MacOsCpuCount
{
    size_t core_count;
    size_t thread_count;
};

double macos_uptime();
int macos_cpu_name(char* buffer, size_t buffer_len);
int macos_cpu_frequency(uint64_t* cpu_frequency);
int macos_cpu_count(struct MacOsCpuCount* info);
#endif