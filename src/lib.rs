//! A rust library to deal with number conversion between radices.

use std::error;
use std::fmt;

const DEBUG: bool = false;

macro_rules! debug {
    ($fmt:expr $(, $args:expr)*) => {{
        if DEBUG {
            use std::io::Write;
            println!($fmt, $($args),*);
            std::io::stdout().flush().unwrap();
        }
    }}
}



pub type RadixResult<T> = Result<T, RadixErr>;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum RadixErr {
    RadixNotSupported(usize),
    EmptyInput,
    FailedToPopFromStack,
    FailedToUppercase,
    IllegalChar(char),
    IllegalDigit(usize),
    InvalidDigit { digit: char, radix: usize },
}

impl error::Error for RadixErr {
    fn description(&self) -> &str {
        match *self {
            RadixErr::RadixNotSupported(_) => "Radix not supported",
            RadixErr::EmptyInput => "Empty Input",
            RadixErr::FailedToPopFromStack => "Failed to pop from stack",
            RadixErr::FailedToUppercase => "Failed to uppercase",
            RadixErr::IllegalChar(_) => "Illegal char",
            RadixErr::IllegalDigit(_) => "Illegal digit",
            RadixErr::InvalidDigit{..} => "Invalid digit",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        None
    }
}

impl fmt::Display for RadixErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RadixErr::RadixNotSupported(ref us) =>
                write!(f, "Radix not supported: {}", &us),
            RadixErr::EmptyInput =>
                write!(f, "There was empty input when converting to Radix"),
            RadixErr::FailedToPopFromStack =>
                write!(f, "Failed to pop from stack"),
            RadixErr::FailedToUppercase =>
                write!(f, "Failed to convert character to uppercase"),
            RadixErr::IllegalChar(ref c) =>
                write!(f, "Illegal character: {}", &c),
            RadixErr::IllegalDigit(ref us) =>
                write!(f, "Illegal digit: {}", &us),
            RadixErr::InvalidDigit{digit: c, radix: us} =>
                write!(f, "Invalid digit: {} {}", &c, &us),
        }
    }
}

const MAX_RADIX: usize = 36;
const MIN_RADIX: usize = 2;

fn is_radix_valid(radix: usize) -> bool {
    radix >= MIN_RADIX && radix <= MAX_RADIX
}


/// A number in some radix.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum RadixNum {
    #[doc(hidden)] Radix2(String),
    #[doc(hidden)] Radix3(String),
    #[doc(hidden)] Radix4(String),
    #[doc(hidden)] Radix5(String),
    #[doc(hidden)] Radix6(String),
    #[doc(hidden)] Radix7(String),
    #[doc(hidden)] Radix8(String),
    #[doc(hidden)] Radix9(String),
    #[doc(hidden)] Radix10(String),
    #[doc(hidden)] Radix11(String),
    #[doc(hidden)] Radix12(String),
    #[doc(hidden)] Radix13(String),
    #[doc(hidden)] Radix14(String),
    #[doc(hidden)] Radix15(String),
    #[doc(hidden)] Radix16(String),
    #[doc(hidden)] Radix17(String),
    #[doc(hidden)] Radix18(String),
    #[doc(hidden)] Radix19(String),
    #[doc(hidden)] Radix20(String),
    #[doc(hidden)] Radix21(String),
    #[doc(hidden)] Radix22(String),
    #[doc(hidden)] Radix23(String),
    #[doc(hidden)] Radix24(String),
    #[doc(hidden)] Radix25(String),
    #[doc(hidden)] Radix26(String),
    #[doc(hidden)] Radix27(String),
    #[doc(hidden)] Radix28(String),
    #[doc(hidden)] Radix29(String),
    #[doc(hidden)] Radix30(String),
    #[doc(hidden)] Radix31(String),
    #[doc(hidden)] Radix32(String),
    #[doc(hidden)] Radix33(String),
    #[doc(hidden)] Radix34(String),
    #[doc(hidden)] Radix35(String),
    #[doc(hidden)] Radix36(String),
}

impl RadixNum {
    /// Convert a `base` encoded in a certain `radix` to a `RadixNum`.
    pub fn from_str(base: &str, radix: usize) -> RadixResult<Self> {
        Self::validate_radix(radix)?;
        let base: String = Self::validate_base(&base, radix)?;
        let decimal: String = Self::radix_x_to_dec(&base, radix)?.to_string();
        RadixNum::Radix10(decimal).with_radix(radix)
    }

    #[inline(always)]
    fn validate_radix(radix: usize) -> RadixResult<()> {
        if !is_radix_valid(radix) {
            return Err(RadixErr::RadixNotSupported(radix));
        }
        Ok(())
    }

    #[inline(always)]
    fn validate_base(base: &str, radix: usize) -> RadixResult<String> {
        if base.is_empty() { return Err(RadixErr::EmptyInput); }
        let base: String = base.trim().to_uppercase();
        let is_valid_digit = |d| {
            let x = '0' <= d  &&  d <= '9';
            let y = 'A' <= d  &&  d <= ('A' as usize + radix - 10) as u8 as char;
            x || y
        };
        for digit in base.chars() { if !is_valid_digit(digit) {
            return Err(RadixErr::InvalidDigit { digit, radix });
        }}
        Ok(base)
    }

    pub fn as_str(&self) -> &str {
        match *self {
            RadixNum::Radix2(ref string) |
            RadixNum::Radix3(ref string) |
            RadixNum::Radix4(ref string) |
            RadixNum::Radix5(ref string) |
            RadixNum::Radix6(ref string) |
            RadixNum::Radix7(ref string) |
            RadixNum::Radix8(ref string) |
            RadixNum::Radix9(ref string) |
            RadixNum::Radix10(ref string) |
            RadixNum::Radix11(ref string) |
            RadixNum::Radix12(ref string) |
            RadixNum::Radix13(ref string) |
            RadixNum::Radix14(ref string) |
            RadixNum::Radix15(ref string) |
            RadixNum::Radix16(ref string) |
            RadixNum::Radix17(ref string) |
            RadixNum::Radix18(ref string) |
            RadixNum::Radix19(ref string) |
            RadixNum::Radix20(ref string) |
            RadixNum::Radix21(ref string) |
            RadixNum::Radix22(ref string) |
            RadixNum::Radix23(ref string) |
            RadixNum::Radix24(ref string) |
            RadixNum::Radix25(ref string) |
            RadixNum::Radix26(ref string) |
            RadixNum::Radix27(ref string) |
            RadixNum::Radix28(ref string) |
            RadixNum::Radix29(ref string) |
            RadixNum::Radix30(ref string) |
            RadixNum::Radix31(ref string) |
            RadixNum::Radix32(ref string) |
            RadixNum::Radix33(ref string) |
            RadixNum::Radix34(ref string) |
            RadixNum::Radix35(ref string) |
            RadixNum::Radix36(ref string) => &string,
        }
    }

    /// Change the radix that `self` is encoded with. This does not change
    /// the represented value, but it does change its representation.
    pub fn with_radix(&self, radix: usize) -> RadixResult<Self> {
        let digits_radix_x: String =
            Self::dec_to_radix_x(self.as_decimal()?, radix)?;
        Ok(match radix {
             2 => RadixNum::Radix2(digits_radix_x),
             3 => RadixNum::Radix3(digits_radix_x),
             4 => RadixNum::Radix4(digits_radix_x),
             5 => RadixNum::Radix5(digits_radix_x),
             6 => RadixNum::Radix6(digits_radix_x),
             7 => RadixNum::Radix7(digits_radix_x),
             8 => RadixNum::Radix8(digits_radix_x),
             9 => RadixNum::Radix9(digits_radix_x),
            10 => RadixNum::Radix10(digits_radix_x),
            11 => RadixNum::Radix11(digits_radix_x),
            12 => RadixNum::Radix12(digits_radix_x),
            13 => RadixNum::Radix13(digits_radix_x),
            14 => RadixNum::Radix14(digits_radix_x),
            15 => RadixNum::Radix15(digits_radix_x),
            16 => RadixNum::Radix16(digits_radix_x),
            17 => RadixNum::Radix17(digits_radix_x),
            18 => RadixNum::Radix18(digits_radix_x),
            19 => RadixNum::Radix19(digits_radix_x),
            20 => RadixNum::Radix20(digits_radix_x),
            21 => RadixNum::Radix21(digits_radix_x),
            22 => RadixNum::Radix22(digits_radix_x),
            23 => RadixNum::Radix23(digits_radix_x),
            24 => RadixNum::Radix24(digits_radix_x),
            25 => RadixNum::Radix25(digits_radix_x),
            26 => RadixNum::Radix26(digits_radix_x),
            27 => RadixNum::Radix27(digits_radix_x),
            28 => RadixNum::Radix28(digits_radix_x),
            29 => RadixNum::Radix29(digits_radix_x),
            30 => RadixNum::Radix30(digits_radix_x),
            31 => RadixNum::Radix31(digits_radix_x),
            32 => RadixNum::Radix32(digits_radix_x),
            33 => RadixNum::Radix33(digits_radix_x),
            34 => RadixNum::Radix34(digits_radix_x),
            35 => RadixNum::Radix35(digits_radix_x),
            36 => RadixNum::Radix36(digits_radix_x),
            radix => return Err(RadixErr::RadixNotSupported(radix)),
        })
    }

    /// Retrieve the radix that `self` is encoded with.
    pub fn radix(&self) -> usize {
        match *self {
            RadixNum::Radix2(_) => 2,
            RadixNum::Radix3(_) => 3,
            RadixNum::Radix4(_) => 4,
            RadixNum::Radix5(_) => 5,
            RadixNum::Radix6(_) => 6,
            RadixNum::Radix7(_) => 7,
            RadixNum::Radix8(_) => 8,
            RadixNum::Radix9(_) => 9,
            RadixNum::Radix10(_) => 10,
            RadixNum::Radix11(_) => 11,
            RadixNum::Radix12(_) => 12,
            RadixNum::Radix13(_) => 13,
            RadixNum::Radix14(_) => 14,
            RadixNum::Radix15(_) => 15,
            RadixNum::Radix16(_) => 16,
            RadixNum::Radix17(_) => 17,
            RadixNum::Radix18(_) => 18,
            RadixNum::Radix19(_) => 19,
            RadixNum::Radix20(_) => 20,
            RadixNum::Radix21(_) => 21,
            RadixNum::Radix22(_) => 22,
            RadixNum::Radix23(_) => 23,
            RadixNum::Radix24(_) => 24,
            RadixNum::Radix25(_) => 25,
            RadixNum::Radix26(_) => 26,
            RadixNum::Radix27(_) => 27,
            RadixNum::Radix28(_) => 28,
            RadixNum::Radix29(_) => 29,
            RadixNum::Radix30(_) => 30,
            RadixNum::Radix31(_) => 31,
            RadixNum::Radix32(_) => 32,
            RadixNum::Radix33(_) => 33,
            RadixNum::Radix34(_) => 34,
            RadixNum::Radix35(_) => 35,
            RadixNum::Radix36(_) => 36,
        }
    }

    pub fn as_decimal(&self) -> RadixResult<usize> {
        Self::radix_x_to_dec(self.as_str(), self.radix())
    }

    pub fn digits<'c>(&'c self) -> impl Iterator<Item=char> + 'c {
        self.as_str().chars()
    }

    fn dec_to_radix_x(number: usize, radix: usize) -> RadixResult<String> {
        Self::validate_radix(radix)?;
        if number == 0 { return Ok(String::from("0")) }

        let mut number: usize = number;
        let mut stack: Vec<char> = vec![];
        let get_offset = |digit: usize| -> RadixResult<u8> {
            match digit {
                0 ... 9 => Ok('0' as u8), //  1u8 => '1',   2u8 =>  '2',  etc
                10 ... 36 => Ok(55),      // 10u8 => 'A',  11u8 =>  'B',  etc
                d => Err(RadixErr::IllegalDigit(d)),
            }
        };

        debug!("\n");
        debug!("[dec_to_radix_x] radix:   {:?}", radix);
        debug!("[dec_to_radix_x] number: {:?}", number);
        debug!("[dec_to_radix_x] stack: {:?}", stack);

        debug!("[dec_to_radix_x] loop:");
        while number > 0 {
            let digit: usize = number / radix;
            debug!("[dec_to_radix_x] digit: {}", digit);
            let remainder: usize = modulus(number, radix);
            debug!("[dec_to_radix_x] remainder: {}", remainder);
            number = number / radix;
            debug!("[dec_to_radix_x] number = {}", number);
            let offset: usize = get_offset(remainder)? as usize;
            debug!("[dec_to_radix_x] offset: {}", offset);
            let target_digit: char = (remainder + offset) as u8 as char;
            stack.push(target_digit);
            debug!("[dec_to_radix_x] pushed remainder to stack");
            debug!("[dec_to_radix_x]   stack:  {:?}", stack);
            debug!("[dec_to_radix_x]   number: {:?}", number);
            debug!("[dec_to_radix_x]   digit:  {:?}", digit);
            debug!("[dec_to_radix_x]   target digit:  {:?}", target_digit);
        }

        let mut return_val: String = String::new();
        while !stack.is_empty() {
            let digit = stack.pop().ok_or(RadixErr::FailedToPopFromStack)?;
            return_val.push(digit);
        }
        debug!("[dec_to_radix_x] return_val: {}", return_val);
        Ok(return_val)
    }

    fn radix_x_to_dec(base: &str, radix: usize) -> RadixResult<usize> {
        Self::validate_radix(radix)?;
        let base: String = Self::validate_base(base, radix)?;
        let mut return_val: usize = 0;

        #[inline(always)]
        fn digit_to_dec(digit: char) -> Result<usize, RadixErr> {
            match digit {
                '0'...'9' => Ok(digit as usize - '0' as u8 as usize),
                'A'...'Z' => Ok(digit as usize - 55),
                c => Err(RadixErr::IllegalChar(c)),
            }
        }

        debug!("\n");
        debug!("[radix_x_to_dec] input radix: {}", radix);
        debug!("[radix_x_to_dec] input base: {}", base);
        debug!("[radix_x_to_dec] return val: {:?}", return_val);
        debug!("[radix_x_to_dec] for loop:");
        for (idx, token) in base.chars().rev().enumerate() {
            let digit: char = token
                .to_uppercase()
                .nth(0)
                .ok_or(RadixErr::FailedToUppercase)?;
            let dec_value: usize = digit_to_dec(digit)? * radix.pow(idx as u32);
            return_val += dec_value;
            debug!("[radix_x_to_dec]   idx: {:?}", idx);
            debug!("[radix_x_to_dec]   digit: {:?}  ({}u8)", digit, digit as u8);
            debug!("[radix_x_to_dec]   decimal value: {}", dec_value);
            debug!("[radix_x_to_dec]   return val: {:?}", return_val);
        }

        Ok(return_val)
    }
}

impl From<usize> for RadixNum {
    fn from(decimal: usize) -> RadixNum { RadixNum::Radix10(decimal.to_string()) }
}

impl From<u8> for RadixNum {
    fn from(decimal: u8) -> RadixNum { RadixNum::Radix10(decimal.to_string()) }
}

impl From<u16> for RadixNum {
    fn from(decimal: u16) -> RadixNum { RadixNum::Radix10(decimal.to_string()) }
}

impl From<u32> for RadixNum {
    fn from(decimal: u32) -> RadixNum { RadixNum::Radix10(decimal.to_string()) }
}

impl From<u64> for RadixNum {
    fn from(decimal: u64) -> RadixNum { RadixNum::Radix10(decimal.to_string()) }
}

impl From<u128> for RadixNum {
    fn from(decimal: u128) -> RadixNum { RadixNum::Radix10(decimal.to_string()) }
}



// Helper functions

#[inline(always)]
fn modulus(a: usize, b: usize) -> usize {
    ((a % b) + b) % b
}



#[cfg(test)]
mod tests {
    use *;

    #[test]
    fn from_str() {
        assert_eq!(
            RadixNum::from(3735928559 as u128).with_radix(16).expect("left"),
            RadixNum::from_str("DEADBEEF", 16).expect("right")
        );
    }

    #[test]
    fn digit_iterator() {
        let num = RadixNum::from(462058535375 as u128)
            .with_radix(32)
            .expect("DEADBEEF");
        let digits: Vec<char> = num.digits().collect();
        assert_eq!(vec!['D', 'E', 'A', 'D', 'B', 'E', 'E', 'F'], digits);
    }

    #[test]
    fn dec_to_radix_bad() {
        assert!(RadixNum::from(10 as u8).with_radix(0).is_err());
        assert!(RadixNum::from(10 as u8).with_radix(1).is_err());
        assert!(RadixNum::from(10 as u8).with_radix(37).is_err());
    }

    #[test]
    fn dec_to_radix2() {
        let to_radix2 = |n: RadixNum| n.with_radix(2).expect("radix 2");
        let num0 = to_radix2(RadixNum::from(        0 as u8   ));
        let num1 = to_radix2(RadixNum::from(        1 as u16  ));
        let num2 = to_radix2(RadixNum::from(       10 as u32  ));
        let num3 = to_radix2(RadixNum::from(       42 as u64  ));
        let num4 = to_radix2(RadixNum::from(      255 as usize));
        let num5 = to_radix2(RadixNum::from(1_000_000 as usize));
        assert_eq!(                   "0", num0.as_str());
        assert_eq!(                   "1", num1.as_str());
        assert_eq!(                "1010", num2.as_str());
        assert_eq!(              "101010", num3.as_str());
        assert_eq!(            "11111111", num4.as_str());
        assert_eq!("11110100001001000000", num5.as_str());
    }

    #[test]
    fn dec_to_radix3() {
        let to_radix3 = |n: RadixNum| n.with_radix(3).expect("radix 3");
        let num0 = to_radix3(RadixNum::from(        0 as u8   ));
        let num1 = to_radix3(RadixNum::from(        1 as u16  ));
        let num2 = to_radix3(RadixNum::from(       10 as u32  ));
        let num3 = to_radix3(RadixNum::from(       42 as u64  ));
        let num4 = to_radix3(RadixNum::from(      255 as usize));
        let num5 = to_radix3(RadixNum::from(1_000_000 as usize));
        assert_eq!(            "0", num0.as_str());
        assert_eq!(            "1", num1.as_str());
        assert_eq!(          "101", num2.as_str());
        assert_eq!(         "1120", num3.as_str());
        assert_eq!(       "100110", num4.as_str());
        assert_eq!("1212210202001", num5.as_str());
    }

    #[test]
    fn dec_to_radix4() {
        let to_radix4 = |n: RadixNum| n.with_radix(4).expect("radix 4");
        let num0 = to_radix4(RadixNum::from(        0 as u8   ));
        let num1 = to_radix4(RadixNum::from(        1 as u16  ));
        let num2 = to_radix4(RadixNum::from(       10 as u32  ));
        let num3 = to_radix4(RadixNum::from(       42 as u64  ));
        let num4 = to_radix4(RadixNum::from(      255 as usize));
        let num5 = to_radix4(RadixNum::from(1_000_000 as usize));
        assert_eq!(         "0", num0.as_str());
        assert_eq!(         "1", num1.as_str());
        assert_eq!(        "22", num2.as_str());
        assert_eq!(       "222", num3.as_str());
        assert_eq!(      "3333", num4.as_str());
        assert_eq!("3310021000", num5.as_str());
    }

    #[test]
    fn dec_to_radix5() {
        let to_radix5 = |n: RadixNum| n.with_radix(5).expect("radix 5");
        let num0 = to_radix5(RadixNum::from(        0 as u8   ));
        let num1 = to_radix5(RadixNum::from(        1 as u16  ));
        let num2 = to_radix5(RadixNum::from(       10 as u32  ));
        let num3 = to_radix5(RadixNum::from(       42 as u64  ));
        let num4 = to_radix5(RadixNum::from(      255 as usize));
        let num5 = to_radix5(RadixNum::from(1_000_000 as usize));
        assert_eq!(        "0", num0.as_str());
        assert_eq!(        "1", num1.as_str());
        assert_eq!(       "20", num2.as_str());
        assert_eq!(      "132", num3.as_str());
        assert_eq!(     "2010", num4.as_str());
        assert_eq!("224000000", num5.as_str());
    }

    #[test]
    fn dec_to_radix6() {
        let to_radix6 = |n: RadixNum| n.with_radix(6).expect("radix 6");
        let num0 = to_radix6(RadixNum::from(        0 as u8   ));
        let num1 = to_radix6(RadixNum::from(        1 as u16  ));
        let num2 = to_radix6(RadixNum::from(       10 as u32  ));
        let num3 = to_radix6(RadixNum::from(       42 as u64  ));
        let num4 = to_radix6(RadixNum::from(      255 as usize));
        let num5 = to_radix6(RadixNum::from(1_000_000 as usize));
        assert_eq!(       "0", num0.as_str());
        assert_eq!(       "1", num1.as_str());
        assert_eq!(      "14", num2.as_str());
        assert_eq!(     "110", num3.as_str());
        assert_eq!(    "1103", num4.as_str());
        assert_eq!("33233344", num5.as_str());
    }

    #[test]
    fn dec_to_radix7() {
        let to_radix7 = |n: RadixNum| n.with_radix(7).expect("radix 7");
        let num0 = to_radix7(RadixNum::from(        0 as u8   ));
        let num1 = to_radix7(RadixNum::from(        1 as u16  ));
        let num2 = to_radix7(RadixNum::from(       10 as u32  ));
        let num3 = to_radix7(RadixNum::from(       42 as u64  ));
        let num4 = to_radix7(RadixNum::from(      255 as usize));
        let num5 = to_radix7(RadixNum::from(1_000_000 as usize));
        assert_eq!(       "0", num0.as_str());
        assert_eq!(       "1", num1.as_str());
        assert_eq!(      "13", num2.as_str());
        assert_eq!(      "60", num3.as_str());
        assert_eq!(     "513", num4.as_str());
        assert_eq!("11333311", num5.as_str());
    }

    #[test]
    fn dec_to_radix8() {
        let to_radix8 = |n: RadixNum| n.with_radix(8).expect("radix 8");
        let num0 = to_radix8(RadixNum::from(        0 as u8   ));
        let num1 = to_radix8(RadixNum::from(        1 as u16  ));
        let num2 = to_radix8(RadixNum::from(       10 as u32  ));
        let num3 = to_radix8(RadixNum::from(       42 as u64  ));
        let num4 = to_radix8(RadixNum::from(      255 as usize));
        let num5 = to_radix8(RadixNum::from(1_000_000 as usize));
        assert_eq!(      "0", num0.as_str());
        assert_eq!(      "1", num1.as_str());
        assert_eq!(     "12", num2.as_str());
        assert_eq!(     "52", num3.as_str());
        assert_eq!(    "377", num4.as_str());
        assert_eq!("3641100", num5.as_str());
    }

    #[test]
    fn dec_to_radix9() {
        let to_radix9 = |n: RadixNum| n.with_radix(9).expect("radix 9");
        let num0 = to_radix9(RadixNum::from(        0 as u8   ));
        let num1 = to_radix9(RadixNum::from(        1 as u16  ));
        let num2 = to_radix9(RadixNum::from(       10 as u32  ));
        let num3 = to_radix9(RadixNum::from(       42 as u64  ));
        let num4 = to_radix9(RadixNum::from(      255 as usize));
        let num5 = to_radix9(RadixNum::from(1_000_000 as usize));
        assert_eq!(      "0", num0.as_str());
        assert_eq!(      "1", num1.as_str());
        assert_eq!(     "11", num2.as_str());
        assert_eq!(     "46", num3.as_str());
        assert_eq!(    "313", num4.as_str());
        assert_eq!("1783661", num5.as_str());
    }

    #[test]
    fn dec_to_radix10() {
        let to_radix10 = |n: RadixNum| n.with_radix(10).expect("radix 10");
        let num0 = to_radix10(RadixNum::from(        0 as u8   ));
        let num1 = to_radix10(RadixNum::from(        1 as u16  ));
        let num2 = to_radix10(RadixNum::from(       10 as u32  ));
        let num3 = to_radix10(RadixNum::from(       42 as u64  ));
        let num4 = to_radix10(RadixNum::from(      255 as usize));
        let num5 = to_radix10(RadixNum::from(1_000_000 as usize));
        assert_eq!(      "0", num0.as_str());
        assert_eq!(      "1", num1.as_str());
        assert_eq!(     "10", num2.as_str());
        assert_eq!(     "42", num3.as_str());
        assert_eq!(    "255", num4.as_str());
        assert_eq!("1000000", num5.as_str());
    }

    #[test]
    fn dec_to_radix11() {
        let to_radix11 = |n: RadixNum| n.with_radix(11).expect("radix 11");
        let num0 = to_radix11(RadixNum::from(        0 as u8   ));
        let num1 = to_radix11(RadixNum::from(        1 as u16  ));
        let num2 = to_radix11(RadixNum::from(       10 as u32  ));
        let num3 = to_radix11(RadixNum::from(       42 as u64  ));
        let num4 = to_radix11(RadixNum::from(      255 as usize));
        let num5 = to_radix11(RadixNum::from(1_000_000 as usize));
        assert_eq!(     "0", num0.as_str());
        assert_eq!(     "1", num1.as_str());
        assert_eq!(     "A", num2.as_str());
        assert_eq!(    "39", num3.as_str());
        assert_eq!(   "212", num4.as_str());
        assert_eq!("623351", num5.as_str());
    }

    #[test]
    fn dec_to_radix12() {
        let to_radix12 = |n: RadixNum| n.with_radix(12).expect("radix 12");
        let num0 = to_radix12(RadixNum::from(        0 as u8   ));
        let num1 = to_radix12(RadixNum::from(        1 as u16  ));
        let num2 = to_radix12(RadixNum::from(       10 as u32  ));
        let num3 = to_radix12(RadixNum::from(       42 as u64  ));
        let num4 = to_radix12(RadixNum::from(      255 as usize));
        let num5 = to_radix12(RadixNum::from(1_000_000 as usize));
        assert_eq!(     "0", num0.as_str());
        assert_eq!(     "1", num1.as_str());
        assert_eq!(     "A", num2.as_str());
        assert_eq!(    "36", num3.as_str());
        assert_eq!(   "193", num4.as_str());
        assert_eq!("402854", num5.as_str());
    }

    #[test]
    fn dec_to_radix13() {
        let to_radix13 = |n: RadixNum| n.with_radix(13).expect("radix 13");
        let num0 = to_radix13(RadixNum::from(        0 as u8   ));
        let num1 = to_radix13(RadixNum::from(        1 as u16  ));
        let num2 = to_radix13(RadixNum::from(       10 as u32  ));
        let num3 = to_radix13(RadixNum::from(       42 as u64  ));
        let num4 = to_radix13(RadixNum::from(      255 as usize));
        let num5 = to_radix13(RadixNum::from(1_000_000 as usize));
        assert_eq!(     "0", num0.as_str());
        assert_eq!(     "1", num1.as_str());
        assert_eq!(     "A", num2.as_str());
        assert_eq!(    "33", num3.as_str());
        assert_eq!(   "168", num4.as_str());
        assert_eq!("290221", num5.as_str());
    }

    #[test]
    fn dec_to_radix14() {
        let to_radix14 = |n: RadixNum| n.with_radix(14).expect("radix 14");
        let num0 = to_radix14(RadixNum::from(        0 as u8   ));
        let num1 = to_radix14(RadixNum::from(        1 as u16  ));
        let num2 = to_radix14(RadixNum::from(       10 as u32  ));
        let num3 = to_radix14(RadixNum::from(       42 as u64  ));
        let num4 = to_radix14(RadixNum::from(      255 as usize));
        let num5 = to_radix14(RadixNum::from(1_000_000 as usize));
        assert_eq!(     "0", num0.as_str());
        assert_eq!(     "1", num1.as_str());
        assert_eq!(     "A", num2.as_str());
        assert_eq!(    "30", num3.as_str());
        assert_eq!(   "143", num4.as_str());
        assert_eq!("1C0608", num5.as_str());
    }

    #[test]
    fn dec_to_radix15() {
        let to_radix15 = |n: RadixNum| n.with_radix(15).expect("radix 15");
        let num0 = to_radix15(RadixNum::from(        0 as u8   ));
        let num1 = to_radix15(RadixNum::from(        1 as u16  ));
        let num2 = to_radix15(RadixNum::from(       10 as u32  ));
        let num3 = to_radix15(RadixNum::from(       42 as u64  ));
        let num4 = to_radix15(RadixNum::from(      255 as usize));
        let num5 = to_radix15(RadixNum::from(1_000_000 as usize));
        assert_eq!(     "0", num0.as_str());
        assert_eq!(     "1", num1.as_str());
        assert_eq!(     "A", num2.as_str());
        assert_eq!(    "2C", num3.as_str());
        assert_eq!(   "120", num4.as_str());
        assert_eq!("14B46A", num5.as_str());
    }

    #[test]
    fn dec_to_radix16() {
        let to_radix16 = |n: RadixNum| n.with_radix(16).expect("radix 16");
        let num0 = to_radix16(RadixNum::from(        0 as u8   ));
        let num1 = to_radix16(RadixNum::from(        1 as u16  ));
        let num2 = to_radix16(RadixNum::from(       10 as u32  ));
        let num3 = to_radix16(RadixNum::from(       42 as u64  ));
        let num4 = to_radix16(RadixNum::from(      255 as usize));
        let num5 = to_radix16(RadixNum::from(1_000_000 as usize));
        assert_eq!(    "0", num0.as_str());
        assert_eq!(    "1", num1.as_str());
        assert_eq!(    "A", num2.as_str());
        assert_eq!(   "2A", num3.as_str());
        assert_eq!(   "FF", num4.as_str());
        assert_eq!("F4240", num5.as_str());
    }

    #[test]
    fn dec_to_radix17() {
        let to_radix17 = |n: RadixNum| n.with_radix(17).expect("radix 17");
        let num0 = to_radix17(RadixNum::from(        0 as u8   ));
        let num1 = to_radix17(RadixNum::from(        1 as u16  ));
        let num2 = to_radix17(RadixNum::from(       10 as u32  ));
        let num3 = to_radix17(RadixNum::from(       42 as u64  ));
        let num4 = to_radix17(RadixNum::from(      255 as usize));
        let num5 = to_radix17(RadixNum::from(1_000_000 as usize));
        assert_eq!(    "0", num0.as_str());
        assert_eq!(    "1", num1.as_str());
        assert_eq!(    "A", num2.as_str());
        assert_eq!(   "28", num3.as_str());
        assert_eq!(   "F0", num4.as_str());
        assert_eq!("BG939", num5.as_str());
    }

    #[test]
    fn dec_to_radix18() {
        let to_radix18 = |n: RadixNum| n.with_radix(18).expect("radix 18");
        let num0 = to_radix18(RadixNum::from(        0 as u8   ));
        let num1 = to_radix18(RadixNum::from(        1 as u16  ));
        let num2 = to_radix18(RadixNum::from(       10 as u32  ));
        let num3 = to_radix18(RadixNum::from(       42 as u64  ));
        let num4 = to_radix18(RadixNum::from(      255 as usize));
        let num5 = to_radix18(RadixNum::from(1_000_000 as usize));
        assert_eq!(    "0", num0.as_str());
        assert_eq!(    "1", num1.as_str());
        assert_eq!(    "A", num2.as_str());
        assert_eq!(   "26", num3.as_str());
        assert_eq!(   "E3", num4.as_str());
        assert_eq!("9987A", num5.as_str());
    }

    #[test]
    fn dec_to_radix19() {
        let to_radix19 = |n: RadixNum| n.with_radix(19).expect("radix 19");
        let num0 = to_radix19(RadixNum::from(        0 as u8   ));
        let num1 = to_radix19(RadixNum::from(        1 as u16  ));
        let num2 = to_radix19(RadixNum::from(       10 as u32  ));
        let num3 = to_radix19(RadixNum::from(       42 as u64  ));
        let num4 = to_radix19(RadixNum::from(      255 as usize));
        let num5 = to_radix19(RadixNum::from(1_000_000 as usize));
        assert_eq!(    "0", num0.as_str());
        assert_eq!(    "1", num1.as_str());
        assert_eq!(    "A", num2.as_str());
        assert_eq!(   "24", num3.as_str());
        assert_eq!(   "D8", num4.as_str());
        assert_eq!("7CF1B", num5.as_str());
    }

    #[test]
    fn dec_to_radix20() {
        let to_radix20 = |n: RadixNum| n.with_radix(20).expect("radix 20");
        let num0 = to_radix20(RadixNum::from(        0 as u8   ));
        let num1 = to_radix20(RadixNum::from(        1 as u16  ));
        let num2 = to_radix20(RadixNum::from(       10 as u32  ));
        let num3 = to_radix20(RadixNum::from(       42 as u64  ));
        let num4 = to_radix20(RadixNum::from(      255 as usize));
        let num5 = to_radix20(RadixNum::from(1_000_000 as usize));
        assert_eq!(    "0", num0.as_str());
        assert_eq!(    "1", num1.as_str());
        assert_eq!(    "A", num2.as_str());
        assert_eq!(   "22", num3.as_str());
        assert_eq!(   "CF", num4.as_str());
        assert_eq!("65000", num5.as_str());
    }

    #[test]
    fn dec_to_radix21() {
        let to_radix21 = |n: RadixNum| n.with_radix(21).expect("radix 21");
        let num0 = to_radix21(RadixNum::from(        0 as u8   ));
        let num1 = to_radix21(RadixNum::from(        1 as u16  ));
        let num2 = to_radix21(RadixNum::from(       10 as u32  ));
        let num3 = to_radix21(RadixNum::from(       42 as u64  ));
        let num4 = to_radix21(RadixNum::from(      255 as usize));
        let num5 = to_radix21(RadixNum::from(1_000_000 as usize));
        assert_eq!(    "0", num0.as_str());
        assert_eq!(    "1", num1.as_str());
        assert_eq!(    "A", num2.as_str());
        assert_eq!(   "20", num3.as_str());
        assert_eq!(   "C3", num4.as_str());
        assert_eq!("52KC1", num5.as_str());
    }

    #[test]
    fn dec_to_radix22() {
        let to_radix22 = |n: RadixNum| n.with_radix(22).expect("radix 22");
        let num0 = to_radix22(RadixNum::from(        0 as u8   ));
        let num1 = to_radix22(RadixNum::from(        1 as u16  ));
        let num2 = to_radix22(RadixNum::from(       10 as u32  ));
        let num3 = to_radix22(RadixNum::from(       42 as u64  ));
        let num4 = to_radix22(RadixNum::from(      255 as usize));
        let num5 = to_radix22(RadixNum::from(1_000_000 as usize));
        assert_eq!(    "0", num0.as_str());
        assert_eq!(    "1", num1.as_str());
        assert_eq!(    "A", num2.as_str());
        assert_eq!(   "1K", num3.as_str());
        assert_eq!(   "BD", num4.as_str());
        assert_eq!("45K2C", num5.as_str());
    }

    #[test]
    fn dec_to_radix23() {
        let to_radix23 = |n: RadixNum| n.with_radix(23).expect("radix 23");
        let num0 = to_radix23(RadixNum::from(        0 as u8   ));
        let num1 = to_radix23(RadixNum::from(        1 as u16  ));
        let num2 = to_radix23(RadixNum::from(       10 as u32  ));
        let num3 = to_radix23(RadixNum::from(       42 as u64  ));
        let num4 = to_radix23(RadixNum::from(      255 as usize));
        let num5 = to_radix23(RadixNum::from(1_000_000 as usize));
        assert_eq!(    "0", num0.as_str());
        assert_eq!(    "1", num1.as_str());
        assert_eq!(    "A", num2.as_str());
        assert_eq!(   "1J", num3.as_str());
        assert_eq!(   "B2", num4.as_str());
        assert_eq!("3D486", num5.as_str());
    }

    #[test]
    fn dec_to_radix24() {
        let to_radix24 = |n: RadixNum| n.with_radix(24).expect("radix 24");
        let num0 = to_radix24(RadixNum::from(        0 as u8   ));
        let num1 = to_radix24(RadixNum::from(        1 as u16  ));
        let num2 = to_radix24(RadixNum::from(       10 as u32  ));
        let num3 = to_radix24(RadixNum::from(       42 as u64  ));
        let num4 = to_radix24(RadixNum::from(      255 as usize));
        let num5 = to_radix24(RadixNum::from(1_000_000 as usize));
        assert_eq!(    "0", num0.as_str());
        assert_eq!(    "1", num1.as_str());
        assert_eq!(    "A", num2.as_str());
        assert_eq!(   "1I", num3.as_str());
        assert_eq!(   "AF", num4.as_str());
        assert_eq!("3082G", num5.as_str());
    }

    #[test]
    fn dec_to_radix25() {
        let to_radix25 = |n: RadixNum| n.with_radix(25).expect("radix 25");
        let num0 = to_radix25(RadixNum::from(        0 as u8   ));
        let num1 = to_radix25(RadixNum::from(        1 as u16  ));
        let num2 = to_radix25(RadixNum::from(       10 as u32  ));
        let num3 = to_radix25(RadixNum::from(       42 as u64  ));
        let num4 = to_radix25(RadixNum::from(      255 as usize));
        let num5 = to_radix25(RadixNum::from(1_000_000 as usize));
        assert_eq!(    "0", num0.as_str());
        assert_eq!(    "1", num1.as_str());
        assert_eq!(    "A", num2.as_str());
        assert_eq!(   "1H", num3.as_str());
        assert_eq!(   "A5", num4.as_str());
        assert_eq!("2E000", num5.as_str());
    }

    #[test]
    fn dec_to_radix26() {
        let to_radix26 = |n: RadixNum| n.with_radix(26).expect("radix 26");
        let num0 = to_radix26(RadixNum::from(        0 as u8   ));
        let num1 = to_radix26(RadixNum::from(        1 as u16  ));
        let num2 = to_radix26(RadixNum::from(       10 as u32  ));
        let num3 = to_radix26(RadixNum::from(       42 as u64  ));
        let num4 = to_radix26(RadixNum::from(      255 as usize));
        let num5 = to_radix26(RadixNum::from(1_000_000 as usize));
        assert_eq!(    "0", num0.as_str());
        assert_eq!(    "1", num1.as_str());
        assert_eq!(    "A", num2.as_str());
        assert_eq!(   "1G", num3.as_str());
        assert_eq!(   "9L", num4.as_str());
        assert_eq!("24N7E", num5.as_str());
    }

    #[test]
    fn dec_to_radix27() {
        let to_radix27 = |n: RadixNum| n.with_radix(27).expect("radix 27");
        let num0 = to_radix27(RadixNum::from(        0 as u8   ));
        let num1 = to_radix27(RadixNum::from(        1 as u16  ));
        let num2 = to_radix27(RadixNum::from(       10 as u32  ));
        let num3 = to_radix27(RadixNum::from(       42 as u64  ));
        let num4 = to_radix27(RadixNum::from(      255 as usize));
        let num5 = to_radix27(RadixNum::from(1_000_000 as usize));
        assert_eq!(    "0", num0.as_str());
        assert_eq!(    "1", num1.as_str());
        assert_eq!(    "A", num2.as_str());
        assert_eq!(   "1F", num3.as_str());
        assert_eq!(   "9C", num4.as_str());
        assert_eq!("1NLK1", num5.as_str());
    }

    #[test]
    fn dec_to_radix28() {
        let to_radix28 = |n: RadixNum| n.with_radix(28).expect("radix 28");
        let num0 = to_radix28(RadixNum::from(        0 as u8   ));
        let num1 = to_radix28(RadixNum::from(        1 as u16  ));
        let num2 = to_radix28(RadixNum::from(       10 as u32  ));
        let num3 = to_radix28(RadixNum::from(       42 as u64  ));
        let num4 = to_radix28(RadixNum::from(      255 as usize));
        let num5 = to_radix28(RadixNum::from(1_000_000 as usize));
        assert_eq!(    "0", num0.as_str());
        assert_eq!(    "1", num1.as_str());
        assert_eq!(    "A", num2.as_str());
        assert_eq!(   "1E", num3.as_str());
        assert_eq!(   "93", num4.as_str());
        assert_eq!("1HFE8", num5.as_str());
    }

    #[test]
    fn dec_to_radix29() {
        let to_radix29 = |n: RadixNum| n.with_radix(29).expect("radix 29");
        let num0 = to_radix29(RadixNum::from(        0 as u8   ));
        let num1 = to_radix29(RadixNum::from(        1 as u16  ));
        let num2 = to_radix29(RadixNum::from(       10 as u32  ));
        let num3 = to_radix29(RadixNum::from(       42 as u64  ));
        let num4 = to_radix29(RadixNum::from(      255 as usize));
        let num5 = to_radix29(RadixNum::from(1_000_000 as usize));
        assert_eq!(    "0", num0.as_str());
        assert_eq!(    "1", num1.as_str());
        assert_eq!(    "A", num2.as_str());
        assert_eq!(   "1D", num3.as_str());
        assert_eq!(   "8N", num4.as_str());
        assert_eq!("1C01M", num5.as_str());
    }

    #[test]
    fn dec_to_radix30() {
        let to_radix30 = |n: RadixNum| n.with_radix(30).expect("radix 30");
        let num0 = to_radix30(RadixNum::from(        0 as u8   ));
        let num1 = to_radix30(RadixNum::from(        1 as u16  ));
        let num2 = to_radix30(RadixNum::from(       10 as u32  ));
        let num3 = to_radix30(RadixNum::from(       42 as u64  ));
        let num4 = to_radix30(RadixNum::from(      255 as usize));
        let num5 = to_radix30(RadixNum::from(1_000_000 as usize));
        assert_eq!(    "0", num0.as_str());
        assert_eq!(    "1", num1.as_str());
        assert_eq!(    "A", num2.as_str());
        assert_eq!(   "1C", num3.as_str());
        assert_eq!(   "8F", num4.as_str());
        assert_eq!("1713A", num5.as_str());
    }

    #[test]
    fn dec_to_radix31() {
        let to_radix31 = |n: RadixNum| n.with_radix(31).expect("radix 31");
        let num0 = to_radix31(RadixNum::from(        0 as u8   ));
        let num1 = to_radix31(RadixNum::from(        1 as u16  ));
        let num2 = to_radix31(RadixNum::from(       10 as u32  ));
        let num3 = to_radix31(RadixNum::from(       42 as u64  ));
        let num4 = to_radix31(RadixNum::from(      255 as usize));
        let num5 = to_radix31(RadixNum::from(1_000_000 as usize));
        assert_eq!(    "0", num0.as_str());
        assert_eq!(    "1", num1.as_str());
        assert_eq!(    "A", num2.as_str());
        assert_eq!(   "1B", num3.as_str());
        assert_eq!(   "87", num4.as_str());
        assert_eq!("12HI2", num5.as_str());
    }

    #[test]
    fn dec_to_radix32() {
        let to_radix32 = |n: RadixNum| n.with_radix(32).expect("radix 32");
        let num0 = to_radix32(RadixNum::from(        0 as u8   ));
        let num1 = to_radix32(RadixNum::from(        1 as u16  ));
        let num2 = to_radix32(RadixNum::from(       10 as u32  ));
        let num3 = to_radix32(RadixNum::from(       42 as u64  ));
        let num4 = to_radix32(RadixNum::from(      255 as usize));
        let num5 = to_radix32(RadixNum::from(1_000_000 as usize));
        assert_eq!(   "0", num0.as_str());
        assert_eq!(   "1", num1.as_str());
        assert_eq!(   "A", num2.as_str());
        assert_eq!(  "1A", num3.as_str());
        assert_eq!(  "7V", num4.as_str());
        assert_eq!("UGI0", num5.as_str());
    }

    #[test]
    fn dec_to_radix33() {
        let to_radix33 = |n: RadixNum| n.with_radix(33).expect("radix 33");
        let num0 = to_radix33(RadixNum::from(        0 as u8   ));
        let num1 = to_radix33(RadixNum::from(        1 as u16  ));
        let num2 = to_radix33(RadixNum::from(       10 as u32  ));
        let num3 = to_radix33(RadixNum::from(       42 as u64  ));
        let num4 = to_radix33(RadixNum::from(      255 as usize));
        let num5 = to_radix33(RadixNum::from(1_000_000 as usize));
        assert_eq!(   "0", num0.as_str());
        assert_eq!(   "1", num1.as_str());
        assert_eq!(   "A", num2.as_str());
        assert_eq!(  "19", num3.as_str());
        assert_eq!(  "7O", num4.as_str());
        assert_eq!("RR91", num5.as_str());
    }

    #[test]
    fn dec_to_radix34() {
        let to_radix34 = |n: RadixNum| n.with_radix(34).expect("radix 34");
        let num0 = to_radix34(RadixNum::from(        0 as u8   ));
        let num1 = to_radix34(RadixNum::from(        1 as u16  ));
        let num2 = to_radix34(RadixNum::from(       10 as u32  ));
        let num3 = to_radix34(RadixNum::from(       42 as u64  ));
        let num4 = to_radix34(RadixNum::from(      255 as usize));
        let num5 = to_radix34(RadixNum::from(1_000_000 as usize));
        assert_eq!(   "0", num0.as_str());
        assert_eq!(   "1", num1.as_str());
        assert_eq!(   "A", num2.as_str());
        assert_eq!(  "18", num3.as_str());
        assert_eq!(  "7H", num4.as_str());
        assert_eq!("PF1Q", num5.as_str());
    }

    #[test]
    fn dec_to_radix35() {
        let to_radix35 = |n: RadixNum| n.with_radix(35).expect("radix 35");
        let num0 = to_radix35(RadixNum::from(        0 as u8   ));
        let num1 = to_radix35(RadixNum::from(        1 as u16  ));
        let num2 = to_radix35(RadixNum::from(       10 as u32  ));
        let num3 = to_radix35(RadixNum::from(       42 as u64  ));
        let num4 = to_radix35(RadixNum::from(      255 as usize));
        let num5 = to_radix35(RadixNum::from(1_000_000 as usize));
        assert_eq!(   "0", num0.as_str());
        assert_eq!(   "1", num1.as_str());
        assert_eq!(   "A", num2.as_str());
        assert_eq!(  "17", num3.as_str());
        assert_eq!(  "7A", num4.as_str());
        assert_eq!("NBBF", num5.as_str());
    }

    #[test]
    fn dec_to_radix36() {
        let to_radix36 = |n: RadixNum| n.with_radix(36).expect("radix 36");
        let num0 = to_radix36(RadixNum::from(        0 as u8   ));
        let num1 = to_radix36(RadixNum::from(        1 as u16  ));
        let num2 = to_radix36(RadixNum::from(       10 as u32  ));
        let num3 = to_radix36(RadixNum::from(       42 as u64  ));
        let num4 = to_radix36(RadixNum::from(      255 as usize));
        let num5 = to_radix36(RadixNum::from(1_000_000 as usize));
        assert_eq!(   "0", num0.as_str());
        assert_eq!(   "1", num1.as_str());
        assert_eq!(   "A", num2.as_str());
        assert_eq!(  "16", num3.as_str());
        assert_eq!(  "73", num4.as_str());
        assert_eq!("LFLS", num5.as_str());
    }



    #[test]
    fn radix2_to_dec() {
        let to_radix2 = |n: RadixNum| n.with_radix(2).expect("radix 2");
        let num0 = to_radix2(RadixNum::from(             0 as   u8));
        let num1 = to_radix2(RadixNum::from(             1 as  u16));
        let num2 = to_radix2(RadixNum::from(            10 as  u32));
        let num3 = to_radix2(RadixNum::from(         32767 as  u64));
        let num4 = to_radix2(RadixNum::from(   15123093122 as  u64));
        let num5 = to_radix2(RadixNum::from(  462058535375 as u128));
        let num6 = to_radix2(RadixNum::from(46597557513433 as u128));
        assert_eq!(             Ok(0), num0.as_decimal());
        assert_eq!(             Ok(1), num1.as_decimal());
        assert_eq!(            Ok(10), num2.as_decimal());
        assert_eq!(         Ok(32767), num3.as_decimal());
        assert_eq!(   Ok(15123093122), num4.as_decimal());
        assert_eq!(  Ok(462058535375), num5.as_decimal());
        assert_eq!(Ok(46597557513433), num6.as_decimal());
    }

    #[test]
    fn radix3_to_dec() {
        let to_radix3 = |n: RadixNum| n.with_radix(3).expect("radix 3");
        let num0 = to_radix3(RadixNum::from(             0 as   u8));
        let num1 = to_radix3(RadixNum::from(             1 as  u16));
        let num2 = to_radix3(RadixNum::from(            10 as  u32));
        let num3 = to_radix3(RadixNum::from(         32767 as  u64));
        let num4 = to_radix3(RadixNum::from(   15123093122 as  u64));
        let num5 = to_radix3(RadixNum::from(  462058535375 as u128));
        let num6 = to_radix3(RadixNum::from(46597557513433 as u128));
        assert_eq!(             Ok(0), num0.as_decimal());
        assert_eq!(             Ok(1), num1.as_decimal());
        assert_eq!(            Ok(10), num2.as_decimal());
        assert_eq!(         Ok(32767), num3.as_decimal());
        assert_eq!(   Ok(15123093122), num4.as_decimal());
        assert_eq!(  Ok(462058535375), num5.as_decimal());
        assert_eq!(Ok(46597557513433), num6.as_decimal());
    }

    #[test]
    fn radix4_to_dec() {
        let to_radix4 = |n: RadixNum| n.with_radix(4).expect("radix 4");
        let num0 = to_radix4(RadixNum::from(             0 as   u8));
        let num1 = to_radix4(RadixNum::from(             1 as  u16));
        let num2 = to_radix4(RadixNum::from(            10 as  u32));
        let num3 = to_radix4(RadixNum::from(         32767 as  u64));
        let num4 = to_radix4(RadixNum::from(   15123093122 as  u64));
        let num5 = to_radix4(RadixNum::from(  462058535375 as u128));
        let num6 = to_radix4(RadixNum::from(46597557513433 as u128));
        assert_eq!(             Ok(0), num0.as_decimal());
        assert_eq!(             Ok(1), num1.as_decimal());
        assert_eq!(            Ok(10), num2.as_decimal());
        assert_eq!(         Ok(32767), num3.as_decimal());
        assert_eq!(   Ok(15123093122), num4.as_decimal());
        assert_eq!(  Ok(462058535375), num5.as_decimal());
        assert_eq!(Ok(46597557513433), num6.as_decimal());
    }

    #[test]
    fn radix5_to_dec() {
        let to_radix5 = |n: RadixNum| n.with_radix(5).expect("radix 5");
        let num0 = to_radix5(RadixNum::from(             0 as   u8));
        let num1 = to_radix5(RadixNum::from(             1 as  u16));
        let num2 = to_radix5(RadixNum::from(            10 as  u32));
        let num3 = to_radix5(RadixNum::from(         32767 as  u64));
        let num4 = to_radix5(RadixNum::from(   15123093122 as  u64));
        let num5 = to_radix5(RadixNum::from(  462058535375 as u128));
        let num6 = to_radix5(RadixNum::from(46597557513433 as u128));
        assert_eq!(             Ok(0), num0.as_decimal());
        assert_eq!(             Ok(1), num1.as_decimal());
        assert_eq!(            Ok(10), num2.as_decimal());
        assert_eq!(         Ok(32767), num3.as_decimal());
        assert_eq!(   Ok(15123093122), num4.as_decimal());
        assert_eq!(  Ok(462058535375), num5.as_decimal());
        assert_eq!(Ok(46597557513433), num6.as_decimal());
    }

    #[test]
    fn radix6_to_dec() {
        let to_radix6 = |n: RadixNum| n.with_radix(6).expect("radix 6");
        let num0 = to_radix6(RadixNum::from(             0 as   u8));
        let num1 = to_radix6(RadixNum::from(             1 as  u16));
        let num2 = to_radix6(RadixNum::from(            10 as  u32));
        let num3 = to_radix6(RadixNum::from(         32767 as  u64));
        let num4 = to_radix6(RadixNum::from(   15123093122 as  u64));
        let num5 = to_radix6(RadixNum::from(  462058535375 as u128));
        let num6 = to_radix6(RadixNum::from(46597557513433 as u128));
        assert_eq!(             Ok(0), num0.as_decimal());
        assert_eq!(             Ok(1), num1.as_decimal());
        assert_eq!(            Ok(10), num2.as_decimal());
        assert_eq!(         Ok(32767), num3.as_decimal());
        assert_eq!(   Ok(15123093122), num4.as_decimal());
        assert_eq!(  Ok(462058535375), num5.as_decimal());
        assert_eq!(Ok(46597557513433), num6.as_decimal());
    }

    #[test]
    fn radix7_to_dec() {
        let to_radix7 = |n: RadixNum| n.with_radix(7).expect("radix 7");
        let num0 = to_radix7(RadixNum::from(             0 as   u8));
        let num1 = to_radix7(RadixNum::from(             1 as  u16));
        let num2 = to_radix7(RadixNum::from(            10 as  u32));
        let num3 = to_radix7(RadixNum::from(         32767 as  u64));
        let num4 = to_radix7(RadixNum::from(   15123093122 as  u64));
        let num5 = to_radix7(RadixNum::from(  462058535375 as u128));
        let num6 = to_radix7(RadixNum::from(46597557513433 as u128));
        assert_eq!(             Ok(0), num0.as_decimal());
        assert_eq!(             Ok(1), num1.as_decimal());
        assert_eq!(            Ok(10), num2.as_decimal());
        assert_eq!(         Ok(32767), num3.as_decimal());
        assert_eq!(   Ok(15123093122), num4.as_decimal());
        assert_eq!(  Ok(462058535375), num5.as_decimal());
        assert_eq!(Ok(46597557513433), num6.as_decimal());
    }

    #[test]
    fn radix8_to_dec() {
        let to_radix8 = |n: RadixNum| n.with_radix(8).expect("radix 8");
        let num0 = to_radix8(RadixNum::from(             0 as   u8));
        let num1 = to_radix8(RadixNum::from(             1 as  u16));
        let num2 = to_radix8(RadixNum::from(            10 as  u32));
        let num3 = to_radix8(RadixNum::from(         32767 as  u64));
        let num4 = to_radix8(RadixNum::from(   15123093122 as  u64));
        let num5 = to_radix8(RadixNum::from(  462058535375 as u128));
        let num6 = to_radix8(RadixNum::from(46597557513433 as u128));
        assert_eq!(             Ok(0), num0.as_decimal());
        assert_eq!(             Ok(1), num1.as_decimal());
        assert_eq!(            Ok(10), num2.as_decimal());
        assert_eq!(         Ok(32767), num3.as_decimal());
        assert_eq!(   Ok(15123093122), num4.as_decimal());
        assert_eq!(  Ok(462058535375), num5.as_decimal());
        assert_eq!(Ok(46597557513433), num6.as_decimal());
    }

    #[test]
    fn radix9_to_dec() {
        let to_radix9 = |n: RadixNum| n.with_radix(9).expect("radix 9");
        let num0 = to_radix9(RadixNum::from(             0 as   u8));
        let num1 = to_radix9(RadixNum::from(             1 as  u16));
        let num2 = to_radix9(RadixNum::from(            10 as  u32));
        let num3 = to_radix9(RadixNum::from(         32767 as  u64));
        let num4 = to_radix9(RadixNum::from(   15123093122 as  u64));
        let num5 = to_radix9(RadixNum::from(  462058535375 as u128));
        let num6 = to_radix9(RadixNum::from(46597557513433 as u128));
        assert_eq!(             Ok(0), num0.as_decimal());
        assert_eq!(             Ok(1), num1.as_decimal());
        assert_eq!(            Ok(10), num2.as_decimal());
        assert_eq!(         Ok(32767), num3.as_decimal());
        assert_eq!(   Ok(15123093122), num4.as_decimal());
        assert_eq!(  Ok(462058535375), num5.as_decimal());
        assert_eq!(Ok(46597557513433), num6.as_decimal());
    }

    #[test]
    fn radix10_to_dec() {
        let to_radix10 = |n: RadixNum| n.with_radix(10).expect("radix 10");
        let num0 = to_radix10(RadixNum::from(             0 as   u8));
        let num1 = to_radix10(RadixNum::from(             1 as  u16));
        let num2 = to_radix10(RadixNum::from(            10 as  u32));
        let num3 = to_radix10(RadixNum::from(         32767 as  u64));
        let num4 = to_radix10(RadixNum::from(   15123093122 as  u64));
        let num5 = to_radix10(RadixNum::from(  462058535375 as u128));
        let num6 = to_radix10(RadixNum::from(46597557513433 as u128));
        assert_eq!(             Ok(0), num0.as_decimal());
        assert_eq!(             Ok(1), num1.as_decimal());
        assert_eq!(            Ok(10), num2.as_decimal());
        assert_eq!(         Ok(32767), num3.as_decimal());
        assert_eq!(   Ok(15123093122), num4.as_decimal());
        assert_eq!(  Ok(462058535375), num5.as_decimal());
        assert_eq!(Ok(46597557513433), num6.as_decimal());
    }

    #[test]
    fn radix11_to_dec() {
        let to_radix11 = |n: RadixNum| n.with_radix(11).expect("radix 11");
        let num0 = to_radix11(RadixNum::from(             0 as   u8));
        let num1 = to_radix11(RadixNum::from(             1 as  u16));
        let num2 = to_radix11(RadixNum::from(            10 as  u32));
        let num3 = to_radix11(RadixNum::from(         32767 as  u64));
        let num4 = to_radix11(RadixNum::from(   15123093122 as  u64));
        let num5 = to_radix11(RadixNum::from(  462058535375 as u128));
        let num6 = to_radix11(RadixNum::from(46597557513433 as u128));
        assert_eq!(             Ok(0), num0.as_decimal());
        assert_eq!(             Ok(1), num1.as_decimal());
        assert_eq!(            Ok(10), num2.as_decimal());
        assert_eq!(         Ok(32767), num3.as_decimal());
        assert_eq!(   Ok(15123093122), num4.as_decimal());
        assert_eq!(  Ok(462058535375), num5.as_decimal());
        assert_eq!(Ok(46597557513433), num6.as_decimal());
    }

    #[test]
    fn radix12_to_dec() {
        let to_radix12 = |n: RadixNum| n.with_radix(12).expect("radix 12");
        let num0 = to_radix12(RadixNum::from(             0 as   u8));
        let num1 = to_radix12(RadixNum::from(             1 as  u16));
        let num2 = to_radix12(RadixNum::from(            10 as  u32));
        let num3 = to_radix12(RadixNum::from(         32767 as  u64));
        let num4 = to_radix12(RadixNum::from(   15123093122 as  u64));
        let num5 = to_radix12(RadixNum::from(  462058535375 as u128));
        let num6 = to_radix12(RadixNum::from(46597557513433 as u128));
        assert_eq!(             Ok(0), num0.as_decimal());
        assert_eq!(             Ok(1), num1.as_decimal());
        assert_eq!(            Ok(10), num2.as_decimal());
        assert_eq!(         Ok(32767), num3.as_decimal());
        assert_eq!(   Ok(15123093122), num4.as_decimal());
        assert_eq!(  Ok(462058535375), num5.as_decimal());
        assert_eq!(Ok(46597557513433), num6.as_decimal());
    }

    #[test]
    fn radix13_to_dec() {
        let to_radix13 = |n: RadixNum| n.with_radix(13).expect("radix 13");
        let num0 = to_radix13(RadixNum::from(             0 as   u8));
        let num1 = to_radix13(RadixNum::from(             1 as  u16));
        let num2 = to_radix13(RadixNum::from(            10 as  u32));
        let num3 = to_radix13(RadixNum::from(         32767 as  u64));
        let num4 = to_radix13(RadixNum::from(   15123093122 as  u64));
        let num5 = to_radix13(RadixNum::from(  462058535375 as u128));
        let num6 = to_radix13(RadixNum::from(46597557513433 as u128));
        assert_eq!(             Ok(0), num0.as_decimal());
        assert_eq!(             Ok(1), num1.as_decimal());
        assert_eq!(            Ok(10), num2.as_decimal());
        assert_eq!(         Ok(32767), num3.as_decimal());
        assert_eq!(   Ok(15123093122), num4.as_decimal());
        assert_eq!(  Ok(462058535375), num5.as_decimal());
        assert_eq!(Ok(46597557513433), num6.as_decimal());
    }

    #[test]
    fn radix14_to_dec() {
        let to_radix14 = |n: RadixNum| n.with_radix(14).expect("radix 14");
        let num0 = to_radix14(RadixNum::from(             0 as   u8));
        let num1 = to_radix14(RadixNum::from(             1 as  u16));
        let num2 = to_radix14(RadixNum::from(            10 as  u32));
        let num3 = to_radix14(RadixNum::from(         32767 as  u64));
        let num4 = to_radix14(RadixNum::from(   15123093122 as  u64));
        let num5 = to_radix14(RadixNum::from(  462058535375 as u128));
        let num6 = to_radix14(RadixNum::from(46597557513433 as u128));
        assert_eq!(             Ok(0), num0.as_decimal());
        assert_eq!(             Ok(1), num1.as_decimal());
        assert_eq!(            Ok(10), num2.as_decimal());
        assert_eq!(         Ok(32767), num3.as_decimal());
        assert_eq!(   Ok(15123093122), num4.as_decimal());
        assert_eq!(  Ok(462058535375), num5.as_decimal());
        assert_eq!(Ok(46597557513433), num6.as_decimal());
    }

    #[test]
    fn radix15_to_dec() {
        let to_radix15 = |n: RadixNum| n.with_radix(15).expect("radix 15");
        let num0 = to_radix15(RadixNum::from(             0 as   u8));
        let num1 = to_radix15(RadixNum::from(             1 as  u16));
        let num2 = to_radix15(RadixNum::from(            10 as  u32));
        let num3 = to_radix15(RadixNum::from(         32767 as  u64));
        let num4 = to_radix15(RadixNum::from(   15123093122 as  u64));
        let num5 = to_radix15(RadixNum::from(  462058535375 as u128));
        let num6 = to_radix15(RadixNum::from(46597557513433 as u128));
        assert_eq!(             Ok(0), num0.as_decimal());
        assert_eq!(             Ok(1), num1.as_decimal());
        assert_eq!(            Ok(10), num2.as_decimal());
        assert_eq!(         Ok(32767), num3.as_decimal());
        assert_eq!(   Ok(15123093122), num4.as_decimal());
        assert_eq!(  Ok(462058535375), num5.as_decimal());
        assert_eq!(Ok(46597557513433), num6.as_decimal());
    }

    #[test]
    fn radix16_to_dec() {
        let to_radix16 = |n: RadixNum| n.with_radix(16).expect("radix 16");
        let num0 = to_radix16(RadixNum::from(             0 as   u8));
        let num1 = to_radix16(RadixNum::from(             1 as  u16));
        let num2 = to_radix16(RadixNum::from(            10 as  u32));
        let num3 = to_radix16(RadixNum::from(         32767 as  u64));
        let num4 = to_radix16(RadixNum::from(   15123093122 as  u64));
        let num5 = to_radix16(RadixNum::from(  462058535375 as u128));
        let num6 = to_radix16(RadixNum::from(46597557513433 as u128));
        assert_eq!(             Ok(0), num0.as_decimal());
        assert_eq!(             Ok(1), num1.as_decimal());
        assert_eq!(            Ok(10), num2.as_decimal());
        assert_eq!(         Ok(32767), num3.as_decimal());
        assert_eq!(   Ok(15123093122), num4.as_decimal());
        assert_eq!(  Ok(462058535375), num5.as_decimal());
        assert_eq!(Ok(46597557513433), num6.as_decimal());
    }

    #[test]
    fn radix17_to_dec() {
        let to_radix17 = |n: RadixNum| n.with_radix(17).expect("radix 17");
        let num0 = to_radix17(RadixNum::from(             0 as   u8));
        let num1 = to_radix17(RadixNum::from(             1 as  u16));
        let num2 = to_radix17(RadixNum::from(            10 as  u32));
        let num3 = to_radix17(RadixNum::from(         32767 as  u64));
        let num4 = to_radix17(RadixNum::from(   15123093122 as  u64));
        let num5 = to_radix17(RadixNum::from(  462058535375 as u128));
        let num6 = to_radix17(RadixNum::from(46597557513433 as u128));
        assert_eq!(             Ok(0), num0.as_decimal());
        assert_eq!(             Ok(1), num1.as_decimal());
        assert_eq!(            Ok(10), num2.as_decimal());
        assert_eq!(         Ok(32767), num3.as_decimal());
        assert_eq!(   Ok(15123093122), num4.as_decimal());
        assert_eq!(  Ok(462058535375), num5.as_decimal());
        assert_eq!(Ok(46597557513433), num6.as_decimal());
    }

    #[test]
    fn radix18_to_dec() {
        let to_radix18 = |n: RadixNum| n.with_radix(18).expect("radix 18");
        let num0 = to_radix18(RadixNum::from(             0 as   u8));
        let num1 = to_radix18(RadixNum::from(             1 as  u16));
        let num2 = to_radix18(RadixNum::from(            10 as  u32));
        let num3 = to_radix18(RadixNum::from(         32767 as  u64));
        let num4 = to_radix18(RadixNum::from(   15123093122 as  u64));
        let num5 = to_radix18(RadixNum::from(  462058535375 as u128));
        let num6 = to_radix18(RadixNum::from(46597557513433 as u128));
        assert_eq!(             Ok(0), num0.as_decimal());
        assert_eq!(             Ok(1), num1.as_decimal());
        assert_eq!(            Ok(10), num2.as_decimal());
        assert_eq!(         Ok(32767), num3.as_decimal());
        assert_eq!(   Ok(15123093122), num4.as_decimal());
        assert_eq!(  Ok(462058535375), num5.as_decimal());
        assert_eq!(Ok(46597557513433), num6.as_decimal());
    }

    #[test]
    fn radix19_to_dec() {
        let to_radix19 = |n: RadixNum| n.with_radix(19).expect("radix 19");
        let num0 = to_radix19(RadixNum::from(             0 as   u8));
        let num1 = to_radix19(RadixNum::from(             1 as  u16));
        let num2 = to_radix19(RadixNum::from(            10 as  u32));
        let num3 = to_radix19(RadixNum::from(         32767 as  u64));
        let num4 = to_radix19(RadixNum::from(   15123093122 as  u64));
        let num5 = to_radix19(RadixNum::from(  462058535375 as u128));
        let num6 = to_radix19(RadixNum::from(46597557513433 as u128));
        assert_eq!(             Ok(0), num0.as_decimal());
        assert_eq!(             Ok(1), num1.as_decimal());
        assert_eq!(            Ok(10), num2.as_decimal());
        assert_eq!(         Ok(32767), num3.as_decimal());
        assert_eq!(   Ok(15123093122), num4.as_decimal());
        assert_eq!(  Ok(462058535375), num5.as_decimal());
        assert_eq!(Ok(46597557513433), num6.as_decimal());
    }

    #[test]
    fn radix20_to_dec() {
        let to_radix20 = |n: RadixNum| n.with_radix(20).expect("radix 20");
        let num0 = to_radix20(RadixNum::from(             0 as   u8));
        let num1 = to_radix20(RadixNum::from(             1 as  u16));
        let num2 = to_radix20(RadixNum::from(            10 as  u32));
        let num3 = to_radix20(RadixNum::from(         32767 as  u64));
        let num4 = to_radix20(RadixNum::from(   15123093122 as  u64));
        let num5 = to_radix20(RadixNum::from(  462058535375 as u128));
        let num6 = to_radix20(RadixNum::from(46597557513433 as u128));
        assert_eq!(             Ok(0), num0.as_decimal());
        assert_eq!(             Ok(1), num1.as_decimal());
        assert_eq!(            Ok(10), num2.as_decimal());
        assert_eq!(         Ok(32767), num3.as_decimal());
        assert_eq!(   Ok(15123093122), num4.as_decimal());
        assert_eq!(  Ok(462058535375), num5.as_decimal());
        assert_eq!(Ok(46597557513433), num6.as_decimal());
    }

    #[test]
    fn radix21_to_dec() {
        let to_radix21 = |n: RadixNum| n.with_radix(21).expect("radix 21");
        let num0 = to_radix21(RadixNum::from(             0 as   u8));
        let num1 = to_radix21(RadixNum::from(             1 as  u16));
        let num2 = to_radix21(RadixNum::from(            10 as  u32));
        let num3 = to_radix21(RadixNum::from(         32767 as  u64));
        let num4 = to_radix21(RadixNum::from(   15123093122 as  u64));
        let num5 = to_radix21(RadixNum::from(  462058535375 as u128));
        let num6 = to_radix21(RadixNum::from(46597557513433 as u128));
        assert_eq!(             Ok(0), num0.as_decimal());
        assert_eq!(             Ok(1), num1.as_decimal());
        assert_eq!(            Ok(10), num2.as_decimal());
        assert_eq!(         Ok(32767), num3.as_decimal());
        assert_eq!(   Ok(15123093122), num4.as_decimal());
        assert_eq!(  Ok(462058535375), num5.as_decimal());
        assert_eq!(Ok(46597557513433), num6.as_decimal());
    }

    #[test]
    fn radix22_to_dec() {
        let to_radix22 = |n: RadixNum| n.with_radix(22).expect("radix 22");
        let num0 = to_radix22(RadixNum::from(             0 as   u8));
        let num1 = to_radix22(RadixNum::from(             1 as  u16));
        let num2 = to_radix22(RadixNum::from(            10 as  u32));
        let num3 = to_radix22(RadixNum::from(         32767 as  u64));
        let num4 = to_radix22(RadixNum::from(   15123093122 as  u64));
        let num5 = to_radix22(RadixNum::from(  462058535375 as u128));
        let num6 = to_radix22(RadixNum::from(46597557513433 as u128));
        assert_eq!(             Ok(0), num0.as_decimal());
        assert_eq!(             Ok(1), num1.as_decimal());
        assert_eq!(            Ok(10), num2.as_decimal());
        assert_eq!(         Ok(32767), num3.as_decimal());
        assert_eq!(   Ok(15123093122), num4.as_decimal());
        assert_eq!(  Ok(462058535375), num5.as_decimal());
        assert_eq!(Ok(46597557513433), num6.as_decimal());
    }

    #[test]
    fn radix23_to_dec() {
        let to_radix23 = |n: RadixNum| n.with_radix(23).expect("radix 23");
        let num0 = to_radix23(RadixNum::from(             0 as   u8));
        let num1 = to_radix23(RadixNum::from(             1 as  u16));
        let num2 = to_radix23(RadixNum::from(            10 as  u32));
        let num3 = to_radix23(RadixNum::from(         32767 as  u64));
        let num4 = to_radix23(RadixNum::from(   15123093122 as  u64));
        let num5 = to_radix23(RadixNum::from(  462058535375 as u128));
        let num6 = to_radix23(RadixNum::from(46597557513433 as u128));
        assert_eq!(             Ok(0), num0.as_decimal());
        assert_eq!(             Ok(1), num1.as_decimal());
        assert_eq!(            Ok(10), num2.as_decimal());
        assert_eq!(         Ok(32767), num3.as_decimal());
        assert_eq!(   Ok(15123093122), num4.as_decimal());
        assert_eq!(  Ok(462058535375), num5.as_decimal());
        assert_eq!(Ok(46597557513433), num6.as_decimal());
    }

    #[test]
    fn radix24_to_dec() {
        let to_radix24 = |n: RadixNum| n.with_radix(24).expect("radix 24");
        let num0 = to_radix24(RadixNum::from(             0 as   u8));
        let num1 = to_radix24(RadixNum::from(             1 as  u16));
        let num2 = to_radix24(RadixNum::from(            10 as  u32));
        let num3 = to_radix24(RadixNum::from(         32767 as  u64));
        let num4 = to_radix24(RadixNum::from(   15123093122 as  u64));
        let num5 = to_radix24(RadixNum::from(  462058535375 as u128));
        let num6 = to_radix24(RadixNum::from(46597557513433 as u128));
        assert_eq!(             Ok(0), num0.as_decimal());
        assert_eq!(             Ok(1), num1.as_decimal());
        assert_eq!(            Ok(10), num2.as_decimal());
        assert_eq!(         Ok(32767), num3.as_decimal());
        assert_eq!(   Ok(15123093122), num4.as_decimal());
        assert_eq!(  Ok(462058535375), num5.as_decimal());
        assert_eq!(Ok(46597557513433), num6.as_decimal());
    }

    #[test]
    fn radix25_to_dec() {
        let to_radix25 = |n: RadixNum| n.with_radix(25).expect("radix 25");
        let num0 = to_radix25(RadixNum::from(             0 as   u8));
        let num1 = to_radix25(RadixNum::from(             1 as  u16));
        let num2 = to_radix25(RadixNum::from(            10 as  u32));
        let num3 = to_radix25(RadixNum::from(         32767 as  u64));
        let num4 = to_radix25(RadixNum::from(   15123093122 as  u64));
        let num5 = to_radix25(RadixNum::from(  462058535375 as u128));
        let num6 = to_radix25(RadixNum::from(46597557513433 as u128));
        assert_eq!(             Ok(0), num0.as_decimal());
        assert_eq!(             Ok(1), num1.as_decimal());
        assert_eq!(            Ok(10), num2.as_decimal());
        assert_eq!(         Ok(32767), num3.as_decimal());
        assert_eq!(   Ok(15123093122), num4.as_decimal());
        assert_eq!(  Ok(462058535375), num5.as_decimal());
        assert_eq!(Ok(46597557513433), num6.as_decimal());
    }

    #[test]
    fn radix26_to_dec() {
        let to_radix26 = |n: RadixNum| n.with_radix(26).expect("radix 26");
        let num0 = to_radix26(RadixNum::from(             0 as   u8));
        let num1 = to_radix26(RadixNum::from(             1 as  u16));
        let num2 = to_radix26(RadixNum::from(            10 as  u32));
        let num3 = to_radix26(RadixNum::from(         32767 as  u64));
        let num4 = to_radix26(RadixNum::from(   15123093122 as  u64));
        let num5 = to_radix26(RadixNum::from(  462058535375 as u128));
        let num6 = to_radix26(RadixNum::from(46597557513433 as u128));
        assert_eq!(             Ok(0), num0.as_decimal());
        assert_eq!(             Ok(1), num1.as_decimal());
        assert_eq!(            Ok(10), num2.as_decimal());
        assert_eq!(         Ok(32767), num3.as_decimal());
        assert_eq!(   Ok(15123093122), num4.as_decimal());
        assert_eq!(  Ok(462058535375), num5.as_decimal());
        assert_eq!(Ok(46597557513433), num6.as_decimal());
    }

    #[test]
    fn radix27_to_dec() {
        let to_radix27 = |n: RadixNum| n.with_radix(27).expect("radix 27");
        let num0 = to_radix27(RadixNum::from(             0 as   u8));
        let num1 = to_radix27(RadixNum::from(             1 as  u16));
        let num2 = to_radix27(RadixNum::from(            10 as  u32));
        let num3 = to_radix27(RadixNum::from(         32767 as  u64));
        let num4 = to_radix27(RadixNum::from(   15123093122 as  u64));
        let num5 = to_radix27(RadixNum::from(  462058535375 as u128));
        let num6 = to_radix27(RadixNum::from(46597557513433 as u128));
        assert_eq!(             Ok(0), num0.as_decimal());
        assert_eq!(             Ok(1), num1.as_decimal());
        assert_eq!(            Ok(10), num2.as_decimal());
        assert_eq!(         Ok(32767), num3.as_decimal());
        assert_eq!(   Ok(15123093122), num4.as_decimal());
        assert_eq!(  Ok(462058535375), num5.as_decimal());
        assert_eq!(Ok(46597557513433), num6.as_decimal());
    }

    #[test]
    fn radix28_to_dec() {
        let to_radix28 = |n: RadixNum| n.with_radix(28).expect("radix 28");
        let num0 = to_radix28(RadixNum::from(             0 as   u8));
        let num1 = to_radix28(RadixNum::from(             1 as  u16));
        let num2 = to_radix28(RadixNum::from(            10 as  u32));
        let num3 = to_radix28(RadixNum::from(         32767 as  u64));
        let num4 = to_radix28(RadixNum::from(   15123093122 as  u64));
        let num5 = to_radix28(RadixNum::from(  462058535375 as u128));
        let num6 = to_radix28(RadixNum::from(46597557513433 as u128));
        assert_eq!(             Ok(0), num0.as_decimal());
        assert_eq!(             Ok(1), num1.as_decimal());
        assert_eq!(            Ok(10), num2.as_decimal());
        assert_eq!(         Ok(32767), num3.as_decimal());
        assert_eq!(   Ok(15123093122), num4.as_decimal());
        assert_eq!(  Ok(462058535375), num5.as_decimal());
        assert_eq!(Ok(46597557513433), num6.as_decimal());
    }

    #[test]
    fn radix29_to_dec() {
        let to_radix29 = |n: RadixNum| n.with_radix(29).expect("radix 29");
        let num0 = to_radix29(RadixNum::from(             0 as   u8));
        let num1 = to_radix29(RadixNum::from(             1 as  u16));
        let num2 = to_radix29(RadixNum::from(            10 as  u32));
        let num3 = to_radix29(RadixNum::from(         32767 as  u64));
        let num4 = to_radix29(RadixNum::from(   15123093122 as  u64));
        let num5 = to_radix29(RadixNum::from(  462058535375 as u128));
        let num6 = to_radix29(RadixNum::from(46597557513433 as u128));
        assert_eq!(             Ok(0), num0.as_decimal());
        assert_eq!(             Ok(1), num1.as_decimal());
        assert_eq!(            Ok(10), num2.as_decimal());
        assert_eq!(         Ok(32767), num3.as_decimal());
        assert_eq!(   Ok(15123093122), num4.as_decimal());
        assert_eq!(  Ok(462058535375), num5.as_decimal());
        assert_eq!(Ok(46597557513433), num6.as_decimal());
    }

    #[test]
    fn radix30_to_dec() {
        let to_radix30 = |n: RadixNum| n.with_radix(30).expect("radix 30");
        let num0 = to_radix30(RadixNum::from(             0 as   u8));
        let num1 = to_radix30(RadixNum::from(             1 as  u16));
        let num2 = to_radix30(RadixNum::from(            10 as  u32));
        let num3 = to_radix30(RadixNum::from(         32767 as  u64));
        let num4 = to_radix30(RadixNum::from(   15123093122 as  u64));
        let num5 = to_radix30(RadixNum::from(  462058535375 as u128));
        let num6 = to_radix30(RadixNum::from(46597557513433 as u128));
        assert_eq!(             Ok(0), num0.as_decimal());
        assert_eq!(             Ok(1), num1.as_decimal());
        assert_eq!(            Ok(10), num2.as_decimal());
        assert_eq!(         Ok(32767), num3.as_decimal());
        assert_eq!(   Ok(15123093122), num4.as_decimal());
        assert_eq!(  Ok(462058535375), num5.as_decimal());
        assert_eq!(Ok(46597557513433), num6.as_decimal());
    }

    #[test]
    fn radix31_to_dec() {
        let to_radix31 = |n: RadixNum| n.with_radix(31).expect("radix 31");
        let num0 = to_radix31(RadixNum::from(             0 as   u8));
        let num1 = to_radix31(RadixNum::from(             1 as  u16));
        let num2 = to_radix31(RadixNum::from(            10 as  u32));
        let num3 = to_radix31(RadixNum::from(         32767 as  u64));
        let num4 = to_radix31(RadixNum::from(   15123093122 as  u64));
        let num5 = to_radix31(RadixNum::from(  462058535375 as u128));
        let num6 = to_radix31(RadixNum::from(46597557513433 as u128));
        assert_eq!(             Ok(0), num0.as_decimal());
        assert_eq!(             Ok(1), num1.as_decimal());
        assert_eq!(            Ok(10), num2.as_decimal());
        assert_eq!(         Ok(32767), num3.as_decimal());
        assert_eq!(   Ok(15123093122), num4.as_decimal());
        assert_eq!(  Ok(462058535375), num5.as_decimal());
        assert_eq!(Ok(46597557513433), num6.as_decimal());
    }

    #[test]
    fn radix32_to_dec() {
        let to_radix32 = |n: RadixNum| n.with_radix(32).expect("radix 32");
        let num0 = to_radix32(RadixNum::from(             0 as   u8));
        let num1 = to_radix32(RadixNum::from(             1 as  u16));
        let num2 = to_radix32(RadixNum::from(            10 as  u32));
        let num3 = to_radix32(RadixNum::from(         32767 as  u64));
        let num4 = to_radix32(RadixNum::from(   15123093122 as  u64));
        let num5 = to_radix32(RadixNum::from(  462058535375 as u128));
        let num6 = to_radix32(RadixNum::from(46597557513433 as u128));
        assert_eq!(             Ok(0), num0.as_decimal());
        assert_eq!(             Ok(1), num1.as_decimal());
        assert_eq!(            Ok(10), num2.as_decimal());
        assert_eq!(         Ok(32767), num3.as_decimal());
        assert_eq!(   Ok(15123093122), num4.as_decimal());
        assert_eq!(  Ok(462058535375), num5.as_decimal());
        assert_eq!(Ok(46597557513433), num6.as_decimal());
    }

    #[test]
    fn radix33_to_dec() {
        let to_radix33 = |n: RadixNum| n.with_radix(33).expect("radix 33");
        let num0 = to_radix33(RadixNum::from(             0 as   u8));
        let num1 = to_radix33(RadixNum::from(             1 as  u16));
        let num2 = to_radix33(RadixNum::from(            10 as  u32));
        let num3 = to_radix33(RadixNum::from(         32767 as  u64));
        let num4 = to_radix33(RadixNum::from(   15123093122 as  u64));
        let num5 = to_radix33(RadixNum::from(  462058535375 as u128));
        let num6 = to_radix33(RadixNum::from(46597557513433 as u128));
        assert_eq!(             Ok(0), num0.as_decimal());
        assert_eq!(             Ok(1), num1.as_decimal());
        assert_eq!(            Ok(10), num2.as_decimal());
        assert_eq!(         Ok(32767), num3.as_decimal());
        assert_eq!(   Ok(15123093122), num4.as_decimal());
        assert_eq!(  Ok(462058535375), num5.as_decimal());
        assert_eq!(Ok(46597557513433), num6.as_decimal());
    }

    #[test]
    fn radix34_to_dec() {
        let to_radix34 = |n: RadixNum| n.with_radix(34).expect("radix 34");
        let num0 = to_radix34(RadixNum::from(             0 as   u8));
        let num1 = to_radix34(RadixNum::from(             1 as  u16));
        let num2 = to_radix34(RadixNum::from(            10 as  u32));
        let num3 = to_radix34(RadixNum::from(         32767 as  u64));
        let num4 = to_radix34(RadixNum::from(   15123093122 as  u64));
        let num5 = to_radix34(RadixNum::from(  462058535375 as u128));
        let num6 = to_radix34(RadixNum::from(46597557513433 as u128));
        assert_eq!(             Ok(0), num0.as_decimal());
        assert_eq!(             Ok(1), num1.as_decimal());
        assert_eq!(            Ok(10), num2.as_decimal());
        assert_eq!(         Ok(32767), num3.as_decimal());
        assert_eq!(   Ok(15123093122), num4.as_decimal());
        assert_eq!(  Ok(462058535375), num5.as_decimal());
        assert_eq!(Ok(46597557513433), num6.as_decimal());
    }

    #[test]
    fn radix35_to_dec() {
        let to_radix35 = |n: RadixNum| n.with_radix(35).expect("radix 35");
        let num0 = to_radix35(RadixNum::from(             0 as   u8));
        let num1 = to_radix35(RadixNum::from(             1 as  u16));
        let num2 = to_radix35(RadixNum::from(            10 as  u32));
        let num3 = to_radix35(RadixNum::from(         32767 as  u64));
        let num4 = to_radix35(RadixNum::from(   15123093122 as  u64));
        let num5 = to_radix35(RadixNum::from(  462058535375 as u128));
        let num6 = to_radix35(RadixNum::from(46597557513433 as u128));
        assert_eq!(             Ok(0), num0.as_decimal());
        assert_eq!(             Ok(1), num1.as_decimal());
        assert_eq!(            Ok(10), num2.as_decimal());
        assert_eq!(         Ok(32767), num3.as_decimal());
        assert_eq!(   Ok(15123093122), num4.as_decimal());
        assert_eq!(  Ok(462058535375), num5.as_decimal());
        assert_eq!(Ok(46597557513433), num6.as_decimal());
    }

    #[test]
    fn radix36_to_dec() {
        let to_radix36 = |n: RadixNum| n.with_radix(36).expect("radix 36");
        let num0 = to_radix36(RadixNum::from(             0 as   u8));
        let num1 = to_radix36(RadixNum::from(             1 as  u16));
        let num2 = to_radix36(RadixNum::from(            10 as  u32));
        let num3 = to_radix36(RadixNum::from(         32767 as  u64));
        let num4 = to_radix36(RadixNum::from(   15123093122 as  u64));
        let num5 = to_radix36(RadixNum::from(  462058535375 as u128));
        let num6 = to_radix36(RadixNum::from(46597557513433 as u128));
        assert_eq!(             Ok(0), num0.as_decimal());
        assert_eq!(             Ok(1), num1.as_decimal());
        assert_eq!(            Ok(10), num2.as_decimal());
        assert_eq!(         Ok(32767), num3.as_decimal());
        assert_eq!(   Ok(15123093122), num4.as_decimal());
        assert_eq!(  Ok(462058535375), num5.as_decimal());
        assert_eq!(Ok(46597557513433), num6.as_decimal());
    }

}

//  LocalWords:  radix
