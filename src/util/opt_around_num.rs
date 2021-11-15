//{{{ OptAroundNum
#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub struct OptAroundNum(u8);

impl OptAroundNum {
    pub fn is_empty(&self) -> bool {
        self.0 == 0
    }
    pub fn num(&self) -> usize {
        self.0 as usize
    }
}

impl ::std::str::FromStr for OptAroundNum {
    type Err = OptAroundNumParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let num = match s.parse::<u8>() {
            Ok(d) => d,
            Err(err) => {
                let s = format!("can not parse '{}': {}", s, err);
                return Err(OptAroundNumParseError::new(s));
            }
        };
        Ok(OptAroundNum(num))
    }
}

impl ::std::fmt::Display for OptAroundNum {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
//}}} OptAroundNum

//{{{ OptAroundNumParseError
#[derive(Debug)]
pub struct OptAroundNumParseError {
    desc: String,
}

impl OptAroundNumParseError {
    fn new(s: String) -> OptAroundNumParseError {
        OptAroundNumParseError { desc: s }
    }
}

impl ::std::fmt::Display for OptAroundNumParseError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.desc.fmt(f)
    }
}

impl ::std::error::Error for OptAroundNumParseError {
    fn description(&self) -> &str {
        self.desc.as_str()
    }
}
//}}} OptAroundNumParseError

#[cfg(test)]
mod tests {
    //use std::error::Error;
    use std::str::FromStr;

    use super::*;

    #[test]
    fn test_display_default() {
        let oan = OptAroundNum::default();
        assert_eq!(format!("{}", oan), "0");
    }
    #[test]
    fn test_display_10() {
        let oan = OptAroundNum(10);
        assert_eq!(format!("{}", oan), "10");
    }
    #[test]
    fn test_from_str_10() {
        let col: OptAroundNum = match FromStr::from_str("10") {
            Ok(c) => c,
            Err(_) => {
                unreachable!();
            }
        };
        assert_eq!(col, OptAroundNum(10));
    }
}
