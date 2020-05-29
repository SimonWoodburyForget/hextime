use formatting::*;
use std::{
    convert::TryInto,
    io::{self, Write},
    thread::sleep,
    time::{Duration, SystemTime, UNIX_EPOCH},
};
use structopt::StructOpt;

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
            use Color::*;
            self.0
                .to_be_bytes()
                .iter()
                .zip([Gray, Gray, Green, LightGray].iter())
                .try_for_each(|(&x, &c)| write!(f, "{} ", Fc(c, Hex2(x))))
        }
    }

    pub struct TermFmt<T>(pub T);
    impl Display for TermFmt<u32> {
        fn fmt(&self, f: &mut Formatter) -> fmt::Result {
            self.0
                .to_be_bytes()
                .iter()
                .try_for_each(|&x| write!(f, "{} ", Hex2(x)))
        }
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
        execute,
        style::{Color::*, Print, ResetColor, SetForegroundColor},
    };
    use std::io::stdout;
    x.to_be_bytes()
        .iter()
        .zip([DarkGrey, DarkGrey, Green, White].iter())
        .try_for_each(|(&x, &c)| {
            execute!(
                stdout(),
                SetForegroundColor(c),
                Print(formatting::Hex2(x)),
                Print(" "),
                ResetColor
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
