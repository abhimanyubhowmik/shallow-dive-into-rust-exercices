use std::u32;

fn addition(a : u32, b : u32) -> u32{
    return  a + b;
}

fn main() {
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1_addition() {
        assert_eq!(addition(1, 1), 2);
    }
}
