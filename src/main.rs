use std::{
    fmt::{self, Display, Formatter},
    thread::sleep,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

const GRAY: u32 = 0xaa_aa_aa;
// const RED: u32 = 0xff_00_00;
// const YELLOW: u32 = 0xff_00_ff;
// const BLUE: u32 = 0x00_00_ff;
const GREEN: u32 = 0x00_ff_00;
// const CYAN: u32 = 0x00_ff_ff;

struct Colored(u32, u8);

impl Display for Colored {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let Self(a, b) = self;
        write!(f, "<fc=#{:06x}>{:02x}</fc>", a, b)
    }
}

fn print_hextime(x: u64) {
    let [.., d, c, b, a] = x.to_be_bytes();
    println!(
        "{} {} {} {}",
        Colored(GRAY, d),
        Colored(GRAY, c),
        Colored(GREEN, b),
        Colored(GRAY, a),
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
