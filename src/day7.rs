use std::{error::Error, str::FromStr};

use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

pub fn run() -> Result<(), Box<dyn Error>> {
    let equations = include_str!("inputs/day7.txt")
        .lines()
        .map(Equation::from_str)
        .collect::<Result<Vec<_>, _>>()?;

    let part1 = part1(&equations);
    println!(
        "Part 1: Sum of the results of the solveable equations is {}",
        part1
    );

    Ok(())
}

fn part1(equations: &[Equation]) -> u64 {
    equations
        .iter()
        .filter(|e| e.has_solution())
        .map(|e| e.result)
        .sum()
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Operator {
    Add,
    Multiply,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum OperatorOrOperand {
    Operator(Operator),
    Operand(u64),
}

#[cfg(test)]
use std::fmt::Display;
#[cfg(test)]
impl Display for OperatorOrOperand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OperatorOrOperand::Operator(Operator::Add) => write!(f, " + "),
            OperatorOrOperand::Operator(Operator::Multiply) => write!(f, " * "),
            OperatorOrOperand::Operand(o) => write!(f, "{}", o),
        }
    }
}

#[derive(Clone)]
struct Equation {
    result: u64,
    operands: Vec<u64>,
}

impl Equation {
    fn all_possible_operators(
        &self,
    ) -> impl Iterator<Item = impl Iterator<Item = OperatorOrOperand> + use<'_>> + use<'_> {
        let num_operators_required = self.operands.len() - 1;
        // repeat_n.multi_cartesian_product is an itertools-suggested way to get "permutations with replacement" when the number
        // of elements in the permutation is greater than the number of elements in the source iterator
        // here it allows the creation of the right number of possible operator combos despite there only being two
        // possible input operators, and it gets all the possible combos of those
        itertools::repeat_n([Operator::Add, Operator::Multiply], num_operators_required)
            .multi_cartesian_product()
            .map(|operators| {
                let operators = operators.into_iter().map(OperatorOrOperand::Operator);
                self.operands
                    .iter()
                    .cloned()
                    .map(OperatorOrOperand::Operand)
                    .interleave(operators)
            })
    }

    fn has_solution(&self) -> bool {
        #[cfg(test)]
        println!(
            "Checking for solution to equation with result {}",
            self.result
        );

        self.all_possible_operators()
            .map(Self::evaluate)
            .any(|result| result == self.result)
    }

    fn evaluate(candidate: impl Iterator<Item = OperatorOrOperand>) -> u64 {
        enum State {
            NeedOperator,
            NeedSecondOperand,
            Start,
        }

        let mut result = 0;
        let mut state = State::Start;
        let mut operator: Option<Operator> = None;

        for ooo in candidate {
            #[cfg(test)]
            print!("{}", ooo);

            match (state, ooo) {
                (State::NeedOperator, OperatorOrOperand::Operator(op)) => {
                    operator = Some(op);
                    state = State::NeedSecondOperand;
                }
                (State::NeedOperator, OperatorOrOperand::Operand(_)) => {
                    panic!("Input equation in bad format. Wanted operator, got operand")
                }
                (State::NeedSecondOperand, OperatorOrOperand::Operator(_)) => {
                    panic!("Input equation in bad format. Wanted operand, got operator")
                }
                (State::NeedSecondOperand, OperatorOrOperand::Operand(operand)) => {
                    match operator {
                        Some(Operator::Add) => result += operand,
                        Some(Operator::Multiply) => result *= operand,
                        None => panic!("I shouldn't be in NeedSecondOperand with operator == None"),
                    }
                    state = State::NeedOperator;
                    operator = None;
                }
                (State::Start, OperatorOrOperand::Operator(_)) => {
                    panic!("Input equation in bad format. Wanted operand, got operator")
                }
                (State::Start, OperatorOrOperand::Operand(operand)) => {
                    result = operand;
                    state = State::NeedOperator;
                }
            }
        }

        #[cfg(test)]
        println!(" = {result}");
        result
    }
}

lazy_static! {
    static ref EQ_REGEX: Regex = Regex::new(r"(\d+): (.*)").unwrap();
}

impl FromStr for Equation {
    type Err = Box<dyn Error>;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        if let Some(captures) = EQ_REGEX.captures(value) {
            let result = u64::from_str(&captures[1])?;
            let operands = captures[2]
                .split_whitespace()
                .map(u64::from_str)
                .collect::<Result<Vec<_>, _>>()?;
            Ok(Equation { result, operands })
        } else {
            Err(format!("Input value '{value}' not well-formed by regex").into())
        }
    }
}

#[cfg(test)]
const TEST_INPUT: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

#[test]
fn test_part1() {
    let equations = TEST_INPUT
        .lines()
        .map(Equation::from_str)
        .collect::<Result<Vec<Equation>, _>>()
        .expect("I expect the input to parse");
    assert_eq!(equations.len(), 9);
    let result = part1(&equations);
    assert_eq!(result, 3749);
}
