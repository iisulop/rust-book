use std::collections::HashMap;
use std::thread;
use std::time::Duration;

fn main() {
    println!("Hello, world!");
}

struct Cacher<T, U>
where
    T: Fn(U) -> U,
    U: std::cmp::Eq + std::hash::Hash + std::marker::Copy,
{
        calculation: T,
        values: HashMap<U, U>,
}

impl<T, U> Cacher<T, U>
where
    T: Fn(U) -> U,
    U: std::cmp::Eq + std::hash::Hash + std::marker::Copy,
{
    fn new(calculation: T) -> Cacher<T, U> {
        Cacher {
            calculation,
            values: HashMap::new(),
        }
    }

    fn value(&mut self, arg: U) -> U {
        *self.values.entry(arg).or_insert((self.calculation)(arg))
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}

#[test]
fn call_with_different_values() {
    let mut c = Cacher::new(|a| a);
    let v1 = c.value(1);
    assert_eq!(v1, 1);
    let v2 = c.value(2);
    assert_eq!(v2, 2);
}

