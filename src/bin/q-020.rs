#[rustfmt::skip]
fn is_valid(s: String) -> bool {
    let mut stack = vec![];
    for c in s.chars() {
        match c {
            open if open == '[' || open == '{' || open == '(' 
                => stack.push(open),
            close if close == ']' || close == '}' || close == ')' 
                => {
                    if stack.is_empty() { return false; }
                    let p = stack.pop().unwrap();
                    match close {
                        ']' => if p == '[' { continue; } else { return false; },
                        '}' => if p == '{' { continue; } else { return false; },
                        ')' => if p == '(' { continue; } else { return false; },
                        _ => ()
                    }
                },
            _ => ()
        }
    }
    stack.is_empty()
}
