// Problem statement:

// A hiker is going on a trail and tracks when they are going up or down hill.
// They always start and end at sealevel (SL). A path s can look like the following:
// s = DDUUUUDD.

// A mountain is a sequence of consecutive steps above SL, starting with
// a step up from SL and ending with a step down to SL.

// A valley is vice versa.

// Given a path s, calculate the number of valleys traversed.

// n: number of steps
// s: path

// Approach:

// We can track their distance from SL by mapping U -> 1 and D -> -1.
// The hiker is entering a valley when their first step is D and they are at
// sealevel.

fn main() {
    let n = 8;
    let s = String::from("UDDDUDUU");

    // Initialize counters for distance from sealevel
    // and number of valleys
    let mut dist_sl = 0;
    let mut n_valleys = 0;

    // Only looping through path once so O(n)
    for i in 1..n {
        // Condition to determine if hiker is entering valley
        if dist_sl == 0 && &s[i-1..i] == "D" {
            n_valleys = n_valleys + 1;
        }

        if &s[i-1..i] == "U" {
            dist_sl = dist_sl + 1;
        } else if &s[i-1..i] == "D" {
            dist_sl = dist_sl - 1;
        }
    }

    println!("{}", n_valleys);
}
