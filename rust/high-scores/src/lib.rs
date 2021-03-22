#[derive(Debug)]
pub struct HighScores<'a> {
    scores: &'a [u32]
}

impl<'a> HighScores<'a> {
    pub fn new(scores: &'a [u32]) -> Self {
        HighScores { scores }
    }

    pub fn scores(&self) -> &[u32] {
        self.scores
    }

    pub fn latest(&self) -> Option<u32> {
        if let Some(&x) = self.scores.last() {
            Some(x)
        } else {
            None
        }
    }

    pub fn personal_best(&self) -> Option<u32> {
        if let Some(&x) = self.scores.iter().max() {
            Some(x)
        } else {
            None
        }
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        unimplemented!("Return 3 highest scores")
    }
}
