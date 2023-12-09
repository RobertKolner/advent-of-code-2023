const EXAMPLE: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

pub fn solve(input_data: Option<String>, advanced: bool) -> String {
    let data = input_data.unwrap_or(String::from(EXAMPLE)).to_string();
    let inputs: Vec<Vec<i64>> = data
        .lines()
        .map(|l| l
            .split_whitespace()
            .map(|n| n
                .parse::<i64>()
                .unwrap())
            .collect::<Vec<i64>>())
        .collect();

    let result: i64 = inputs.iter().map(|i| extrapolate(i, advanced)).sum();
    format!("{}", result)
}

fn extrapolate(series: &Vec<i64>, advanced: bool) -> i64 {
    if series.len() == 0 || series.iter().all(|v| *v == 0) {
        return 0;
    }

    let diffs = differentiate(series);
    return if advanced {
        series[0] - extrapolate(&diffs, advanced)
    } else {
        series[series.len() - 1] + extrapolate(&diffs, advanced)
    }
}

fn differentiate(series: &Vec<i64>) -> Vec<i64> {
    return if series.len() < 2 {
        vec![]
    } else {
        let chunked: Vec<(i64, i64)> = series.iter().zip(series.iter().skip(1)).map(|(a, b)| (*a, *b)).collect();
        chunked.iter().map(|(a, b)| *b - *a).collect()
    }
}