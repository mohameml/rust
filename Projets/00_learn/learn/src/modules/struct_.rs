#[derive(Debug)]
pub struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

#[derive(Debug)]
pub struct Rect {
    width: i32,
    height: i32,
}

impl Rect {
    fn area(&self) -> i32 {
        return self.width * self.height;
    }

    fn can_hold(&self, other: &Rect) -> bool {
        let my_area1 = self.area();
        let other_area = other.area();

        return my_area1 > other_area;
    }
}

impl PartialEq for Rect {
    fn eq(&self, other: &Self) -> bool {
        return self.area() > other.area();
    }

    // fn ne(&self, other: &Self) -> bool {}
}

impl Rect {
    fn square(size: i32) -> Rect {
        Rect {
            width: size,
            height: size,
        }
    }
}

// impl User {
//     pub
// }

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

// Tuples Struct

struct Color(u32, u32, u32);

struct Point(i32, i32, i32);

pub fn struct_() {
    let mut user1 = User {
        active: true,
        username: String::from("sidi"),
        email: String::from("sidi@email.com"),
        sign_in_count: 1,
    };

    let name: &String = &user1.username;

    println!("name = {name:?}");
    user1.username = String::from("Ahmed");

    println!("user1 = {user1:#?}");

    let user2 = build_user(String::from("kh@mail.com"), String::from("kh"));

    println!("user2 = {user2:?}");

    let user3 = User {
        email: String::from("user3@mail.com"),
        ..user2
    };

    println!("user3 =  {user3:?}");

    // Rect :
    let rec1 = Rect {
        width: 10,
        height: 2,
    };

    let area_rect1: i32 = rec1.area();

    println!("rect1 = {rec1:?} with the area {area_rect1}");

    let rec2 = Rect {
        width: 13,
        height: 2,
    };

    println!("rec1 == rec2 : {}", rec1 == rec2);

    let carre = Rect::square(20);

    println!("carre = {carre:?}");
}
