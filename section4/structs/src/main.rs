struct User{
    active: bool,
    username: String,
    sign_in_count: u32,
}

struct Coordinates(i32,i32,i32);

struct UnitStruct;

fn main() {
    let user1 = User{active:true, username: String::from("Rf"), sign_in_count: 12};
    println!("{}", user1.username);


    let user2 = build_user(String::from("Tyler"));
    println!("{}", user2.username);


    let coords = Coordinates(1,2,3);

    //1..5 UnitStruct  .. = Range -------- Range(start:1,end:5)
}

fn build_user(username:String)->User{
    User{
        username,
        active:true,
        sign_in_count:1,
    }
}
