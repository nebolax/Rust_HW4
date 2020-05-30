use crate::Figures::TraitFig::Figure;

pub struct Field {
    data : Vec<Vec<i32>>,
    figs: Vec<(i32, i32, Vec<(i32, i32)>)>
}

impl Field {
    pub fn new() -> Self {
        let mut res = Vec::new();
        for i in 0..8 {
            let to_add: Vec<i32> = Vec::new();
            res.push(to_add);
            for a in 0..8 {
                res[i].push(0);
            }
        }
        Self {
            data: res,
            figs: Vec::new()
        }
    }
    pub fn add_figure<T: Figure>(&mut self, fig: T, x: i32, y: i32) {
        self.figs.push((x, y, fig.danger_zone()));
    }
    fn sum_danger_zones(&self) -> Vec<(i32, i32)> {
        let mut res: Vec<(i32, i32)> = Vec::new();
        for el in self.figs.clone() {
            let cur = el.2;
            for el2 in cur {
                res.push(el2);
            }
        }
        res
    }
    fn map_dangers(&self) -> Vec<Vec<bool>> {
        let zone = self.sum_danger_zones();
        let mut res: Vec<Vec<bool>> = Vec::new();
        for i in 0..8 {
            let to_add: Vec<bool> = Vec::new();
            res.push(to_add);
            for a in 0..8 {
                res[i].push(false);
            }
        }
        for el in zone {
            res[el.0 as usize][el.1 as usize] = true;
        }

        res
    }
    pub fn show_zone(&self) {
        let zone = self.map_dangers();

        println!("Зона, которая бьется фигурами:");
        for el in zone {
            for el2 in el {
                print!("{} ", el2 as i32);
            }
            println!();
        }
    }
    pub fn show_figs(&self) {
        println!("Расположение фиугр:");
        let mut res = Vec::new();
        for i in 0..8 {
            res.push(Vec::new());
            for a in 0..8 {
                res[i].push(0);
            }
        }
        for el in self.figs.clone() {
            res[el.0 as usize][el.1 as usize] = 2;
        }
        for i in 0..8 {
            for a in 0..8 {
                print!("{} ", res[i][a]);
            }
            println!();
        }
    }
    pub fn show_errors(&self) {
        let mut res: Vec<(i32, i32)> = Vec::new();
        let zone = self.map_dangers();
        for el in self.figs.clone() {
            if zone[el.0 as usize][el.1 as usize] == true {
                res.push((el.0, el.1));
            }
        }

        if res.len() == 0 {
            println!("Все хорошо, никакие фигуры не бьются.");
        } else {
            println!("Фигуры в этих позициях бьются (X, Y):");
            for el in res {
                println!("{}, {}", el.0, el.1);
            }
        }
    }
}