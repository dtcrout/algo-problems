// Problem statement:
//
// Given an array of clouds numbered 0 if safe or 1 if they must be avoided,
// find the minimum number of jumps to get to the end. You can only jump
// 1 or 2 steps ahead and you can always win the game.
//
// c: clouds
//
// e.g. c = [0, 1, 0, 0, 0, 1, 0]
// The shortest path is c[0] -> c[2] -> c[4] -> c[6] and the minimum
// number of steps is 3.
//
// Approach:
//
// You can always win the game, therefore we can assume the last cloud
// will always be 0. We can be greedy with our paths as well. If the
// second cloud from us is 0, then we take it.
//
// Iterate through every cloud except for the last two. Check if the second one
// is 0, else the first one is 0. If we end up on the second last cloud, just move ahead.x

fn main() {
    // let c = [0, 0, 1, 0, 0, 1, 0];  // output is 4
    let c = [0, 0, 0, 1, 0, 0];  // output is 3

    // Find out how many clouds there are
    let n = c.len();

    // Initialize counting variable
    let mut count = 0;

    // Initialize iteration counter
    let mut i = 0;

    // Iterate through each cloud except for the last two; O(n)
    while i < (n - 2) {
        if c[i + 2] == 0 {
            i = i + 2;
            count = count + 1;
        } else if c[i + 1] == 0 {
            i = i + 1;
            count = count + 1;
        }
    }

    if i == (n - 2) {
        count = count + 1;
    }

    println!("{}", count);
}
