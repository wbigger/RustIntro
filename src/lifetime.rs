

// fn skip_prefix(line: &str, prefix: &str) -> &str {
fn skip_prefix<'a>(line: &'a str, prefix: &str) -> &'a str {
    let (s1,s2) = line.split_at(prefix.len());
    s2
}

fn print_hello() {
    let line = "lang:en=Hello World!";
    let v;
    {
        let p = "lang:en=";
        v = skip_prefix(line, p);
    }
    println!("{}", v);
}


fn main() {
    print_hello();
}
