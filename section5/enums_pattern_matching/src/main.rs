enum Pet{dog,cat,fish}

impl Pet{
    fn what_am_i(self) -> &'static str{
        match self{
            Pet::dog => "I am a dog",
            Pet::cat => "I am a cat",
            Pet::fish => "I am a fish",
        }
    }
}

enum IpAddrKind{
    V4(String),
    V6,
}

struct IpAddr{
    kind: IpAddrKind,
    addr: String,
}

fn main() {
    let dog2 = Pet::dog;
    println!("{}", dog2.what_am_i())

    let home = IpAddrKind::V4(String::from("127.0.0.1"));

    let loopack = IpAddr{kind:IpAddrKind::V6, addr:String.from("::1")},

}
