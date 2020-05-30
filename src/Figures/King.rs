use crate::Figures::TraitFig::Figure;

pub struct King {
    x : i32,
    y : i32,
    fw : i32,
    fh : i32
}

impl King {
    pub fn new(cx:i32, cy:i32, w:i32, h:i32) -> Self {
        Self {
            x : cx,
            y : cy,
            fw : w,
            fh : h
        }
    }
}

impl Figure for King {
    fn danger_zone(&self) -> Vec<(i32, i32)> {
        let mut res : Vec<(i32, i32)> = Vec::new();

        if self.x > 0 {
            res.push((self.x-1, self.y));
        }
        if self.y > 0 {
            res.push((self.x, self.y-1));
        }
        if self.x < self.fw-1 {
            res.push((self.x+1, self.y));
        }
        if self.y < self.fh-1 {
            res.push((self.x, self.y+1));
        }
        if self.x > 0 && self.y > 0{
            res.push((self.x-1, self.y-1));
        }
        if self.x < self.fw-1 && self.y < self.fh-1 {
            res.push((self.x+1, self.y+1));
        }
        if self.x > 0 && self.y < self.fh-1 {
            res.push((self.x-1, self.y+1));
        }
        if self.x < self.fw-1 && self.y > 0 {
            res.push((self.x+1, self.y-1));
        }

        return res;
    }
}