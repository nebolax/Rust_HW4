use crate::Figures::TraitFig::Figure;

pub struct Ferzy {
    pub x : i32,
    pub y : i32,
    pub fw : i32,
    pub fh : i32
}

impl Figure for Ferzy {
    fn danger_zone(&self) -> Vec<(i32, i32)> {
        let mut res : Vec<(i32, i32)> = Vec::new();
        let mut cx = self.x;
        let mut cy = self.y;

        while cx-1 >= 0 {
            cx -= 1;
            res.push((cx, cy));
        }
        cx = self.x;
        cy = self.y;
        while cx+1 < self.fw {
            cx += 1;
            res.push((cx, cy));
        }
        cx = self.x;
        cy = self.y;
        while cy-1 >= 0 {
            cy -= 1;
            res.push((cx, cy));
        }
        cx = self.x;
        cy = self.y;
        while cy+1 < self.fh {
            cy += 1;
            res.push((cx, cy));
        }
        cx = self.x;
        cy = self.y;
        while cx-1 >= 0 && cy-1 >= 0 {
            cx -= 1;
            cy -= 1;
            res.push((cx, cy))
        }
        cx = self.x;
        cy = self.y;
        while cx+1 < self.fw && cy+1 < self.fh {
            cx += 1;
            cy += 1;
            res.push((cx, cy))
        }
        cx = self.x;
        cy = self.y;
        while cx+1 < self.fh && cy-1 >= 0 {
            cx += 1;
            cy -= 1;
            res.push((cx, cy))
        }
        cx = self.x;
        cy = self.y;
        while cx-1 >= 0 && cy+1 < self.fh {
            cx -= 1;
            cy += 1;
            res.push((cx, cy))
        }

        return res;
    }
}

impl Ferzy {
    pub fn new(cx:i32, cy:i32, w:i32, h:i32) -> Self {
        Self {
            x : cx,
            y : cy,
            fw : w,
            fh : h
        }
    }
}

