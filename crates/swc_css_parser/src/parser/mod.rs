use std::mem::take;

use serde::{Deserialize, Serialize};
use swc_css_ast::*;

use self::input::{Buffer, ParserInput};
use crate::{error::Error, Parse};

#[macro_use]
mod macros;
mod at_rules;
pub mod input;
mod selectors;
mod syntax;
#[cfg(test)]
mod tests;
mod util;
mod values_and_units;

pub type PResult<T> = Result<T, Error>;

#[derive(
    Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
#[serde(rename_all = "camelCase")]
pub struct ParserConfig {
    /// If this is `true`, **wrong** comments starting with `//` will be treated
    /// as a comment.
    ///
    /// This option exists because there are so many css-in-js tools and people
    /// use `//` as a comment because it's javascript file.
    ///
    /// Defaults to `false`.
    #[serde(default)]
    pub allow_wrong_line_comments: bool,

    /// If enabled, errors for css modules selectors will be ignored.
    ///
    ///
    /// Defaults to `false`.
    #[serde(default)]
    pub css_modules: bool,

    /// If this is `true`, the nested selectors without `&` will be parsed as
    /// valid selectors
    ///
    /// Defaults to `false`.
    #[serde(default)]
    pub legacy_nesting: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlockContentsGrammar {
    StyleBlock,
    DeclarationList,
    RuleList,
    Stylesheet,
    DeclarationValue,
}

impl Default for BlockContentsGrammar {
    fn default() -> Self {
        BlockContentsGrammar::Stylesheet
    }
}

#[derive(Debug, Default, Clone, Copy)]
struct Ctx {
    is_top_level: bool,
    block_contents_grammar: BlockContentsGrammar,

    in_keyframes_at_rule: bool,
    in_supports_at_rule: bool,
    in_import_at_rule: bool,
    in_page_at_rule: bool,
    in_container_at_rule: bool,
    in_font_feature_values_at_rule: bool,
    is_trying_legacy_nesting: bool,
}

#[derive(Debug, Clone)]
pub struct Parser<I>
where
    I: ParserInput,
{
    #[allow(dead_code)]
    config: ParserConfig,
    input: Buffer<I>,
    ctx: Ctx,
    errors: Vec<Error>,
}

impl<I> Parser<I>
where
    I: ParserInput,
{
    pub fn new(input: I, config: ParserConfig) -> Self {
        Parser {
            config,
            input: Buffer::new(input),
            ctx: Default::default(),
            errors: Default::default(),
        }
    }

    pub fn dump_cur(&mut self) -> String {
        format!("{:?}", self.input.cur())
    }

    /// Take **recovered** errors.
    pub fn take_errors(&mut self) -> Vec<Error> {
        take(&mut self.errors)
    }

    pub fn parse_all(&mut self) -> PResult<Stylesheet> {
        self.parse()
    }
}
