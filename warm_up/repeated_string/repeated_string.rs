// Problem statement:

// Given a string s and an integer n, print the number of a's in the repeated string.

// e.g. s = 'aba', n = 10, repeated_string = 'abaabaabaa' and n_a = 4.

// Approach:

// The repeated string is constructed via the following formula:

// remainder = n mod(length(s))
// s * (n // lenght_s) + s[remainder:]

// We need to find the number of a's in the two substrings s and s[remainder:]. Given
// a function count_as(), the number of a's is:

// n_a = count_as(s) * (n // length_s) + count_as(s[remainder:]).

// If the length of s is a perfect divisor of n, simply count the a's and multiply by that factor.

// Count number of a's in a string s
fn count_as(s: String) -> usize {
    let mut count = 0;

    for i in 1..s.len() + 1 {
        if &s[i-1..i] == "a" {
            count = count + 1;
        }
    }

    count
}

fn main() {
    let s = String::from("aba");
    let n = 10;

    // Find length of string
    let len_s = s.len();

    // Determine if there is a trailing piece of the string
    let tail = n % len_s;

    // Initialize counter for a's
    let count: usize;

    if tail != 0 {
        let s_tail = &s[0..tail];
        let tail_count = count_as(s_tail.to_string());

        count = count_as(s) * (n / len_s) + tail_count;
    } else {
        count = count_as(s) * (n / len_s);
    }

    println!("{}", count);
}
