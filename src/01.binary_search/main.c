#include <stdio.h>
#include "binary_search.h"

int main()
{
    int a_list[] = {1, 2, 4, 5, 6, 7, 10, 31};
    size_t len = sizeof(a_list) / sizeof(int);
    printf("THe result of the search for 7 is: %d\n", do_binary_search(a_list, len, 7));
    printf("THe result of the search for 3 is: %d\n", do_binary_search(a_list, len, 3));
}

