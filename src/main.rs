use clap::Parser;
use dotenv;
use tokio;

mod aoc;
mod solutions;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(required = true)]
    day: u8,

    #[arg(long)]
    adv: bool,

    #[arg(long)]
    solve: bool,
}
#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let aoc_session = std::env::var("AOC_SESSION").unwrap_or(String::new());

    let args = Args::parse();
    let mut input_data: Option<String> = None;
    if args.solve {
        let data = aoc::datafiles::load_data(2023, args.day, aoc_session)
            .await
            .unwrap();
        input_data = Some(data);
    }

    let solution = solutions::solver::solve_for_day(args.day, input_data, args.adv);

    println!("Solution: {}", solution);
}