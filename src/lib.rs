//! This is documentation for the `gb2260` crate.
//!
//! The foo crate is meant to be used for looking up the Chinese administrative divisions.

#[allow(dead_code)]
extern crate phf;

include!(concat!(env!("OUT_DIR"), "/data.rs"));

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Source {
    GB,
    Stats,
    // Mca,
}

#[derive(Debug, Clone)]
pub struct Division<'a> {
    pub source: Source,
    pub code: String,
    pub name: &'a str,
    pub revision: &'a str,
}

impl<'a> PartialEq for Division<'a> {
    fn eq(&self, other: &Division<'a>) -> bool {
        self.code == other.code && self.name == other.name && self.revision == other.revision
    }
}

impl<'a> Division<'a> {
    pub fn province(&self) -> Division<'a> {
        let (province_code, _, _) = self.parsed_code();

        get(self.source, self.revision, format!("{}0000", province_code))
    }

    pub fn prefecture(&self) -> Division<'a> {
        let (province_code, prefecture_code, _) = self.parsed_code();

        get(self.source, self.revision, format!("{}{}00", province_code, prefecture_code))
    }

    pub fn is_province(&self) -> bool {
        is_province(&self.code)
    }

    pub fn is_prefecture(&self) -> bool {
        is_prefecture(&self.code)
    }

    pub fn is_county(&self) -> bool {
        is_county(&self.code)
    }

    pub fn prefectures(&self) -> Option<Vec<Division<'a>>> {
        let (province_code, _) = self.code.split_at(2);

        if self.is_province() {
            let vec = data(self.source)
                        .get(self.revision).unwrap()
                        .entries
                        .iter()
                        .filter(|entry| is_prefecture(entry.0) && entry.0.starts_with(province_code))
                        .map(|entry|
                            Division {
                                source: self.source,
                                code: entry.0.to_string(),
                                name: entry.1,
                                revision: self.revision
                            }
                        ).collect();

            Some(vec)
        } else {
            None
        }
    }

    pub fn counties(&self) -> Option<Vec<Division<'a>>> {
        let (prefecture_code, _) = self.code.split_at(4);

        if self.is_prefecture() {
            let vec = data(self.source)
                        .get(self.revision).unwrap()
                        .entries
                        .iter()
                        .filter(|entry| entry.0.starts_with(prefecture_code))
                        .map(|entry|
                            Division {
                                source: self.source,
                                code: entry.0.to_string(),
                                name: entry.1,
                                revision: self.revision
                            }
                        ).collect();

            Some(vec)
        } else {
            None
        }
    }

    fn parsed_code(&self) -> (&str, &str, &str) {
        let (province_code, tail) = self.code.split_at(2);
        let (prefecture_code, county_code) = tail.split_at(2);

        (province_code, prefecture_code, county_code)
    }
}

pub fn get<'a>(source: Source, revision: &'a str, code: String) -> Division<'a> {
    let data = data(source).get(revision).unwrap();
    let name = data.get(code.as_str()).unwrap();

    Division {
        source: source,
        code: code.to_owned(),
        name: name,
        revision: revision
    }
}

pub fn revisions(source: Source) -> Vec<&'static str> {
    data(source)
        .entries
        .iter()
        .map(|entry| entry.0)
        .collect()
}

pub fn data(source: Source) -> &'static phf::Map<&'static str, &'static phf::Map<&'static str, &'static str>> {
    match source {
        Source::GB => &GB_DATA,
        Source::Stats => &STATS_DATA
    }
}

pub fn provinces<'a>(source: Source, revision: &'a str) -> Vec<Division<'a>> {
    data(source).get(revision).unwrap()
                .entries
                .iter()
                .filter(|entry| is_province(entry.0))
                .map(|entry|
                    Division {
                        source: source,
                        code: entry.0.to_string(),
                        name: entry.1,
                        revision: revision
                    }
                ).collect()
}

pub fn prefectures<'a>(source: Source, revision: &'a str) -> Vec<Division<'a>> {
    data(source).get(revision).unwrap()
                .entries
                .iter()
                .filter(|entry| is_prefecture(entry.0))
                .map(|entry|
                    Division {
                        source: source,
                        code: entry.0.to_string(),
                        name: entry.1,
                        revision: revision
                    }
                ).collect()
}

pub fn is_province<'a>(code: &'a str) -> bool {
    code.ends_with("0000")
}

pub fn is_prefecture<'a>(code: &'a str) -> bool {
    code.ends_with("00") && !code.ends_with("0000")
}

pub fn is_county<'a>(code: &'a str) -> bool {
    !is_province(code) && !is_prefecture(code)
}

#[test]
fn it_parses_code() {
    let division = Division {
        source: Source::GB,
        code: "110105".to_string(),
        name: "朝阳区",
        revision: "2014"
    };

    assert_eq!(("11", "01", "05"), division.parsed_code());
}
