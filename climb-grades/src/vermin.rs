use std::str::FromStr;
use std::fmt::Display;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Grade {
    value: u8,
}

impl Grade {
    pub fn new(value: u8) -> Grade {
        Grade { value }
    }

    pub fn value(& self) -> u8 {
        self.value
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseGradeError;

impl FromStr for Grade {
    type Err = ParseGradeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v = s.strip_prefix('V')
            .ok_or(ParseGradeError)?;

        let value = v.parse::<u8>().map_err(|_| ParseGradeError)?;
        Ok(Grade { value })
    }
}

impl Display for Grade {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = self.value;
        write!(f, "V{value}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let grade = Grade::new(1);
        assert_eq!(grade.value, 1);
    }

    #[test]
    fn value() {
        let grade = Grade { value: 1 };
        assert_eq!(grade.value(), 1);
    }

    #[test]
    fn from_str() {
        assert_eq!(Grade::from_str(""), Err(ParseGradeError));
        assert_eq!(Grade::from_str("V"), Err(ParseGradeError));
        assert_eq!(Grade::from_str("1"), Err(ParseGradeError));
        assert_eq!(Grade::from_str("V1"), Ok(Grade { value: 1 }));
    }

    #[test]
    fn fmt() {
        assert_eq!(format!("{}", Grade { value: 1 }), "V1");
    }
}

