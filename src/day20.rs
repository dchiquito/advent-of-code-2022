use core::fmt;

use crate::advent;

struct LList {
    nodes: Vec<i64>,
    links: Vec<(usize, usize)>,
}

impl LList {
    fn new() -> LList {
        LList {
            nodes: Vec::new(),
            links: Vec::new(),
        }
    }
    fn push(&mut self, value: i64) {
        self.nodes.push(value);
        let len = self.nodes.len();
        if self.links.len() == 0 {
            self.links.push((0, 0));
        } else if self.links.len() == 1 {
            self.links.push((0, 0));
            self.links[0] = (1, 1);
        } else {
            // this is the node before the new one
            let (prev_prev, prev_next) = self.links[len - 2];
            // this is the node after the new one
            let (next_prev, next_next) = self.links[0];
            // create a new node that links to the ones before and after
            self.links.push((next_prev, prev_next));
            // update the before/after nodes to reference the new one
            self.links[len - 2] = (prev_prev, len - 1);
            self.links[0] = (len - 1, next_next);
        }
    }
    fn apply_swap(&mut self, start: usize) {
        let distance = self.nodes[start];
        let distance = distance + 10 * 811589153 * (self.nodes.len() as i64 - 1);
        let distance = distance % (self.nodes.len() as i64 - 1);
        let distance = distance + 1;
        let mut distance = distance as u64;
        let mut end = start;
        while distance > 0 {
            (_, end) = self.links[end];
            distance -= 1;
        }
        // Remove start by linking its prev/next together
        let (s_p, s_n) = self.links[start];
        let (s_p_p, _s_p_n) = self.links[s_p];
        let (_s_n_p, s_n_n) = self.links[s_n];
        self.links[s_p] = (s_p_p, s_n);
        self.links[s_n] = (s_p, s_n_n);
        // Inject start between end and its predescesor
        let (e_p, e_n) = self.links[end];
        let (e_p_p, _e_p_n) = self.links[e_p];
        self.links[e_p] = (e_p_p, start);
        self.links[end] = (start, e_n);
        self.links[start] = (e_p, end);
    }
}

impl fmt::Display for LList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        let mut node: usize = 0;
        write!(f, "{}", self.nodes[node])?;
        (_, node) = self.links[node];
        while node != 0 {
            write!(f, ", {}", self.nodes[node])?;
            (_, node) = self.links[node];
        }
        write!(f, "]")?;
        write!(f, " ({:?})", self.links)?;
        Ok(())
    }
}

fn build_list() -> LList {
    let lines = advent::read_input(20);
    let mut list = LList::new();
    for line in lines {
        list.push(line.parse().unwrap());
    }
    list
}
pub fn solve_1() -> i64 {
    let mut list = build_list();
    for i in 0..list.nodes.len() {
        list.apply_swap(i);
    }
    let mut index = 0;
    while list.nodes[index] != 0 {
        (_, index) = list.links[index];
    }
    let mut sum = 0;
    for _ in 0..3 {
        for _ in 0..1000 {
            (_, index) = list.links[index];
        }
        sum += list.nodes[index];
    }
    sum
}
pub fn solve_2() -> i64 {
    let mut list = build_list();
    list.nodes = list.nodes.iter().map(|v| v * 811589153).collect();
    for _ in 0..10 {
        for i in 0..list.nodes.len() {
            list.apply_swap(i);
        }
    }
    let mut index = 0;
    while list.nodes[index] != 0 {
        (_, index) = list.links[index];
    }
    let mut sum = 0;
    for _ in 0..3 {
        for _ in 0..1000 {
            (_, index) = list.links[index];
        }
        sum += list.nodes[index];
    }
    sum
}
pub fn solve() {
    println!("{}", solve_1());
    println!("{}", solve_2());
}

#[cfg(test)]
mod test {
    use super::LList;

    #[test]
    fn zero_0() {
        let mut list = LList::from(&vec![0, 9, 9, 9]);
        list.apply_swap(0);
        assert_eq!(list.links, vec![(3, 1), (0, 2), (1, 3), (2, 0)]);
    }
    #[test]
    fn zero_1() {
        let mut list = LList::from(&vec![1, 9, 9, 9]);
        list.apply_swap(0);
        assert_eq!(list.links, vec![(1, 2), (3, 0), (0, 3), (2, 1)]);
    }
    #[test]
    fn zero_2() {
        let mut list = LList::from(&vec![2, 9, 9, 9]);
        list.apply_swap(0);
        assert_eq!(list.links, vec![(2, 3), (3, 2), (1, 0), (0, 1)]);
    }
    #[test]
    fn zero_3() {
        let mut list = LList::from(&vec![3, 9, 9, 9]);
        list.apply_swap(0);
        assert_eq!(list.links, vec![(3, 1), (0, 2), (1, 3), (2, 0)]);
    }
    #[test]
    fn zero_4() {
        let mut list = LList::from(&vec![4, 9, 9, 9]);
        list.apply_swap(0);
        assert_eq!(list.links, vec![(1, 2), (3, 0), (0, 3), (2, 1)]);
    }
}
