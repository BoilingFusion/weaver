pub mod matrix;
mod node;

fn main() {
    let mut m = matrix::Matrix::new();
    m.insert_col(1);
    let mut cursor = m.cusor_cut();
    cursor.move_right();
    println!("{:?}", cursor.get_col());
    let cut = cursor.cut_col().expect("Cut failed");
    let mut cut_cursor = cut.cursor();
    cut_cursor.move_right();
    println!("{:?}", cut_cursor.get_col());
    drop(cut);
    println!("{:?}", cursor.get_col());
}
