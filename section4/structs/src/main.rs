// struct User{
//     active: bool,
//     username: String,
//     sign_in_count: u32,
// }

// struct Coordinates(i32,i32,i32);

// struct UnitStruct;


// struct Square{
//     width:u32,
//     height:u32,
// }

// impl Square{
//     fn area(&self)->u32{
//         self.width*self.height
//     }
//     fn whats_my_width(&self)->u32{
//         self.width
//     }

//     fn change_width(&mut self, new_width: u32){
//         self.width = new_width;
//     }
// }

// struct MyString<'a>{
//     text: &'a str,
// }


                                        //section 4 Assignment
                    /*Create a struct called Car with the fields: mpg, color, and top_speed. 
                    Once the struct is created, implement the following methods: set_mpg, set_color, and set_top_speed.
                    Once you have created these methods, create a car, provide it default values, 
                    and then set the mpg, color, and top speed and then print them out. */

struct Car{
    mpg: u32,
    color: String,
    top_speed: u32,
}

impl Car{
    fn set_mpg(&mut self, new_mpg: u32){
        self.mpg = new_mpg;
    }
    fn set_color(&mut self, new_color:String){
        self.color = new_color;
    }
    fn set_top_speed(&mut self, new_top_speed:u32){
        self.top_speed = new_top_speed;
    }
}


fn main() {

    let mut car1 = Car{mpg:3, color:String::from("Blue"),top_speed:100};
    println!("{}",car1.mpg);
    println!("{}",car1.color);
    println!("{}",car1.top_speed);
    car1.set_mpg(5);
    car1.set_color(String::from("Red"));
    car1.set_top_speed(120);
    println!("{}",car1.mpg);
    println!("{}",car1.color);
    println!("{}",car1.top_speed);

    // let str1 = String::from("This is my string");
    // let x = MyString{text:str1.as_str()};
    // let s :&'static str = "I have a static Lifetime";

//     let user1 = User{active:true, username: String::from("Rf"), sign_in_count: 12};
//     println!("{}", user1.username);


//     let user2 = build_user(String::from("Tyler"));
//     println!("{}", user2.username);


//     let coords = Coordinates(1,2,3);

//     //1..5 UnitStruct  .. = Range -------- Range(start:1,end:5)
// }

// let mut square = Square{width: 5, height: 5};
    // println!("{}", square.area());
    // println!("{}", square.whats_my_width());
    // square.change_width(12);
    // println!("{}", square.whats_my_width());

    // let r;
    // {
    //     let x=5;
    //     r = &x;
    // }// x is dropped here
    // println!("{}", r); //borrow used(no value)

    //i32
    //&'a i32
    //&'a mut i32

    // fn example<'a, 'b>(x: &'a str, y: &'b str) -> 'a str{
    //     x
 //'a for a parameter, 'b for a second parameter
}


// fn build_user(username:String)->User{
//     User{
//         username,
//         active:true,
//         sign_in_count:1,
//     }