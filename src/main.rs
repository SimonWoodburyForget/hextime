use formatting::*;
use std::{
    convert::TryInto,
    io::{self, Write},
    thread::sleep,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

mod formatting {
    use std::fmt::{self, Display, Formatter};

    #[derive(Clone, Copy)]
    pub enum Color {
        Gray,
        LightGray,
        Red,
        Yellow,
        Blue,
        Green,
        Cyan,
    }

    impl From<Color> for u32 {
        fn from(other: Color) -> u32 {
            use Color::*;
            match other {
                Gray => 0x99_99_99,
                LightGray => 0xcc_cc_cc,
                Red => 0xff_00_00,
                Yellow => 0xff_00_ff,
                Blue => 0x00_00_ff,
                Green => 0x00_ff_00,
                Cyan => 0x00_ff_ff,
            }
        }
    }

    impl Display for Color {
        fn fmt(&self, f: &mut Formatter) -> fmt::Result {
            let x: u32 = (*self).into();
            write!(f, "{:06x}", x)
        }
    }

    pub struct Fc<T>(pub Color, pub T);
    impl<T: Display> Display for Fc<T> {
        fn fmt(&self, f: &mut Formatter) -> fmt::Result {
            let Self(a, b) = self;
            write!(f, "<fc=#{}>{}</fc>", a, b)
        }
    }

    pub struct Hex2<T>(pub T);
    impl<T: Display + fmt::UpperHex> Display for Hex2<T> {
        fn fmt(&self, f: &mut Formatter) -> fmt::Result {
            write!(f, "{:02X}", self.0)
        }
    }

    pub struct XmobarFmt<T>(pub T);
    impl Display for XmobarFmt<u32> {
        fn fmt(&self, f: &mut Formatter) -> fmt::Result {
            let [d, c, b, a] = self.0.to_be_bytes();
            write!(
                f,
                "{} {} {} {}",
                Fc(Color::Gray, Hex2(d)),
                Fc(Color::Gray, Hex2(c)),
                Fc(Color::Green, Hex2(b)),
                Fc(Color::LightGray, Hex2(a)),
            )
        }
    }

    pub struct TermFmt<T>(pub T);
    impl Display for TermFmt<u32> {
        fn fmt(&self, f: &mut Formatter) -> fmt::Result {
            let [d, c, b, a] = self.0.to_be_bytes();
            write!(f, "{} {} {} {}", Hex2(d), Hex2(c), Hex2(b), Hex2(a))
        }
    }
}

fn print_xbar(x: u32) {
    print!("{}", XmobarFmt(x));
}

fn print_term(x: u32) {
    print!("{}", TermFmt(x));
}

fn print_two(x: u32) {
    let [.., b, a] = x.to_be_bytes();
    print!("{} {}", Fc(Color::Cyan, b), Fc(Color::LightGray, a),);
}

/// Returns an iterator of distinct 32-bit seconds.
fn seconds() -> impl Iterator<Item = u32> {
    std::iter::repeat_with(SystemTime::now)
        .map(|time| time.duration_since(UNIX_EPOCH))
        .filter_map(|result| result.map(|x| x.as_secs()).ok())
        .map(|secs| secs.try_into().expect("Time overflowed."))
}

fn main() {
    seconds().for_each(|now| {
        print_xbar(now);
        io::stdout().flush().unwrap();
        print!("\r");
        sleep(Duration::from_secs(1));
    });
}
