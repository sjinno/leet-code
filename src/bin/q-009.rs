fn is_palindrome(x: i32) -> bool {
    match x {
        (0..=9) => return true,
        n if n < 0 => return false,
        _ => {}
    }
    
    let mut rems = vec![];
    let mut num = x;

    loop {
        let quo = num / 10;
        let rem = num % 10;

        if quo == 0 {
            rems.push(rem);
            break;
        }

        rems.push(rem);
        num = quo;
    }

    let mut m = 10_u64.pow(rems.len() as u32);
    let res = rems.iter().fold(0, |acc, x| {
        m /= 10;
        acc + (*x as u64) * m
    });

    x == res as i32
}
