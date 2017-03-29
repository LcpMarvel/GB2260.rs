extern crate gb2260;

use gb2260::Division;
use gb2260::Source;

#[test]
fn it_inits_a_division() {
    assert_eq!("拱墅区", example_division().name);
}

#[test]
fn it_finds_a_divistion() {
    let division = gb2260::get(Source::GB, "200712", "330105".to_string());

    assert_eq!(division, example_division());
}

#[test]
fn it_returns_all_revisions_by_source() {
    let gb_revisions = vec!["200712", "200212"];

    assert_eq!(gb_revisions, gb2260::revisions(Source::GB));
}

#[test]
fn it_has_prefecture_and_province() {
    let division = example_division();

    let prefecture_division = division.prefecture();
    let province_division = division.province();

    assert_eq!(prefecture_division.code, "330100".to_string());
    assert_eq!(province_division.code, "330000".to_string());
}

#[test]
fn it_has_prefectures() {
    let division = Division {
        source: Source::GB,
        code: "110000".to_string(),
        name: "北京市",
        revision: "200712"
    };

    assert!(division.prefectures().unwrap().iter().any(|ref d| d.code.as_str() == "110100"));
}

#[test]
fn it_has_counties() {
    let division = Division {
        source: Source::GB,
        code: "110100".to_string(),
        name: "市辖区",
        revision: "200712"
    };

    assert!(division.counties().unwrap().iter().any(|ref d| d.code.as_str() == "110101"));
}

#[test]
fn it_list_all_provinces() {
    let vec = gb2260::provinces(Source::GB, "200712");

    assert!(vec.iter().any(|ref d| d.code.as_str() == "110000"));
}

#[test]
fn it_list_all_prefectures() {
    let vec = gb2260::prefectures(Source::GB, "200712");

    assert!(vec.iter().any(|ref d| d.code.as_str() == "110100"));
}

fn example_division<'a>() -> Division<'a> {
    Division {
        source: Source::GB,
        code: "330105".to_string(),
        name: "拱墅区",
        revision: "200712"
    }
}
