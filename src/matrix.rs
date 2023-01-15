use crate::node;

pub struct Matrix {
    root: *mut node::Node,
}

impl Matrix {
    pub fn new() -> Self {
        let node = node::Node::new(0, 0);
        unsafe {
            node::Node::loop_matrix(node);
        }
        Self { root: node }
    }

    pub fn print(&self) {
        let mut col_curs = self.cursor();
        print!("<-[ ]-");
        col_curs.move_right();
        while let Some(x) = col_curs.get_col() {
            print!("[{x}]-");
            col_curs.move_right();
        }
        println!(">");
        let mut row_curs = self.cursor();
        row_curs.move_down();
        while let Some(y) = row_curs.get_row() {
            print!("   |  ");
            col_curs.move_right();
            while let Some(_) = col_curs.get_col() {
                print!(" |  ");
                col_curs.move_right();
            }
            println!();
            print!("<-[{y}]-");
            row_curs.move_right();
            col_curs.move_right();
            while let Some(x) = row_curs.get_col() {
                while col_curs.get_col().expect("Every node to have a column") != x {
                    print!("-+--");
                    col_curs.move_right();
                }
                print!("({x})-");
                row_curs.move_right();
                col_curs.move_right();
            }
            while let Some(_) = col_curs.get_col() {
                print!("-+--");
                col_curs.move_right();
            }
            println!(">");
            row_curs.move_down();
        }
    }

    pub fn cursor(&self) -> Cursor {
        Cursor {
            matrix: self,
            cursor: node::NodeCursor(self.root),
        }
    }
    pub fn cusor_cut(&mut self) -> CursorCut {
        CursorCut {
            cursor: node::NodeCursor(self.root),
            matrix: self,
        }
    }

    pub fn insert_col(&mut self, col: usize) -> bool {
        let mut cursor = self.cursor();
        if cursor.search_col(col) {
            return false;
        }
        let node = node::Node::new(col, 0);
        unsafe {
            node::Node::loop_col(node, cursor.cursor.0);
        }
        true
    }
    pub fn insert_row(&mut self, row: usize) -> bool {
        let mut cursor = self.cursor();
        if cursor.search_row(row) {
            return false;
        }
        let node = node::Node::new(0, row);
        unsafe {
            node::Node::loop_row(node, cursor.cursor.0);
        }
        true
    }
    pub fn insert(&mut self, col: usize, row: usize) -> bool {
        self.insert_col(col);
        self.insert_row(row);
        let mut col_cursor = self.cursor();
        col_cursor.search_col(col);
        if col_cursor.search_row(row) {
            return false;
        }
        let mut row_cursor = self.cursor();
        row_cursor.search_row(row);
        row_cursor.search_col(col);
        let node = node::Node::new(col, row);
        unsafe {
            node::Node::loop_entry(node, row_cursor.cursor.0, col_cursor.cursor.0);
        }
        true
    }
}

pub struct Cursor<'a> {
    matrix: &'a Matrix,
    pub cursor: node::NodeCursor,
}

impl<'a> Cursor<'a> {
    pub fn move_col(&mut self) {
        unsafe { self.cursor.move_col() }
    }
    pub fn move_row(&mut self) {
        unsafe { self.cursor.move_row() }
    }
    pub fn move_up(&mut self) {
        unsafe { self.cursor.move_up() }
    }
    pub fn move_left(&mut self) {
        unsafe { self.cursor.move_left() }
    }
    pub fn move_down(&mut self) {
        unsafe { self.cursor.move_down() }
    }
    pub fn move_right(&mut self) {
        unsafe { self.cursor.move_right() }
    }

    pub fn search_col(&mut self, col: usize) -> bool {
        unsafe { self.cursor.search_col(col) }
    }
    pub fn search_row(&mut self, row: usize) -> bool {
        unsafe { self.cursor.search_row(row) }
    }

    pub fn on_col(&self) -> bool {
        unsafe { self.cursor.on_col() }
    }
    pub fn on_row(&self) -> bool {
        unsafe { self.cursor.on_row() }
    }
    pub fn get_col(&self) -> Option<usize> {
        unsafe {
            if self.cursor.on_row() {
                None
            } else {
                Some(self.cursor.get_x())
            }
        }
    }
    pub fn get_row(&self) -> Option<usize> {
        unsafe {
            if self.cursor.on_col() {
                None
            } else {
                Some(self.cursor.get_y())
            }
        }
    }

    pub fn matrix(&self) -> &Matrix {
        self.matrix
    }
}

pub struct CursorCut<'a> {
    matrix: &'a mut Matrix,
    cursor: node::NodeCursor,
}

impl<'a> CursorCut<'a> {
    pub fn move_col(&mut self) {
        unsafe { self.cursor.move_col() }
    }
    pub fn move_row(&mut self) {
        unsafe { self.cursor.move_row() }
    }
    pub fn move_up(&mut self) {
        unsafe { self.cursor.move_up() }
    }
    pub fn move_left(&mut self) {
        unsafe { self.cursor.move_left() }
    }
    pub fn move_down(&mut self) {
        unsafe { self.cursor.move_down() }
    }
    pub fn move_right(&mut self) {
        unsafe { self.cursor.move_right() }
    }

    pub fn on_col(&self) -> bool {
        unsafe { self.cursor.on_col() }
    }
    pub fn on_row(&self) -> bool {
        unsafe { self.cursor.on_row() }
    }
    pub fn get_col(&self) -> Option<usize> {
        if self.on_row() {
            None
        } else {
            unsafe { Some(self.cursor.get_x()) }
        }
    }
    pub fn get_row(&self) -> Option<usize> {
        if self.on_col() {
            None
        } else {
            unsafe { Some(self.cursor.get_y()) }
        }
    }

    pub fn cut_col(&mut self) -> Option<ColCut> {
        unsafe {
            if self.cursor.on_row() {
                return None;
            }
            self.cursor.cut_col();
        }
        Some(ColCut {
            matrix: &mut self.matrix,
            cut: &mut self.cursor,
        })
    }

    pub fn matrix(&self) -> &Matrix {
        self.matrix
    }
}

pub struct ColCut<'a> {
    matrix: &'a mut Matrix,
    cut: &'a mut node::NodeCursor,
}

impl<'a> ColCut<'a> {
    pub fn cursor(&self) -> Cursor {
        self.matrix.cursor()
    }
    pub fn cursor_cut(&mut self) -> CursorCut {
        self.matrix.cusor_cut()
    }
    pub fn matrix(&self) -> &Matrix {
        self.matrix
    }
}

impl<'a> Drop for ColCut<'a> {
    fn drop(&mut self) {
        unsafe { self.cut.stitch_col() }
    }
}

pub struct RowCut<'a> {
    matrix: &'a mut Matrix,
    cut: &'a mut node::NodeCursor,
}

impl<'a> RowCut<'a> {
    pub fn cursor(&self) -> Cursor {
        self.matrix.cursor()
    }
    pub fn cursor_cut(&mut self) -> CursorCut {
        self.matrix.cusor_cut()
    }
    pub fn matrix(&self) -> &Matrix {
        self.matrix
    }
}

impl<'a> Drop for RowCut<'a> {
    fn drop(&mut self) {
        unsafe { self.cut.stitch_row() }
    }
}
