use std::fs::File;
use std::collections::HashMap;
use std::io::{ Lines, BufReader };

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    // qs{s>3448:A,lnx}
    pub static ref RULE_SET_RE: Regex = Regex::new(r"([a-zA-Z]+)\{(.+)}").unwrap();

    pub static ref RULE_RE: Regex = Regex::new(r"([xmas])([<>])(\d+):(.+)").unwrap();

    // {x=787,m=2655,a=1222,s=2876}
    pub static ref MACHINE_PART_RE: Regex = Regex::new(r"\{x=(.+),m=(.+),a=(.+),s=(.+)}").unwrap();
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct MachinePart {
    x: i32,
    m: i32,
    a: i32,
    s: i32,
}

impl MachinePart {
    fn sum(&self) -> i32 {
        self.x + self.m + self.a + self.s
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RuleAction {
    Accept,
    Reject,
    Condition,
    Route,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Comparison {
    LT,
    GT,
}

#[derive(Debug)]
struct Condition {
    left_operand: char,
    operator: Comparison,
    right_operand: i32,
    if_true: String,
}

impl Condition {
    fn evaluate(&self, part: &MachinePart) -> Option<String> {
        let val = match self.left_operand {
            'x' => part.x,
            'm' => part.m,
            'a' => part.a,
            's' => part.s,
            _ => panic!("Invalid left operand!")
        };
        let eval = match self.operator {
            Comparison::LT => val < self.right_operand,
            Comparison::GT => val > self.right_operand,
        };
        if eval {
            return Option::Some(self.if_true.clone());
        }
        return Option::None;
    }
}

#[derive(Debug)]
struct Rule {
    action: RuleAction,
    condition: Option<Condition>,
    next: Option<String>,
}

impl Rule {
    fn new_accept() -> Self {
        Rule {
            action: RuleAction::Accept,
            condition: Option::None,
            next: Option::None,
        }
    }

    fn new_reject() -> Self {
        Rule {
            action: RuleAction::Reject,
            condition: Option::None,
            next: Option::None,
        }
    }

    fn new_route(next: &str) -> Self {
        Rule {
            action: RuleAction::Route,
            condition: Option::None,
            next: Option::Some(next.to_string()),
        }
    }

    fn new_condition(condition: Condition) -> Self {
        Rule {
            action: RuleAction::Condition,
            condition: Option::Some(condition),
            next: Option::None,
        }
    }

    fn evaluate(&self, part: &MachinePart) -> Option<String> {
        return match self.action {
            RuleAction::Accept => Option::Some("A".to_string()),
            RuleAction::Reject => Option::Some("R".to_string()),
            RuleAction::Route => self.next.clone(),
            RuleAction::Condition => self.condition.as_ref()
                .expect("Condition is required for Condition rules")
                .evaluate(part),
        }
    }
}

struct RuleSet {
    rules: Vec<Rule>
}

impl RuleSet {
    fn evaluate(&self, part: &MachinePart) -> Option<String> {
        for rule in &self.rules {
            let result = rule.evaluate(part);
            if result.is_some() {
                return result;
            }
        }
        Option::None
    }
}

type RuleMap = HashMap<String, RuleSet>;

pub(crate) fn solve_part1(input: Lines<BufReader<File>>)  -> String {
    let data: Vec<String> = input.map(|x| x.unwrap()).collect();
    let (rules, parts) = parse_input(data);
    part_1_solver(rules, parts).to_string()
}

pub(crate) fn solve_part2(_input: Lines<BufReader<File>>)  -> String {
    return String::from("TODO");
}

fn parse_input(data: Vec<String>) -> (RuleMap, Vec<MachinePart>) {
    let mut rules: HashMap<String, RuleSet> = HashMap::new();
    let mut parts: Vec<MachinePart> = vec!();

    let mut handling_rules = true;

    for line in data {
        if line.len() == 0 {
            handling_rules = false;
            continue;
        }
        if handling_rules {
            for (_, [name, content]) in RULE_SET_RE
                .captures_iter(&line)
                .map(|x| x.extract()) {
                    let rule_set = parse_ruleset(content);
                    rules.insert(name.to_string(), rule_set);
                }

        } else {
            for (_, [x, m, a, s]) in MACHINE_PART_RE
                .captures_iter(&line)
                .map(|z| z.extract()) {
                    let x = x.parse::<i32>().unwrap();
                    let m = m.parse::<i32>().unwrap();
                    let a = a.parse::<i32>().unwrap();
                    let s = s.parse::<i32>().unwrap();
                    parts.push(MachinePart{x, m, a, s});
                }
        }
    }
    return (rules, parts);
}

fn parse_ruleset(rule: &str) -> RuleSet {
    let rules: Vec<Rule> = rule.split(",").map(|txt| {
        if txt == "A" {
            return Rule::new_accept();
        }
        if txt == "R" {
            return Rule::new_reject();
        }
        if !txt.contains(":") {
            return Rule::new_route(txt);
        }

        for (_, [left, operator, right, if_true]) in RULE_RE
            .captures_iter(&txt)
            .map(|z| z.extract()) {
                let operator = if operator.chars().next().unwrap() == '<' {
                    Comparison::LT
                } else {
                    Comparison::GT
                };
                let cond = Condition{
                    left_operand: left.chars().next().unwrap(),
                    operator,
                    right_operand: right.parse::<i32>().expect("Not an i32??"),
                    if_true: if_true.to_string(),
                };
                return Rule::new_condition(cond);
            }
        panic!("Rule does not conform to expected formats!");

    }).collect();

    RuleSet{rules}
}

fn part_1_solver(rules: RuleMap, parts: Vec<MachinePart>) -> i32 {
    let a = "A".to_string();
    let r = "R".to_string();
    let mut accepted = 0;

    for part in parts {
        let mut ruleset = rules.get("in").unwrap();
        while true {
            let result = ruleset.evaluate(&part);
            let val = result.expect("None result is unexpected.");
            if a == val {
                accepted += part.sum();
                break;
            }
            if r == val {
                break;
            }
            ruleset = rules.get(&val).expect("Expected routing to always lead to real rule");
        }
    }

    return accepted
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let data = vec!(
            String::from("px{a<2006:qkq,m>2090:A,rfg}"),
            String::from("pv{a>1716:R,A}"),
            String::from("lnx{m>1548:A,A}"),
            String::from("rfg{s<537:gd,x>2440:R,A}"),
            String::from("qs{s>3448:A,lnx}"),
            String::from("qkq{x<1416:A,crn}"),
            String::from("crn{x>2662:A,R}"),
            String::from("in{s<1351:px,qqz}"),
            String::from("qqz{s>2770:qs,m<1801:hdj,R}"),
            String::from("gd{a>3333:R,R}"),
            String::from("hdj{m>838:A,pv}"),
            String::from(""),
            String::from("{x=787,m=2655,a=1222,s=2876}"),
            String::from("{x=1679,m=44,a=2067,s=496}"),
            String::from("{x=2036,m=264,a=79,s=2244}"),
            String::from("{x=2461,m=1339,a=466,s=291}"),
            String::from("{x=2127,m=1623,a=2188,s=1013}"),
        );
        let (rule_map, parts) = parse_input(data);
        assert_eq!(true, rule_map.contains_key("hdj"));
        assert_eq!(5, parts.len());
        let expected = MachinePart{x:2127, m: 1623, a:2188, s: 1013};
        assert_eq!(true, parts.iter().any(|p| *p == expected));
    }

    #[test]
    fn test_parse_ruleset() {
        let txt = "a<2006:qkq,m>2090:A,rfg";
        let rule_set = parse_ruleset(txt);
        assert_eq!(3, rule_set.rules.len());

        for (i, e)  in rule_set.rules.iter().enumerate() {
            if i == 0 {
                assert_eq!(RuleAction::Condition, e.action);
                let cond = e.condition.as_ref().unwrap();
                assert_eq!('a', cond.left_operand);
                assert_eq!(2006, cond.right_operand);
            } else if i == 1 {
                assert_eq!(RuleAction::Condition, e.action);
                let cond = e.condition.as_ref().unwrap();
                assert_eq!('m', cond.left_operand);
                assert_eq!(Comparison::GT, cond.operator);
                assert_eq!("A", &cond.if_true);
            } else if i == 2 {
                assert_eq!(RuleAction::Route, e.action);
                assert_eq!("rfg", e.next.as_ref().unwrap());
            }
        }
    }

    #[test]
    fn test_evaluate_condition_lt() {
        let c = Condition {
            left_operand: 'a',
            operator: Comparison::LT,
            right_operand: 2006,
            if_true: String::from("qkq"),
        };
        let part = MachinePart{x: 1, m: 1, a: 1, s: 1};
        let result = c.evaluate(&part);
        assert_eq!("qkq", &result.unwrap());

        let part = MachinePart{x: 1, m: 1, a: 4000, s:1};
        let result = c.evaluate(&part);
        assert_eq!(Option::None, result);
    }

    #[test]
    fn test_evaluate_condition_gt() {
        let c = Condition {
            left_operand: 'x',
            operator: Comparison::GT,
            right_operand: 10,
            if_true: String::from("qkq"),
        };
        let part = MachinePart{x: 11, m: 1, a: 1, s: 1};
        let result = c.evaluate(&part);
        assert_eq!("qkq", &result.unwrap());

        let part = MachinePart{x: 1, m: 1, a: 4000, s:1};
        let result = c.evaluate(&part);
        assert_eq!(Option::None, result);
    }

    #[test]
    fn test_evaluate_ruleset() {
        let txt = "a<2006:qkq,m>2090:A,rfg";
        let rule_set = parse_ruleset(txt);

        let route = MachinePart{x:1, m: 1, a: 2005, s: 1};
        let route_result = rule_set.evaluate(&route);
        assert_eq!(true, route_result.is_some());
        assert_eq!("qkq", &route_result.unwrap());

        let accept = MachinePart{x:1, m:2091, a: 2007, s: 1};
        let accept_result = rule_set.evaluate(&accept);
        assert_eq!(true, accept_result.is_some());
        assert_eq!("A", &accept_result.unwrap());

        let fallthrough = MachinePart{x: 1, m: 1, a: 2090, s: 1};
        let fallthrough_result = rule_set.evaluate(&fallthrough);
        assert_eq!(true, fallthrough_result.is_some());
        assert_eq!("rfg", &fallthrough_result.unwrap());
    }

    #[test]
    fn test_part_1_solver() {
        let data = vec!(
            String::from("px{a<2006:qkq,m>2090:A,rfg}"),
            String::from("pv{a>1716:R,A}"),
            String::from("lnx{m>1548:A,A}"),
            String::from("rfg{s<537:gd,x>2440:R,A}"),
            String::from("qs{s>3448:A,lnx}"),
            String::from("qkq{x<1416:A,crn}"),
            String::from("crn{x>2662:A,R}"),
            String::from("in{s<1351:px,qqz}"),
            String::from("qqz{s>2770:qs,m<1801:hdj,R}"),
            String::from("gd{a>3333:R,R}"),
            String::from("hdj{m>838:A,pv}"),
            String::from(""),
            String::from("{x=787,m=2655,a=1222,s=2876}"),
            String::from("{x=1679,m=44,a=2067,s=496}"),
            String::from("{x=2036,m=264,a=79,s=2244}"),
            String::from("{x=2461,m=1339,a=466,s=291}"),
            String::from("{x=2127,m=1623,a=2188,s=1013}"),
        );
        let (rules, parts) = parse_input(data);
        let result = part_1_solver(rules, parts);
        assert_eq!(19114, result);
    }
}
