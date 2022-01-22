use crate::question::Kind;
use colored::Colorize;
use std::collections::HashMap;

#[derive(Debug)]
pub struct State {
    pub result: HashMap<Kind, u8>,
}

impl State {
    pub fn new() -> Self {
        State {
            result: HashMap::from([
                (Kind::イエベ春, 0),
                (Kind::ブルベ夏, 0),
                (Kind::イエベ秋, 0),
                (Kind::ブルベ冬, 0),
            ]),
        }
    }
    pub fn print_answer(self) {
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
