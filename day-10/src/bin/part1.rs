fn main() {
    println!("Part 1");
    let input = include_str!("../../input.txt");
    let result = solve(input);
    println!("Result = {}", result);
}

#[derive(Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn get_indices(&self) -> (isize, isize) {
        use Direction::*;
        match self {
            North => (0, -1),
            South => (0, 1),
            East => (1, 0),
            West => (-1, 0),
        }
    }
    fn iter() -> impl Iterator<Item = Direction> {
        use Direction::*;
        vec![North, South, East, West].into_iter()
    }
}

#[derive(Debug, PartialEq)]
enum Tile {
    StartingPosition,
    NorthSouth,
    EastWest,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
    Ground,
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        use Tile::*;
        match c {
            '.' => Ground,
            '|' => NorthSouth,
            '-' => EastWest,
            'L' => NorthEast,
            'J' => NorthWest,
            '7' => SouthEast,
            'F' => SouthWest,
            'S' => StartingPosition,
            _ => panic!("Unknown tile: {}", c),
        }
    }
}

impl Tile {
    fn get_direction(&self) -> Direction {
        use Direction::*;
        match self {
            Tile::NorthSouth => North,
            Tile::EastWest => East,
            Tile::NorthEast => North,
            Tile::NorthWest => West,
            Tile::SouthEast => East,
            Tile::SouthWest => South,
            _ => panic!("Cannot get direction from tile: {:?}", self),
        }
    }
    fn can_connect(&self, other: Self) -> bool {
        use Tile::*;
        match self {
            NorthSouth => matches!(other, NorthSouth | NorthEast | NorthWest),
            EastWest => matches!(other, EastWest | NorthEast | SouthEast),
            NorthEast => matches!(other, NorthSouth | EastWest | NorthEast),
            NorthWest => matches!(other, NorthSouth | EastWest | NorthWest),
            SouthEast => matches!(other, NorthSouth | EastWest | SouthEast),
            SouthWest => matches!(other, NorthSouth | EastWest | SouthWest),
            StartingPosition => matches!(other, NorthSouth | EastWest | StartingPosition),
            Ground => false,
        }
    }
    fn are_connected(&self, walk_direction: Direction, other: &Self) -> bool {
        match self {
            Tile::StartingPosition => match walk_direction {
                Direction::North => {
                    matches!(other, Tile::NorthSouth | Tile::NorthEast | Tile::NorthWest)
                }
                Direction::South => {
                    matches!(other, Tile::NorthSouth | Tile::SouthEast | Tile::SouthWest)
                }
                Direction::East => {
                    matches!(other, Tile::EastWest | Tile::NorthEast | Tile::SouthEast)
                }
                Direction::West => {
                    matches!(other, Tile::EastWest | Tile::NorthWest | Tile::SouthWest)
                }
            },
            Tile::NorthSouth => match walk_direction {
                Direction::North => {
                    matches!(other, Tile::NorthSouth | Tile::NorthEast | Tile::NorthWest)
                }
                Direction::South => {
                    matches!(other, Tile::NorthSouth | Tile::SouthEast | Tile::SouthWest)
                }
                _ => false,
            },
            Tile::EastWest => match walk_direction {
                Direction::East => {
                    matches!(other, Tile::EastWest | Tile::NorthEast | Tile::SouthEast)
                }
                Direction::West => {
                    matches!(other, Tile::EastWest | Tile::NorthWest | Tile::SouthWest)
                }
                _ => false,
            },
            Tile::NorthEast => match walk_direction {
                Direction::North => {
                    matches!(other, Tile::NorthSouth | Tile::EastWest | Tile::NorthEast)
                }
                Direction::East => {
                    matches!(other, Tile::EastWest | Tile::NorthEast | Tile::SouthEast)
                }
                _ => false,
            },
            Tile::NorthWest => match walk_direction {
                Direction::North => {
                    matches!(other, Tile::NorthSouth | Tile::EastWest | Tile::NorthWest)
                }
                Direction::West => {
                    matches!(other, Tile::EastWest | Tile::NorthWest | Tile::SouthWest)
                }
                _ => false,
            },
            Tile::SouthEast => match walk_direction {
                Direction::South => {
                    matches!(other, Tile::NorthSouth | Tile::EastWest | Tile::SouthEast)
                }
                Direction::East => {
                    matches!(other, Tile::EastWest | Tile::NorthEast | Tile::SouthEast)
                }
                _ => false,
            },
            Tile::SouthWest => match walk_direction {
                Direction::South => {
                    matches!(other, Tile::NorthSouth | Tile::EastWest | Tile::SouthWest)
                }
                Direction::West => {
                    matches!(other, Tile::EastWest | Tile::NorthWest | Tile::SouthWest)
                }
                _ => false,
            },
            _ => false,
        }
    }
}

struct Path {
    tiles: Vec<Vec<Tile>>,
    starting_position: (usize, usize),
    current_position: (usize, usize),
    direction: Option<Direction>,
}

impl Path {
    fn with(tiles: Vec<Vec<Tile>>) -> Self {
        let starting_position = Self::get_starting_position(&tiles);
        Path {
            tiles,
            starting_position,
            current_position: (0, 0),
            direction: None,
        }
    }
    fn find_farthest_point_steps(&mut self) -> usize {
        let mut steps = 0;
        let (x, y) = self.starting_position;
        let mut connected_tiles = Vec::new();
        for direction in Direction::iter() {
            let (dx, dy) = direction.get_indices();
            let (x, y) = (x.saturating_add_signed(dx), y.saturating_add_signed(dy));
            let tile = &self.tiles[y][x];
            if tile.are_connected(direction, &Tile::StartingPosition) {
                connected_tiles.push((x, y));
            }
        }
        self.current_position = *connected_tiles.first().unwrap();
        while self.tiles[self.current_position.1][self.current_position.0] != Tile::StartingPosition
        {
            let next_pipe = self.get_next_pipe().unwrap();
            self.current_position = next_pipe;
            steps += 1;
        }
        if steps % 2 == 0 {
            steps / 2
        } else {
            steps / 2 + 1
        }
    }
    fn get_next_pipe(&self) -> Option<(usize, usize)> {
        let (x, y) = self.current_position;
        for direction in Direction::iter() {
            let (dx, dy) = direction.get_indices();
            let (nx, ny) = (x.saturating_add_signed(dx), y.saturating_add_signed(dy));
            let tile = &self.tiles[ny][nx];
            let old_tile = &self.tiles[y][x];
            if tile.are_connected(direction, old_tile) {
                return Some((nx, ny));
            }
        }
        None
    }
    fn get_starting_position(tiles: &[Vec<Tile>]) -> (usize, usize) {
        for (y, row) in tiles.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                if let Tile::StartingPosition = tile {
                    return (x, y);
                }
            }
        }
        panic!("No starting position found");
    }
}

fn solve(input: &str) -> String {
    let tiles = parse(input);
    let mut path = Path::with(tiles);
    let farthest_point = path.find_farthest_point_steps();
    format!("{:?}", farthest_point)
}

fn parse(input: &str) -> Vec<Vec<Tile>> {
    let mut result = Vec::new();
    for line in input.lines() {
        let mut row = Vec::new();
        for c in line.chars() {
            let tile = Tile::from(c);
            row.push(tile);
        }
        result.push(row);
    }
    result
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn example_input() {
        let input = "-L|F7
7S-7|
L|7||
-L-J|
L|-JF
";
        let output = solve(input);
        assert_eq!(output, "4".to_string());
    }

    #[test]
    fn example_input_1() {
        let input = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...
";
        let output = solve(input);
        assert_eq!(output, "8".to_string());
    }
}

