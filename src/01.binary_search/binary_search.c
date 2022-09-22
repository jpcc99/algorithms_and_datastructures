#include "binary_search.h"

int do_binary_search(int array[], const size_t array_len, int item)
{
    /*
     * low and high keep track of which part of the list you'll
     * search in
     */
    int low = 0,
        high = array_len - 1;

    /*
     * While you haven't narrowed it down to one element
     */
    while (low <= high)
    {
        int mid = (low + high) / 2,
            guess = array[mid];

        if (guess == item) // Found the item
            return mid;
        else if (guess > item) // Too high
            high = mid - 1;
        else
            low = mid + 1; // Too low
    }
    return -1; // Didn't find the item int the array
}
