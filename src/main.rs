use std::{
    fmt::{self, Display, Formatter},
    thread::sleep,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

struct Colored(u32, u8);

impl Display for Colored {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let Self(a, b) = self;
        write!(f, "<fc=#{:02x}>{:02x}</fc>", a, b)
    }
}

fn print_hextime(x: u64) {
    let [a, b, c, d, ..] = x.to_le_bytes();
    println!(
        "{} {} {} {}",
        Colored(0xaaaaaa, d),
        Colored(0xaaaaaa, c),
        Colored(0xff0000, b),
        Colored(0xff0000, a),
    );
    sleep(Duration::from_secs(1));
}

fn main() {
    std::iter::repeat_with(SystemTime::now)
        .map(|t| t.duration_since(UNIX_EPOCH))
        .for_each(|r| match r {
            Ok(x) => print_hextime(x.as_secs()),
            Err(_) => println!("..time-travelling"),
        })
}
