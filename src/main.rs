#![allow(unused)]

use clap::Parser;
use pasona::question::{Choice, Kind, Question};
use std::{collections::HashMap, error::Error, fmt::Result, num::ParseIntError, slice::SliceIndex};

/// あなたのパーソナルカラーを診断します
#[derive(Parser)]
struct Cli {}

#[derive(Debug)]
struct State {
    result: HashMap<Kind, u8>,
}

impl State {
    fn new() -> Self {
        State {
            result: HashMap::from([
                (Kind::イエベ春, 0),
                (Kind::ブルベ夏, 0),
                (Kind::イエベ秋, 0),
                (Kind::ブルベ冬, 0),
            ]),
        }
    }
}

fn main() {
    let mut state = State::new();
    let args = Cli::parse();
    let questions = Question::new_questions();

    let mut loop_cnt = 0;
    loop {
        let question = questions.get(loop_cnt).expect("存在しない質問です。");
        let question_content = &question.q;
        println!("{}", question_content);
        question.choices.iter().enumerate().for_each(|(i, x)| {
            println!("{}: {:?}", i, x.answer);
        });
        let mut word = String::new();
        let input_result = std::io::stdin().read_line(&mut word);
        match input_result {
            Ok(input) => {
                let answer = word.trim();
                let parsed_result = answer.parse::<usize>();
                match parsed_result {
                    Ok(num) => {
                        let choice = question.choices.get(num);
                        match choice {
                            Some(choice) => {
                                for (key, value) in choice.score.iter() {
                                    let current_value = state.result.get_mut(key).unwrap();
                                    *current_value += 1;
                                }
                                if (questions.len() == loop_cnt + 1) {
                                    break;
                                }
                                loop_cnt += 1;
                            }
                            None => {
                                println!("範囲外の入力です。やり直してください。");
                            }
                        }
                    }
                    Err(err) => {
                        println!("無効な入力です。やり直してください。");
                    }
                }
            }
            Err(_) => {
                println!("入力に失敗しました。");
            }
        }
    }
}
