use smallvec;

pub struct Range {
    start: u32,
    end: u32
}

pub struct IDPool {
    used: u32,
    free: Vec<Range>
}

impl IDPool {
    pub fn fetch_id(&mut self) -> u32 {
        match self.free.last_mut() {
            Some(range) => {
                let id = range.start;
                range.start += 1;
                if range.start > range.end {
                    self.free.pop();
                }
                id
            },
            None => {
                let id = self.used;
                self.used += 1;
                id 
            }
        }

    }


    pub fn return_id(&mut self, id: u32)  {
        if self.free.is_empty() {
            self.free.push(Range { start: id, end: id})
        }

        match self.free.binary_search_by(|probe| {
            if id >= probe.start && id <= probe.end {
                std::cmp::Ordering::Equal
            } else {
                probe.start.cmp(&id)
            }
        }) {
            Ok(_index) => {
                panic!("id returned multiple times to the pool");
            },
            Err(index) => {
                let current_range = &mut self.free[index]; 
                if id + 1 == current_range.start {
                    current_range.start -= 1;
                    if index > 0  {
                        let update_end = current_range.end;
                        let previous_range = &mut self.free[index - 1];
                        if previous_range.end + 1 == id {
                            previous_range.end = update_end;
                            self.free.remove(index);
                        }
                    }
                }



            }
        }

    }
}
