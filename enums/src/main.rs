use int_enum::IntEnum;

enum IpAddrKind {
    v4,
    v6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}
#[repr(u8)]
#[derive(IntEnum)]
enum IpAddr2 {
    V4(u8, u8, u8, u8),
    V6(String),
}



fn main() {
    let home1: IpAddr = IpAddr {
        kind: IpAddrKind::v4,
        address: String::from("127.0.0.1"),
    };

    let loopback1: IpAddr = IpAddr {
        kind: IpAddrKind::v6,
        address: String::from("::1"),
    };
    

    let home2: IpAddr2 = IpAddr2::V4(127, 0, 0, 1);
    let loopback2: IpAddr2 = IpAddr2::V6(String::from("::1"));

    let version: u32 = match home2 {
        IpAddr2::V4(n1, n2, n3, n4) => 4,
        IpAddr2::V6(addr) => 6,
    };

    let data = [[1, 2, 1], [2, 3, 400], [1, 1, 2]];
    println!("largest {:?}", largest2(&data));

    println!("Hello, world! {}", version);
    test(5);
}


fn test(n: u32) {
    for i in 0..n {
        println!("n: {}", i)
    }
}

fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest_i = 0;
    for (i, item) in list.iter().enumerate() {
        if item > &list[largest_i] {
            largest_i = i;
        }
    }
   &list[largest_i]
}

fn largest2<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest: &T = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}