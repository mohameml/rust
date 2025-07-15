mod restaurant {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn save_order() {}

        fn take_payement() {}
    }
}

fn eat_at_rs() {
    restaurant::hosting::add_to_waitlist();
}
