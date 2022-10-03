use std::fs;
use std::fs::File;

const MAX: u32 = 1000;
const PREFIX_NAME: &str = "x";

#[cfg(windows)]
const LINE_ENDING: &'static str = "\r\n";
#[cfg(not(windows))]
const LINE_ENDING: &'static str = "\n";

pub fn split(file_path: &str) {
    let content = fs::read_to_string(file_path).expect("should have been able to read");

    let mut control: u32 = 0;
    let mut sufix: u32 = 0;
    let mut text = format!("");

    let lines = content.split("\n").collect::<Vec<&str>>();
    let sz = lines.len();

    (0..sz).for_each(|line| {
        if control == MAX || line == sz - 1 {
            control = 0;
            sufix += 1;
            let file_name = format!("{}{:03}", PREFIX_NAME, sufix);
            let _ = File::create(&file_name);
            let _ = fs::write(&file_name, &text);
            text = format!("");
        }
        text.push_str(&format!("{}{}", lines[line], LINE_ENDING));
        control += 1;
    });
}

#[test]
fn test_odd() {
    let file_name = "input";
    let _ = File::create(&file_name);
    let _ = fs::write::<&&str, String>(
        &file_name,
        "edward scissorhands\n"
            .repeat((MAX * 2 + 1).try_into().unwrap())
            .try_into()
            .unwrap(),
    );

    split(file_name);

    let paths = fs::read_dir("./").unwrap();
    let mut acc = 0;
    for path in paths {
        let s = format!("{}", path.unwrap().path().display());
        println!("{}", s);
        if s.starts_with(&format!("./{}", PREFIX_NAME)) {
            acc += 1;
        }
    }

    fs::remove_file("input");
    fs::remove_file("x001");
    fs::remove_file("x002");
    fs::remove_file("x003");

    assert_eq!(acc, 3)
}
