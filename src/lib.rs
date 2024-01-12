pub trait CaseConvert {
    fn uppercase_first(&self) -> String;
}

impl CaseConvert for String {
    /// Converts the first letter of a String to uppercase
    ///
    /// Example:
    /// ```
    /// use case_convert::CaseConvert;
    /// let s = String::from("test");
    /// assert_eq!(s.uppercase_first(), "Test");
    /// ```
    fn uppercase_first(&self) -> String {
        let mut c = self.chars();
        match c.next() {
            None => String::new(),
            Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn u_first() {
        let s = String::from("test");
        let result = s.uppercase_first();
        assert_eq!(result, "Test");
    }
}
