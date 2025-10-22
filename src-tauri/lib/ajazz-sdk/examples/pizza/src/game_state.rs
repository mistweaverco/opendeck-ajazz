pub const MOVE_RIGHT: i8 = 1;
pub const MOVE_LEFT: i8 = -1;

#[derive(Clone)]
pub struct GameState {
    pub pizza_position: u8,
    pub food_positions: Vec<bool>,
    pub direction: i8,
    pub pizza_eating: bool,
    previous_pizza_position: u8,
    previous_pizza_eating: bool,
    previous_direction: i8,
}

impl GameState {
    pub fn new(display_key_count: u8) -> Self {
        let mut food_positions = vec![false; display_key_count as usize];

        // Fill all positions except first (where pizza starts) with food
        for i in 1..display_key_count {
            food_positions[i as usize] = true;
        }

        Self {
            pizza_position: 0,
            food_positions,
            direction: MOVE_RIGHT,
            pizza_eating: false,
            previous_pizza_position: 0,
            previous_pizza_eating: false,
            previous_direction: MOVE_RIGHT,
        }
    }

    pub fn move_pizza(&mut self, display_key_count: u8) {
        self.previous_pizza_position = self.pizza_position;

        self.pizza_position = match self.direction {
            MOVE_RIGHT => {
                if self.pizza_position == display_key_count - 1 {
                    0 // Wrap around
                } else {
                    self.pizza_position + 1
                }
            }
            MOVE_LEFT => {
                if self.pizza_position == 0 {
                    display_key_count - 1 // Wrap around
                } else {
                    self.pizza_position - 1
                }
            }
            _ => self.pizza_position, // Invalid direction, stay in place
        };
    }

    pub fn set_direction(&mut self, direction: i8) {
        self.previous_direction = self.direction;
        self.direction = direction;
    }

    pub fn set_eating_state(&mut self, eating: bool) {
        self.previous_pizza_eating = self.pizza_eating;
        self.pizza_eating = eating;
    }

    pub fn add_food(&mut self, position: u8) {
        if let Some(pos) = self.food_positions.get_mut(position as usize) {
            *pos = true;
        }
    }

    pub fn eat_food(&mut self, position: u8) -> bool {
        if let Some(pos) = self.food_positions.get_mut(position as usize) {
            if *pos {
                *pos = false;
                return true;
            }
        }
        false
    }

    pub fn has_food(&self, position: u8) -> bool {
        self.food_positions
            .get(position as usize)
            .copied()
            .unwrap_or(false)
    }

    pub fn has_state_changed(&self) -> bool {
        self.previous_pizza_position != self.pizza_position
            || self.previous_pizza_eating != self.pizza_eating
            || self.previous_direction != self.direction
    }

    pub fn position_changed(&self) -> bool {
        self.previous_pizza_position != self.pizza_position
    }

    pub fn get_previous_position(&self) -> u8 {
        self.previous_pizza_position
    }
}
