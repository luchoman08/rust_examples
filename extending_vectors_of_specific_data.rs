mod user_mod {
    use std::fmt;
    
    pub struct User {
        id: u32,
        state: bool
    }
    pub trait Validable {
        fn make_all_valid(&mut self)-> &mut Self;
        fn make_all_invalid(&mut self)-> &mut Self;
    }
    impl Validable for Vec<User> {
        fn make_all_valid(&mut self)-> &mut Vec<User> {
            for mut item in &mut self.into_iter() {
                item.make_valid();
            }
            self
        }
        fn make_all_invalid(&mut self)-> &mut Vec<User> {
            for mut item in &mut self.into_iter() {
                item.state = false;
            }
            self
        }
    }
    impl fmt::Display for User {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "({}, {})", self.id, self.state)
        }
    }
    
    impl User {
        pub fn new(id: u32, state: bool ) -> User {
            User { id: id, state: state}
        }
        pub fn make_valid(&mut self) {
            self.state = true;
        }
        pub fn get_id(&self) -> u32 {
            self.id
        }
        pub fn valid(&self) -> bool {
            self.state
        }
    }
}

fn main() {
    use user_mod::*;
    let user = User::new (0, false);
    let user2 = User::new (1, true);
    let user3 = User::new (3, true);
    let mut users: Vec<User> = Vec::new();
    users.append(&mut vec![user, user2, user3]);
    users
        .make_all_valid()
        .make_all_invalid()
        .make_all_valid();
    println!("{}", users[0]);
}

