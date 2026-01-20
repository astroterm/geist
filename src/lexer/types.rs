#![allow(unused)]

use std::ops::Range;

struct GistLexer<'a, I>
where 
    I: Iterator<Item = (usize, char)>
{
    src: &'a str,
    chars: I
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