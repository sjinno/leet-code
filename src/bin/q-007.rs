fn reverse(x: i32) -> i32 {
    let res = x.to_string()
               .chars()
               .rev()
               .take_while(|c| c != &'-')
               .collect::<String>()
               .parse::<i32>();
    
    if x < 0 {
        -1 * res.unwrap_or_else(|_| 0)
    } else {
        res.unwrap_or_else(|_| 0)
    }
}