use std::collections::HashMap;

#[derive(Debug)]
pub struct Question {
    pub q: String,
    pub choices: Vec<Choice>,
}

impl Question {
    pub fn new_questions() -> Vec<Self> {
        vec![
            Question {
                q: "肌の色に近い色は？".to_string(),
                choices: vec![
                    Choice {
                        answer: "イエロー系".to_string(),
                        score: HashMap::from([(Kind::イエベ春, 1), (Kind::イエベ秋, 1)]),
                    },
                    Choice {
                        answer: "ピンク系".to_string(),
                        score: HashMap::from([(Kind::ブルベ夏, 1), (Kind::ブルベ冬, 1)]),
                    },
                ],
            },
            Question {
                q: " 瞳の色は？".to_string(),
                choices: vec![
                    Choice {
                        answer: "黒".to_string(),
                        score: HashMap::from([(Kind::ブルベ冬, 1), (Kind::イエベ秋, 1)]),
                    },
                    Choice {
                        answer: "茶".to_string(),
                        score: HashMap::from([(Kind::ブルベ夏, 1), (Kind::イエベ春, 1)]),
                    },
                ],
            },
            Question {
                q: " 顔うつりのいい色は".to_string(),
                choices: vec![
                    Choice {
                        answer: "グレー".to_string(),
                        score: HashMap::from([(Kind::ブルベ冬, 1), (Kind::ブルベ夏, 1)]),
                    },
                    Choice {
                        answer: "ベージュ".to_string(),
                        score: HashMap::from([(Kind::イエベ春, 1), (Kind::イエベ秋, 1)]),
                    },
                ],
            },
            Question {
                q: "好きなアクセサリーは？".to_string(),
                choices: vec![
                    Choice {
                        answer: "ゴールド".to_string(),
                        score: HashMap::from([(Kind::イエベ春, 1), (Kind::イエベ秋, 1)]),
                    },
                    Choice {
                        answer: "シルバー".to_string(),
                        score: HashMap::from([(Kind::ブルベ夏, 1), (Kind::ブルベ冬, 1)]),
                    },
                ],
            },
            Question {
                q: "深みのある服を着るとどう見えますか？".to_string(),
                choices: vec![
                    Choice {
                        answer: "大人っぽく見える".to_string(),
                        score: HashMap::from([(Kind::ブルベ冬, 1), (Kind::イエベ秋, 1)]),
                    },
                    Choice {
                        answer: "老けて見える".to_string(),
                        score: HashMap::from([(Kind::ブルベ夏, 1), (Kind::イエベ春, 1)]),
                    },
                ],
            },
            Question {
                q: "パステルカラーの服を着るとどう見えますか？".to_string(),
                choices: vec![
                    Choice {
                        answer: "似合う".to_string(),
                        score: HashMap::from([(Kind::イエベ春, 1), (Kind::ブルベ夏, 1)]),
                    },
                    Choice {
                        answer: "安く見える".to_string(),
                        score: HashMap::from([(Kind::ブルベ冬, 1), (Kind::イエベ秋, 1)]),
                    },
                ],
            },
            Question {
                q: "頬は赤くなりやすいですか？".to_string(),
                choices: vec![
                    Choice {
                        answer: "似合う".to_string(),
                        score: HashMap::from([(Kind::イエベ春, 1), (Kind::ブルベ夏, 1)]),
                    },
                    Choice {
                        answer: "安く見える".to_string(),
                        score: HashMap::from([(Kind::ブルベ冬, 1), (Kind::イエベ秋, 1)]),
                    },
                ],
            },
        ]
    }
}

#[derive(Debug)]
pub struct Choice {
    pub answer: String,
    pub score: HashMap<Kind, u8>,
}

#[derive(PartialEq, Eq, Hash, Debug)]
pub enum Kind {
    イエベ春,
    ブルベ夏,
    イエベ秋,
    ブルベ冬,
}
