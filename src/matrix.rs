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

    pub fn cursor(&self) -> Cursor {
        Cursor {
            _matrix: self,
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
        todo!()
    }
}

pub struct Cursor<'a> {
    _matrix: &'a Matrix,
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

    pub fn search_col(&mut self, col: usize) -> bool {
        unsafe { self.cursor.search_col(col) }
    }
    pub fn search_row(&mut self, row: usize) -> bool {
        todo!()
    }

    pub fn on_col(&self) -> bool {
        todo!()
    }
    pub fn on_row(&self) -> bool {
        todo!()
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
}

impl<'a> Drop for RowCut<'a> {
    fn drop(&mut self) {
        unsafe { self.cut.stitch_row() }
    }
}
