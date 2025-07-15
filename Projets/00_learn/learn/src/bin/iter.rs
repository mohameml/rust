use std::fmt::Display;

struct Counter {
    max_value: u32,
    curr_value: u32,
}

impl Counter {
    fn new(capacity: u32) -> Self {
        Counter {
            max_value: capacity,
            curr_value: 0,
        }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.curr_value < self.max_value {
            let value = self.curr_value;
            self.curr_value += 1;
            Some(value)
        } else {
            None
        }
    }
}

fn main() {
    let v1 = vec![1, 2, 3, 5];

    let v_iter = v1.iter();

    for val in v_iter {
        println!("GET {}", val);
    }

    let total = v1.iter().sum::<u32>();

    println!("the sum of v1 is {}", total);

    let res = sum(v1.into_iter());
    println!("res = {}", res);

    //  test Counter :
    let mut counter = Counter::new(3);

    println!("next first time : {}", counter.next().unwrap());
    println!("next first time : {}", counter.next().unwrap());
    println!("next first time : {}", counter.next().unwrap());

    println!("c.value = {}", counter.max_value);

    let t: u32 = Counter::new(10)
        .zip(Counter::new(10))
        .map(|(a, b)| a * b)
        .filter(|x| x % 3 == 0)
        .skip(2)
        .sum();

    println!("t = {}", t);

    let v1 = vec![1, 2, 3]; // Longueur 3
    let v2 = vec!["a", "b"]; // Longueur 2

    let zipped: Vec<_> = v1.iter().zip(v2.iter()).collect();
    println!("{:?}", zipped); // Affiche `[(1, "a"), (2, "b")]` (3 est ignoré)

    let c = Counter::new(10);

    for val in c {
        println!("{}", val);
    }

    // for val in c {
    //     println!("{}", val);
    // }
}

#[test]
fn test_next() {
    let v1 = vec![1, 2, 4];

    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&4));
    assert_eq!(v1_iter.next(), None);
}

fn sum<T, U>(v: T) -> U
where
    T: IntoIterator<Item = U>,
    U: std::ops::Add<Output = U> + std::iter::Sum + Copy + Default + Display,
{
    let mut res: U = U::default();
    println!("res defualt is {}", res);

    for vl in v {
        res = res + vl;
    }

    res
}
