use std::io::Read;

pub fn read_from_stdin() -> String {
    load_text_input(std::io::stdin().lock())
}

pub fn load_text_input<R: Read>(mut input: R) -> String {
    let mut buffer = String::new();
    input.read_to_string(&mut buffer).unwrap();
    return buffer;
}
