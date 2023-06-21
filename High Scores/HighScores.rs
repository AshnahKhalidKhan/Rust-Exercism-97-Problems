#[derive(Debug)]
pub struct HighScores<'a>
{
    scores: &'a [u32]
}

impl<'a> HighScores<'a>
{
    pub fn new(scores: &'a [u32]) -> Self
    {
        //unimplemented!("Construct a HighScores struct, given the scores: {scores:?}")
        Self
        {
            scores
        }
    }

    pub fn scores(&self) -> &[u32]
    {
        //unimplemented!("Return all the scores as a slice")
        self.scores
    }

    pub fn latest(&self) -> Option<u32>
    {
        //unimplemented!("Return the latest (last) score")
        if self.scores.len() > 0
        {
            return Some(self.scores[self.scores.len() - 1]);
        }
        else
        {
            return None;
        }
    }

    pub fn personal_best(&self) -> Option<u32>
    {
        //unimplemented!("Return the highest score")
        if self.scores.len() > 0
        {
            let mut top: u32 = 0;
            for i in 0..self.scores.len()
            {
                if self.scores[i] > top
                {
                    top = self.scores[i];
                }
            }
            return Some(top);
        }
        else
        {
            return None;
        }
    }

    pub fn personal_top_three(&self) -> Vec<u32>
    {
        //unimplemented!("Return 3 highest scores")
        let mut all: Vec<u32> = self.scores.clone().to_vec();
        all.sort();
        let mut top3: Vec<u32> = Vec::new();
        while all.len() > 0 && top3.len() < 3
        {
            top3.push(all[all.len() - 1]);
            all.remove(all.len() - 1);
        }
        top3
    }
}
