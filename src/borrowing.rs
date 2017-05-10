
fn user(vec: &Vec<i32>) {
//    vec.push(3);
}

fn lender() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    user(&vec);
}

fn main() {
    lender();
}
