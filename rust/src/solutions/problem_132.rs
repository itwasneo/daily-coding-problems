/*
    This question was asked by Riot Games.

    Design and implement a HitCounter class that keeps track of requests (or hits). It should support the following operations:

    record(timestamp): records a hit that happened at timestamp
    total(): returns the total number of hits recorded
    range(lower, upper): returns the number of hits that occurred between timestamps lower and upper (inclusive)
    Follow-up: What if our system has limited memory?
*/

#[allow(dead_code)]
struct HitCounter {
    hits: Vec<u32>,
}

impl HitCounter {
    #[allow(dead_code)]
    fn new() -> Self {
        Self { hits: vec![] }
    }

    #[allow(dead_code)]
    fn record(&mut self, ts: u32) {
        self.hits.push(ts);
    }

    #[allow(dead_code)]
    fn total(&self) -> usize {
        self.hits.len()
    }

    #[allow(dead_code)]
    fn range(&self, lo: u32, hi: u32) -> usize {
        self.hits.iter().fold(0, |a, h| {
            if h >= &lo && h <= &hi {
                return a + 1;
            }
            a
        }) as usize
    }
}

#[cfg(test)]
mod tests {
    use super::HitCounter;

    #[test]
    fn problem_132() {
        let mut hc = HitCounter::new();
        hc.record(1);
        hc.record(2);
        hc.record(3);
        hc.record(4);
        assert_eq!(4, hc.total());
        assert_eq!(2, hc.range(2, 3));
    }
}
