use std::fs::File;
use std::io::{BufReader, Read};

struct Colors {
    red: usize,
    green: usize,
    blue: usize,
}

fn check_game(limit: Colors, tests: &Vec<Colors>) -> bool {
    tests
        .iter()
        .all(|test| test.red <= limit.red && test.green <= limit.green && test.blue <= limit.blue)
}

fn parse_game(str: &str) -> (usize, Vec<Colors>) {
    dbg!(str);

    let vec: Vec<&str> = str.split(':').collect();

    let game_id = vec[0][5..].parse::<usize>().unwrap();
    let game = vec[1];

    let game = game
        .split(';')
        .map(|test| {
            dbg!(test);
            let colors = test
                .split(',')
                .map(|color| {
                    dbg!(color);
                    color.trim().split(' ').collect() })
                .filter(|arr: &Vec<&str>| arr.len() == 2)
                .map(|arr| (arr[0], arr[1]))
                .map(|(count, color)| (color, count.parse::<usize>().unwrap()));

            let mut red: usize = 0;
            let mut green: usize = 0;
            let mut blue: usize = 0;

            for (color, count) in colors {
                match color {
                    "red" => red = count,
                    "green" => green = count,
                    "blue" => blue = count,
                    _ => panic!("Unknown color"),
                }
            }

            Colors { red, green, blue }
        })
        .collect::<Vec<Colors>>();
    (game_id, game)
}

fn read_input() -> String {
    let a = File::open("/Users/ramil/Projects/advents/ofcode-2023/adoc-02/src/input")
        .expect("Unable to open input file");
    let mut input = String::new();
    BufReader::new(a)
        .read_to_string(&mut input)
        .expect("Unable to read input file");
    input
}

fn main() {
    let sum: usize = read_input()
        .split('\n')
        .filter(|string| !string.is_empty())
        .map(|line| parse_game(line))
        .map(|(game_id, game)| {
            let min_cubes: Colors = get_min_cubes(&game);
            let power = get_power(&min_cubes);
            (game_id, power)
        })
        // .filter(|(_, game)| {
        //     check_game(
        //         Colors {
        //             red: 12,
        //             green: 13,
        //             blue: 14,
        //         },
        //         game,
        //     )
        // })
        .map(|(game_id, power)| power).sum();

    println!("{}", sum);
}

fn get_power(p0: &Colors) -> usize {
    p0.red * p0.green * p0.blue
}

fn get_min_cubes(p0: &Vec<Colors>) -> Colors {
       let mut min_cubes = Colors {
            red: 0,
            green: 0,
            blue: 0,
        };

        for colors in p0 {
            if colors.red > min_cubes.red {
                min_cubes.red = colors.red;
            }
            if colors.green > min_cubes.green {
                min_cubes.green = colors.green;
            }
            if colors.blue > min_cubes.blue {
                min_cubes.blue = colors.blue;
            }
        }

        min_cubes
}
