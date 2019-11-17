use rand::prelude::*;

pub fn add_one(x: i32) -> i32 {
    let y: u8 = random();
    println!("{}", y);
    x + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(3, add_one(2));
    }
}
