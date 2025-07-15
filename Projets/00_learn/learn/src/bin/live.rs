use std::fmt::Display;

#[derive(Debug)]
struct Article<'a> {
    txt: &'a str,
}

fn main() {
    // let str1 = String::from("s1");
    // let str2 = String::from("s22");

    // let res = longest(str1.as_str(), str2.as_str());
    // println!("{}", res);

    let s1 = String::from("longue chaîne");
    let result: &str;
    {
        let s2 = String::from("xyz");
        result = longest_with_ann(s1.as_str(), s2.as_str(), "Hello"); //longest(s1.as_str(), s2.as_str());
        println!("La plus longue est {}", result); // OK
    }
    // println!("La plus longue est {}", result); // ERREUR! s2 n'existe plus

    let a;
    {
        let txt: &str = "string info";

        a = Article { txt: txt };
    }

    println!("Article a = {:?}", a);

    println!("txt = {}", a.txt);
}

// &i32
// &mut i32
//  &'a i32
//  &'a mut i32

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

fn longest_with_ann<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    longest(x, y)
}
