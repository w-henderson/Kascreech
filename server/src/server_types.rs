use crate::types::Colour;

use serde::{Deserialize, Serialize};

// The global game that's directly indexed by actix_web
pub struct Game {
    questions: Questions,
}

#[derive(Deserialize, Serialize)]

// This struct should be serialized and deserialized to the .json files (Could use other files for better efficiency)
pub struct Questions<'a> {
    questions: Vec<QAndA<'a>>,
}

#[derive(Deserialize, Serialize)]
// A struct showing a single round
pub struct QAndA<'a> {
    // The given input question
    question: String,
    // A list of possible answers
    answers: Vec<(String, Colour)>,
    // The true answer (Should always be one of 'answers')
    answer: &'a str,
}

impl<'a> QAndA<'a> {
    pub fn new(question: String, answers: Vec<(String, Colour)>, answer: usize) -> Self {
        let answer = answers[answer].0.as_str();
        Self {
            question,
            answers,
            answer,
        }
    }
    pub fn is_right(&self, guess: &str) -> bool {
        self.answer == guess
    }
    pub fn get_answer(&self) -> &str {
        &self.answer
    }
    pub fn check_colour(&self, check_colour: Colour) -> Option<bool> {
        for (answer, colour) in self.answers.iter() {
            if colour == &check_colour {
                Some(self.is_right(answer))
            }
        }
        None
    }
}
