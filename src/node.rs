pub struct Node {
    x: usize,
    y: usize,

    col: *mut Node,
    row: *mut Node,

    up: *mut Node,
    left: *mut Node,
    down: *mut Node,
    right: *mut Node,
}

#[derive(Copy, Clone)]
pub struct NodeCursor(*mut Node);

impl NodeCursor {
    pub unsafe fn move_col(&mut self) {
        self.0 = (*self.0).col;
    }
    pub unsafe fn move_row(&mut self) {
        self.0 = (*self.0).row;
    }
    pub unsafe fn move_up(&mut self) {
        self.0 = (*self.0).up;
    }
    pub unsafe fn move_left(&mut self) {
        self.0 = (*self.0).left;
    }
    pub unsafe fn move_down(&mut self) {
        self.0 = (*self.0).down;
    }
    pub unsafe fn move_right(&mut self) {
        self.0 = (*self.0).right;
    }

    pub unsafe fn search_col(&mut self, col: usize) -> bool {
        unimplemented!()
    }
    pub unsafe fn search_row(&mut self, row: usize) -> bool {
        unimplemented!()
    }

    pub unsafe fn cut_col(&mut self) {
        unimplemented!()
    }
    pub unsafe fn cut_row(&mut self) {
        unimplemented!()
    }
    pub unsafe fn cut_node(&mut self) {
        unimplemented!()
    }

    pub unsafe fn stitch_col(&mut self) {
        unimplemented!()
    }
    pub unsafe fn stitch_row(&mut self) {
        unimplemented!()
    }
    pub unsafe fn stitch_node(&mut self) {
        unimplemented!()
    }

    pub unsafe fn on_col(&self) -> bool {
        unimplemented!()
    }
    pub unsafe fn on_row(&self) -> bool {
        unimplemented!();
    }
}
