fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    #[test]
    fn test_sort() {
        let x = 1;
        let y = 1;
        assert_eq!(&x,&y);
    }
}