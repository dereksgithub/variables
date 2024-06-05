enum IPAddress{
    // previously known as IPAddrKind
    //V4,
    //V6,
    // allows us to enumerate a list of possible values
    V4(u8, u8, u8, u8),
    V6(String),
}

enum message{
    //
    quit,
    Move{x: i32, y: i32},
    write(String),
    changeColor(i32, i32, i32),
    
}
// we can also define methods on enums, just like structs:
impl message{
    fn call(&self){
        // method body would be defined here
        println!("call method");
    }
}


struct IpAddr{
    // we can use the enum as a type:
    //kind: IPAddrKind,
    //address: String,

}

fn main(){
    //let four: IPAddrKind = IPAddrKind::V4;
    //let six: IPAddrKind = IPAddrKind::V6;
    
    /*let localhost:IpAddr = IpAddr{
        kind: IPAddrKind::V6("127.88.0.0"),
        address:String::from("127.88.0.0")};
    */
    let newMessage = message::write(String::from("hello"));

}


//in Rust, there are nulls,
// instead, Rust has an enum that can encode the concept of a value being present or absent.
// This enum is `Option<T>`, and it is defined by the standard library as follows:
// enums can also be generic:
enum Option<T>{
    Some(T),
    None,
}
// add implements to the enum:
impl<T> Option<T>{
    fn unwrap(self) -> T{
        // if the value is Some, return the value
        // if the value is None, panic
    
    }
}

// we can also define our own Option enum:
enum MyOption<T>{
    MySome(T),
    MyNone,
}

// we can also define our own Result enum:
enum MyResult<T, E>{
    MyOk(T),
    MyErr(E),
}

// MyOk and MyErr are called the variants of the enum
// we can also define methods on enums:
impl<T, E> MyResult<T, E>{
    fn unwrap(self) -> T{
        // if the value is MyOk, return the value
        // if the value is MyErr, panic
    
    }
}

// we can also define a struct with generics:
struct Point<T>{
    x: T,
    y: T,
}
// implementing methods on generic types:
impl<T> Point<T>{
    // self is an instance of the struct
    // &self is a reference to the instance of the struct
    // &mut self is a mutable reference to the instance of the struct
    // we can also define methods that only apply to a specific instance of the generic type:
    fn x(&self) -> &T{
        &self.x
    }
}


fn route(ip_type: IPAddress){




}