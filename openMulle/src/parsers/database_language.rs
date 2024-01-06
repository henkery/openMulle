use std::collections::HashMap;

use bevy::input;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::{
        complete::{alpha1, char, digit1, space1, self, multispace0},
        streaming::alphanumeric1,
    },
    combinator::{map, map_res, opt, recognize},
    multi::{separated_list0, separated_list1},
    sequence::{delimited, pair, preceded, separated_pair, terminated},
    ErrorConvert, IResult,
};

// #[derive(Debug)]
// struct KeyValue {
//     key: String,
//     value: Value,
// }

#[derive(Clone)]
pub enum Value {
    Number(i32),
    Array(Vec<(String, Value)>),
    ArraySingle(Vec<Value>),
    String(String),
    Tag(String),
    Point((i32, i32)),
    Bool(bool),
    Nothing()
}
fn parse_bool(input: &str) -> IResult<&str, Value> {
    map(alt((tag("TRUE"), tag("FALSE"))), |s| {Value::Bool(s == "TRUE")})(input)
}

fn parse_array_like_structure(input: &str) -> IResult<&str, Value> {
    map(preceded(char('['), terminated(opt(separated_list0( char(','), preceded(multispace0, parse_string_or_number_or_dictish_structure_or_array_like_structure_or_point))), char(']'))),
    |s| match s {
        Some(s) => Value::ArraySingle(s),
        None => Value::Nothing()
    } )(input)
}

fn parse_quoted_string(input: &str) -> IResult<&str, Value> {
    map(
        preceded(char('"'), terminated(alt((alphanumeric1, tag(""))), char('"'))),
        |s: &str| Value::String(s.to_owned()),
    )(input)
 }

 fn parse_number_as_value(input: &str) -> IResult<&str, Value> {
    map(complete::i32, |number| { Value::Number(number) })(input)
 }

 fn parse_tag_as_value(input: &str) -> IResult<&str, Value> {
    map(preceded(char('#'), alphanumeric1), |s: &str| Value::Tag(s.to_string()))(input)
 }

 fn parse_tag_or_number(input: &str) -> IResult<&str, Value> {
    alt((parse_tag_as_value, parse_number_as_value))(input)
 }


 fn parse_point(input: &str) -> IResult<&str, Value> {
    map(preceded(tag("point("), terminated(separated_pair(parse_number_as_value, char(','), preceded(multispace0, parse_number_as_value)), char(')'))), 
    |(s1, s2)| Value::Point((match s1 { Value::Number(num) => num, _ => 0}, match s2 { Value::Number(num) => num, _ => 0}))
)(input)
 }

fn parse_string_or_number_or_dictish_structure_or_array_like_structure_or_point(input: &str) -> IResult<&str, Value> {
    alt((parse_number_as_value, parse_tag_as_value, parse_point, parse_dictish_structure, parse_array_like_structure, parse_quoted_string, parse_bool))(input)
}

fn parse_key_value(input: &str) -> IResult<&str, (String, Value)> {
    map(separated_pair(opt(parse_tag_or_number), char(':'), preceded(multispace0, opt(parse_string_or_number_or_dictish_structure_or_array_like_structure_or_point))),
    |(s1, s2)| (match s1 { None => String::from(""), Some(s) => {match s {
        Value::Tag(s_tag) => s_tag.to_owned(),
        Value::Number(s_number) => s_number.to_string(),
        _ => String::from(""),
    }}}, match s2 { None => Value::Nothing(), Some(s) => s})
)(input)
}

pub fn parse_dictish_structure(input: &str) -> IResult<&str, Value> {
    map(preceded(
        char('['),
        terminated(
            separated_list0(char(','), preceded(multispace0, parse_key_value)),
            char(']'),
        ),
    ), |s: Vec<(String, Value)>| Value::Array(s))(input)
}

pub fn get_hashmap_from_dblang(input: String) -> Option<HashMap<String, Value>> {
    if let Ok((_, Value::Array(db_array))) =
    parse_dictish_structure(
        input.as_str(),
    )
    {
        let mut map = HashMap::<String, Value>::new();
        for (key, value) in db_array {
            map.insert(key, value);
        }
        return Some(map)
    }
    None
}