use rand::prelude::*;

pub fn generate_puzzle<'a>(width: i16, height: i16, words: &'a Vec<&'a str>)  -> (Vec<Vec<char>>, Vec<&'a str>) {

    let mut puzzle = generate_empty_puzzle(width, height);
    let words_not_placed = add_words_to_puzzle(&mut puzzle, words);
    remove_empty_spots(&mut puzzle);

    (puzzle, words_not_placed)
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Direction {
    Up = 1,
    Down,
    Left,
    Right,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}

impl Direction {
    fn from_number(number: u8) -> Option<Direction> {
        match number {
            1 => Some(Direction::Up),
            2 => Some(Direction::Down),
            3 => Some(Direction::Left),
            4 => Some(Direction::Right),
            5 => Some(Direction::UpLeft),
            6 => Some(Direction::UpRight),
            7 => Some(Direction::DownLeft),
            8 => Some(Direction::DownRight),
            _ => None,
        }
    }

    fn get_next_direction(&self) -> Direction {
        Direction::from_number((*self as u8) + 1).unwrap_or_else(|| Direction::Up)
    }
}

#[cfg(test)]
mod direction_tests {
    use super::*;

    #[test]
    fn it_knows_the_next_direction() {
        assert_eq!(Direction::Up.get_next_direction(), Direction::Down);
        assert_eq!(Direction::DownRight.get_next_direction(), Direction::Up);
    }

    #[test]
    fn it_can_create_directions_from_numbers() {
        assert_eq!(Direction::from_number(1), Some(Direction::Up));
        assert_eq!(Direction::from_number(2), Some(Direction::Down));
        assert_eq!(Direction::from_number(8), Some(Direction::DownRight));
        assert_eq!(Direction::from_number(9), None);
    }
}



#[derive(Debug, PartialEq, Clone, Copy)]
struct Coordinate {
    row: i16,
    column: i16,
}



impl Coordinate {
    fn new(row: i16, column: i16) -> Coordinate {
        Coordinate {
            row,
            column,
        }
    }

    fn get_next_coordinate(&self, direction: &Direction) -> Coordinate {
        match direction {
            Direction::Up => Coordinate::new(self.row - 1, self.column),
            Direction::Down => Coordinate::new(self.row + 1, self.column),
            Direction::Left => Coordinate::new(self.row, self.column - 1),
            Direction::Right => Coordinate::new(self.row, self.column + 1),
            Direction::UpLeft => Coordinate::new(self.row - 1, self.column - 1),
            Direction::UpRight => Coordinate::new(self.row - 1, self.column + 1),
            Direction::DownLeft => Coordinate::new(self.row + 1, self.column - 1),
            Direction::DownRight => Coordinate::new(self.row + 1, self.column + 1),
        }
    }

    fn get_next_coordinate_for_size(&self, width: i16, height: i16) -> Coordinate {
        if self.column + 1 < width {
            Coordinate::new(self.row, self.column + 1)
        } else if self.row + 1 < height {
            Coordinate::new(self.row + 1, 0)
        } else {
            Coordinate::new(0, 0)
        }
    }

    fn valid(&self, width: i16, height: i16) -> bool {
        self.row >= 0 && self.row < height && self.column >= 0 && self.column < width
    }
}

#[cfg(test)]
mod coordinate_tests {
    use super::*;

    #[test]
    fn it_can_get_the_next_coordinate() {
        assert_eq!(Coordinate::new(1, 3), Coordinate::new(1, 2).get_next_coordinate(&Direction::Right));
        assert_eq!(Coordinate::new(3, 3), Coordinate::new(2, 2).get_next_coordinate(&Direction::DownRight));
    }

    #[test]
    fn it_can_get_the_next_coordinate_for_the_size() {
        assert_eq!(Coordinate::new(0, 1), Coordinate::new(0, 0).get_next_coordinate_for_size(2, 2));
        assert_eq!(Coordinate::new(1, 0), Coordinate::new(0, 1).get_next_coordinate_for_size(2, 2));
        assert_eq!(Coordinate::new(1, 1), Coordinate::new(1, 0).get_next_coordinate_for_size(2, 2));
        assert_eq!(Coordinate::new(0, 0), Coordinate::new(1, 1).get_next_coordinate_for_size(2, 2));
    }

    #[test]
    fn it_knows_if_a_coordinate_is_valid() {
        assert!(Coordinate::new(0, 0).valid(2, 2));
        assert!(Coordinate::new(1, 1).valid(2, 2));
        assert!(!Coordinate::new(2, 2).valid(2, 2));
        assert!(!Coordinate::new(-1, 0).valid(2, 2));
        assert!(!Coordinate::new(0, -1).valid(2, 2));
    }

}

pub fn generate_empty_puzzle(width: i16, height: i16) -> Vec<Vec<char>> {
    let mut puzzle: Vec<Vec<char>> = Vec::with_capacity(height as usize);
    for _ in 0..height {
        let row = vec![' '; width as usize];
        puzzle.push(row);
    }
    puzzle
}

pub fn add_words_to_puzzle<'a>(puzzle: &mut Vec<Vec<char>>, words: &'a Vec<&'a str>) -> Vec<&'a str> {
    let mut words_not_added: Vec<&str> = Vec::new();
    for word in words {
        if !add_word_to_puzzle(puzzle, &word.to_uppercase()) {
            words_not_added.push(word);

        }
    }
    words_not_added
}

pub fn remove_empty_spots(puzzle: &mut Vec<Vec<char>>) {
    for row in puzzle.iter_mut() {
        for cell in row.iter_mut() {
            if *cell == ' ' {
                *cell = generate_random_character();
            }
        }
    }
}

fn add_word_to_puzzle(puzzle: &mut Vec<Vec<char>>, word: &str) -> bool {
    let mut rng = thread_rng();
    let original_coordinate = Coordinate::new(rng.gen_range(0..puzzle.len()) as i16, rng.gen_range(0..puzzle[0].len()) as i16);
    let mut coordinate = original_coordinate;
    let original_direction = Direction::from_number(rng.gen_range(1..=8)).unwrap();
    let mut direction = original_direction;

    while !word_fits(puzzle, &coordinate, &direction, word.len()) {
        direction = direction.get_next_direction();
        if direction == original_direction {
            coordinate = coordinate.get_next_coordinate_for_size(puzzle[0].len() as i16, puzzle.len() as i16);
            if coordinate == original_coordinate {
                return false;
            }
        }
    }
    place_word(puzzle, &coordinate, &direction, word);
    true
}

fn place_word(puzzle: &mut Vec<Vec<char>>, coordinate: &Coordinate, direction: &Direction, word: &str) {
    if !word.is_empty() {
        puzzle[coordinate.row as usize][coordinate.column as usize] = word.chars().next().unwrap();
        let next_coordinate = coordinate.get_next_coordinate(direction);
        place_word(puzzle, &next_coordinate, direction, &word[1..]);
    }
}

fn word_fits(puzzle: &Vec<Vec<char>>, coordinate: &Coordinate, direction: &Direction, word_length: usize) -> bool {
    return word_length <= 0 ||
        (coordinate.valid(puzzle[0].len() as i16, puzzle.len() as i16) &&
            puzzle[coordinate.row as usize][coordinate.column as usize] == ' ' &&
            word_fits(puzzle, &coordinate.get_next_coordinate(direction), direction, word_length - 1));
}

fn generate_random_character() -> char {
    let mut rng = thread_rng();
    let random_number = rng.gen_range(0..26);
    let random_character = (random_number + 65) as u8 as char;
    random_character
}

#[cfg(test)]
mod puzzle_tests {
    use super::*;

    #[test]
    fn it_can_generate_an_empty_puzzle() {
        let puzzle = generate_empty_puzzle(10, 10);
        assert_eq!(puzzle.len(), 10);
        assert_eq!(puzzle[0].len(), 10);

        for row in puzzle.iter() {
            for cell in row.iter() {
                assert_eq!(' ', *cell);
            }
        }
    }

    #[test]
    fn it_knows_if_a_word_fits() {
        let puzzle = generate_empty_puzzle(10, 10);
        assert!(word_fits(&puzzle, &Coordinate::new(0, 0), &Direction::Right, 5));
        assert!(word_fits(&puzzle, &Coordinate::new(0, 0), &Direction::Right, 10));
        assert!(!word_fits(&puzzle, &Coordinate::new(0, 0), &Direction::Right, 11));

        assert!(word_fits(&puzzle, &Coordinate::new(0, 0), &Direction::DownRight, 1));
        assert!(word_fits(&puzzle, &Coordinate::new(0, 0), &Direction::DownRight, 10));
        assert!(word_fits(&puzzle, &Coordinate::new(0, 0), &Direction::Down, 10));

        assert!(word_fits(&puzzle, &Coordinate::new(0, 0), &Direction::DownLeft, 1));
        assert!(!word_fits(&puzzle, &Coordinate::new(0, 0), &Direction::DownLeft, 2));


        assert!(word_fits(&puzzle, &Coordinate::new(0, 0), &Direction::Left, 1));
        assert!(!word_fits(&puzzle, &Coordinate::new(0, 0), &Direction::Left, 2));

        assert!(!word_fits(&puzzle, &Coordinate::new(0, 0), &Direction::UpLeft, 2));
        assert!(!word_fits(&puzzle, &Coordinate::new(0, 0), &Direction::Up, 2));
        assert!(!word_fits(&puzzle, &Coordinate::new(0, 0), &Direction::UpRight, 2));
    }
}
