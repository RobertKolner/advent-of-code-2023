use regex::Regex;

static EXAMPLE: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

pub fn solve(input_data: Option<String>, advanced: bool) -> String {
    let data = input_data.unwrap_or(String::from(EXAMPLE)).to_string();

    let games: Vec<(u32, Vec<(u32, u32, u32)>)> = data.lines().map(parse_game).collect();

    if advanced {
        let sum_games: u32 = games.iter().map(|(_, r)| game_power(r)).sum();
        format!("{}", sum_games)
    } else {
        let sum_games: u32 = games.iter().filter_map(|(g, r)| if game_is_valid(r) { Some(g) } else { None }).sum::<u32>();
        format!("{}", sum_games)
    }
}

fn game_is_valid(reveals: &Vec<(u32, u32, u32)>) -> bool {
    reveals.iter().filter(|(r, g, b)| *r > 12 || *g > 13 || *b > 14).count() == 0
}

fn game_power(reveals: &Vec<(u32, u32, u32)>) -> u32 {
    let min_red = reveals.iter().map(|(r, _, _)| r).max().unwrap();
    let min_green = reveals.iter().map(|(_, g, _)| g).max().unwrap();
    let min_blue = reveals.iter().map(|(_, _, b)| b).max().unwrap();
    return min_red * min_green * min_blue;
}

fn parse_game(line: &str) -> (u32, Vec<(u32, u32, u32)>) {
    let game_re = Regex::new("^Game ([0-9]+):").unwrap();
    let game_number = game_re.captures(line).unwrap().get(1).unwrap().as_str().parse::<u32>().unwrap();

    let parts: Vec<&str> = line.split(":").collect();
    let reveals = parts[1].split(";").map(parse_reveal);
    return (game_number, reveals.collect());
}

fn parse_reveal(reveal: &str) -> (u32, u32, u32) {
    let red_re = Regex::new("([0-9]+) red").unwrap();
    let green_re = Regex::new("([0-9]+) green").unwrap();
    let blue_re = Regex::new("([0-9]+) blue").unwrap();

    let capture = |re: Regex| -> u32 {
        re.captures_iter(reveal)
            .map(|c| c
                .get(1)
                .unwrap()
                .as_str()
                .parse::<u32>().unwrap())
            .sum()
    };

    let red = capture(red_re);
    let green = capture(green_re);
    let blue = capture(blue_re);
    return (red, green, blue);
}