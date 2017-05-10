
fn take(vec: Vec<i32>) {
    //…
}

fn give() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    take(vec);
//    vec.push(3);
}

fn main() {
    give();
}
