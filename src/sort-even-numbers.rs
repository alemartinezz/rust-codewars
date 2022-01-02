
// Sort even numbers in array while leaving odds in original position

use itertools::Itertools;

fn sort_array(xs: &[i32]) -> Vec<i32> {
    let mut os = xs.iter().filter(|&x| x % 2 != 0).sorted();
    xs.iter().map(|x| if x % 2 != 0 { os.next().unwrap() } else { x }).cloned().collect()
}