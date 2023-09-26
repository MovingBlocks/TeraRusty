
pub struct Range {
    start: u32,
    end: u32
}

pub struct IDPool {
    avaliable: Vec<Range>
}

impl IDPool {
    pub fn new(reserve: u32) -> Self {
        Self {
            avaliable: vec![
                Range { 
                    start: 0, 
                    end: reserve - 1
            }]
        }
    }

    pub fn fetch_id(&mut self) -> u32 {
        match self.avaliable.last_mut() {
            Some(range) => {
                let res = range.end;
                if range.end == range.start {
                    self.avaliable.pop();
                } else {
                    range.end -= 1;
                }
               res 
            },
            None => {
               return u32::MAX
            }
        }

    }

    pub fn return_id(&mut self, index: u32)  {
        if self.avaliable.is_empty() {
            self.avaliable.push(Range { start: id, end: id});
            return;
        }
        
        let mut lower: usize = 0;
        let mut upper: usize = self.avaliable.len() - 1;
        let mut mid: usize  = lower ;
        while lower != upper  {
            mid = lower + ((upper - lower) / 2);
            if index >= self.avaliable[mid].start && index <= self.avaliable[mid].end {
                assert!(false)
                //assert(false && "found within range");
                return;
            } else if index < self.avaliable[mid].start {
                upper = mid;
            } else {
                lower = mid + 1;
            }
        }
        if index < self.avaliable[mid].start {
            if self.avaliable[mid].start - index == 1 {
                self.avaliable[mid].start -= 1;
                if mid - 1 == 0 &&
                    self.avaliable[mid - 1].end + 1 == self.avaliable[mid].start {
                    self.avaliable[mid - 1].end = self.avaliable[mid].end;
                    self.avaliable.remove(mid);
                }
            } else {
                self.avaliable.insert(mid, Range {start: index, end: index});
            }
        } else if index > self.avaliable[mid].end {
            if index - self.avaliable[mid].end == 1 {
                self.avaliable[mid].end += 1;
                if (mid + 1 < self.avaliable.len() - 1)
                    && self.avaliable[mid + 1].start + 1 == self.avaliable[mid].end {
                    self.avaliable[mid + 1].start = self.avaliable[mid].start;
                    self.avaliable.remove(mid);
                }
            } else {
                self.avaliable.insert(mid + 1, Range{start: index, end: index});
            }
        }
    }
}
