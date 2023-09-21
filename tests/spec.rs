use chrono::NaiveTime;
use once_cell::sync::Lazy;
use subtitles::vtt::{Cue, WebVTT};

struct SpecTest {
    raw: &'static str,
    parsed: WebVTT,
}

const EXAMPLES: Lazy<Vec<SpecTest>> = Lazy::new(|| {
    vec![
        SpecTest {
            raw: r#"WEBVTT

identifier
00:01.000 --> 00:04.000
Never drink liquid nitrogen.

00:05.000 --> 00:09.000
— It will perforate your stomach.
— You could die.

00:10.000 --> 00:14.000
The Organisation for Sample Public Service Announcements accepts no liability for the content of this advertisement, or for the consequences of any actions taken on the basis of the information provided.
"#,
        parsed: WebVTT {
            header: "WEBVTT".to_string(),
            cues: vec![Cue {
                identifier: Some("identifier".to_string()),
                start: NaiveTime::from_hms_milli_opt(0, 0, 1, 0).unwrap(),
                end: NaiveTime::from_hms_milli_opt(0, 0, 4, 0).unwrap(),
                text: "Never drink liquid nitrogen.".to_string(),
            },
            Cue {
                identifier: None,
                start: NaiveTime::from_hms_milli_opt(0, 0, 5, 0).unwrap(),
                end: NaiveTime::from_hms_milli_opt(0, 0, 9, 0).unwrap(),
                text: "— It will perforate your stomach.\n— You could die.".to_string(),
            },
            Cue {
                identifier: None,
                start: NaiveTime::from_hms_milli_opt(0, 0, 10, 0).unwrap(),
                end: NaiveTime::from_hms_milli_opt(0, 0, 14, 0).unwrap(),
                text: "The Organisation for Sample Public Service Announcements accepts no liability for the content of this advertisement, or for the consequences of any actions taken on the basis of the information provided.".to_string(),
            },
            ]
        }},]
});

#[cfg(test)]
mod tests {
    use super::*;
    use subtitles::vtt::{self};

    #[test]
    fn examples() {
        for (i, example) in EXAMPLES.iter().enumerate() {
            let (_, parsed) = vtt::parse_webvtt(example.raw).unwrap();
            assert_eq!(parsed, example.parsed, "example {}", i);
        }
    }
}
