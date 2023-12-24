use nom::branch::alt;
use nom::bytes::complete::{tag, take_until};
use nom::character::complete::{digit1, multispace0};
use nom::combinator::{map_res, opt, rest};
use nom::multi::many1;
use nom::sequence::{delimited, preceded};
use nom::IResult;
use std::fs::File;
use std::io::{prelude::*, BufReader};

#[derive(Debug)]
struct BagState {
    game: usize,
    draws: Vec<Draws>,
}

impl BagState {
    fn new(game: usize, draws: Vec<Draws>) -> BagState {
        BagState { game, draws }
    }
}

#[derive(Debug)]
struct Draws {
    red: usize,
    green: usize,
    blue: usize,
}

impl Draws {
    fn new(red: usize, green: usize, blue: usize) -> Draws {
        Draws { red, green, blue }
    }

    fn from_vec(list: Vec<Color>) -> Draws {
        let mut red = 0;
        let mut blue = 0;
        let mut green = 0;
        for x in list.iter() {
            match x {
                Color::Red(val) => red = *val,
                Color::Green(val) => green = *val,
                Color::Blue(val) => blue = *val,
            }
        }
        Draws { red, green, blue }
    }
}

enum Color {
    Red(usize),
    Green(usize),
    Blue(usize),
}

impl Color {
    fn from_str(input: &str, num: usize) -> Result<Color, String> {
        match input {
            "red" => Ok(Color::Red(num)),
            "blue" => Ok(Color::Blue(num)),
            "green" => Ok(Color::Green(num)),
            _ => Err("booo".to_string()),
        }
    }
}

fn main() {
    let init_state = Draws::new(12, 13, 14);
    let file = File::open("data.txt").unwrap();
    let reader = BufReader::new(file);
    let mut states: Vec<BagState> = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        let (_remain, state) = parse_line(&line).unwrap();
        states.push(state)
    }
    let mut sum = 0;
    for x in states {
        // println!("{:?}", x);
        match compare_state(&init_state, &x) {
            Some(x) => sum += x,
            None => (),
        }
    }
    println!("Total: {}", sum);
}

fn compare_state(init: &Draws, second: &BagState) -> Option<usize> {
    for game in second.draws.iter() {
        // compare
        if !(game.blue <= init.blue && game.green <= init.green && game.red <= init.red) {
            return None;
        }
    }
    Some(second.game)
}

fn parse_line(line: &str) -> IResult<&str, BagState> {
    // strat is to parse game num, then parse the game results multiple times
    let (input, game_num) = parse_game_num(line)?;
    let (input, games) = many1(parse_set)(input)?;
    let bag = BagState::new(game_num, games);
    Ok((input, bag))
}

fn parse_game_num(input: &str) -> IResult<&str, usize> {
    delimited(
        tag("Game "),
        map_res(digit1, |x: &str| x.parse::<usize>()),
        tag(": "),
    )(input)
}

fn parse_set(input: &str) -> IResult<&str, Draws> {
    let (input, set) = alt((take_until(";"), rest))(input)?;
    let (input, _) = opt(tag("; "))(input)?;
    let (_remain, list) = many1(parse_draw)(set)?;
    let draw = Draws::from_vec(list);
    Ok((input, draw))
}

fn parse_draw(input: &str) -> IResult<&str, Color> {
    // println!("{:?}", input);
    let (input, draw) = alt((take_until(","), rest))(input)?;
    let (input, _) = opt(tag(","))(input)?;
    let (remain, digit) =
        preceded(multispace0, map_res(digit1, |x: &str| x.parse::<usize>()))(draw)?;
    let (remain, _) = multispace0(remain)?;
    let (_, color) = map_res(rest, |s: &str| Color::from_str(s, digit))(remain)?;
    Ok((input, color))
}
