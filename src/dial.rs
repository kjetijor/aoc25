pub struct Dial {
    pub position: u64,
    size: u64,
    pub zero_hits: u64,
}

pub enum DialError {
    InvalidCommand(String),
}

impl std::fmt::Debug for DialError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DialError::InvalidCommand(msg) => write!(f, "DialError::InvalidCommand({})", msg),
        }
    }
}

impl Dial {
    pub fn do_move(&mut self, movement: &str) -> Result<(), DialError> {
        if movement.is_empty() {
            return Err(DialError::InvalidCommand("Empty command".to_string()));
        }
        let lr = movement.chars().next().unwrap();
        let distance = movement[1..].parse::<u64>().map_err(|_| {
            DialError::InvalidCommand(format!("Invalid distance in command: {}", movement))
        })?;
        if lr == 'L' {
            self.left(distance);
        } else if lr == 'R' {
            self.right(distance);
        } else {
            return Err(DialError::InvalidCommand(format!(
                "Invalid direction in command: {}",
                movement
            )));
        }
        Ok(())
    }

    pub fn new(size: u64, position: u64) -> Self {
        Self { size, position, zero_hits: 0 }
    }

    pub fn left(&mut self, n: u64) {
        self.zero_hits += n / self.size;
        let (newpos, overflowed) = self.position.overflowing_sub(n % self.size);
        if overflowed {
            self.position = newpos.overflowing_add(self.size).0;
            self.zero_hits += 1;
        } else {
            self.position = newpos;
        }
//        self.position = newpos % self.size;
    }

    pub fn right(&mut self, n: u64) {
        let truncated = n % self.size;
        self.zero_hits += n / self.size;
        if self.position + truncated >= self.size {
            self.zero_hits += 1;
        }
        self.position = (self.position + n) % self.size;
    }

    pub fn set(&mut self, position: u64) {
        self.position = position % self.size;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_testcase_zero_passes() {
        /*
The dial starts by pointing at 50.
The dial is rotated L68 to point at 82; during this rotation, it points at 0 once.
The dial is rotated L30 to point at 52.
The dial is rotated R48 to point at 0.
The dial is rotated L5 to point at 95.
The dial is rotated R60 to point at 55; during this rotation, it points at 0 once.
The dial is rotated L55 to point at 0.
The dial is rotated L1 to point at 99.
The dial is rotated L99 to point at 0.
The dial is rotated R14 to point at 14.
The dial is rotated L82 to point at 32; during this rotation, it points at 0 once.

// FIXME: multiple revolutions ?
*/
        struct TestCase {
            movement: &'static str,
            expected_pos: u64,
            expected_zero_hits: u64,
        }
        let moves = vec![
            TestCase { movement: "L68", expected_pos: 82, expected_zero_hits: 1 },
            TestCase { movement: "L30", expected_pos: 52, expected_zero_hits: 1 },
            TestCase { movement: "R48", expected_pos: 0, expected_zero_hits: 2 },
            TestCase { movement: "L5", expected_pos: 95, expected_zero_hits: 2 },
        ];
        let mut dial = Dial::new(100, 50);
        for tc in moves {
            let startpos = dial.position;
            let start_zero_hits = dial.zero_hits;
            
            dial.do_move(tc.movement).unwrap();
            assert_eq!(dial.position, tc.expected_pos, "Failed position for move {}", tc.movement);
            assert_eq!(dial.zero_hits, tc.expected_zero_hits, "Failed zero hits for move {}-{}->{} zp {} -> {}", startpos, tc.movement, dial.position, start_zero_hits, dial.zero_hits);
        }

    }

    #[test]
    fn test_zero_passes() {
        enum Move {
            Left(u64),
            Right(u64),
        }
        struct TestCase {
            startpos: u64,
            m: Move,
            expected_pos: u64,
            expected_zero_passes: u64,
            desc: &'static str,
        }
        let test_cases = vec![
            TestCase { startpos: 0, m: Move::Right(10), expected_pos: 0, expected_zero_passes: 1, desc: "Right full circle" },
            TestCase { startpos: 5, m: Move::Right(7), expected_pos: 2, expected_zero_passes: 1, desc: "Right with wrap" },
            TestCase { startpos: 3, m: Move::Left(4), expected_pos: 9, expected_zero_passes: 1, desc: "Left with wrap" },
            TestCase { startpos: 0, m: Move::Left(20), expected_pos: 0, expected_zero_passes: 2, desc: "Left two full circles" },
            TestCase { startpos: 0, m: Move::Right(20), expected_pos: 0, expected_zero_passes: 2, desc: "Right two full circles" },
            TestCase { startpos: 8, m: Move::Right(25), expected_pos: 3, expected_zero_passes: 3, desc: "Right over two circles" },
            TestCase { startpos: 2, m: Move::Left(25), expected_pos: 7, expected_zero_passes: 3, desc: "Left over two circles" },
        ];
        for tc in test_cases {
            let mut dial = Dial::new(10, tc.startpos);
            match tc.m {
                Move::Left(n) => dial.left(n),
                Move::Right(n) => dial.right(n),
            }
            assert_eq!(dial.position, tc.expected_pos, "Failed position for {}", tc.desc);
            assert_eq!(dial.zero_hits, tc.expected_zero_passes, "Failed zero passes for {}", tc.desc);
        }
    }

    #[test]
    fn test_dial_command() {
        let mut dial = Dial::new(20, 0);
        assert_eq!(dial.position, 0);
        dial.do_move("R5").unwrap();
        assert_eq!(dial.position, 5);
        dial.do_move("L3").unwrap();
        assert_eq!(dial.position, 2);
        dial.do_move("R18").unwrap();
        assert_eq!(dial.position, 0);
        let err = dial.do_move("X10").unwrap_err();
        match err {
            DialError::InvalidCommand(msg) => {
                assert_eq!(msg, "Invalid direction in command: X10");
            }
        }
        let err = dial.do_move("Rabc").unwrap_err();
        match err {
            DialError::InvalidCommand(msg) => {
                assert_eq!(msg, "Invalid distance in command: Rabc");
            }
        }
    }

    #[test]
    fn dial_given_example() {
        let mut dial = Dial::new(100, 50);
        struct TestCase {
            movement: &'static str,
            expected_pos: u64,
        }
        let moves = vec![
            TestCase { movement: "L68", expected_pos: 82 },
            TestCase { movement: "L30", expected_pos: 52 },
            TestCase { movement: "R48", expected_pos: 0 },
            TestCase { movement: "L5", expected_pos: 95 },
            TestCase { movement: "R60", expected_pos: 55 },
            TestCase { movement: "L55", expected_pos: 0 },
            TestCase { movement: "L1", expected_pos: 99 },
            TestCase { movement: "L99", expected_pos: 0 },
            TestCase { movement: "R14", expected_pos: 14 },
            TestCase { movement: "L82", expected_pos: 32 },
        ];
        for tc in moves {
            let curr_pos = dial.position;
            dial.do_move(tc.movement).unwrap();
            assert_eq!(dial.position, tc.expected_pos, "Failed position for move {} from {} to {}", tc.movement, curr_pos, tc.expected_pos);
        }
    }

    #[test]
    fn test_dial_movement() {
        let mut dial = Dial::new(10, 0);
        dial.right(3);
        assert_eq!(dial.position, 3);
        dial.left(4);
        assert_eq!(dial.position, 9);
        dial.right(12);
        assert_eq!(dial.position, 1);
        dial.left(11);
        assert_eq!(dial.position, 0);
    }
}
