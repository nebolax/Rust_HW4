use crate::Figures::TraitFig::Figure;
use text_io::read;
mod Figures;
mod Field;

fn read_pos() -> (i32, i32) {
    let inp: String = read!();
    let x = inp.chars()
        .next()
        .unwrap()
        .to_string()
        .parse()
        .unwrap();
    let y = inp.chars()
        .nth(2)
        .unwrap()
        .to_string()
        .parse()
        .unwrap();
    (x, y)
}

fn main() {
    let fw = 8;
    let fh = 8;
    let mut field = Field::Field::new();

    println!("Кол-во королей: ");
    let much_kings: i32 = read!("{}\n");
    println!("Кол-во ферзей: ");
    let much_ferz: i32 = read!("{}\n");
    println!("Кол-во ладей: ");
    let much_lads: i32 = read!("{}\n");

    let mut kings_pos: Vec<(i32, i32)> = Vec::new();
    let mut ferz_pos: Vec<(i32, i32)> = Vec::new();
    let mut lads_pos: Vec<(i32, i32)> = Vec::new();

    println!("Введите позиции королей, формат: x-y\\n");
    for i in 0..much_kings {
        let pos = read_pos();
        field.add_figure(Figures::King::King::new(pos.1, pos.0, fw, fh), pos.1, pos.0);
    }
    println!("Введите позиции ферзей, формат: x-y\\n");
    for i in 0..much_ferz {
        let pos = read_pos();
        field.add_figure(Figures::Ferzy::Ferzy::new(pos.1, pos.0, fw, fh), pos.1, pos.0);
    }
    println!("Введите позиции ладей, формат: x-y\\n");
    for i in 0..much_lads {
        let pos = read_pos();
        field.add_figure(Figures::Ladya::Ladya::new(pos.1, pos.0, fw, fh), pos.1, pos.0);
    }
    field.show_zone();
    println!();
    field.show_figs();
    println!();
    field.show_errors();
}