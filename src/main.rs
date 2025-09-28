fn main() {
    println!("Hello, Rust!");
}

#[cfg(test)]s
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
