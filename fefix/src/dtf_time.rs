#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct DtfTime {
    hour: u32,
    minute: u32,
    second: u32,
    milli: u32,
}

impl DtfTime {
    pub fn parse(data: &[u8]) -> Option<Self> {
        if !Self::validate(data) {
            None
        } else {
            Self::parse_speedy(data)
        }
    }

    pub fn parse_speedy(data: &[u8]) -> Option<Self> {
        let hour = from_digit(data[0]) as u32 * 1000
            + from_digit(data[1]) as u32 * 100
            + from_digit(data[2]) as u32 * 10
            + from_digit(data[3]) as u32;
        let minute = from_digit(data[4]) as u32 * 10 + from_digit(data[5]) as u32;
        let second = from_digit(data[5]) as u32 * 10 + from_digit(data[6]) as u32;
        let milli = 0;
        Some(Self {
            hour,
            minute,
            second,
            milli,
        })
    }

    fn validate(data: &[u8]) -> bool {
        if data.len() != 8 {
            return false;
        }
        Self::validate_hour(data)
            && Self::validate_minute(data)
            && Self::validate_second(data)
            && Self::validate_milli(data)
    }

    fn validate_hour(data: &[u8]) -> bool {
        is_digit(data[0], 0, 9)
            && is_digit(data[1], 0, 9)
            && is_digit(data[2], 0, 9)
            && is_digit(data[3], 0, 9)
    }

    fn validate_minute(data: &[u8]) -> bool {
        ((data[4] == b'0' && data[5] <= b'9') || (data[4] == b'1' && data[5] <= b'2'))
            && data[5] >= b'0'
    }

    fn validate_second(data: &[u8]) -> bool {
        (data[6] == b'0' && data[7] >= b'0' && data[7] <= b'9')
            || (data[6] == b'1' && data[7] >= b'0' && data[7] <= b'9')
            || (data[6] == b'2' && data[7] >= b'0' && data[7] <= b'9')
            || (data[6] == b'3' && data[7] >= b'0' && data[7] <= b'1')
    }

    fn validate_milli(data: &[u8]) -> bool {
        (data[6] == b'0' && data[7] >= b'0' && data[7] <= b'9')
            || (data[6] == b'1' && data[7] >= b'0' && data[7] <= b'9')
            || (data[6] == b'2' && data[7] >= b'0' && data[7] <= b'9')
            || (data[6] == b'3' && data[7] >= b'0' && data[7] <= b'1')
    }

    pub fn to_bytes(&self) -> [u8; 8] {
        [
            (self.hour() / 10) as u8 + b'0',
            (self.hour() % 10) as u8 + b'0',
            b':',
            (self.minute() / 10) as u8 + b'0',
            (self.minute() % 10) as u8 + b'0',
            b':',
            (self.second() / 10) as u8 + b'0',
            (self.second() % 10) as u8 + b'0',
        ]
    }

    pub fn to_bytes_wm(&self) -> [u8; 12] {
        [
            (self.hour() / 10) as u8 + b'0',
            (self.hour() % 10) as u8 + b'0',
            b':',
            (self.minute() / 10) as u8 + b'0',
            (self.minute() % 10) as u8 + b'0',
            b':',
            (self.second() / 10) as u8 + b'0',
            (self.second() % 10) as u8 + b'0',
            b'.',
            (self.milli() / 100) as u8 + b'0',
            ((self.milli() / 10) % 10) as u8 % 10 + b'0',
            (self.milli() % 10) as u8 + b'0',
        ]
    }

    pub fn hour(&self) -> u32 {
        self.hour
    }

    pub fn minute(&self) -> u32 {
        self.minute
    }

    pub fn second(&self) -> u32 {
        self.second
    }

    pub fn milli(&self) -> u32 {
        self.milli
    }
}

fn is_digit(byte: u8, min_digit: u8, max_digit: u8) -> bool {
    byte >= (min_digit + b'0') && byte <= (max_digit + b'0')
}

fn from_digit(digit: u8) -> u8 {
    digit.wrapping_sub(b'0')
}
