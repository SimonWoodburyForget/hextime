use formatting::*;
use std::{
    convert::TryInto,
    io::{self, Write},
    thread::sleep,
    time::{Duration, SystemTime, UNIX_EPOCH},
};
use structopt::StructOpt;

mod cli {
    use structopt::StructOpt;

    #[derive(Debug, Clone, Copy)]
    pub enum Mode {
        Xmobar,
        Terminal,
        ColorTerm,
    }

    impl Mode {
        const STRS: [&'static str; 3] = ["xmobar", "terminal", "cterm"];
        const MODE: [Mode; 3] = [Mode::Xmobar, Mode::Terminal, Mode::ColorTerm];
    }

    impl std::str::FromStr for Mode {
        type Err = &'static str;
        fn from_str(input: &str) -> Result<Self, Self::Err> {
            Mode::STRS
                .iter()
                .zip(Mode::MODE.iter())
                .find_map(|(&x, &m)| if x == input { Some(m) } else { None })
                .ok_or("no match")
        }
    }

    #[derive(StructOpt, Debug)]
    pub struct Opt {
        /// Printing mode.
        #[structopt(short, long, possible_values = &Mode::STRS)]
        pub mode: Mode,
    }
}

mod formatting {
    use super::HexTime;
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
        fn from(other: Color) -> Self {
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

    #[derive(Clone)]
    pub struct Hex2<T>(pub T);
    impl<T: Display + fmt::UpperHex> Display for Hex2<T> {
        fn fmt(&self, f: &mut Formatter) -> fmt::Result {
            write!(f, "{:02X}", self.0)
        }
    }

    pub struct XmobarFmt<T>(pub T);
    impl Display for XmobarFmt<u32> {
        fn fmt(&self, f: &mut Formatter) -> fmt::Result {
            HexTime(self.0)
                .segmented()
                .map(|(s, x)| Fc(Color::from(s), Hex2(x)))
                .try_for_each(|x| write!(f, "{} ", x))
        }
    }

    pub struct TermFmt<T>(pub T);
    impl Display for TermFmt<u32> {
        fn fmt(&self, f: &mut Formatter) -> fmt::Result {
            HexTime(self.0)
                .bytes()
                .try_for_each(|x| write!(f, "{} ", Hex2(x)))
        }
    }
}

mod seg {
    /// Segment of HexTime.
    #[derive(Clone, Copy)]
    pub struct Segment(usize);

    impl Segment {
        /// Return offset segment of HexTime. *Value must be smaller then 8.*
        pub fn new(x: usize) -> Self {
            if x < 8 {
                Self(x)
            } else {
                panic!("Time segment is too large.")
            }
        }
    }

    impl From<Segment> for crossterm::style::Color {
        fn from(seg: Segment) -> Self {
            use crossterm::style::Color::*;
            match seg.0 {
                0 => White,
                1 => Green,
                _ => DarkGrey,
            }
        }
    }

    impl From<Segment> for crate::formatting::Color {
        fn from(seg: Segment) -> Self {
            use crate::formatting::Color::*;
            match seg.0 {
                0 => LightGray,
                1 => Green,
                _ => Gray,
            }
        }
    }
}

#[derive(Clone, Copy)]
struct HexTime(u32);

impl HexTime {
    /// Iterator over increasing bytes of time value.
    fn bytes(self) -> impl Iterator<Item = u8> {
        let bytes = self.0.to_be_bytes();
        (0..bytes.len()).map(move |idx| bytes[idx])
    }

    /// Iterator over increasing bytes of time value, with an offset from zero segment.
    fn segmented(self) -> impl Iterator<Item = (seg::Segment, u8)> {
        self.bytes()
            .enumerate()
            .map(move |(a, b)| (seg::Segment::new(self.0.to_be_bytes().len() - 1 - a), b))
    }
}

fn print_xbar(x: u32) {
    print!("{}", XmobarFmt(x));
}

fn print_term(x: u32) {
    print!("{}", TermFmt(x));
}

fn print_term_color(x: u32) {
    use crossterm::{
        cursor, execute,
        style::{Print, ResetColor, SetForegroundColor},
    };
    use std::io::stdout;
    HexTime(x)
        .segmented()
        .try_for_each(|(s, x)| {
            execute!(
                stdout(),
                SetForegroundColor(s.into()),
                Print(formatting::Hex2(x)),
                Print(" "),
                ResetColor,
                cursor::Hide
            )
        })
        .unwrap();
}

/// Returns an iterator of distinct 32-bit seconds.
fn seconds() -> impl Iterator<Item = u32> {
    std::iter::repeat_with(SystemTime::now)
        .map(|time| time.duration_since(UNIX_EPOCH))
        .filter_map(|result| result.map(|x| x.as_secs()).ok())
        .map(|secs| secs.try_into().expect("Time overflowed."))
}

fn main() {
    let print = {
        use cli::{Mode::*, Opt};
        match Opt::from_args() {
            Opt { mode: Xmobar } => print_xbar,
            Opt { mode: Terminal } => print_term,
            Opt { mode: ColorTerm } => print_term_color,
        }
    };

    seconds().for_each(|now| {
        print(now);
        io::stdout().flush().unwrap();
        print!("\r");
        sleep(Duration::from_secs(1));
    });
}
