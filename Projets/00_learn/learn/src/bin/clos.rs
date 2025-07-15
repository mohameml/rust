use std::collections::HashMap;
use std::hash::Hash;
use std::thread;
use std::time::Duration;

fn simulate_expensive_calculation(intensity: u32) -> u32 {
    println!("calc slowly ............");
    thread::sleep(Duration::from_secs(2));
    intensity
}

struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calc: T) -> Cacher<T> {
        Cacher {
            calculation: calc,
            value: None,
        }
    }

    fn value(&mut self, args: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                // let v = self.calculation(args);

                let v = (self.calculation)(args);
                self.value = Some(v);
                v
            }
        }
    }
}

fn main() {
    let sim_intensity = 10;
    let mut random_n = 7;

    (gen_work)(sim_intensity, random_n);

    // test applay :
    let f = |x: u32| x * 2;
    println!("res of f(2) = {}", (f)(2));
    let fmut = |x: u32| {
        random_n += 1;
        x * 2
    };

    let f = double;
    let res = apply(f, 3);
    println!("res : {}", res);

    let mut h: HashMap<u32, u32> = HashMap::new();
    h.insert(1, 1);
    h.insert(2, 1);

    let key = 1;

    let res = h.get(&key);

    // href.get(1);
}

// Fn
// FnMut
// FnOnce

fn apply<F>(f: F, x: u32) -> u32
where
    F: Fn(u32) -> u32,
{
    f(x)
}

fn applymut<F>(f: &mut F, x: u32) -> u32
where
    F: FnMut(u32) -> u32,
{
    f(x)
}

fn double(x: u32) -> u32 {
    x * 2
}

fn gen_work(intensity: u32, random_n: u32) {
    let mut cached = Cacher::new(|num: u32| {
        println!("calc slowly ............");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", cached.value(intensity));
        println!("Next, do {} situps!", cached.value(intensity));
    } else {
        if random_n == 3 {
            println!("Take a break toady!");
        } else {
            println!("Today, do {} pushups!", cached.value(intensity));
        }
    }
}
