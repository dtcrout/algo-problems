// Problem statement:
//
// Given an array of integers representing the color of each sock, determine
// how many pairs of socks with matching colors are there.
//
// n: number of socks
// ar: array of socks
//
// Find the number of pairs of socks.
//
// Approach:
//
// Create auxillary array where the index corresponds to the color of the sock.
// Count each sock, divide by two and count the number of pairs.

fn main() {
    let n = 7;  // number of socks
    let arr = [1, 2, 1, 2, 1, 3, 2];  // socks

    // Find max value in order to create auxillary array; O(n)
    let mut n_colors = arr[0];

    for i in 1..arr.len() {
        if arr[i] > n_colors {
            n_colors = arr[i];
        }
    }

    // Create auxillary array to hold counts of pairs
    // Arrays cannot be dynamic, therefore we need to use vector
    let mut c = vec![0; n_colors];

    // Count each sock; O(n)
    for i in 0..n {
        let key = arr[i];
        c[key - 1] = c[key - 1] + 1;
    }

    // Divide each pair by 2 to get the count; O(n)
    for i in 0..n_colors {
        c[i] = c[i] / 2;
    }

    // Add all the pairs; O(n)
    let mut n_pairs = 0;

    for i in 0..n_colors {
        n_pairs = n_pairs + c[i];
    }

    println!("{}", n_pairs);
}
