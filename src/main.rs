pub mod matrix;
mod node;

fn main() {
    let mut m = matrix::Matrix::new();
    //m.insert(2, 2);
    m.insert(1, 1);
    println!("Matrix with (1,1):");
    m.print();
    println!("Matrix with (1,1) and (3,3):");
    m.insert(3, 3);
    m.print();
    let mut cursor = m.cusor_cut();
    cursor.move_right();
    println!("{:?}", cursor.get_col());
    let cut = cursor.cut_col().expect("Cut to work");
    println!("Matrix with (1,1) and (3,3) where the 1 column is cut:");
    cut.matrix().print();
    let mut cut_cursor = cut.cursor();
    cut_cursor.move_right();
    println!("{:?}", cut_cursor.get_col());
    drop(cut);
    println!("{:?}", cursor.get_col());
}
