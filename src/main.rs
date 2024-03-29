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

    let circle = Circle { radius: 1.25 };
    let triangle = Triangle { length: 2.5, height: 1.34 };
    let square = Square { length: 32.3 };
    println!("area of circle: {}", get_area(&circle));
    println!("area of triangle: {}", get_area(&triangle));
    println!("area of square: {}", get_area(&square));
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

trait Shape {
    fn area(&self) -> f64;
}

struct Circle {
    radius: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        3.14 * self.radius * self.radius
    }
}

struct Triangle {
    length: f64,
    height: f64,
}

impl Shape for Triangle {
    fn area(&self) -> f64 {
        self.length * self.height / 2.0
    }
}

struct Square {
    length: f64,
}

impl Shape for Square {
    fn area(&self) -> f64 {
        self.length * self.length
    }
}

fn get_area<T: Shape>(s: &T) -> f64 {
    s.area()
}