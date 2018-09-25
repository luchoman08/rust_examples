
mod user {
use std::fmt;
use std::clone;
    pub struct User {
        id: u32,
        pub name: String,
        valid: bool
    }
    impl User {
        pub fn new (id: u32, name: String, valid: bool) -> User {
            User {id: id, name: name, valid: valid} 
        }
        pub fn to_valid(&mut self) {
            self.valid = true;
        }
    }
    impl clone::Clone for User {
        fn clone(&self) -> User {
            User{ id: self.id, name: self.name.clone(), valid: self.valid}
        }
    }
    pub trait ValidableVector {
        fn make_all_valid(&mut self)->&Self;
    }
    impl ValidableVector for Vec<User> {
        fn make_all_valid(&mut self) -> &Self {
            for ref mut element in self.iter_mut() {
                element.valid = true;
            }
            self
        }
    }
    impl<'a> ValidableVector for Vec <&'a mut User> {
        fn make_all_valid(&mut self) -> &Self {
            for ref mut element in self.iter_mut() {
                element.valid = true;
            }
            self
        }
    }
    impl fmt::Display for User {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "(User {}, {}, {})", self.id, self.name, self.valid)
        }
    }
}

fn main() {
    use user::*;
    let mut user = User::new(1, "Luis Manrique".to_string(), false);
    let mut user2 = user.clone();
    user2.name = "Gabriel".to_string();
    let mut users: Vec<&mut User> = vec![&mut user, &mut user2];
    users.make_all_valid();
    println!("Hello {}", users[1]);
}
