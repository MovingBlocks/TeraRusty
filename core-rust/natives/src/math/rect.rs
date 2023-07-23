
#[derive(Copy, Clone, PartialEq)]
pub struct Rect {
    pub min: [f32; 2], 
    pub max: [f32; 2],
}

impl Rect {
    pub fn size(&self) -> [f32; 2] {
        [
            self.max[0] - self.min[0],
            self.max[1] - self.min[1]
        ]
    }
   
    pub fn zero() -> Self{
        Rect {
            min: [0.0,0.0],
            max: [0.0,0.0]
        }
    }

    pub fn intersect(&self, rect: &Rect) -> bool{
        return self.min[0] < rect.max[0] && self.max[0] > rect.min[0] &&
                self.max[1] > rect.min[1] && self.min[1] < rect.max[1];
    }
    
    pub fn combine(&self, other: &Rect) -> Rect {
        let mut result = Rect::zero();
        result.min[0] = if self.min[0] < other.min[0] { self.min[0] } else { other.min[0]};
        result.min[1] = if self.min[1] < other.min[1] { self.min[1] } else { other.min[1]};
        result.max[0] = if self.max[0] > other.max[0] { self.max[0] } else { other.max[0]};
        result.max[1] = if self.max[1] > other.max[1] { self.max[1] } else { other.max[1]};
        return result;
    }

}
