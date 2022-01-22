use clap::Parser;
use pasona::{question::Question, state::State};

/// あなたのパーソナルカラーを診断します
#[derive(Parser)]
struct Cli {}

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
