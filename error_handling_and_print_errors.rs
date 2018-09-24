
mod user {
use std::fmt;
    pub enum ErrorKind {
        RepeatedId,
        NameSoShort,
        ShouldBeValidUser
    }
    impl ErrorKind {
        fn as_str(&self) -> &'static str{
            match *self {
                ErrorKind::RepeatedId => "repeated id",
                ErrorKind:: ShouldBeValidUser => "should be valid user",
                _ => "no message for this error "
            }
        }
    }
    impl fmt::Display for ErrorKind {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }        
    }
    pub struct User {
        id: u32, 
        name: String,
        valid: bool
    }
    pub trait UserVectorFunctions {
        fn make_all_valid(&mut self) -> &Self;
        fn valid(&self)->bool;
    }

    impl UserVectorFunctions for Vec<User> {
        fn make_all_valid(&mut self) -> &Self {
            for user in  self.iter_mut() {
                user.valid = true;
            }
            self
        }
        fn valid(&self) -> bool {
            for user in self.iter() {
                if user.valid { 
                continue; 
                } else {
                    return false;
                }
            }
            true
        }
    }
    impl User {
        pub fn new(id: u32, name: String, valid: bool) -> User {
            User {id: id, name: name, valid: valid}
        }
        pub fn save(&self) -> Result<&Self, ErrorKind> {
            if !self.valid {
                return Err(ErrorKind::ShouldBeValidUser);
            }
            return Ok(&self);
            
        }
    }
    
    impl fmt::Display for User {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "User {} ({}, {})", self.id, self.name, self.valid)
        }
}
}
use user::*;
fn main() {

    let user = User::new(5, "Luis Manrique".to_string(), true);
    let user2 = User::new(0, "Olga Gomez".to_string(), false);
    match user2.save() {
        Ok(_)=> println!("Se ha guardado con exito"),
        Err(err) => println!("Se ha encontrado un error: {}", err)
    }
    let mut users: Vec<User> = vec![user, user2];
    println!("{}", users.valid());
    users.make_all_valid();
    println!("{}", users[1].to_string());
    println!("{}", users.valid());
}
