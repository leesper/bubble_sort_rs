use std::cmp::PartialOrd;
fn main() {
    let mut integers = vec![1, 0, 2, 2, 5, 4, 0, 6];
    bubble_sort(&mut integers);
    println!("{:?}", integers);

    let mut decimals = vec![1.1, 0.2, 2.3, 2.4, 5.5, 4.6, 0.7, 6.8];
    bubble_sort(&mut decimals);
    println!("{:?}", decimals);

    let green = TrafficLight::Green;
    println!("duration: {}", green.duration());
    let red = TrafficLight::Red;
    println!("duration: {}", red.duration());
    let yellow = TrafficLight::Yellow;
    println!("duration: {}", yellow.duration());

    let v: [u32; 3] = [1, 2, 3];
    let sum = sum_integers(&v);
    println!("sum: {}", sum.unwrap());
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

enum TrafficLight {
    Red,
    Green,
    Yellow,
}

trait Duration {
    fn duration(&self) -> u8;
}

impl Duration for TrafficLight {
    fn duration(&self) -> u8 {
        match self {
            TrafficLight::Red => 10,
            TrafficLight::Green => 20,
            TrafficLight::Yellow =>3,
        }
    }
}

fn sum_integers(v: &[u32]) -> Option<u32> {
    let mut sum: u32 = 0;
    for e in v.iter() {
        match sum.checked_add(*e) {
            Some(x) => sum += x,
            None => return None
        }
    }
    Some(sum)
}