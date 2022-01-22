use colored::Colorize;

use clap::Parser;
use pasona::question::{Kind, Question};
use std::collections::HashMap;

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
    fn print_answer(self) {
        let mut data_vec: Vec<_> = self.result.into_iter().collect();
        data_vec.sort_by(|a, b| b.1.cmp(&a.1));
        let max_answer = data_vec
            .get(0)
            .expect("データがありません。想定外のエラーです。");
        println!();
        println!();
        println!();
        println!();
        println!();
        match max_answer.0 {
            Kind::イエベ春 => {
                println!(
                    "{}",
                    "あなたは イエベ春 です。あなたに似合うライブラリは Svelte です。"
                        .bright_yellow()
                );
                println!("{}", "https://svelte.jp/".red());
            }
            Kind::ブルベ夏 => {
                println!(
                    "{}",
                    "あなたは ブルベ夏 です。あなたに似合うライブラリは React です。".cyan()
                );
                println!("{}", "https://ja.reactjs.org/".cyan());
            }
            Kind::イエベ秋 => {
                println!(
                    "{}",
                    "あなたは イエベ秋 です。あなたに似合うライブラリは D3.js です。".yellow()
                );
                println!("{}", "https://d3js.org/".yellow());
            }
            Kind::ブルベ冬 => {
                println!(
                    "{}",
                    "あなたは ブルベ冬 です。あなたに似合うライブラリは jQuery です。".blue()
                );
                println!("{}", "https://jquery.com/".blue());
            }
        }
        println!();
        println!();
        println!();
        println!();
        println!();
    }
}

fn main() {
    let mut state = State::new();
    Cli::parse();
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
            Ok(_) => {
                let answer = word.trim();
                let parsed_result = answer.parse::<usize>();
                match parsed_result {
                    Ok(num) => {
                        let choice = question.choices.get(num);
                        match choice {
                            Some(choice) => {
                                for (key, _) in choice.score.iter() {
                                    let current_value = state.result.get_mut(key).unwrap();
                                    *current_value += 1;
                                }
                                if questions.len() == loop_cnt + 1 {
                                    break;
                                };
                                loop_cnt += 1;
                                println!();
                            }
                            None => {
                                println!("範囲外の入力です。やり直してください。");
                            }
                        }
                    }
                    Err(_) => {
                        println!("無効な入力です。やり直してください。");
                    }
                }
            }
            Err(_) => {
                println!("入力に失敗しました。");
            }
        }
    }
    state.print_answer();
}
