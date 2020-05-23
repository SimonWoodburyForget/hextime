use std::{thread::sleep, time::Duration};
use time::{OffsetDateTime, PrimitiveDateTime, Time, UtcOffset};

mod display_impl {
    use super::{Color, Fc, Hex2};
    use std::fmt::{self, Display, Formatter};

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

    impl<A: Display, B: Display> Display for Fc<A, B> {
        fn fmt(&self, f: &mut Formatter) -> fmt::Result {
            let Self(a, b) = self;
            write!(f, "<fc=#{}>{}</fc>", a, b)
        }
    }

    impl<T: Display + fmt::UpperHex> Display for Hex2<T> {
        fn fmt(&self, f: &mut Formatter) -> fmt::Result {
            write!(f, "{:02X}", self.0)
        }
    }
}

#[derive(Clone, Copy)]
enum Color {
    Gray,
    LightGray,
    Red,
    Yellow,
    Blue,
    Green,
    Cyan,
}

struct Fc<A, B>(A, B);
struct Hex2<T>(T);

fn print_full(x: u32) {
    match x.to_be_bytes() {
        [d, c, b, a] => print!(
            "{} {} {} {}",
            Fc(Color::Gray, Hex2(d)),
            Fc(Color::Gray, Hex2(c)),
            Fc(Color::Green, Hex2(b)),
            Fc(Color::LightGray, Hex2(a)),
        ),
    }
}

fn print_two(x: u32) {
    let [.., b, a] = x.to_be_bytes();
    print!("{} {}", Fc(Color::Cyan, b), Fc(Color::LightGray, a),);
}

fn main() {
    std::iter::repeat_with(OffsetDateTime::now_local).for_each(|now| {
        print_full(now.timestamp() as u32);
        print!(" - ");
        print_two(
            PrimitiveDateTime::new(now.date().next_day(), Time::midnight())
                .assume_offset(UtcOffset::current_local_offset())
                .timestamp() as u32,
        );
        println!();
        sleep(Duration::from_secs(1));
    });
}
