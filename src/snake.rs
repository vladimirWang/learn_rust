use std::collections::{LinkedList};

#[derive(Copy, Clone, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right
}

impl Direction {
    pub fn opposite(&self) -> Direction {
        match *self {
            Direction::Down => Direction::Up,
            Direction::Up => Direction::Down,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

struct Block {
    x: i32,
    y: i32,
}

pub struct Snake {
    direction: Direction,
    body: LinkedList<Block>,
    tail: Option<Block>
} 

impl Snake {
    pub fn new (x: i32, y: i32) -> Snake {
        let body: LinkedList<Block> = LinkedList::new();

        Snake {
            direction: Direction::Right,
            body,
            tail: None
        }
    }

    fn head_position(&self) -> (i32, i32) {
        let head_pos = self.body.front().unwrap();
        (head_pos.x, head_pos.y)
    }
    pub fn head_direction(&self) -> Direction {
        self.direction
    }

    pub fn move_forward(&self, dir: Option<Direction>) {
        
    }
    pub fn next_head(&self, dir: Option<Direction>) -> (i32, i32) {
        let (head_x, head_y) = self.head_position();
        let mut moving_dir = self.direction;
        match dir {
            Some(d) => moving_dir = d,
            None => {}
        }
        match moving_dir {
            Direction::Down => (head_x, head_y + 1),
            Direction::Up => (head_x, head_y - 1),
            Direction::Left => (head_x - 1, head_y),
            Direction::Right => (head_x + 1, head_y)
        }
    }

    pub fn overlap_tail(&self, x: i32, y: i32) -> bool {
        let mut ch = 0;
        for block in &self.body {
            if x == block.x && y == block.y {
                return true;
            }
            ch += 1;
            if ch == self.body.len() - 1 {
                break;
            }
        }
        return false;
    }
}