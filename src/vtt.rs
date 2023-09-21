extern crate nom;

use chrono::NaiveTime;
use nom::branch::alt;
use nom::bytes::complete::is_not;
use nom::character::complete::{char, digit1, multispace1};
use nom::combinator::{eof, peek, recognize};
use nom::multi::many_till;
use nom::sequence::terminated;
use nom::{
    bytes::complete::tag,
    character::complete::{line_ending, multispace0},
    combinator::map_res,
    multi::separated_list1,
    sequence::{preceded, tuple},
    IResult,
};

#[derive(Debug, PartialEq)]
pub struct WebVTT {
    pub header: String,
    pub cues: Vec<Cue>,
}

#[derive(Debug, PartialEq)]
pub struct Cue {
    pub identifier: Option<String>,
    pub start: NaiveTime,
    pub end: NaiveTime,
    pub text: String,
}

pub fn parse_webvtt(input: &str) -> IResult<&str, WebVTT> {
    let (input, _) = tag("WEBVTT")(input)?;
    let (input, _) = multispace0(input)?;
    let (input, cues) = separated_list1(line_ending, parse_cue)(input)?;

    Ok((
        input,
        WebVTT {
            header: "WEBVTT".to_string(),
            cues,
        },
    ))
}
fn match_until_empty_line(input: &str) -> IResult<&str, &str> {
    recognize(many_till(
        terminated(is_not("\n"), line_ending),
        alt((peek(line_ending), peek(eof))),
    ))(input)
}

fn parse_cue(input: &str) -> IResult<&str, Cue> {
    let (input, identifier) = if peek(parse_timestamp_line)(input).is_err() {
        let (input, id) = parse_cue_identifier(input)?;
        (input, Some(id))
    } else {
        (input, None)
    };
    let (input, (start, end)) = parse_timestamp_line(input)?;
    let (input, _) = line_ending(input)?;
    let (input, text) = match_until_empty_line(input)?;

    let mut text = text.to_string();
    if text.ends_with('\n') {
        text = text.trim_end_matches('\n').to_string();
    }

    Ok((
        input,
        Cue {
            identifier,
            start,
            end,
            text,
        },
    ))
}

fn parse_cue_identifier(input: &str) -> IResult<&str, String> {
    let (input, identifier) = nom::character::complete::not_line_ending(input)?;
    let (input, _) = line_ending(input)?;

    Ok((input, identifier.to_string()))
}

fn parse_u32(input: &str) -> nom::IResult<&str, u32> {
    map_res(digit1, |s: &str| s.parse::<u32>())(input)
}

fn parse_time_long(input: &str) -> nom::IResult<&str, NaiveTime> {
    let (input, (hours, _, minutes, _, seconds, _, milliseconds)) = tuple((
        parse_u32,
        char(':'),
        parse_u32,
        char(':'),
        parse_u32,
        char('.'),
        parse_u32,
    ))(input)?;

    let time_opt = NaiveTime::from_hms_milli_opt(hours, minutes, seconds, milliseconds);
    match time_opt {
        Some(time) => Ok((input, time)),
        None => Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::MapRes,
        ))),
    }
}

fn parse_time_short(input: &str) -> nom::IResult<&str, NaiveTime> {
    let (input, (minutes, _, seconds, _, milliseconds)) =
        tuple((parse_u32, char(':'), parse_u32, char('.'), parse_u32))(input)?;

    let time_opt = NaiveTime::from_hms_milli_opt(0, minutes, seconds, milliseconds);
    match time_opt {
        Some(time) => Ok((input, time)),
        None => Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::MapRes,
        ))),
    }
}

fn parse_timestamp(input: &str) -> nom::IResult<&str, NaiveTime> {
    nom::branch::alt((parse_time_short, parse_time_long))(input)
}

fn parse_timestamp_line(input: &str) -> nom::IResult<&str, (NaiveTime, NaiveTime)> {
    let (input, start_time) = parse_timestamp(input)?;
    let (input, _) = preceded(multispace1, tuple((tag("-->"), multispace1)))(input)?;
    let (input, end_time) = parse_timestamp(input)?;

    Ok((input, (start_time, end_time)))
}
