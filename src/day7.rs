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

fn part2(equations: &[Equation]) -> u64 {
    equations
        .iter()
        .filter(|e| e.has_solution_part2())
        .map(|e| e.result)
        .sum()
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Operator {
    Add,
    Multiply,
    Concatenate,
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
            OperatorOrOperand::Operator(Operator::Concatenate) => write!(f, " || "),
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
    fn all_possible_operators_part1(
        &self,
    ) -> impl Iterator<Item = impl Iterator<Item = OperatorOrOperand> + use<'_>> + use<'_> {
        self.all_possible_operators_impl(vec![Operator::Add, Operator::Multiply])
    }

    fn all_possible_operators_part2(
        &self,
    ) -> impl Iterator<Item = impl Iterator<Item = OperatorOrOperand> + use<'_>> + use<'_> {
        self.all_possible_operators_impl(vec![
            Operator::Add,
            Operator::Multiply,
            Operator::Concatenate,
        ])
    }

    fn all_possible_operators_impl(
        &self,
        operators: Vec<Operator>,
    ) -> impl Iterator<Item = impl Iterator<Item = OperatorOrOperand> + use<'_>> + use<'_> {
        let num_operators_required = self.operands.len() - 1;
        // repeat_n.multi_cartesian_product is an itertools-suggested way to get "permutations with replacement" when the number
        // of elements in the permutation is greater than the number of elements in the source iterator
        // here it allows the creation of the right number of possible operator combos despite there only being two
        // possible input operators, and it gets all the possible combos of those
        itertools::repeat_n(operators, num_operators_required)
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

        self.all_possible_operators_part1()
            .map(Self::evaluate)
            .any(|result| result == self.result)
    }

    fn has_solution_part2(&self) -> bool {
        #[cfg(test)]
        println!(
            "Checking for solution to equation with result {}",
            self.result
        );

        self.all_possible_operators_part2()
            .map(Self::evaluate)
            .any(|result| result == self.result)
    }

    fn evaluate(candidate: impl Iterator<Item = OperatorOrOperand>) -> u64 {
        #[derive(Clone, Copy, Debug, PartialEq)]
        enum State {
            NeedOperator,
            NeedSecondOperand,
            Start,
        }

        let mut preprocessed = Vec::new();

        let mut state = State::Start;
        let mut last_operand = 0;
        let mut concatenating = false;
        for ooo in candidate {
            match (ooo, state) {
                (OperatorOrOperand::Operator(o), State::NeedOperator) => {
                    if o == Operator::Concatenate {
                        concatenating = true;
                    } else {
                        preprocessed.push(OperatorOrOperand::Operand(last_operand));
                        preprocessed.push(OperatorOrOperand::Operator(o));
                    }

                    state = State::NeedSecondOperand;
                }
                (OperatorOrOperand::Operator(_), State::NeedSecondOperand) => {
                    panic!("Bad input")
                }
                (OperatorOrOperand::Operator(_), State::Start) => panic!("Bad input"),
                (OperatorOrOperand::Operand(_), State::NeedOperator) => panic!("Bad input"),
                (OperatorOrOperand::Operand(o), State::NeedSecondOperand) => {
                    if concatenating {
                        let new_operand = u64::from_str(&format!("{}{}", last_operand, o)).expect(
                            "If this doesn't make a valid u64 then we need to go to u128 I guess",
                        );
                        concatenating = false;
                        last_operand = new_operand;
                    } else {
                        last_operand = o;
                    }
                    state = State::NeedOperator;
                }
                (OperatorOrOperand::Operand(o), State::Start) => {
                    last_operand = o;
                    state = State::NeedOperator;
                }
            }
        }

        if state == State::NeedOperator {
            preprocessed.push(OperatorOrOperand::Operand(last_operand));
        }

        #[cfg(test)]
        println!("{:?}", preprocessed);

        // it's possible we now just have a single operand, which if so is the answer
        // this will be because concat was the only operator
        if preprocessed.len() == 1 {
            match preprocessed[0] {
                OperatorOrOperand::Operator(_) => {
                    panic!("We shouldn't be able to preprocess into a list of one operator")
                }
                OperatorOrOperand::Operand(o) => return o,
            }
        }

        let mut state = State::Start;
        let mut result = 0;
        let mut operator: Option<Operator> = None;

        for ooo in preprocessed {
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
                        Some(Operator::Concatenate) => {
                            panic!("Preprocessor should have removed all of these")
                        }
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

#[test]
fn test_part2() {
    let equations = TEST_INPUT
        .lines()
        .map(Equation::from_str)
        .collect::<Result<Vec<Equation>, _>>()
        .expect("I expect the input to parse");
    assert_eq!(equations.len(), 9);
    let result = part2(&equations);
    assert_eq!(result, 11387);
}
