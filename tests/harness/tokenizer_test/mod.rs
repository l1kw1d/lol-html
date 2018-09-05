mod decoder;
mod parsing_result;
mod token;
mod unescape;

use self::parsing_result::ParsingResult;
pub use self::token::TestToken;
use self::unescape::Unescape;
use cool_thing::lex_unit::LexUnit;
use cool_thing::tag_name_hash::get_tag_name_hash;
use cool_thing::tokenizer::{TextParsingMode, Tokenizer};
use cool_thing::tree_builder_simulator::TreeBuilderSimulator;
use serde_json;
use std::cell::Cell;

macro_rules! assert_eql {
    ($actual:expr, $expected:expr, $cs:expr, $input:expr, $msg:expr) => {
        assert!(
            $actual == $expected,
            "{}\n\
             state: {:?}\n\
             input: {:?}\n\
             actual: {:#?}\n\
             expected: {:#?}",
            $msg,
            $input,
            $cs,
            $actual,
            $expected
        );
    };
}

pub fn default_initial_states() -> Vec<String> {
    vec![String::from("Data state")]
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenizerTest {
    pub description: String,
    pub input: String,

    #[serde(rename = "output")]
    pub expected_tokens: Vec<TestToken>,

    #[serde(default = "default_initial_states")]
    pub initial_states: Vec<String>,

    #[serde(default)]
    pub double_escaped: bool,

    #[serde(default)]
    pub last_start_tag: String,

    #[serde(skip)]
    pub ignored: bool,
}

impl Unescape for TokenizerTest {
    fn unescape(&mut self) -> Result<(), serde_json::error::Error> {
        if self.double_escaped {
            self.double_escaped = false;
            self.input.unescape()?;

            for token in &mut self.expected_tokens {
                token.unescape()?;
            }
        }

        Ok(())
    }
}

impl TokenizerTest {
    pub fn init(&mut self) {
        self.ignored = self.unescape().is_err();

        // NOTE: tokenizer should always produce EOF token
        self.expected_tokens.push(TestToken::Eof);
    }

    fn parse(&self, input: Vec<u8>, initial_text_parsing_mode: TextParsingMode) -> ParsingResult {
        let mut result = ParsingResult::default();

        {
            let text_parsing_mode = Cell::new(TextParsingMode::Data);
            let mut text_parsing_mode_change_handler = |mode| text_parsing_mode.set(mode);

            let lex_res_handler =
                |lex_res: LexUnit| result.add_lex_res(lex_res, text_parsing_mode.get());

            let mut tokenizer = Tokenizer::new(2048, TreeBuilderSimulator::new(lex_res_handler));

            tokenizer.set_text_parsing_mode_change_handler(&mut text_parsing_mode_change_handler);
            tokenizer.set_state(initial_text_parsing_mode.into());
            tokenizer.set_last_start_tag_name_hash(get_tag_name_hash(&self.last_start_tag));

            tokenizer
                .write(input)
                .expect("Tokenizer buffer capacity exceeded");
        }

        result
    }

    fn assert_tokens_have_correct_raw_strings(&self, actual: ParsingResult) {
        if let Some(token_raw_pairs) = actual.into_token_raw_pairs() {
            for (token, raw, text_parsing_mode) in token_raw_pairs {
                let mut actual = self.parse(raw.bytes().collect(), text_parsing_mode);

                assert_eql!(
                    *actual.get_tokens(),
                    vec![token.to_owned(), TestToken::Eof],
                    raw,
                    text_parsing_mode,
                    "Token's raw string doesn't produce the same token"
                );
            }
        }
    }

    pub fn run(&self) {
        for cs in &self.initial_states {
            let cs = TextParsingMode::from(cs.as_str());
            let actual = self.parse(self.input.bytes().collect(), cs);

            assert_eql!(
                *actual.get_tokens(),
                self.expected_tokens,
                self.input,
                cs,
                "Token mismatch"
            );

            assert_eql!(
                actual.get_cumulative_raw_string(),
                self.input,
                self.input,
                cs,
                "Cumulative raw strings mismatch"
            );

            self.assert_tokens_have_correct_raw_strings(actual);
        }
    }
}