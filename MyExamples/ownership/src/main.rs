// Playing around with String and str types examples to understand Ownership
// https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html

fn main() {
    let mut s1 = String::from("hello");

    s1.push_str(", world!");

    println!("s1 = {}", s1);

    // The following 2 lines will not compile - s1 becomes invalid after assigning to s2
    //let s2 = s1;
    //println!("s1 = {}, s2 = {}", s1, s2);

    let mut s3 = s1.clone();
    println!("s1 = {}, s3 = {}", s1, s3);

    let s4 = add_me_to_string_v1(s1);
    println!("s4 = {}", s4);
    // the following line will not compile, as s1 loses ownership via the add_me_to_string_v1 function
    //println!("s1 = {}", s1);
    let mut s5 = add_me_to_string_v2(&mut s3);
    println!("s5 = {}, s3 = {}", s5, s3);

    add_me_to_string_v2(&mut s5);
    //println!("s6 = {}, s5 = {}, s3 = {}", s6, s5, s3);
    println!("s5 = {}, s3 = {}", s5, s3);
    
    let s6 = remove_letter_l_v1(&s5);
    println!("s5 = {}\ns6 = {}", s5, s6);

    remove_letter_l_v2(&mut s5);
    println!("s5 = {}", s5);

    let sl1 = &s4[0..5];
    println!("sl1 = {}", sl1);

    // pass a string slice arg (str)
    let mut s7 = remove_letter_l_v3(&s5[0..5]);
    println!("s7 = {}", s7);
}

// takes ownership
fn add_me_to_string_v1(mut s: String) -> String {
    s.push_str(", from CRAIG1!");
    s
}

// borrows mutably - no need to return string here - just done for example purpose
fn add_me_to_string_v2(s: &mut String) -> String {
    s.push_str(": from CRAIG2!");
    s.to_string()
}

// borrows immutably
fn remove_letter_l_v1(s: &String) -> String {
    let bytes = s.as_bytes();
    let mut result = String::from("");

    for (_i, &a_char) in bytes.iter().enumerate() {
        if a_char != b'l' && a_char != b'L' {
            result.push(a_char as char);
        }
    }
    result
}

fn remove_letter_l_v2(s: &mut String) {
    let bytes = s.as_bytes();
    let mut result = String::from("");

    for (_i, &a_char) in bytes.iter().enumerate() {
        if a_char != b'l' && a_char != b'L' {
            result.push(a_char as char);
        }
    }
    *s = result;
}

fn remove_letter_l_v3(s: &str) -> String {
    let bytes = s.as_bytes();
    let mut result = String::from("");

    for (_i, &a_char) in bytes.iter().enumerate() {
        if a_char != b'l' && a_char != b'L' {
            result.push(a_char as char);
        }
    }
    result
}
