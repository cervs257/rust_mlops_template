// associated functions and regular methods

struct User {
    username: String,
    email: String,
    uri: String,
    active: bool,
}

impl User {
    // create a new instance of User. This is an associated function as it's related to struct but not an instance of struct
    fn new(username: String, email: String, uri: String) -> Self {
        Self {
            username,
            email,
            uri,
            active: true,
        }
    }
    // deactivate existing user. This is a regular method where self must be passed. It's associated with a particular instance of a type
    fn deactivate(&mut self) {
        self.active = false;
    }
    // create a new user using its email address. This is an associated function as it's related to struct but not an instance of struct
    fn from_email(email: String) -> Self {
        // define username as what comes before @ in the email input
        let username = email.split('@').next().unwrap_or("").to_string();
        Self {
            username,
            email,
            uri: String::from(""), // empty string
            active: true,
        }
    }
}

fn main() {
    let mut new_user = User::new(
        String::from("alfredodeza"),
        String::from("alfreodeza@example.com"),
        String::from("https://alfredodeza.com"),
    );
    println!("Hello, {}!", new_user.username);
    println!(
        "Account {} status is: {}",
        new_user.username, new_user.active
    );
    new_user.deactivate();
    println!(
        "Account {} status is: {}",
        new_user.username, new_user.active
    );
    let mut newer_user = User::from_email("newer_user@rust.com".to_string());
    println!("Hello, {}!", newer_user.username);
}
