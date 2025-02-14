#[allow(dead_code)]
struct ProductOfNumbers {
    its: Vec<i32>,
}

impl ProductOfNumbers {
    #[allow(dead_code)]
    fn new() -> Self {
        Self { its: vec![1] }
    }

    #[allow(dead_code)]
    fn add(&mut self, num: i32) {
        if num == 0 {
            self.its = vec![1];
        } else {
            self.its.push(self.its.last().unwrap() * num);
        }
    }

    #[allow(dead_code)]
    fn get_product(&self, k: i32) -> i32 {
        if self.its.len() <= 1 || k as usize >= self.its.len() {
            0
        } else {
            self.its.last().unwrap() / self.its[self.its.len() - k as usize - 1]
        }
    }
}

fn main() {
    println!("nothing here");
}

#[cfg(test)]
mod tests {
    use super::ProductOfNumbers;

    #[test]
    fn test_case1() {
        let mut pon = ProductOfNumbers::new();
        pon.add(3);
        pon.add(0);
        pon.add(2);
        pon.add(5);
        pon.add(4);
        assert_eq!(pon.get_product(2), 20);
        assert_eq!(pon.get_product(3), 40);
        assert_eq!(pon.get_product(4), 0);
        pon.add(8);
        assert_eq!(pon.get_product(2), 32);
    }
}
