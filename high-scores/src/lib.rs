use binary_heap_plus::*;

#[derive(Debug)]
pub struct HighScores<'a> {
    highest: Option<u32>,
    scores: &'a[u32],
    kmax: BinaryHeap<u32, MinComparator>,
}

impl<'a> HighScores<'a> {

    pub fn new(scores: &'a[u32]) -> Self {
        let mut hs = HighScores {highest: None, scores: scores, kmax: BinaryHeap::new_min()};
        let k = 3; // K highest scores

        for item in scores.iter() {
            if hs.highest == None || *item > hs.highest.unwrap() {
                hs.highest = Some(*item);
            }
            if hs.kmax.len() < k {
                hs.kmax.push(*item);
            } 
            else {
                if *hs.kmax.peek().unwrap() < *item {
                    hs.kmax.pop();
                    hs.kmax.push(*item);
                }
            }
        }
        hs
    }

    pub fn scores(&self) -> &[u32] {
        self.scores
    }

    pub fn latest(&self) -> Option<u32> {
        self.scores.last().cloned()
    }

    pub fn personal_best(&self) -> Option<u32> {
        self.highest
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        self.kmax.clone().into_sorted_vec() 
    }
}
