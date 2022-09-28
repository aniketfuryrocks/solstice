







pub fn add()->i32{
    2+2
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        
        assert_eq!(4, add());
    }
}
