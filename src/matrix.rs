use crate::node;

pub struct Matrix {
    root: *mut node::Node,
}

impl Matrix {
    pub fn new() -> Self {
        todo!()
    }

    pub fn cursor(&self) -> Cursor {
        unimplemented!()
    }
    pub fn cusor_cut(&mut self) -> CursorCut {
        unimplemented!()
    }
}

pub struct Cursor<'a> {
    matrix: &'a Matrix,
    cursor: node::NodeCursor,
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
}

impl<'a> Drop for ColCut<'a> {
    fn drop(&mut self) {
        unsafe { self.cut.stitch_col() }
    }
}
