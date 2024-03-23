use std::cmp::PartialOrd;
fn main() {
    let mut integers = vec![1, 0, 2, 2, 5, 4, 0, 6];
    bubble_sort(&mut integers);
    println!("{:?}", integers);

    let mut decimals = vec![1.1, 0.2, 2.3, 2.4, 5.5, 4.6, 0.7, 6.8];
    bubble_sort(&mut decimals);
    println!("{:?}", decimals);
}

fn bubble_sort<T: PartialOrd>(s: &mut [T]) {
    for i in 0..s.len()-1 {
        for j in 0..s.len()-1-i {
            if s[j] > s[j+1] {
                s.swap(j, j+1);
            }
        }
    }
}