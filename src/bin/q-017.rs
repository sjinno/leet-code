// 17. Letter Combinations of a Phone Number
// class Solution:
//     def letterCombinations(self, digits: str) -> List[str]:
//         # If digits is empty, return [].
//         if digits == "":
//             return []

//         phone_number = {
//             '2': 'abc',
//             '3': 'def',
//             '4': 'ghi',
//             '5': 'jkl',
//             '6': 'mno',
//             '7': 'pqrs',
//             '8': 'tuv',
//             '9': 'wxyz',
//         }

//         for d in digist:
//             if d == '2':

//         stack = []
//         for d in digits:
//             stack.append(phone_number[d])

//         tmp = [ch for ch in stack.pop()]
//         # If stack is empty, that means digits has the length of 1.
//         # So return tmp.
//         if stack == []:
//             return tmp

//         ans = []
//         while stack != []:
//             chars = stack.pop()
//             if tmp != []:
//                 while tmp != []:
//                     c = tmp.pop()
//                     for ch in chars:
//                         ans.append(ch + c)
//             else:
//                 while ans != []:
//                     c = ans.pop()
//                     for ch in chars:
//                         tmp.append(ch + c)

//         return ans if tmp == [] else tmp

fn letter_combinations(digits: String) -> Vec<String> {
    if digits.is_empty() {
        return vec![];
    }

    let mut stack = vec![];
    digits.chars().into_iter().for_each(|d| match d {
        '2' => stack.push("abc"),
        '3' => stack.push("def"),
        '4' => stack.push("ghi"),
        '5' => stack.push("jkl"),
        '6' => stack.push("mno"),
        '7' => stack.push("pqrs"),
        '8' => stack.push("tuv"),
        '9' => stack.push("wxyz"),
        _ => (),
    });

    let mut ans = Vec::<String>::new();
    let last_elt = stack.pop().unwrap();
    let mut tmp = last_elt
        .chars()
        .map(|c| c.to_string())
        .collect::<Vec<String>>();
    if stack.is_empty() {
        return tmp;
    }

    while let Some(s) = stack.pop() {
        if !tmp.is_empty() {
            while let Some(c) = tmp.pop() {
                s.chars().into_iter().for_each(|ch| {
                    let combination = format!("{}{}", ch, c);
                    ans.push(combination);
                });
            }
        } else {
            while let Some(c) = ans.pop() {
                s.chars().into_iter().for_each(|ch| {
                    let combination = format!("{}{}", ch, c);
                    tmp.push(combination);
                });
            }
        }
    }

    if tmp.is_empty() {
        ans
    } else {
        tmp
    }
}
