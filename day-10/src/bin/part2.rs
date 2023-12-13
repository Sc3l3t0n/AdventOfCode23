// Special Thanks to "Bewelge" for the Idea to skip certain Pipes
// https://www.reddit.com/r/adventofcode/comments/18evyu9/comment/kcsal0o/?utm_source=share&utm_medium=web3x&utm_name=web3xcss&utm_term=1&utm_content=share_button

fn main() {
    println!("Part 2");
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

#[derive(Debug, PartialEq, Clone, Copy)]
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
    fn find_path(&mut self) {
        while self.tiles.get(self.current_position).unwrap() != &Tile::StartingPosition {
            if let Some(next_pipe) = self.get_next_pipe() {
                self.current_position = next_pipe;
            } else {
                break;
            }
        }
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

enum Mode {
    Within,
    Out,
}

#[derive(Debug, Clone, Default, PartialEq)]
enum AreaType {
    Pipe {
        tile_type: Tile,
    },
    Enclosed,
    Outerspace,
    #[default]
    Unknown,
}

struct EncloseSearcher {
    tile_map: TileMap,
    path: Vec<(usize, usize)>,
    enclosing_map: Vec<Vec<AreaType>>,
    mode: Mode,
}

impl EncloseSearcher {
    fn with(tiles: Vec<Vec<Tile>>, path: Vec<(usize, usize)>) -> Self {
        let enclosing_map = vec![vec![AreaType::Unknown; tiles[0].len()]; tiles.len()];
        EncloseSearcher {
            tile_map: TileMap { tiles },
            path,
            enclosing_map,
            mode: Mode::Out,
        }
    }
    fn find_enclosed_areas(&mut self) -> u32 {
        let mut result = 0;
        self.fill_enclosing_map();
        for row in self.enclosing_map.iter_mut() {
            for tile in row {
                if tile == &mut AreaType::Enclosed {
                    result += 1;
                }
            }
        }
        result
    }
    fn fill_enclosing_map(&mut self) {
        use AreaType::*;
        use Mode::*;
        use Tile::*;

        for (x, y) in self.path.iter() {
            self.enclosing_map[*y][*x] = Pipe {
                tile_type: self.tile_map.tiles[*y][*x],
            };
        }

        for row in self.enclosing_map.iter_mut() {
            for tile in row.iter_mut() {
                match tile {
                    Unknown => match self.mode {
                        Within => *tile = Enclosed,
                        Out => *tile = Outerspace,
                    },
                    Pipe { tile_type } => match tile_type {
                        NorthSouth | SouthWest | SouthEast | StartingPosition => match self.mode {
                            Within => self.mode = Out,
                            Out => self.mode = Within,
                        },
                        _ => {}
                    },
                    _ => {}
                }
            }
            self.mode = Out;
        }
    }
    #[allow(dead_code)]
    fn print(&self) {
        for row in &self.enclosing_map {
            for tile in row {
                print!("|{:?}", tile)
            }
            println!("|");
        }
    }
}

fn solve(input: &str) -> String {
    let tiles = parse(input);
    let mut path = Path::with(tiles);
    path.setup();
    path.find_path();
    let mut enclose_searcher = EncloseSearcher::with(path.tiles.tiles, path.path);
    let enclosed_areas = enclose_searcher.find_enclosed_areas();
    //enclose_searcher.print();
    enclosed_areas.to_string()
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
        let input = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........
";

        let output = solve(input);
        assert_eq!(output, "4".to_string());
    }

    #[test]
    fn example_input2() {
        let input = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";
        let output = solve(input);
        assert_eq!(output, "8".to_string());
    }

    #[test]
    fn example_input3() {
        let input = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";
        let output = solve(input);
        assert_eq!(output, "10".to_string());
    }
}

