pub struct Game {
    rolls: [int, ..21],
    current_roll: uint
}

impl Game {
    pub fn new() -> Game {
        Game { current_roll: 0, rolls: [0, ..21] }
    }

    pub fn roll(&mut self, pins: int) {
        self.rolls[self.current_roll] = pins;
        self.current_roll += 1;
    }

    pub fn score(&self) -> int {
        let mut score = 0;
        let mut frame_index = 0u;
        for frame in range(0u, 10u) {
            if self.is_strike(frame_index) {
                score += 10 + self.strike_bonus(frame_index);
                frame_index += 1;

            } else if self.is_spare(frame_index) {
                score += 10 + self.spare_bonus(frame_index);
                frame_index += 2;
            } else { 
                score += self.sum_of_balls_in_frame(frame_index);
                frame_index += 2;
            }
        }

        score
    }

    fn sum_of_balls_in_frame(&self, frame_index: uint) -> int {
        self.rolls[frame_index] + self.rolls[frame_index + 1]
    }

    fn spare_bonus(&self, frame_index: uint) -> int {
        self.rolls[frame_index + 2]
    }

    fn strike_bonus(&self, frame_index: uint) -> int {
        self.rolls[frame_index + 1] + self.rolls[frame_index + 2]
    }

    fn is_spare(&self, frame_index: uint) -> bool {
       (self.rolls[frame_index] + self.rolls[frame_index + 1]) == 10
    }

    fn is_strike(&self, frame_index: uint) -> bool {
        self.rolls[frame_index] == 10
    }
}
