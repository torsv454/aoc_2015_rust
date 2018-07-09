use std::fs::File;
use std::io::Read;

pub fn slurp(file: &str) -> String {
    let mut f = File::open(file).expect(&format!("File {} not found", file));
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    return contents;
}

pub fn input(day: usize) -> String {
    slurp(&format!("src/bin/day{}/input.txt", day))
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
