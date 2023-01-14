use std::ptr;

#[derive(Debug)]
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

impl Node {
    pub fn new(x: usize, y: usize) -> *mut Self {
        Box::into_raw(Box::new(Node {
            x,
            y,

            col: ptr::null_mut(),
            row: ptr::null_mut(),

            up: ptr::null_mut(),
            left: ptr::null_mut(),
            down: ptr::null_mut(),
            right: ptr::null_mut(),
        }))
    }

    pub unsafe fn link_horizontal(left: *mut Node, right: *mut Node) {
        (*left).right = right;
        (*right).left = left;
    }
    pub unsafe fn link_vertical(up: *mut Node, down: *mut Node) {
        (*up).down = down;
        (*down).up = up;
    }

    pub unsafe fn loop_matrix(node: *mut Node) {
        (*node).col = node;
        (*node).row = node;
        Self::link_horizontal(node, node);
        Self::link_vertical(node, node);
    }
    pub unsafe fn loop_col(node: *mut Node, right: *mut Node) {
        (*node).col = node;
        (*node).row = (*right).row;
        Self::link_horizontal((*right).left, node);
        Self::link_horizontal(node, right);
        Self::link_vertical(node, node);
    }
}

#[derive(Copy, Clone)]
pub struct NodeCursor(pub *mut Node);

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
        self.move_row();
        self.move_right();
        while !self.on_row() {
            let x = self.get_x();
            if col == x {
                return true;
            }
            if col < x {
                return false;
            }
        }
        false
    }
    pub unsafe fn search_row(&mut self, row: usize) -> bool {
        self.move_col();
        self.move_down();
        while !self.on_col() {
            let y = self.get_y();
            if row == y {
                return true;
            }
            if row < y {
                return false;
            }
        }
        false
    }

    pub unsafe fn cut_col(&mut self) {
        Node::link_horizontal((*self.0).left, (*self.0).right);
        let start = self.0;
        self.move_down();
        while start != self.0 {
            Node::link_horizontal((*self.0).left, (*self.0).right);
            self.move_down();
        }
    }
    pub unsafe fn cut_row(&mut self) {
        todo!()
    }
    pub unsafe fn cut_node(&mut self) {
        todo!()
    }

    pub unsafe fn stitch_col(&mut self) {
        Node::link_horizontal((*self.0).left, self.0);
        Node::link_horizontal(self.0, (*self.0).right);
        let start = self.0;
        self.move_down();
        while start != self.0 {
            Node::link_horizontal((*self.0).left, self.0);
            Node::link_horizontal(self.0, (*self.0).right);
            self.move_down();
        }
    }
    pub unsafe fn stitch_row(&mut self) {
        todo!()
    }
    pub unsafe fn stitch_node(&mut self) {
        todo!()
    }

    pub unsafe fn on_col(&self) -> bool {
        self.0 == (*self.0).col
    }
    pub unsafe fn on_row(&self) -> bool {
        self.0 == (*self.0).row
    }

    pub unsafe fn get_x(&self) -> usize {
        (*self.0).x
    }
    pub unsafe fn get_y(&self) -> usize {
        (*self.0).y
    }
}
