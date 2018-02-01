use std::collections::LinkedList;
use piston_window::{Context,G2d};
use piston_window::types::Color;

use draw::draw_block;

const SNAKE_COLOR: Color = [0.0, 0.8, 0.0, 1.0];

#[derive(Copy, Clone, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn opposite(&self) -> Direction {
        match *self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

#[derive(Debug, Clone)]
struct Block {
    x: i32,
    y: i32,
}

pub struct Snake {
    direction: Direction,
    body: LinkedList<Block>,
    tail: Option<Block>,
}

impl Snake {
    pub fn new(x: i32, y: i32) -> Snake {
        let mut body: LinkedList<Block> = LinkedList::new();
        body.push_back(Block{
            x: x + 2,
            y
        });
        body.push_back(Block{
            x: x + 1,
            y
        });
        body.push_back(Block{
            x,
            y
        });
        Snake {
            direction: Direction::Right,
            body,
            tail: None,
        }
    }

    pub fn draw(&self, con: &Context, g: &mut G2d) {
        for block in &self.body {
            draw_block(SNAKE_COLOR, block.x, block.y, con, g);
        }
    }

    pub fn head_position(&self) -> (i32, i32) {
        let head_block = self.body.front().unwrap();
        (head_block.x, head_block.y)
    }

    pub fn move_forward(&mut self, dir: Option<Direction>, width: i32, height: i32) {
        match dir {
            Some(d) => self.direction = d,
            None => (),
        }
        let (last_x, last_y): (i32, i32) = self.head_position();

        let new_block = match self.direction {
            Direction::Right => Block{
                x: if last_x < width - 1 {last_x + 1} else {0},
                y: last_y,
            },
            Direction::Left => Block{
                x: if last_x > 0 {last_x - 1} else {width - 1},
                y: last_y,
            },
            Direction::Down => Block{
                x: last_x,
                y: if last_y < height - 1 {last_y + 1} else {0},
            },
            Direction::Up => Block{
                x: last_x,
                y: if last_y > 0 {last_y - 1} else {height - 1},
            },
        };
        let removed_block = self.body.pop_back().unwrap();
        self.body.push_front(new_block);
        self.tail = Some(removed_block);
    }

    pub fn head_direction(&self) -> Direction {
        self.direction // clone
    }

    pub fn next_head(&self, dir: Option<Direction>, width: i32, height: i32) -> (i32, i32) {
        let (head_x, head_y): (i32, i32) = self.head_position();

        let mut moving_dir = self.direction;
        match dir {
            Some(d) => moving_dir = d,
            None => {}
        }

        match moving_dir {
            Direction::Up => (head_x, if head_y > 0 {head_y - 1} else {height - 1}),
            Direction::Down => (head_x, if head_y < height - 1 {head_y + 1} else {0},),
            Direction::Right => (if head_x < width - 1 {head_x + 1} else {0}, head_y),
            Direction::Left => (if head_x > 0 {head_x - 1} else {width - 1}, head_y),
        }
    }

    pub fn restore_tail(&mut self) {
        let block = self.tail.clone().unwrap();
        self.body.push_back(block);
    }

    pub fn overlap_tail(&self, x:i32, y:i32) -> bool {
        let mut ch = 0;
        for block in &self.body {
            if x == block.x && y == block.y {
                return true
            }

            ch +=1;

            if ch == self.body.len() - 1 {
                break;
            }
        }
        return false;
    }
}
