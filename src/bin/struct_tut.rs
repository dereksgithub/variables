// Path: src\bin\struct_tut.rs
struct User{
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

struct Session{
    user: User,
    time: u64,
    deposit: u64,
    balance: u64,
    membership: bool,
    id: u64,
}



fn main(){
    let mut user_1 = User{
        email: String::from("sample@usersample.com"),
        username: String::from("sample_user"),
        active: true,
        sign_in_count: 1,
    };

    //let name = user_1.username;
    //println!("{}", name);
    // modify a struct:
    user_1.username = String::from("sample_user2");
    //println!("{}", user_1.username);

    // session to register a new user:
    let user_3 = Session::register_user(String::from("user3@email.com"), String::from("username"), true, 1);
    user_3.print_user();

    //user_1.print_user();

    let user_2: User = build_user(
    String::from("user2@email.com"),
    String::from("Jack User2"),
    true, 
    1
    );
    //user_2.print_user();
        


    let session_1 = Session{
        user: user_2,
        time: 123123,
        deposit: 0000,
        balance: 1899,
        membership: true,
        id: 888999,

    };

    session_1.print_session();
    
}


fn build_user(email: String, username: String, active: bool, sign_in_count: u64) -> User{
    User{
        email,// field init shorthand syntax
        username: username,
        active: active,
        sign_in_count: sign_in_count,
    }

}

impl User{
    fn print_user(&self){
        println!("Username is: {}", self.username);
        println!("Email is: {}", self.email);
        println!("Active: {}", self.active);
        println!("Sign in count: {}", self.sign_in_count);
    }
}

impl Session{

    // equivalent to signing up
    fn register_user(email: String, username: String, active: bool, sign_in_count: u64)-> User{
        User{
            email,// field init shorthand syntax
            username: username,
            active: active,
            sign_in_count: sign_in_count,
        }

    }
    fn print_session(&self){
        self.user.print_user();
        println!("Email: {}", self.time);
        println!("Active: {}", self.deposit);
        println!("Balance: {}", self.balance);
        println!("Member: {}", self.membership);
        println!("User ID is: {}", self.id);
    }
}


