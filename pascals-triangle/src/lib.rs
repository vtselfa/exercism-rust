pub struct PascalsTriangle {
    row_count: u32,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        Self {row_count: row_count}
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        let mut result = vec![];
        let prev: Vec<u32> = vec![];
        let mut prev = &prev[..];
        for _ in 0 .. self.row_count {
            let row = self.new_row(prev); 
            result.push(row);
            prev = &result.last().unwrap()[..];
        }
        result
    }

    fn new_row(&self, prev: &[u32]) -> Vec<u32> {
        match prev.len() {
            0 => vec![1],
            1 => vec![1, 1],
            _ => {
                let mut row = vec![1];
                let iter1 = prev[0..].iter();
                let iter2 = prev[1..].iter();
                for (a,b) in iter1.zip(iter2) {
                   row.push(a + b); 
                }
                row.push(1);
                row
            }
        }
    }
}
