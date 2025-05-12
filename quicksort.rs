use std::fmt::Display; // Import the Display trait so we can print generic types

// Function to print the array elements on the same line
fn print_arr<T: Display>(a: &[T]) {
    // Loop over each element and print it
    for i in a {
        print!("{} ", i);
    }
    println!(""); // Print newline at the end
}

// Partition function for quicksort
// It rearranges the elements such that all elements smaller than the pivot
// come before it and all larger ones after
fn partition<T: Copy, F>(a: &mut [T], low: usize, high: usize, test: &F) -> usize
where
    F: Fn(T, T) -> bool, // test is a function that compares two elements
{
    let pivot = a[high]; // Choose the last element as the pivot
    let mut i = low;     // Index for placing smaller elements

    // Iterate from low to high - 1
    for j in low..high {
        // If current element should come before pivot according to test
        if test(a[j], pivot) {
            a.swap(i, j); // Swap it into the correct position
            i += 1;       // Move the boundary for smaller elements
        }
    }

    a.swap(i, high); // Move pivot to its correct position
    i // Return the index of the pivot
}

// Recursive quicksort function
fn quicksort<T: Copy, F>(a: &mut [T], low: usize, high: usize, test: &F)
where
    F: Fn(T, T) -> bool,
{
    // Only sort if there are at least 2 elements
    if low < high {
        let pi = partition(a, low, high, test); // Partition the array

        // Recursively sort the subarrays
        if pi > 0 {
            quicksort(a, low, pi - 1, test); // Left of pivot
        }
        quicksort(a, pi + 1, high, test);    // Right of pivot
    }
}

// Public sort function that the user calls
fn sort<T: Copy, F>(a: &mut [T], test: F)
where
    F: Fn(T, T) -> bool,
{
    if a.len() > 1 {
        // Only sort if there are more than 1 element
        quicksort(a, 0, a.len() - 1, &test); // Call quicksort on the full array
    }
}

// Main function: the program starts executing here
fn main() {
    // Declare a mutable array of integers
    let mut nums = [9, 4, 13, 2, 22, 17, 8, 9, 1];

    // Define a comparison function for ascending order
    // You can change this to `x > y` for descending sort
    let asc = |x: i32, y: i32| x < y;

    print_arr(&nums[..]);         // Print the unsorted array
    sort(&mut nums[..], asc);     // Sort using quicksort
    print_arr(&nums[..]);         // Print the sorted array
}
4