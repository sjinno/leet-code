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
