fn main() {
    {
        enum IpAddr {
            V4(u8, u8, u8, u8),
            V6(String),
        }

        let home = IpAddr::V4(127, 0, 0, 1);
        let loopback = IpAddr::V6(String::from("::1"));
    }

    {
        enum Message {
            Quit,
            Move { x: i32, y: i32},
            Write(String),
            ChangeColor(i32, i32, i32),
        }
    }
}
