#ifdef __APPLE__
#include <time.h>
#include <string.h>
#include "cpu.h"
#include <sys/sysctl.h>
#include <sys/types.h>

double macos_uptime()
{
    struct timeval boottime = {0, 0};
    size_t boottime_len = sizeof(boottime);
    int name[2] = {CTL_KERN, KERN_BOOTTIME};
    if (sysctl(name, 2, &boottime, &boottime_len, NULL, 0) < 0)
    {
        return -1;
    }
    time_t bsec = boottime.tv_sec, csec = time(NULL);
    return difftime(csec, bsec);
}

int macos_cpu_name(char *buffer, size_t buffer_len)
{
    char brand_string[2048];
    size_t brand_string_size = sizeof(brand_string);
    if (sysctlbyname("machdep.cpu.brand_string", &brand_string, &brand_string_size, NULL, 0) < 0)
        return -1;

    if (buffer_len < strlen(brand_string) + 1)
        return -2;
    else
        strcpy(buffer, brand_string);
    return 0;
}

int macos_cpu_frequency(uint64_t* cpu_frequency)
{
    uint64_t frequency = 0;
    size_t frequency_size = sizeof(frequency);

    if (sysctlbyname("hw.cpufrequency", &frequency, &frequency_size, NULL, 0) < 0)
    {
        return -1;
    }
    *cpu_frequency = frequency / 1000000;
    return 0;
}

int macos_cpu_count(struct MacOsCpuCount* info)
{
    size_t core_count = 0, thread_count =0;
    size_t core_count_size = sizeof(core_count), thread_count_size = sizeof(thread_count);

    if (sysctlbyname("machdep.cpu.core_count", &core_count, &core_count_size, NULL, 0) < 0)
    {
        return -1;
    }

    if (sysctlbyname("machdep.cpu.thread_count", &thread_count, &thread_count_size, NULL, 0) < 0)
    {
        return -1;
    }

    info->core_count = core_count;
    info->thread_count = thread_count;
    return 0;
}

#else
double macos_uptime()
{
    return -1.0
}

int macos_cpu_name(char *buffer, size_t buffer_len)
{
    return -1
}
#endif