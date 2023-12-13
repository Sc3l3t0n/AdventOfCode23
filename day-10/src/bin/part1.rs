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
            '7' => SouthWest,
            'F' => SouthEast,
            'S' => StartingPosition,
            _ => panic!("Unknown tile: {}", c),
        }
    }
}

impl Tile {
    fn are_connected(&self, walk_direction: Direction, other: &Self) -> bool {
        match self {
            Tile::StartingPosition => match walk_direction {
                Direction::North => {
                    matches!(other, Tile::NorthSouth | Tile::SouthEast | Tile::SouthWest)
                }
                Direction::South => {
                    matches!(other, Tile::NorthSouth | Tile::NorthEast | Tile::NorthWest)
                }
                Direction::East => {
                    matches!(other, Tile::EastWest | Tile::NorthWest | Tile::SouthWest)
                }
                Direction::West => {
                    matches!(other, Tile::EastWest | Tile::NorthEast | Tile::SouthEast)
                }
            },
            Tile::NorthSouth => match walk_direction {
                Direction::North => {
                    matches!(other, Tile::NorthSouth | Tile::SouthEast | Tile::SouthWest)
                }
                Direction::South => {
                    matches!(other, Tile::NorthSouth | Tile::NorthEast | Tile::NorthWest)
                }
                _ => false,
            },
            Tile::EastWest => match walk_direction {
                Direction::East => {
                    matches!(other, Tile::EastWest | Tile::NorthWest | Tile::SouthWest)
                }
                Direction::West => {
                    matches!(other, Tile::EastWest | Tile::NorthEast | Tile::SouthEast)
                }
                _ => false,
            },
            Tile::NorthEast => match walk_direction {
                Direction::North => {
                    matches!(other, Tile::NorthSouth | Tile::SouthWest | Tile::SouthEast)
                }
                Direction::East => {
                    matches!(other, Tile::EastWest | Tile::NorthWest | Tile::SouthWest)
                }
                _ => false,
            },
            Tile::NorthWest => match walk_direction {
                Direction::North => {
                    matches!(other, Tile::NorthSouth | Tile::SouthEast | Tile::SouthWest)
                }
                Direction::West => {
                    matches!(other, Tile::EastWest | Tile::NorthEast | Tile::SouthEast)
                }
                _ => false,
            },
            Tile::SouthEast => match walk_direction {
                Direction::South => {
                    matches!(other, Tile::NorthSouth | Tile::NorthWest | Tile::NorthEast)
                }
                Direction::East => {
                    matches!(other, Tile::EastWest | Tile::NorthWest | Tile::SouthWest)
                }
                _ => false,
            },
            Tile::SouthWest => match walk_direction {
                Direction::South => {
                    matches!(other, Tile::NorthSouth | Tile::NorthEast | Tile::NorthWest)
                }
                Direction::West => {
                    matches!(other, Tile::EastWest | Tile::NorthEast | Tile::SouthEast)
                }
                _ => false,
            },
            _ => false,
        }
    }
}

struct TileMap {
    tiles: Vec<Vec<Tile>>,
}

impl TileMap {
    fn get(&self, position: (usize, usize)) -> Option<&Tile> {
        match &self.tiles.get(position.1) {
            Some(row) => row.get(position.0),
            None => None,
        }
    }
}

struct Path {
    tiles: TileMap,
    starting_position: (usize, usize),
    current_position: (usize, usize),
    path: Vec<(usize, usize)>,
}

impl Path {
    fn with(tiles: Vec<Vec<Tile>>) -> Self {
        let starting_position = Self::get_starting_position(&tiles);
        Path {
            tiles: TileMap { tiles },
            starting_position,
            current_position: (0, 0),
            path: Vec::new(),
        }
    }
    fn setup(&mut self) {
        let (x, y) = self.starting_position;
        self.path.push((x, y));
        for direction in Direction::iter() {
            let (dx, dy) = direction.get_indices();
            let pos = (x.saturating_add_signed(dx), y.saturating_add_signed(dy));
            let tile = self.tiles.get(pos).expect("Should be there");
            if self
                .tiles
                .get(self.starting_position)
                .expect("Should be there")
                .are_connected(direction, tile)
            {
                self.current_position = pos;
                self.path.push(pos);
                break;
            }
        }
    }
    fn find_farthest_point_steps(&mut self) -> usize {
        while self.tiles.get(self.current_position).unwrap() != &Tile::StartingPosition {
            if let Some(next_pipe) = self.get_next_pipe() {
                self.current_position = next_pipe;
            } else {
                break;
            }
        }
        self.path.len() / 2
    }
    fn get_next_pipe(&mut self) -> Option<(usize, usize)> {
        let (x, y) = self.current_position;
        for direction in Direction::iter() {
            let (dx, dy) = direction.get_indices();
            let pos = (x.saturating_add_signed(dx), y.saturating_add_signed(dy));
            let tile = match self.tiles.get(pos) {
                Some(tile) => tile,
                None => continue,
            };
            let old_tile = self
                .tiles
                .get(self.current_position)
                .expect("Should be there");
            if old_tile.are_connected(direction, tile) && !self.path.contains(&pos) {
                self.path.push(pos);
                return Some(pos);
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
    path.setup();
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

