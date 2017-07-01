//! A rust library to deal with number conversion between radices.

#![feature(conservative_impl_trait)]
#![feature(i128_type)]

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
}


/// A number in some radix.
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
    pub fn with_radix(&self, radix: usize) -> RadixResult<RadixNum> {
        let digits_radix_x: String =
            Self::dec_to_radix_x(radix,  self.as_decimal()?,  None)?;
        Ok(match radix {
             2 =>  RadixNum::Radix2(digits_radix_x),
             3 =>  RadixNum::Radix3(digits_radix_x),
             4 =>  RadixNum::Radix4(digits_radix_x),
             5 =>  RadixNum::Radix5(digits_radix_x),
             6 =>  RadixNum::Radix6(digits_radix_x),
             7 =>  RadixNum::Radix7(digits_radix_x),
             8 =>  RadixNum::Radix8(digits_radix_x),
             9 =>  RadixNum::Radix9(digits_radix_x),
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

    pub fn as_decimal(&self) -> RadixResult<usize> {
        Self::radix_x_to_dec(self.radix(), self.as_str())
    }

    pub fn digits<'c>(&'c self) -> impl Iterator<Item=char> {
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
            RadixNum::Radix36(ref string) => {
                // TODO: this impl is suboptimal, but should work for now.
                string.chars().collect::<Vec<char>>().into_iter()
            },
        }
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

    fn dec_to_radix_x(radix: usize, number: usize, len: Option<usize>)
                     -> Result<String, RadixErr> {
        if number == 0 { return Ok(String::from("0")) }

        let digits: Vec<char> = number.to_string().chars().collect();
        let mut number: usize = number;
        let mut stack: Vec<char> = vec![];

        debug!("\n");
        debug!("[dec_to_radix_x] radix:   {:?}", radix);
        debug!("[dec_to_radix_x] number: {:?}", number);
        debug!("[dec_to_radix_x] len:    {:?}", len);
        debug!("[dec_to_radix_x] digits: {:?}", digits);
        debug!("[dec_to_radix_x] stack: {:?}", stack);

        let get_offset = |digit: usize| -> Result<u8, RadixErr> {
            match digit {
                0 ...  9 => Ok('0' as u8), //  1u8 => '1',   2u8 =>  '2',  etc
                10 ... 31 => Ok(55),        // 10u8 => 'A',  11u8 =>  'B',  etc
                d => Err(RadixErr::IllegalDigit(d)),
            }
        };

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
            match stack.pop() {
                Some(digit) => return_val += digit.to_string().as_ref(),
                None => return Err(RadixErr::FailedToPopFromStack),
            }
        }
        if let Some(len) = len {
            let padding: String = repeat("0", len);
            let total: String = padding + return_val.trim();
            return_val = total.chars()
                .skip(total.len() - len)
                .collect();
        }
        debug!("[dec_to_radix_x] return_val: {}", return_val);
        Ok(return_val)
    }

    fn radix_x_to_dec(radix: usize, number: &str) -> RadixResult<usize> {
        if number.is_empty() { return Err(RadixErr::EmptyInput); }

        let number: &str = number.trim();
        let char_count: usize = number.chars().count();
        let number_chars: Vec<char> = number.chars().collect();
        let mut return_val: usize = 0;

        fn digit_offset(digit: char) -> Result<u8, RadixErr> {
            println!("[digit_offset] digit: {}", digit);
            match digit {
                '0'...'9' => Ok('0' as u8), // '1' =>  1u8,  '2' =>  2u8 etc
                'A'...'Z' => Ok(55),        // 'A' => 10u8,  'B' => 11u8 etc
                c => Err(RadixErr::IllegalChar(c)),
            }
        }

        debug!("\n");
        debug!("[radix_x_to_dec] input radix: {}", radix);
        debug!("[radix_x_to_dec] input number: {}", number);
        debug!("[radix_x_to_dec] trimmed number: {}", number);
        debug!("[radix_x_to_dec] char count: {}", char_count);
        debug!("[radix_x_to_dec] char vec: {:?}", number_chars);
        debug!("[radix_x_to_dec] return val: {:?}", return_val);
        debug!("[radix_x_to_dec] for loop:");
        for (idx, &token) in number_chars.iter().rev().enumerate() {
            let token: char = token
                .to_uppercase()
                .nth(0)
                .ok_or_else(|| RadixErr::FailedToUppercase)?;
            debug!("[radix_x_to_dec]   idx: {:?}", idx);
            debug!("[radix_x_to_dec]   token: {:?}  ({}u8)", token, token as u8);
            let offset = digit_offset(token)?;
            let token: char = (token as u8 - offset) as char;
            return_val += token as usize * radix.pow(idx as u32) /*- '0' as usize*/;
            debug!("[radix_x_to_dec]   token altered: {:?}  ({}u8)", token, token as u8);
            debug!("[radix_x_to_dec]   return val: {:?}", return_val);
        }

        Ok(return_val)
    }
}

impl From<usize> for RadixNum {
    fn from(decimal: usize) -> RadixNum {  RadixNum::Radix10(decimal.to_string())  }
}

impl From<u8> for RadixNum {
    fn from(decimal: u8) -> RadixNum {  RadixNum::Radix10(decimal.to_string())  }
}

impl From<u16> for RadixNum {
    fn from(decimal: u16) -> RadixNum {  RadixNum::Radix10(decimal.to_string())  }
}

impl From<u32> for RadixNum {
    fn from(decimal: u32) -> RadixNum {  RadixNum::Radix10(decimal.to_string())  }
}

impl From<u64> for RadixNum {
    fn from(decimal: u64) -> RadixNum {  RadixNum::Radix10(decimal.to_string())  }
}

impl From<u128> for RadixNum {
    fn from(decimal: u128) -> RadixNum {  RadixNum::Radix10(decimal.to_string())  }
}



// Helper functions

fn modulus(a: usize, b: usize) -> usize {
    ((a % b) + b) % b
}

fn repeat(token: &str, times: usize) -> String {
    let mut string = String::new();
    for _ in 0 .. times {  string.push_str(token);  }
    string
}







#[cfg(test)]
mod tests {
    use *;

    #[test]
    fn dec_to_radix_x() {
        let num0 = RadixNum::from(           0 as   u8).with_radix(32).expect("0");
        let num1 = RadixNum::from(           1 as  u16).with_radix(32).expect("1");
        let num2 = RadixNum::from(          10 as  u32).with_radix(32).expect("A");
        let num3 = RadixNum::from(       32767 as  u64).with_radix(32).expect("VVV");
        let num4 = RadixNum::from(462058535375 as u128).with_radix(32).expect("DEADBEEF");
        assert_eq!(       "0", num0.as_str());
        assert_eq!(       "1", num1.as_str());
        assert_eq!(       "A", num2.as_str());
        assert_eq!(     "VVV", num3.as_str());
        assert_eq!("DEADBEEF", num4.as_str());

    }

    #[test]
    fn radix_x_to_dec() {
        let num0 = RadixNum::from(           0 as   u8).with_radix(32).expect("0");
        let num1 = RadixNum::from(           1 as  u16).with_radix(32).expect("1");
        let num2 = RadixNum::from(          10 as  u32).with_radix(32).expect("A");
        let num3 = RadixNum::from(       32767 as  u64).with_radix(32).expect("VVV");
        let num4 = RadixNum::from(462058535375 as u128).with_radix(32).expect("DEADBEEF");
        assert_eq!(           Ok(0), num0.as_decimal());
        assert_eq!(           Ok(1), num1.as_decimal());
        assert_eq!(          Ok(10), num2.as_decimal());
        assert_eq!(       Ok(32767), num3.as_decimal());
        assert_eq!(Ok(462058535375), num4.as_decimal());
    }

    #[test]
    fn digit_iterator() {
        let num = RadixNum::from(462058535375 as u128)
            .with_radix(32)
            .expect("DEADBEEF");
        let digits: Vec<char> = num.digits().collect();
        assert_eq!(vec!['D', 'E', 'A', 'D', 'B', 'E', 'E', 'F'], digits);
    }

}
