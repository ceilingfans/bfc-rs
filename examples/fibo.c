/* This file was generated by bfc-rs (https://github.com/ceilingfans/bfc-rs) */
#include <stdio.h>

int main()
{
    char arr[30000] = {0}; char* ptr = arr;
    *ptr += 11;
    ptr += 1;
    *ptr += 1;
    ptr += 4;
    *ptr += 44;
    ptr += 1;
    *ptr += 32;
    ptr += -6;
    while (*ptr)
    {
        ptr += 1;
        while (*ptr)
        {
            ptr += 6;
            *ptr += 1;
            ptr += 1;
            *ptr += 1;
            ptr += -7;
            *ptr += -1;
        }
        ptr += 7;
        while (*ptr)
        {
            ptr += -7;
            *ptr += 1;
            ptr += 7;
            *ptr += -1;
        }
        ptr += -1;
        while (*ptr)
        {
            ptr += 1;
            *ptr += 10;
            while (*ptr)
            {
                *ptr += -1;
                ptr += -1;
                *ptr += -1;
                while (*ptr)
                {
                    ptr += 2;
                    *ptr += 1;
                    ptr += 1;
                    *ptr += 1;
                    ptr += -3;
                    *ptr += -1;
                }
                ptr += 3;
                while (*ptr)
                {
                    ptr += -3;
                    *ptr += 1;
                    ptr += 3;
                    *ptr += -1;
                }
                *ptr += 1;
                ptr += -1;
                while (*ptr)
                {
                    ptr += 1;
                    *ptr = 0;
                    ptr += -1;
                    *ptr = 0;
                }
                ptr += 1;
                while (*ptr)
                {
                    ptr += -2;
                    while (*ptr)
                    {
                        ptr += 3;
                        *ptr += 1;
                        ptr += -3;
                        *ptr += -1;
                    }
                    ptr += 2;
                    *ptr = 0;
                }
                ptr += -2;
            }
            ptr += 3;
            while (*ptr)
            {
                ptr += 2;
                *ptr += 1;
                ptr += 1;
                *ptr += 1;
                ptr += -3;
                *ptr += -1;
            }
            ptr += 3;
            while (*ptr)
            {
                ptr += -3;
                *ptr += 1;
                ptr += 3;
                *ptr += -1;
            }
            *ptr += 1;
            ptr += -1;
            while (*ptr)
            {
                ptr += 1;
                *ptr = 0;
                ptr += -1;
                *ptr = 0;
            }
            ptr += 1;
            while (*ptr)
            {
                ptr += -2;
                *ptr += 1;
                ptr += 2;
                *ptr = 0;
            }
            ptr += -7;
        }
        ptr += 5;
        while (*ptr)
        {
            *ptr += 48;
            putchar(*ptr);
            *ptr = 0;
        }
        *ptr += 10;
        ptr += -1;
        while (*ptr)
        {
            *ptr += -1;
            ptr += 1;
            *ptr += -1;
            ptr += -1;
        }
        ptr += 1;
        *ptr += 48;
        putchar(*ptr);
        *ptr = 0;
        ptr += -12;
        while (*ptr)
        {
            ptr += 3;
            *ptr += 1;
            ptr += 1;
            *ptr += 1;
            ptr += -4;
            *ptr += -1;
        }
        ptr += 4;
        while (*ptr)
        {
            ptr += -4;
            *ptr += 1;
            ptr += 4;
            *ptr += -1;
        }
        ptr += -1;
        *ptr += -1;
        while (*ptr)
        {
            ptr += 2;
            putchar(*ptr);
            ptr += 1;
            putchar(*ptr);
            ptr += -3;
            *ptr = 0;
        }
        ptr += -2;
        while (*ptr)
        {
            ptr += 2;
            *ptr += 1;
            ptr += 1;
            *ptr += 1;
            ptr += -3;
            *ptr += -1;
        }
        ptr += 3;
        while (*ptr)
        {
            ptr += -3;
            *ptr += 1;
            ptr += 3;
            *ptr += -1;
        }
        ptr += -2;
        while (*ptr)
        {
            ptr += -1;
            *ptr += 1;
            ptr += 1;
            *ptr += -1;
        }
        ptr += 1;
        while (*ptr)
        {
            ptr += -1;
            *ptr += 1;
            ptr += 1;
            *ptr += -1;
        }
        ptr += -3;
        *ptr += -1;
    }
    return 0;
}
