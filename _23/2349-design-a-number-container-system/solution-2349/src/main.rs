use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

#[allow(dead_code)]
struct NumberContainers {
    pos: HashMap<i32, i32>,
    num: HashMap<i32, BinaryHeap<Reverse<i32>>>,
}

impl NumberContainers {
    #[allow(dead_code)]
    fn new() -> Self {
        Self {
            pos: HashMap::new(),
            num: HashMap::new(),
        }
    }

    #[allow(dead_code)]
    fn change(&mut self, index: i32, number: i32) {
        if self.pos.contains_key(&index) {
            let ex = *self.pos.get(&index).unwrap();
            if ex == number {
                return;
            } else if let Some(bh) = self.num.get_mut(&ex) {
                bh.retain(|x| x.0 != index);
            }
        }
        self.pos.insert(index, number);
        if let Some(bh) = self.num.get_mut(&number) {
            bh.push(Reverse(index));
        } else {
            let mut bh = BinaryHeap::new();
            bh.push(Reverse(index));
            self.num.insert(number, bh);
        }
    }

    #[allow(dead_code)]
    fn find(&self, number: i32) -> i32 {
        let ret = if let Some(bh) = self.num.get(&number) {
            bh.peek().unwrap_or(&Reverse(-1)).0
        } else {
            -1
        };
        ret
    }
}

fn main() {
    println!("nothing here");
}

#[cfg(test)]
mod tests {
    use super::NumberContainers;

    #[test]
    fn test_case1() {
        let mut nc = NumberContainers::new();
        assert_eq!(nc.find(10), -1);
        nc.change(2, 10);
        nc.change(1, 10);
        nc.change(3, 10);
        nc.change(5, 10);
        assert_eq!(nc.find(10), 1);
        nc.change(1, 20);
        assert_eq!(nc.find(10), 2);
    }

    #[test]
    fn test_case2() {
        let mut nc = NumberContainers::new();
        nc.change(75, 40);
        assert_eq!(nc.find(14), -1);
        assert_eq!(nc.find(41), -1);
        assert_eq!(nc.find(40), 75);
        nc.change(27, 40);
        nc.change(22, 14);
        nc.change(85, 14);
        nc.change(22, 40);
        nc.change(18, 34);
        nc.change(92, 41);
        nc.change(22, 40);
        nc.change(75, 40);
        nc.change(59, 34);
        assert_eq!(nc.find(40), 22);
        nc.change(27, 14);
        assert_eq!(nc.find(34), 18);
        assert_eq!(nc.find(14), 27);
        nc.change(13, 34);
        assert_eq!(nc.find(40), 22);
        nc.change(18, 41);
    }
}
