use bevy::utils::hashbrown::HashMap;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::{
        complete::{self, char, multispace0},
        streaming::alphanumeric1,
    },
    combinator::{map, opt},
    multi::separated_list0,
    sequence::{pair, preceded, separated_pair, terminated, tuple},
    IResult,
};

use crate::systems::mulle_car::{PartDB, PartNew};

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
    Nothing(),
}

fn parse_bool(input: &str) -> IResult<&str, Value> {
    map(alt((tag("TRUE"), tag("FALSE"))), |s| {
        Value::Bool(s == "TRUE")
    })(input)
}

fn parse_array_like_structure(input: &str) -> IResult<&str, Value> {
    map(
        preceded(
            char('['),
            terminated(
                opt(separated_list0(
                    commaspace,
                    parse_string_or_number_or_dictish_structure_or_array_like_structure_or_point,
                )),
                char(']'),
            ),
        ),
        |s| match s {
            Some(s) => Value::ArraySingle(s),
            None => Value::Nothing(),
        },
    )(input)
}

fn parse_quoted_string(input: &str) -> IResult<&str, Value> {
    map(
        preceded(
            char('"'),
            terminated(alt((alphanumeric1, tag(""))), char('"')),
        ),
        |s: &str| Value::String(s.to_owned()),
    )(input)
}

fn parse_number_as_value(input: &str) -> IResult<&str, Value> {
    map(complete::i32, Value::Number)(input)
}

fn parse_tag_as_value(input: &str) -> IResult<&str, Value> {
    map(parse_tag, |s: &str| Value::Tag(s.to_string()))(input)
}

fn parse_tag(input: &str) -> IResult<&str, &str> {
    preceded(char('#'), alphanumeric1)(input)
}

fn parse_tag_or_number(input: &str) -> IResult<&str, Value> {
    alt((parse_tag_as_value, parse_number_as_value))(input)
}

fn parse_point(input: &str) -> IResult<&str, Point> {
    map(
        preceded(
            tag("point("),
            terminated(
                separated_pair(complete::i32, commaspace, complete::i32),
                char(')'),
            ),
        ),
        |(s1, s2)| Point { x: s1, y: s2 },
    )(input)
}

fn parse_point_as_value(input: &str) -> IResult<&str, Value> {
    map(
        preceded(
            tag("point("),
            terminated(
                separated_pair(parse_number_as_value, commaspace, parse_number_as_value),
                char(')'),
            ),
        ),
        |(s1, s2)| {
            Value::Point((
                match s1 {
                    Value::Number(num) => num,
                    _ => 0,
                },
                match s2 {
                    Value::Number(num) => num,
                    _ => 0,
                },
            ))
        },
    )(input)
}

fn parse_string_or_number_or_dictish_structure_or_array_like_structure_or_point(
    input: &str,
) -> IResult<&str, Value> {
    alt((
        parse_number_as_value,
        parse_tag_as_value,
        parse_point_as_value,
        parse_dictish_structure,
        parse_array_like_structure,
        parse_quoted_string,
        parse_bool,
    ))(input)
}

fn parse_tag_number_pair(input: &str) -> IResult<&str, HashMap<String, i32>> {
    map(
        preceded(
            char('['),
            terminated(separated_list0(commaspace, parse_key_numvalue), char(']')),
        ),
        |value| {
            let mut map = HashMap::<String, i32>::new();
            for (key, value) in value {
                map.insert(key, value);
            }
            map
        },
    )(input)
}

fn tag_to_animation_type(input: &str) -> Option<MulleAnimationActionType> {
    match input {
        "Wait" => Some(MulleAnimationActionType::Wait),
        "Still" => Some(MulleAnimationActionType::Still),
        "suck" => Some(MulleAnimationActionType::suck),
        "RndHold" => Some(MulleAnimationActionType::RndHold),
        "WaveFromCar" => Some(MulleAnimationActionType::WaveFromCar),
        "buffa" => Some(MulleAnimationActionType::buffa),
        "yr" => Some(MulleAnimationActionType::yr),
        "LookAround" => Some(MulleAnimationActionType::LookAround),
        "Scratch" => Some(MulleAnimationActionType::Scratch),
        "ScratchHead" => Some(MulleAnimationActionType::ScratchHead),
        "TalkToMe" => Some(MulleAnimationActionType::TalkToMe),
        "TalkWait" => Some(MulleAnimationActionType::TalkWait),
        "Talk" => Some(MulleAnimationActionType::Talk),
        "Vink" => Some(MulleAnimationActionType::Vink),
        "Svag" => Some(MulleAnimationActionType::Svag),
        "Sitt" => Some(MulleAnimationActionType::Sitt),
        "GetDown" => Some(MulleAnimationActionType::GetDown),
        "GetUp" => Some(MulleAnimationActionType::GetUp),
        "freeze" => Some(MulleAnimationActionType::freeze),
        "Sleep" => Some(MulleAnimationActionType::Sleep),
        "Shrug" => Some(MulleAnimationActionType::Shrug),
        "WalkLeft" => Some(MulleAnimationActionType::WalkLeft),
        "WalkLeftStart" => Some(MulleAnimationActionType::WalkLeftStart),
        "WalkRight" => Some(MulleAnimationActionType::WalkRight),
        "WalkRightStart" => Some(MulleAnimationActionType::WalkRightStart),
        _ => None,
    }
}

fn parse_action_entry(input: &str) -> IResult<&str, MulleAnimationAction> {
    // can look like
    // 0
    // #suck
    // #suck:[0,0,0,0]
    alt((
        map(complete::i32, MulleAnimationAction::Number),
        map(parse_action_list, |list| {
            MulleAnimationAction::Action(Action {
                anim_type: None,
                sub_actions: list,
            })
        }),
        map(
            separated_pair(
                parse_tag,
                char(':'),
                preceded(multispace0, parse_action_list),
            ),
            |(tag, actions)| {
                MulleAnimationAction::Action(Action {
                    anim_type: tag_to_animation_type(tag),
                    sub_actions: actions,
                })
            },
        ),
        map(parse_tag, |tag| {
            MulleAnimationAction::Action(Action {
                anim_type: tag_to_animation_type(tag),
                sub_actions: Vec::default(),
            })
        }),
    ))(input)
}

fn parse_action_list(input: &str) -> IResult<&str, Vec<MulleAnimationAction>> {
    // can look like
    // 0,0,0,0,0
    // #suck:[0,0,0,0,0]
    // #suck
    // 0,0,0,#suck:[0,0,0,0]
    preceded(
        char('['),
        terminated(separated_list0(commaspace, parse_action_entry), char(']')),
    )(input)
}

fn parse_key_t<'a, T, F, O1>(
    input: &'a str,
    parse: &dyn Fn(&str) -> IResult<&str, O1>,
    f: F,
) -> IResult<&'a str, (String, T)>
where
    F: Fn(O1) -> T,
{
    map(
        separated_pair(
            opt(parse_tag_or_number),
            char(':'),
            preceded(multispace0, parse),
        ),
        |(s1, s2)| {
            (
                s1.map_or_else(String::new, |s| match s {
                    Value::Tag(s_tag) => s_tag,
                    Value::Number(s_number) => s_number.to_string(),
                    _ => String::new(),
                }),
                f(s2),
            )
        },
    )(input)
}

fn parse_opt_value(input: &str) -> IResult<&str, Option<Value>> {
    opt(parse_string_or_number_or_dictish_structure_or_array_like_structure_or_point)(input)
}

fn parse_key_value(input: &str) -> IResult<&str, (String, Value)> {
    parse_key_t(input, &parse_opt_value, |s2| match s2 {
        None => Value::Nothing(),
        Some(s) => s,
    })
}

fn parse_key_stringvalue(input: &str) -> IResult<&str, (String, String)> {
    parse_key_t(input, &parse_quoted_string, |s2| match s2 {
        Value::String(s) => s,
        _ => String::new(),
    })
}

fn parse_enclosed_i32_tuple(input: &str) -> IResult<&str, (i32, i32)> {
    preceded(
        char('['),
        terminated(
            separated_pair(complete::i32, commaspace, complete::i32),
            char(']'),
        ),
    )(input)
}

fn parse_key_point(input: &str) -> IResult<&str, (String, Point)> {
    parse_key_t(input, &parse_enclosed_i32_tuple, |s2| Point {
        x: s2.0,
        y: s2.1,
    })
}

fn parse_key_action(input: &str) -> IResult<&str, (String, Vec<MulleAnimationAction>)> {
    parse_key_t(input, &parse_action_list, |s2| s2)
}

fn parse_key_numvalue(input: &str) -> IResult<&str, (String, i32)> {
    parse_key_t(input, &parse_number_as_value, |s2| match s2 {
        Value::Number(s) => s,
        _ => 0,
    })
}

fn parse_hashmap(input: &str) -> IResult<&str, HashMap<String, i32>> {
    alt((
        parse_tag_number_pair,
        map(complete::i32, |_| HashMap::new()),
    ))(input)
}

fn parse_key_hashmap(input: &str) -> IResult<&str, (String, HashMap<String, i32>)> {
    parse_key_t(input, &parse_hashmap, |s2| s2)
}

fn parse_key_path(input: &str) -> IResult<&str, String> {
    map(
        separated_pair(
            opt(parse_tag_or_number),
            char(':'),
            preceded(
                multispace0,
                preceded(char('['), terminated(multispace0, char(']'))),
            ),
        ),
        |(s1, _s2)| {
            // (
            s1.map_or_else(String::new, |s| match s {
                Value::Tag(s_tag) => s_tag,
                Value::Number(s_number) => s_number.to_string(),
                _ => String::new(),
            })
            // s2
            // )
        },
    )(input)
}

fn parse_numtuple_to_point(input: &str) -> IResult<&str, Point> {
    map(
        preceded(
            char('['),
            terminated(
                separated_pair(complete::i32, commaspace, complete::i32),
                char(']'),
            ),
        ),
        |s| Point { x: s.0, y: s.1 },
    )(input)
}

fn tuplepl(input: &str) -> IResult<&str, (i32, Point, Vec<InnerValue>)> {
    map(
        tuple((
            complete::i32,
            preceded(commaspace, parse_point),
            opt(preceded(commaspace, parse_dictish_structure)),
        )),
        |(num, point, innervalues)| {
            let mut realinners = Vec::<InnerValue>::new();
            if let Some(Value::Array(values)) = innervalues {
                for (key, value) in values {
                    if key == "Show" {
                        if let Value::Number(num) = value {
                            realinners.push(InnerValue::Show(num));
                        }
                    } else if key == "HillType" {
                        if let Value::Tag(tag) = value {
                            if tag == "BigHill" {
                                realinners.push(InnerValue::HillType(HillType::BigHill));
                            } else {
                                realinners.push(InnerValue::HillType(HillType::SmallHill));
                            }
                        }
                    } else if key == "InnerRadius" {
                        if let Value::Number(num) = value {
                            realinners.push(InnerValue::InnerRadius(num));
                        }
                    } else if key == "Direction" {
                        if let Value::Number(num) = value {
                            realinners.push(InnerValue::Direction(num));
                        }
                    }
                }
            }
            (num, point, realinners)
        },
    )(input)
}

fn parse_key_innervalues_array(input: &str) -> IResult<&str, (String, Vec<Object>)> {
    map(
        separated_pair(
            parse_tag,
            char(':'),
            preceded(
                multispace0,
                preceded(
                    char('['),
                    terminated(
                        opt(separated_list0(
                            commaspace,
                            preceded(char('['), terminated(tuplepl, char(']'))),
                        )),
                        char(']'),
                    ),
                ),
            ),
        ),
        |(s1, s2)| {
            s2.map_or_else(
                || (s1.to_owned(), Vec::<Object>::new()),
                |vec| {
                    let mut objects = Vec::<Object>::new();
                    for (num, point, realinners) in vec {
                        objects.push(Object {
                            id: num,
                            point,
                            inner_values: realinners,
                        });
                    }
                    (s1.to_owned(), objects)
                },
            )
        },
    )(input)
}

fn parse_key_num_or_numarray(input: &str) -> IResult<&str, (String, Vec<i32>)> {
    separated_pair(
        map(parse_tag, std::string::ToString::to_string),
        pair(char(':'), multispace0),
        alt((
            preceded(
                char('['),
                terminated(
                    separated_list0(commaspace, preceded(multispace0, complete::i32)),
                    char(']'),
                ),
            ),
            map(complete::i32, |num| vec![num]),
        )),
    )(input)
}

fn parse_key_newvalue(input: &str) -> IResult<&str, (String, Vec<PartNew>)> {
    map(
        separated_pair(
            parse_tag,
            pair(char(':'), multispace0),
            alt((
                preceded(
                    char('['),
                    terminated(
                        opt(separated_list0(
                            commaspace,
                            preceded(
                                char('['),
                                terminated(
                                    opt(separated_list0(
                                        commaspace,
                                        tuple((
                                            parse_tag,
                                            preceded(commaspace, parse_numtuple_to_point),
                                            preceded(commaspace, parse_numtuple_to_point),
                                        )),
                                    )),
                                    char(']'),
                                ),
                            ),
                        )),
                        char(']'),
                    ),
                ),
                map(complete::i32, |_| None),
            )),
        ),
        |(tag, tuple)| {
            let mut vec = Vec::<PartNew>::new();
            for value in &tuple {
                for value in value.iter().flatten() {
                    for value in value {
                        vec.push(PartNew {
                            tag: value.0.to_owned(),
                            point1: value.1.clone(),
                            point2: value.2.clone(),
                        });
                    }
                }
            }

            (tag.to_owned(), vec)
        },
    )(input)
}

fn parse_key_tagarray(input: &str) -> IResult<&str, (String, Vec<String>)> {
    map(
        separated_pair(
            parse_tag,
            pair(char(':'), multispace0),
            alt((
                preceded(
                    char('['),
                    terminated(
                        opt(preceded(
                            multispace0,
                            separated_list0(commaspace, parse_tag),
                        )),
                        char(']'),
                    ),
                ),
                map(complete::i32, |_| None),
            )),
        ),
        |(tag, tuple)| {
            let mut vec = Vec::<String>::new();
            for value in &tuple {
                for value in value {
                    vec.push((*value).to_owned());
                }
            }
            (tag.to_owned(), vec)
        },
    )(input)
}

pub fn parse_dictish_structure(input: &str) -> IResult<&str, Value> {
    map(
        preceded(
            char('['),
            terminated(separated_list0(commaspace, parse_key_value), char(']')),
        ),
        |s: Vec<(String, Value)>| Value::Array(s),
    )(input)
}

#[derive(Debug, Clone)]
pub struct MapData {
    pub map_id: i32,
    pub objects: Vec<Object>,
    pub map_image: String,
    pub topology: String,
}
#[derive(Debug, Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}
#[derive(Debug, Clone)]
enum HillType {
    SmallHill,
    BigHill,
}
#[derive(Debug, Clone)]
enum InnerValue {
    InnerRadius(i32),
    Show(i32),
    HillType(HillType),
    Direction(i32),
}
#[derive(Debug, Clone)]
pub struct Object {
    id: i32,
    point: Point,
    inner_values: Vec<InnerValue>,
}

fn try_parse_mapdata(input: &str) -> IResult<&str, MapData> {
    map(
        preceded(
            char('['),
            terminated(
                tuple((
                    parse_key_numvalue,
                    preceded(commaspace, parse_key_innervalues_array),
                    preceded(commaspace, parse_key_stringvalue),
                    preceded(commaspace, parse_key_stringvalue),
                )),
                char(']'),
            ),
        ),
        |(mapid, objects, map_image, topology)| MapData {
            map_id: mapid.1,
            objects: objects.1,
            map_image: map_image.1,
            topology: topology.1,
        },
    )(input)
}

fn try_parse_partdb(input: &str) -> IResult<&str, PartDB> {
    map(
        preceded(
            char('['),
            terminated(
                tuple((
                    parse_key_numvalue,                              //partid
                    preceded(commaspace, parse_key_numvalue),        //master
                    preceded(commaspace, parse_key_num_or_numarray), //morphsto
                    preceded(commaspace, parse_key_stringvalue),     //description
                    preceded(commaspace, parse_key_stringvalue),     //junkview
                    preceded(commaspace, parse_key_stringvalue),     //useview
                    preceded(commaspace, parse_key_stringvalue),     //useview2
                    preceded(commaspace, parse_key_point),           //offset
                    preceded(commaspace, parse_key_hashmap),         //properties
                    preceded(commaspace, parse_key_tagarray),        //requires
                    preceded(commaspace, parse_key_tagarray),        //covers
                    preceded(commaspace, parse_key_newvalue),        //new
                )),
                char(']'),
            ),
        ),
        |(
            partid,
            master,
            morphto,
            description,
            junkview,
            useview,
            useview2,
            offset,
            properties,
            requires,
            covers,
            new,
        )| {
            PartDB {
                part_id: partid.1,
                master: master.1,
                morphs_to: morphto.1,
                description: description.1,
                junk_view: junkview.1,
                use_view: useview.1,
                use_view_2: useview2.1,
                offset: offset.1,
                properties: properties.1,
                requires: requires.1,
                covers: covers.1,
                new: new.1,
            }
        },
    )(input)
}

fn try_parse_animation(input: &str) -> IResult<&str, MulleAnimation> {
    map(
        preceded(
            char('['),
            terminated(
                tuple((
                    parse_key_action,                     //action
                    preceded(commaspace, parse_key_path), //path
                )),
                char(']'),
            ),
        ),
        |uts| MulleAnimation {
            actions: uts.0 .1,
            paths: Vec::default(),
        },
    )(input)
}

fn commaspace(input: &str) -> IResult<&str, (char, &str)> {
    pair(char(','), multispace0)(input)
}
#[derive(Clone)]
pub enum MulleDB {
    PartDB(PartDB),
    MapData(MapData),
}

fn try_map_or_part(input: &str) -> IResult<&str, MulleDB> {
    alt((
        map(try_parse_mapdata, MulleDB::MapData),
        map(try_parse_partdb, MulleDB::PartDB),
    ))(input)
}

pub fn try_get_mulledb(input: String) -> Option<MulleDB> {
    if let Ok((_, mulledb)) = try_map_or_part(&input) {
        return Some(mulledb);
    }
    None
}

pub fn try_get_animation(input: String) -> Option<MulleAnimation> {
    match try_parse_animation(&input) {
        Err(error) => {
            println!("failed to parse: {:?}", error);
            None
        }
        Ok(animation) => Some(animation.1),
    }
}

// pub fn get_hashmap_from_dblang(input: String) -> Option<HashMap<String, Value>> {
//     if let Ok((_, Value::Array(db_array))) = parse_dictish_structure(input.as_str()) {
//         let mut map = HashMap::<String, Value>::new();
//         for (key, value) in db_array {
//             map.insert(key, value);
//         }
//         return Some(map);
//     }
//     None
// }

enum MulleAnimationAction {
    Action(Action),
    Number(i32),
}

struct Action {
    anim_type: Option<MulleAnimationActionType>,
    sub_actions: Vec<MulleAnimationAction>,
}

enum MulleAnimationActionType {
    Still,
    Wait,
    RndHold,
    WaveFromCar,
    buffa,
    Talk,
    suck,
    yr,
    LookAround,
    Scratch,
    ScratchHead,
    TalkWait,
    TalkToMe,
    Vink,
    Svag,
    Sitt,
    freeze,
    Shrug,
    Sleep,
    GetUp,
    WalkRight,
    WalkRightStart,
    WalkLeft,
    WalkLeftStart,
    GetDown,
}

struct MulleAnimationPaths {}
pub struct MulleAnimation {
    actions: Vec<MulleAnimationAction>,
    paths: Vec<MulleAnimationPaths>,
}
