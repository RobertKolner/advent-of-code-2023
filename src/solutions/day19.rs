use std::{cmp, fmt};
use std::collections::HashMap;
use std::fmt::Formatter;
use nom::bytes::complete::{is_not, is_a, take_while1};
use nom::AsChar;
use nom::character::complete::{char, one_of};
use nom::combinator::opt;
use nom::IResult;
use nom::multi::{many0, many1};
use nom::sequence::{delimited};

const EXAMPLE: &str = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";

#[derive(Debug, Clone, Copy)]
struct Item {
    x: i64,
    m: i64,
    a: i64,
    s: i64,
}


#[derive(Clone, Copy)]
struct ItemRange {
    x: (i64, i64),
    m: (i64, i64),
    a: (i64, i64),
    s: (i64, i64),
}

impl fmt::Debug for ItemRange {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{{x={:04}-{:04} m={:04}-{:04} a={:04}-{:04} s={:04}-{:04}}}", self.x.0, self.x.1, self.m.0, self.m.1, self.a.0, self.a.1, self.s.0, self.s.1)
    }
}

impl ItemRange {
    fn var_value(&self, variable: &str) -> (i64, i64) {
        match variable {
            "x" => self.x,
            "m" => self.m,
            "a" => self.a,
            "s" => self.s,
            _ => panic!("Invalid variable: {}", variable),
        }
    }

    fn set_var_value(&mut self, variable: &str, value: (i64, i64)) {
        match variable {
            "x" => self.x = value,
            "m" => self.m = value,
            "a" => self.a = value,
            "s" => self.s = value,
            _ => panic!("Invalid variable: {}", variable),
        }
    }

    fn default() -> ItemRange {
        ItemRange {
            x: (1, 4000),
            m: (1, 4000),
            a: (1, 4000),
            s: (1, 4000),
        }
    }

    fn empty() -> ItemRange {
        ItemRange {
            x: (0, 0),
            m: (0, 0),
            a: (0, 0),
            s: (0, 0),
        }
    }

    fn is_empty(&self) -> bool {
        self.x == (0, 0) && self.m == (0, 0) && self.a == (0, 0) && self.s == (0, 0)
    }

    fn split(&self, var_name: &str, operator: char, value: i64) -> Option<(ItemRange, ItemRange)> {
        let original_range = self.var_value(var_name);

        if value < original_range.0 || value > original_range.1 {
            return None;
        }

        let new_lower_range = (original_range.0, cmp::min(original_range.1, value) - if operator == '<' { 1 } else { 0 });
        let new_upper_range = (cmp::max(original_range.0, value) + if operator == '>' { 1 } else { 0 }, original_range.1);

        let mut r1 = self.clone();
        let mut r2 = self.clone();

        r1.set_var_value(var_name, new_lower_range);
        r2.set_var_value(var_name, new_upper_range);
        // println!("Splitting: {} {} {} {} -> {} {} {} {}", original_range.0, original_range.1, operator, value, new_lower_range.0, new_lower_range.1, new_upper_range.0, new_upper_range.1);
        return Some((r1, r2));
    }

    fn degrees_of_freedom(&self) -> i64 {
        ["x", "m", "a", "s"]
            .iter()
            .map(|var| self.var_value(var))
            .map(|(l, h)| h - l + 1)
            .filter(|&diff| diff > 0)
            .fold(1, |acc, diff| acc * diff)
    }
}

#[derive(Debug)]
struct Rule {
    variable: Option<String>,
    operator: Option<core::primitive::char>,
    value: Option<i64>,
    workflow: String,
}

impl Rule {
    fn is_default(&self) -> bool {
        self.variable.is_none() || self.operator.is_none() || self.value.is_none()
    }
    fn apply(&self, item: &Item) -> Option<String> {
        if self.is_default() {
            return Some(self.workflow.clone());
        }
        let rule_var = self.variable.as_ref().unwrap().as_str();
        let item_value = match rule_var {
            "x" => item.x,
            "m" => item.m,
            "a" => item.a,
            "s" => item.s,
            _ => panic!("Invalid variable: {}", rule_var),
        };

        let rule_operator = self.operator.unwrap();
        let rule_value = self.value.unwrap();
        match rule_operator {
            '<' => if item_value < rule_value { return Some(self.workflow.clone()); },
            '>' => if item_value > rule_value { return Some(self.workflow.clone()); },
            _ => panic!("Invalid operator: {}", self.operator.unwrap()),
        }
        return None;
    }

    fn apply_split(&self, item: &ItemRange) -> IResult<ItemRange, (String, ItemRange)> {
        if self.is_default() {
            // nothing fails by default
            return Ok((ItemRange::empty(), (self.workflow.clone(), *item)));
        }
        let rule_var = self.variable.as_ref().unwrap().as_str();
        let rule_value = self.value.unwrap();
        let rule_operator = self.operator.unwrap();

        let (r1, r2) = match item.split(rule_var, rule_operator, rule_value) {
            Some((r1, r2)) => (r1, r2),
            None => return Ok((*item, ("".to_string(), ItemRange::empty()))),
        };

        let (passing_range, failing_range) = match rule_operator {
            '<' => (r1, r2),
            '>' => (r2, r1),
            _ => panic!("Invalid operator: {}", self.operator.unwrap()),
        };
        Ok((failing_range, (self.workflow.clone(), passing_range)))
    }
}

#[derive(Debug)]
struct Workflow {
    name: String,
    rules: Vec<Rule>,
}

impl Workflow {
    fn apply(&self, item: &Item) -> String {
        for rule in &self.rules {
            if let Some(workflow) = rule.apply(item) {
                return workflow;
            }
        }
        panic!("No workflow found for item: {:?}", item);
    }

    fn apply_split(&self, item: &ItemRange) -> Vec<(String, ItemRange)> {
        let mut result = Vec::new();
        let mut current_item = item.clone();
        for rule in &self.rules {
            let (failing_range, (workflow, passing_range)) = rule.apply_split(&current_item).unwrap();
            if !passing_range.is_empty() {
                result.push((workflow, passing_range));
            }
            current_item = failing_range;
        }
        if !current_item.is_empty() {
            println!("Workflow rules: {:?}", self.rules);
            panic!("Failed to apply workflow: {:?}", item);
        }
        return result;
    }
}

pub fn solve(input_data: Option<String>, advanced: bool) -> String {
    let data = input_data.unwrap_or(String::from(EXAMPLE)).trim().to_string();
    let (wf_str, item_str) = data.split_once("\n\n").unwrap();

    let workflow_map = wf_str
        .lines()
        .map(|l| parse_workflow(l).unwrap().1)
        .map(|w| (w.name.clone(), w)).collect::<HashMap<_, _>>();
    let items = item_str.lines().map(|l| parse_item(l).unwrap().1).collect::<Vec<_>>();

    let result = if advanced {
        apply_with_ranges(&workflow_map, "in".to_string())
    } else {
        items
            .into_iter()
            .filter(|i| is_accepted(&workflow_map, i))
            .map(|i| i.x + i.m + i.a + i.s)
            .sum::<i64>() as u64
    };

    format!("{}", result)
}

fn is_accepted(workflow_map: &HashMap<String, Workflow>, item: &Item) -> bool {
    let default_workflow_name = "in".to_string();
    let mut current_workflow_name = default_workflow_name;
    while current_workflow_name != "R" && current_workflow_name != "A" {
        let current_workflow = workflow_map.get(&current_workflow_name).unwrap();
        current_workflow_name = current_workflow.apply(&item);
    }

    return match current_workflow_name.as_str() {
        "A" => true,
        "R" => false,
        _ => panic!("Invalid workflow: {:?}", current_workflow_name),
    };
}

fn apply_with_ranges(workflow_map: &HashMap<String, Workflow>, workflow_name: String) -> u64 {
    let mut result = Vec::new();
    let mut queue = vec![(workflow_name.clone(), ItemRange::default())];
    while !queue.is_empty() {
        let (workflow_name, item) = queue.pop().unwrap();
        let workflow = workflow_map.get(&workflow_name).unwrap();
        let new_items = workflow.apply_split(&item);

        for (new_workflow_name, new_item) in new_items {
            match new_workflow_name.as_str() {
                "A" => result.push(new_item),
                "R" => continue,
                _ => queue.push((new_workflow_name, new_item)),
            }
        }
    }

    result.iter().map(|i| i.degrees_of_freedom()).sum::<i64>() as u64
}

fn parse_rule(input: &str) -> IResult<&str, Rule> {
    // example: a<2006:qkq,m>2090:A,rfg
    let (input, variable) = is_a("xmas")(input)?;
    let (input, operator) = is_a("<>")(input)?;
    let (input, value) = is_a("0123456789")(input)?;
    let (input, _) = char(':')(input)?;
    let (input, workflow_name) = take_while1(AsChar::is_alpha)(input)?;
    let (input, _) = char(',')(input)?;
    Ok((input, Rule {
        variable: Some(variable.to_string()),
        operator: Some(operator.chars().next().unwrap()),
        value: Some(value.parse::<i64>().unwrap()),
        workflow: workflow_name.to_string(),
    }))
}

fn parse_rule_default(input: &str) -> IResult<&str, Rule> {
    // example: rfg
    let (input, workflow_name) = take_while1(AsChar::is_alpha)(input)?;
    Ok((input, Rule {
        variable: None,
        operator: None,
        value: None,
        workflow: workflow_name.to_string(),
    }))
}

fn parse_workflow(input: &str) -> IResult<&str, Workflow> {
    // example: px{a<2006:qkq,m>2090:A,rfg}
    let (input, name) = take_while1(AsChar::is_alpha)(input)?;
    let (_, input) = delimited(is_a("{"), is_not("}"), is_a("}"))(input)?;
    let (input, rules) = many0(parse_rule)(input)?;
    let (_, default_rule) = parse_rule_default(input)?;
    let rules = rules.into_iter().chain(std::iter::once(default_rule)).collect::<Vec<_>>();

    Ok((input, Workflow {
        name: name.to_string(),
        rules,
    }))
}

fn parse_var(input: &str) -> IResult<&str, (char, i64)> {
    // example: x=787
    let (input, variable) = one_of("xmas")(input)?;
    let (input, _) = char('=')(input)?;
    let (input, value) = take_while1(AsChar::is_dec_digit)(input)?;
    let (input, _) = opt(char(','))(input)?;
    Ok((input, (variable, value.parse::<i64>().unwrap())))
}

fn parse_item(input: &str) -> IResult<&str, Item> {
    // example: {x=787,m=2655,a=1222,s=2876}
    let (_, input, ) = delimited(is_a("{"), is_not("}"), is_a("}"))(input)?;
    let (input, variables) = many1(parse_var)(input)?;

    let mut item = Item { x: 0, m: 0, a: 0, s: 0 };
    for (name, value) in variables {
        match name {
            'x' => item.x = value,
            'm' => item.m = value,
            'a' => item.a = value,
            's' => item.s = value,
            _ => panic!("Invalid variable: {}", name)
        }
    }
    return Ok((input, item));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_workflow() {
        let input = "hlb{x>1866:fzj,a>1466:gk,xdx}";
        let (_, workflow) = parse_workflow(input).unwrap();
        assert_eq!(workflow.name, "hlb");
        assert_eq!(workflow.rules.len(), 3);
        assert_eq!(workflow.rules[0].variable, Some("x".to_string()));
        assert_eq!(workflow.rules[0].operator, Some('>'));
        assert_eq!(workflow.rules[0].value, Some(1866));
        assert_eq!(workflow.rules[2].workflow, "xdx");
    }

    #[test]
    fn test_rule_apply_split() {
        let mut rule = Rule {
            variable: Some("x".to_string()),
            operator: Some('>'),
            value: Some(1866),
            workflow: "xdx".to_string(),
        };

        let range_item = ItemRange {
            x: (1, 4000),
            m: (1, 4000),
            a: (1, 4000),
            s: (1, 4000),
        };

        let (rest, (wf, result)) = rule.apply_split(&range_item).expect("Should apply");
        assert_eq!(wf, rule.workflow);
        assert_eq!(result.x, (1867, 4000));
        assert_eq!(rest.x, (1, 1866));

        rule.operator = Some('<');
        let (rest, (wf, result)) = rule.apply_split(&range_item).expect("Should apply");
        assert_eq!(wf, rule.workflow);
        assert_eq!(result.x, (1, 1865));
        assert_eq!(rest.x, (1866, 4000));
    }
}