#[macro_use]
extern crate nom;

use std::fs::File;
use std::str;
use std::io::prelude::*;
use std::vec::Vec;

use nom::*;

#[derive(Debug, Clone)]
struct Data<'a> {
    value: &'a [u8],
    children: Vec<Data<'a>>,
}

impl<'a> Data<'a> {
    fn leaf(value: &'a [u8]) -> Data<'a> {
        return Data {
            value: value,
            children: Vec::new(),
        }
    }

    fn from_bytes(input: &'a [u8]) -> Result<Vec<Data<'a>>, Err<u32>> {
        let iresult = complete!(input, many0!(call!(node, b"")));
        iresult.to_result()
    }
}

// Remove comments (any characters after a # in a line),
// trailing whitespace, and any pure-whitespace lines.
fn clean(data: Vec<u8>) -> Vec<u8> {
    let mut cleaned_contents = Vec::new();
    let mut spaces = Vec::new();
    let mut in_comment = false;
    let mut line_filled = false;

    for b in data {
        if b == b'\n' {
            if line_filled {
                cleaned_contents.push(b'\n');
            }
            spaces.clear();
            in_comment = false;
            line_filled = false;
        } else if b == b' ' || b == b'\t' {
            if !in_comment {
                spaces.push(b);
            }
        } else if b == b'#' {
            in_comment = true;
        } else {
            if !in_comment {
                cleaned_contents.extend(spaces.iter());
                cleaned_contents.push(b);
                line_filled = true;
                spaces.clear();
            }
        }
    }

    return cleaned_contents;
}

fn match_indentation<'a, 'b>(input: &'a [u8], indentation: &'b [u8]) -> IResult<&'a [u8], ()> {
    do_parse!(input,
        verify!(opt!(is_a!(" \t")), |line_ind: Option<&[u8]>| line_ind.unwrap_or(&[]) == indentation) >>
        (())
    )
}

fn deeper_indentation<'a, 'b>(input: &'a [u8], indentation: &'b [u8]) -> IResult<&'a [u8], &'a [u8]> {
    do_parse!(input,
        new_indentation: verify!(
            map!(opt!(is_a!(" \t")), |x: Option<&'a [u8]>| x.unwrap_or(b"")),
            |line_ind: &[u8]| line_ind.starts_with(indentation) && line_ind != indentation
        ) >>
        (new_indentation)
    )
}

fn inline_node<'a, 'b>(input: &'a [u8], indentation: &'b [u8]) -> IResult<&'a [u8], Data<'a>> {
    do_parse!(input, 
        call!(match_indentation, indentation) >>
        value: is_not!(":\n") >>
        tag!(":") >>
        is_a!(" \t") >>
        children: separated_list!(
                do_parse!(
                    tag!(",") >>
                    opt!(is_a!(" \t")) >>
                    (())
                ),
                map!(is_not!(",\n"), Data::leaf)
            ) >>
        tag!("\n") >>
        (Data {
            value: value,
            children: children,
        })
    )
}

fn nested_node<'a, 'b>(input: &'a [u8], indentation: &'b [u8]) -> IResult<&'a [u8], Data<'a>> {
    do_parse!(input,
        call!(match_indentation, indentation) >>
        value: is_not!(":\n") >>
        tag!("\n") >>
        children: opt!(do_parse!(
            new_indentation: peek!(call!(deeper_indentation, indentation)) >>
            children: many1!(call!(node, new_indentation)) >>
            (children)
        )) >>
        (Data {
            value: value,
            children: children.unwrap_or_else(Vec::new),
        })
    )
}

fn node<'a, 'b>(input: &'a [u8], indentation: &'b [u8]) -> IResult<&'a [u8], Data<'a>> {
    alt!(input, call!(inline_node, indentation) | call!(nested_node, indentation))
}

fn main() {
    let mut contents = Vec::new();
    let mut input = File::open("coal_to_solid").unwrap();
    input.read_to_end(&mut contents).unwrap();

    let clean_contents = clean(contents);

    let result = Data::from_bytes(&clean_contents);
    println!("{:?}", result);
}
