mod s1431;

use s1431::Solution;

fn main() {
    for i in Solution::kids_with_candies(vec![2, 3, 5, 1, 3], 3)
        .iter()
        .cloned()
    {
        println!("{}", i);
    }
}
