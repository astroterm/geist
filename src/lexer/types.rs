#![allow(unused)]

use std::{iter::Peekable, ops::Range, str::CharIndices};

struct GistLexer<'a> {
    src: &'a str,
    chars: Peekable<CharIndices<'a>>,
    options: GistOptions<'a>,
}

struct GistOptions<'a> {
    inline_comment_syntax: Vec<&'a str>,
    terminated_comment_syntax: Vec<(&'a str, &'a str)>,
    ignore_case: bool,
}

struct GistToken<'a> {
    kind: GistKind<'a>,
    range: Range<usize>
}
enum GistKind<'a> {
    Identifier(&'a str),
    Text(&'a str),
    Whitespace,
    Colon,
    Pipe,
}

impl<'a> GistLexer<'a> {
    pub fn new(src: &'a str, options: GistOptions<'a>) -> Self {
        let chars = src
            .char_indices()
            .peekable();

        Self { src, chars, options }
    }

    fn run(&mut self) {
        let mut tokens: Vec<GistToken> = Vec::new();
        let chr_in_comment = self.search_inline_comment();
        while let Some((idx, chr)) = self.chars.next() {
            match chr {
                chr if chr_in_comment(chr) => todo!()
            }
        }
    }

    fn search_inline_comment(&self) -> impl FnOnce(char) -> bool {
        |chr| {
            self.options.inline_comment_syntax
                .iter()
                .cloned()
                .any(
                |s| s.contains(chr)
            )
        }
    }

}