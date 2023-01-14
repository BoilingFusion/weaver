pub mod matrix;
mod node;

fn main() {
    let mut m = matrix::Matrix::new();
    let mut cursor = m.cusor_cut();
    let cut = cursor.cut_col().expect("Test");
    let mut cut_cursor = cut.cursor();
    cut_cursor.move_down();
    drop(cut);
    cursor.move_down();
}
