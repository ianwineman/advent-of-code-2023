use regex::Regex;
use itertools::Itertools;

fn main() {
    let input: &str = "..??#???##??#?? 4,2,2
.#?????????.?. 9,1
.????#..??#? 4,2
??#.#???#? 2,1,1
?#??.???#?#????? 4,1,1,2,3
.???#???#?#????? 2,3,3,1,2
???.????.#??###? 2,3,6
????#?#?#?.??# 1,6,1
.??????.?##. 1,3
?????.?#??.#????#? 1,1,1,3,2,4
?????##?##. 1,4,2
?#??.?????.# 2,1,2,1
?#??.??##???????? 4,2,3
?#????????##?. 1,1,2,3
??#?????.???. 2,1
.?????#??#?#?#??.. 2,9
???..#.?.?#?????# 2,1,3,1,1
????#????#??#???. 1,3,2,4
##?#..?????? 4,3
#???#?.?.?#?? 6,1,1
??#????#.?. 2,2
????????#??? 2,1,6
#?????#.##???. 5,1,5
?????.???.????#?. 1,1,1,6
.#?????#?..?.. 1,2,2,1
?#??#???.??##?# 7,1,2,1
???.?????.##..??. 1,2,1,2,1
.?????..?#? 3,1
??.#?#?????#?#?. 1,11
??.????????.##??? 3,3
?#????#.????? 1,2,1,1
?##??????#.????? 9,1
????????.# 1,2,1
???????##??#.??. 1,3,4,1,1
#?####????.??????. 9,3,2
??#?#????.#??# 7,4
????###.???#? 6,4
??#????#??. 6,2
?.?#.?###????. 2,5,1
??????#??.??#?#. 5,5
??..??????????#?#?#? 1,6,5
?#??.??#?#??#?.?#. 1,1,1,1,2,2
?.?#?#??.???? 6,2
..?#..???#??##???. 2,7
?.?????#??#???? 1,1,1,6,1
??????###? 1,1,3
.#???.?.?##????#? 3,1,8
#?.????.??..?#?????? 1,1,1,1,8
?.???????#?? 1,4
??.???#?..? 1,4
#.????.?#?#??#.? 1,1,1,7,1
.??????###???## 1,2,3,1,2
#??????..?.? 1,1,1,1
???#??#?#?? 3,5
??????#?????..??#? 3,3,1,1,1
????##??????.?#??#? 1,1,2,1,1,6
.?#?..??????. 2,4
??????.??#? 2,3
??.?#??##????#?????? 1,8,1,1,1
??????????#?? 4,4
????#???#?.? 5,1,1
???????#????#? 5,1,3
????.???#????? 2,7
??#?##????#??????.? 12,1,1,1
????#?????. 4,2
..#??????# 2,2,1
??????#???.????.?? 1,1,1,1,3,1
.???##???????#?? 1,7,4
??.??#?#???????#?.. 1,13
##.?#???.???#???.?#. 2,1,2,7,1
??????#?????.????. 10,2
??.???.??.?????#??? 2,1,1,3,1,1
.??#????.#?????#. 1,2,1,1,1
?##???#???.???#?? 6,1,1,1,1
?.???????##?.#?# 1,1,4,3
?.#???#....#?????? 1,1,2,4,1
?????????? 3,3
.#??..#??..#?#.???? 2,3,3,3
??.??##..????.???? 2,1,2,2,2
..??????#?. 1,1,1
#??????.#?##????#??? 1,3,6,1,1
????#?#??#?#?##???? 2,1,6,2,1,1
???..???#? 1,3
?#??????#..?#??.#? 6,1,2,1,1
?.???????##??..? 2,7,1
????.??.?.?? 3,2
???????#??.?? 9,1
??.?.?.#?..???.#?? 1,1,1,1,1,3
???#?????????#?? 4,9
#.??##?#??..???#??? 1,1,6,4,1
?#????.?###?#???#??. 4,9
#???##?#??#.. 8,2
.#?#?.??#??#.?#???? 4,6,3,1
##?????##??#.??.?.. 12,1
????#??##?#.???#? 4,2,1,5
??.??#???##?? 1,1,5
??#????#.??#???#? 3,3,7
?##???????.??#???#.? 5,2,1,2,2,1
????#??.?#?.? 4,1,1
????.??#??#?#???#.#? 4,10,1,1
?.???#.??#? 1,4,2
??????.????#???#???# 4,2,1,1,3,1
#??#??.#?.#?.?? 1,3,1,1,1
????.?????.????? 3,1,1,1,1
??.?.?#????. 2,1,2
??????????.?#.??#?? 3,4,1,1,3
????#?.?##??? 3,4
?#?.???.??..#? 1,1,2,2
???#?#??#?????#???#? 9,1,3,2
?.???#????. 3,2
?###?#?#?#?#.???? 11,1
?????###??. 2,6
.?#.?#..?. 1,1,1
??????####???????.? 1,6,3
????#??????..#?#??? 1,7,4,1
???#????#.? 3,1
?#.??#????.?#???? 1,7,3,1
#??#.??#???#? 4,7
#???##??.#..???? 7,1,1
?..???##?#???#? 4,1
.?#????????#?.?? 4,5
????.##?#????##??#?? 1,11,1
#??..#.??##????#? 1,1,1,6,2
???##?#?????.??? 1,9,1,1
?.#?????##??.?#?? 3,4,3
??.#??#.#? 1,1,1
.?.???##??#?.##?? 7,4
#?????????? 1,4
???#???.?????. 3,1,3,1
#?#?####???.#??. 10,1,1
?#????.????..# 1,2,2,1
?#?#???#?.#.??#.?? 4,3,1,1,1,1
???##?##?.?????#??. 7,6,1
?????.?#?# 1,2,3
?.??#?#???.?. 1,7
??.???#?#?#?##? 1,7,2
.?#?.????#?? 1,1,3
#?..#??????.??#??? 2,7,5
?.?#?.?.?????????# 3,9
????????#?##???. 1,3,6
????#???##?????#. 14,1
..???????#? 4,1
???.?####??? 1,5,1
.?#???#?#???? 1,7
???#?#.?.?????.??? 4,5
.?#?#?????#??# 4,1,1,1
?.??###.??.??.??# 1,5,1,1,1
?..????.?#.????? 1,1,1,1,3
?#???#??.????.???? 1,1,1,2,1,1
.???????.?#?#???.? 1,1,3,5,1,1
??#?#???.?????#??.? 6,1,1,4
?#?#????#?#????.#?# 4,9,1,1
?#????..?##?????? 4,1,4,1
???.??#??.#?#?? 2,3,3
????#???.??.??#? 1,5,1,1
...?????.#?.? 2,1
?#???#?.?#???.# 1,2,1,2,1
.?#?#??#??##???#?.?? 1,1,7,2,2
?#??.???????.### 1,1,4,1,3
??#?????##??????#? 4,6,1,2
??????#????????##??? 1,1,1,1,1,9
??#?.???#??#??#.?.. 3,8,1
.?????.??? 2,1,1
?#????#???. 1,4
.??#?##???.????##?? 5,1,8
??#???#?#??#?.??? 1,8,2
??.???????#??. 4,4
.??.?#?##???#??# 1,11
??.??????? 1,1,4
??.?#?###????#?#.## 2,5,1,1,1,2
?##???#???. 3,5
.?#???#?##?#?.#? 1,8,1
???????#?#??? 4,5,1
????#??.??? 2,2,1
??????#?##??#? 1,1,5,2
?..###???.??#???? 3,1,6
.????..???# 4,1,1
.?#??#??#?## 1,8
?.???.????#????.? 1,7
???????#??.#?????? 1,7,1,5
#?????###?#?#???? 2,8,1,1
##????????# 2,1,3
???#.????#?###?#???. 1,9
.???.????# 1,5
..#?#?????#?? 3,2,2
?.?..??????????????? 3,6
??#????.??#??? 4,1,3,1
.?##???#?###??#??? 10,2
??.?##???.?#??? 6,2,1
??##?.?#???#???? 3,1,1,1,1
????????????#??????. 7,8
??#.#?.??#?..?#.? 2,2,1,2,2
???.?#?????? 1,2,1
?.##??.???#??? 2,1,3
??.???#??????????# 1,5,1,1,3,1
???.??.?.?? 2,1
???#?##.?????#.? 6,1,1
??.????????? 1,2,2
???#??.##?? 4,2
??#?##?##?.?..??. 10,1,1
???????#?..????????. 2,1,1,8
??.?#?##?? 1,2,1
#??#????.#?? 1,4,1,1
?????.???????????. 2,10
??.????.??? 1,2,2
.?.?#??.??? 4,1
?.?#.#?.?###??##?#.. 1,1,1,4,4
#???#???.?#????#???? 8,2,3,2
..#?#..???. 3,1
?.?????#.?.?.#? 2,1,1,1,1
?.?.???.#???? 1,2,1,1
??##??#.?#.??#??.?.? 5,1,2,1,3,1
??.????????????#??. 2,4
.#??.?#???. 1,2,1
#???.?#?#?????.?? 2,7,2
???????#?### 4,2,3
#?#??.???????????#? 5,1,1,3,2
?????.????? 4,1,2
????.???#?#.. 2,5
?.??#.?.?? 1,1,2
?.?????????#?. 1,1,1,3
.??.??#??? 1,3
.???#?##???.#? 2,5,1,2
??#?????.???##??? 2,2,2,2,1
???#?????.#????? 1,6,3
?##????#??.#??? 5,3,2,1
?.???#????### 1,1,1,6
?.???..??..??? 1,1
????.???????#?#?? 2,12
??????.###..?#??#?? 4,3,5,1
#.?#???????? 1,4,1,1
.?????..#??????. 2,1,1
?#?#??.???##?#?###. 5,7,3
???#?##?#?? 5,1
.?#?..?????#? 3,1,2
???.?????.????.??? 1,5,1,2,1
?????.?##??????? 1,1,3,1,1
?#.???????. 2,1,4
?.#.??????#?#.??? 1,1,5,1
?????.?.??.?#???? 1,1,1,1,1
??.#?.?#?.??.???#?## 1,2,1,1,6
???#??????#?#?##? 1,5,1,1,4
#??.#????##??#??#??# 2,2,11
.????#????#?????? 6,3,2
?#??????.? 2,3,1
?????.??##?.?#???### 4,1,2,8
#??.?.?.##??. 3,1,2
.#?#.??#?. 3,2
?#?#?#??#?.?????? 8,1,2
?..??##.??.??? 3,1
??#?#????.??? 1,5,1,2
??#..????#???# 2,1,3,2
#???##????.#???? 2,7,2
?.??????????##? 1,1,1,4
?#??..??.?##? 2,1,4
???????.?.. 2,2,1
?..?##????? 1,3,1
??????#????????. 7,1,2
???????#..#?#??? 6,3,2
????.?????#????? 3,1,6
???#??.??????#?? 3,2,2,1
#.????#.???#??#??? 1,4,1,7
????####???#?#??? 2,4,2,1,2
???.?..#????#?#???#? 1,1,13
??..?????? 2,1,1
???#?#?#?#??????#??? 1,8,1,1,1,1
?.#.?#???? 1,3
?.##????#????..? 2,6
?...???.#?????###. 1,1,1,1,5
???#????#. 1,1,3
?#??????#?#. 3,1,3
.?#??.#?##??#?# 4,9
?.?#???????? 5,1
?.?#?????##?.??????? 3,3,2
????##???.?????.#?.? 2,6,2,1,1,1
?????.?.###???#.?? 2,1,1,3,3,1
?????.?#??? 2,3
?#??.??????????? 3,2,4
.??..????? 2,2
?#?..?..#?. 3,1,1
.?.#???????#? 6,1
??????.???#?. 4,2
?????#?.?. 2,2
?#?#??.?????? 6,1
??#.?..???. 2,3
?##??#?#??.???##?#? 2,6,6
?#???..??#??#. 4,5
#?#??#?????.??.?#?? 11,1,2,1
????#####?##??.?? 14,1
.?.????.#??#.??.?. 1,2,4,1,1
?.?.???###??.. 1,6
??#??###?.?.? 2,5,1,1
..##?#?#?#? 2,1,3
?????.?#?. 2,3
#??#??#.???#?#.??? 7,1,4,1
??#..????#???#??? 3,3,1,3,1
?#??#??.##???.??#? 7,5,2
?????...???? 3,2,1
.?##?.???.. 2,1
?.##??#??..??? 5,1,3
?????????????#??? 1,1,7,4
.?###?#???? 5,1
?#?..??.?? 2,2
.#??#????#?.??#????? 6,3,2,1,1
.?#?#?#???#.#???. 7,1,1,1
??.????#?. 2,1,2
#????????# 1,1,2
.??.????#???..#..? 5,1
.?#???#?#?? 3,6
?.???#??#????#??# 1,12,1
.#.???.??#? 1,2,2
#?.??????#? 1,3,3
??.#?.??#?##??#?# 1,1,1,7,1
?#?.???#??# 2,5,1
??????#?##?????.# 3,9,1
?????##???#?#?#?#.? 1,12
??.????#?##.#?. 1,7,1
?#?.?##????###?#?.?. 1,12
?????#??#??????? 4,1
.#????#??#??#.? 6,5
.?##?.???. 4,1
??####?.#????##??. 6,2,3
?#?#?##??#???? 10,2
??#???...?#?? 5,2
????????## 6,2
?.#?????###?#??. 1,1,8
.????#?..?? 1,2,1
.?.#??.?#? 1,3,1
??????.??.???#?? 1,3,2,6
?????.???? 1,2,3
????????#?????.? 6,1,2,1
.??????##????#? 1,1,2,3
???..#?#??????.??.#. 2,5,3,1,1
##?#?..??#.. 5,1
?????#???.. 2,1
.?#??.?#??.#???.?#? 1,1,1,1,4,1
#?.#####?.?## 1,5,3
????.#??.????#?#??? 1,1,3,1,8
???.???#?##?? 1,7
????##???.??????#?? 6,7
??????#?.# 2,2,1
?.?###??.?..????? 5,3
??####??????? 4,3
..#??#???? 2,1,1
?.#?.????????? 2,3
?.????.?##???##??.?? 1,1,1,9
#.???.?#??#?#?#.? 1,2,3,5,1
???????##.??????. 1,3,1
?#?????.????? 6,1
#???????#???? 1,1,1,5
.?????#?.? 1,3
??#?#?????? 3,1
?.??????????##?#.? 1,8,5
?.?.??.???.??..? 1,1
#???????.?.??? 7,1,1,1
.?.?.?????? 1,4
?##????????# 3,1,1,2
.#????##.?? 2,4,1
?????#????? 1,4
??###?.?.?? 5,1
##?#??.?#? 4,2
?#?#?????#?? 2,4,3
????...????.? 2,1
?.??????#???# 1,3,2,3
???.?.?????? 1,1,4
?#?#??#???????????? 5,2,8
????????#.?.?? 7,2
.?.?????.??. 5,1
?.??????..? 1,2,1
?#?#??##?.??????. 2,1,3,4
??????.???#?? 5,1,1
??#####?.??.??#??? 8,2
.??.????#????#??. 1,12
??##???#????##?# 3,1,1,3,1
??##????#?..?? 3,2,1
??#??#??#??.??? 10,1,1
..?#?#.#???.?.#. 4,4,1
..??##?.?.? 4,1
.?#.????#.? 1,3
?#????????. 5,1
??.?????.#.?##??#..? 1,1,2,1,6,1
??#???#?#?#?? 4,6
?#.???#?##????#?? 1,10
??.??#.?#?? 3,3
#???#?..???.??.? 1,3,3,1
.?#?????.??? 6,1
???.?????#? 1,2,2
??.?#?#??.?? 1,1,3,2
?#??.???#.?????. 4,1,2,1,1
????????.#??#.??#.? 1,1,1,4,3,1
??????##?#????##?. 3,5,3
??#????#???????## 4,1,1,1,3
####?.???##???#.?. 4,6,1,1
??????#?????#???#? 3,2,2,2
????#.?####????? 3,9
.????????#???# 1,8
?????.??..?? 3,1,1
####???#???.???#??? 4,6,1,4
#??#???#???#?????? 5,1,1,1,2
?#?#???..?? 5,2
.#?#???.?.? 5,1
???#???#?#? 3,3
?#..???##????? 2,1,2,1
???.??#????# 1,3,1,1
??????????? 1,1,6
???.??.?###? 3,4
?.??#.??#?##??? 1,5
??.????????#???? 1,4,3
???#???#???????.??? 9,1,1
.??##??.?#?? 4,1,1
????#?????????? 1,2,6,1
?????.???????#?????? 5,1,11
.??????????#??? 2,1,2,4
??????????..???????? 10,2,1,1
?#.???????#????.?? 2,7,1,1
???#?.???.???#?#?#? 1,1,2,1,5
??##???????#?##? 3,2,5
?????#???.?#.?? 6,1
..???#?#???#. 1,1,2,1
???#?##.?#??###?#? 1,4,10
????..??#?## 2,1,6
?#???????. 3,1,1
?#?????##?#?#?????? 3,13
???#????##?? 4,4
?????.????.?#??#?. 5,3,5
.????#?????##?? 2,3,5
.?.?.##??? 1,4
.???.#?.?#??? 3,1,3
??.#??#??#???#??. 9,3
???.???????#?#?# 1,2,3,1
?#?#???#??.#?..?#?? 6,1,1,3
????.?#??#???. 1,1,6,1
..??????#?? 1,4
##.?.??#.? 2,1,1
??##?##?.????##?. 1,5,1,4
??#???????. 2,2,2
??????.??? 1,1,1
??#??????.?#?.# 1,6,1,1
?.?.????????????? 1,1,1,8
????.????# 2,2,1
??#?#####??.??? 2,5,1,2
#..???##?#? 1,1,6
#????.?#???#???# 2,1,7,2
.#?????#?#??.# 1,4,3,1
??#??????#.#???# 2,1,1,2,5
?.???#???? 1,2,1
????.?.?????????? 3,1,2
?#??????#?????.? 4,6,1
?.#?#??.#????.?? 1,3,3,2
#?????.?#?##?.#.?..? 6,5,1,1,1
???.?#.??.???...? 1,1
.??????..????.? 1,2
#???????#?#??.?????? 1,1,2,3,1,6
??.?????????? 1,1,5
#??#?.#?#. 5,1,1
.#??.?????. 2,4
.?#???###???? 1,5
?..?????#? 4,2
?.#??????#????###?.? 1,1,9
.?#??????. 1,1,1
?##.?????. 2,2,1
?#????.??#????. 3,1,3,2
???#??#?##?? 1,1,5,1
.????#.?##? 4,3
?.?#?.##?#??????#?#. 1,3,13
??????##??#?? 4,3,1
???.#?#.?..?#??##?. 3,2,2
?????????.? 1,6
????#?#?.????.?.??# 1,1,2,1,1,3
???#?#??#.??.????.?? 7,1,2,1
??##?.??#???#? 1,2,3,2
??????#?#?###??##?? 11,3
#??????##..??????. 1,7,5
??.??...??????#??#. 1,10
??#????#??#.?#??#??? 1,1,1,1,7
??.?#??#???#???.#? 1,1,8,1
#??..?#?.#??.#?#?? 2,2,2,4
?##????????#??????# 7,6,1,1
???.?.#??#?????#?.? 1,1,1,3,2,1
#...??.????#????. 1,2,6
????#.?????#. 3,1,1,1
?.??#..???????? 1,3,1,2,1
?????.?.#.???.#..? 5,1,1,2,1,1
?#.?#?.###??????. 1,1,7,1
?.?#????###?#.?# 1,2,5,1
.?????.????.??. 3,3,1
????#?????????????# 7,1,2,1,1,1
?#????##?????. 2,6
?#??##?#?#.???#?# 10,3
#?#?#?????????.??#?? 8,2,1,4
?#??####??.?##. 8,2
???.??????? 2,5
????##?##???#????. 1,12,1
??????.#?#?#??? 5,7
??#?#????#?##??.?? 8,1,2,1,1
#?????.??#?## 1,2,4
?????..??????#????? 4,8,1
?.??#?#??????#.????? 4,3,1,2
.???.#????. 1,2
???##?###??...? 9,1
.#????.???#?. 2,5
...#??#????? 1,1,4
.?#?#??.#???#?##??? 2,2,1,9
.????#??#???##.. 2,9
...#???#.?????. 1,3,3
.###?#?.?????#. 5,1,2
?????#.??#?#?.#?#.? 1,2,4,1,1
#?..?#???#?#? 2,7
??????###?#???. 1,7,2,1
?#??.?????? 3,1,1
??.#??..?##?#? 2,3,6
?????.??????#?.??## 2,1,1,2,2,4
??##?#??#????#..? 8,2
???.#.????? 3,1,3
?.#??.#.?? 1,1,1
??###??#??.###?.? 10,3,1
.?.?#??.?.#???????? 1,4,2,5
#??.???.?#?#????.? 1,1,2,5,1,1
######??.???. 6,1
??.?#?????#?#?. 2,11
?.?.?...????.?#.? 1,3,1
??.##??.???.?????? 1,2,1,5
??.??..?#?#???..#?. 1,1,7,2
?????.??#??????##?? 2,11
#??#?#??..??..?.. 8,1,1
#?#???#?#??#????#?.? 1,15,1
?.?##???#.?#. 7,1
???..?.?#?? 2,2
???#?#??.?? 6,1
?..???????? 1,1,2
?.?????????? 4,1
?#?.?.?#?????#?# 2,1,1,3,1
??#??#?.???##. 4,4
.??.???#..?? 1,4
?.???#?#?? 1,1,4
.??????.??????#?#.. 6,1,1,5
#?#????##?#?.##? 4,3,1,3
.?#??????????? 1,1,8
???????????#? 6,1
???.#?.#?#???#????# 3,1,1,1,8
?#????#?.?#???. 2,2,3
??#.?????.??#??#?? 3,1,3,1,1,1
?.?#??##??? 2,5
.?#???#??#??? 1,7
.?.?????..???. 5,1
??????.??? 1,1,1
.??#???#?.??#??# 1,1,2,1,1
???#??##?#?# 1,1,7
?.#??.?..#.??#?.#? 1,3,1,1,1,1
???#?###?..?.??????? 9,1,3,1
..????#????# 1,7
##??????#??#?? 3,2,4,1
?#?.????#?? 2,2,1
?##??#?#???????..#?? 6,1,1,3,1,1
?.??#?#??..?????. 5,2
????.?##.? 1,1,2
???.?#?????????#??? 2,3,8
##??#.?#?#?? 2,1,1,1
?#.?#??.?? 1,2
???#?#??...##?#??? 5,5
#??##??#????..#??? 5,2,2,3
#??#?????? 4,2,1
????.?.???#? 3,1,4
??????#??? 4,3
????#??????#????#??? 10,2
????#???????#? 1,2,8
?.????#???#.#??.? 1,6,1,1
?.???????????#?#? 2,8
???????.??? 1,1,1
??????#??????.? 1,1,3,1,1
????###??.?.???#? 7,1,2
???.????#???? 2,1,5
.?#??#???#? 1,2,2
###?.???##?? 4,6
????#??.????????. 3,1,1,2,1
??#????#????##?.??? 10,3,1,1
???????.???##???#?# 4,1,9
##?#???#??.?????.??# 4,4,2,1,2
??????????##????#??. 1,12
?##?????###????.?? 12,1,1
???#??..?#?#??.? 2,5,1
??#.?.##???.##??#?.# 1,1,5,5,1
#??????????.#???#? 1,5,1,5
?.??#..?#??. 1,2
.?.??#??###????????? 1,11,1
???.?????#??????##? 1,3,3,1,4
???#??.??.????#? 4,1,1,1,3
?.????.?#??.?#??#?? 1,3,4,2,3
?.?????.???##??#??? 1,1,1,1,7
?##??#???????? 5,1,2
#?????.#????#???##.? 3,1,1,9,1
#?.?.#???#???. 1,1,1,3
????#????##.? 5,2
##??.????##?. 4,1,3
???????.#? 5,1,1
#???#????#??. 1,3,3
???.?.?#?????#?# 3,1,2,4
?#??#??##?. 2,5
???#?##??.??.#?.??? 5,1,1,2
??????#??##????..#? 1,10,2,1
?.?.?.#?###??.?##? 1,1,5,1,2
???????#???. 1,7
#???..??????? 3,1,4
??#?#..?#????.?# 5,1,1,2
?????????#???#.?? 3,2,2,3,1
?#?.??##????#?##? 2,1,11
???#?????# 1,5,1
??##??..#?????# 5,7
??#?????.???##.#?#? 6,1,2,3
????#????. 1,2,1
..??##?#?#??#?.?.?. 12,1
??##?##?.??.. 6,1
?##.??#?#??#?? 3,7
.?.??#??#???#? 1,7,1
??#?##?.#?#??.???? 6,5,1
???#??????#?.???.? 1,6,2,1,1,1
?????????###??# 1,10
?.#???????.? 6,1
.??#.?.?????#??. 1,1,2,3
???.#???.? 1,1,1
????#?????##.#? 1,2,2,2,2
??.#?.?????????.?. 1,1,6,1,1
????#????##???????.. 11,3,1
.?#.????#?. 2,1,2
#?#?.?????#? 4,1,3
??????#???????? 2,2,1,1,3
.????.???#?#?.???##? 4,5,2
?.##??##????##??.?? 12,1
??????#?.?#??. 6,2
??#?.?..#.??##??? 2,1,1,5
.#???#..??#????#??? 1,3,2,4
??#?????#?#?.?????.? 9,4,1
?##?.?##??##?##.?? 2,10
.?.??#???????#?#..? 1,2,1,5,1
??????#.?.??#??#.# 3,1,1,4,1
#?????#??#????##. 12,3
???.???.#????##? 2,3,3,3
#?####?#?#.??????# 10,1,2,1
??.???#??.????.??.? 1,1,4,1,1,1
????????.#? 4,2
..????#.????.?.?#? 2,1,3
?????#???.???? 1,1,3,1
??.#..?#???# 1,1,2,1
?##?????##? 2,1,3
?#?..??.??? 1,2,1
.????#???##.?? 1,6
?#??????#????# 3,3,1,1
.???.???????##?# 2,5,5
??.??????.?????? 1,4,3
#??#???#????.??##. 1,7,1,2
???.??#???##??. 2,1,1,3,1
?#?##???.#?##? 5,1,5
??.?#?.???.???????.? 1,3,1,6,1
?#.#????.??#??#??#?? 2,1,2,4,6
..????..???#???#???? 1,6
#??#???????.???#?#? 1,5,1,1,6
???##?????..#??????? 7,8
?##?????..?????? 5,1,1,1
#??????.?#?. 2,3,2
???.???????????.??. 1,1,1,7,1,1
.??#????##??#.#???? 1,2,6,2
?.##?#??????#??#?? 7,4
?#.##????##?? 1,3,4
..???.????. 2,3
?#?????#?#.? 2,1,3
???..??.?? 1,1,1
??????????#?#???#??# 3,12,1
#??##?#????.. 1,6,2
???.????#???? 1,1,3,1
#?????##?#?##??# 1,1,10
?..##????? 1,2,1
?..?.????#? 1,2
.???.?.????.??#??? 2,1,4
???????#?. 3,3
??????.#???.???? 1,1,2,1,1
.?????.?#?#?. 4,1,2
????###?##?.?#?.? 1,8,3
??#???????#.????. 5,1,1,1
.#?..??.???#.??#? 2,1,3,3
?#??#..????..?.????? 5,1,1,1,1,2
?#?#?#?????.# 1,4,1,1
.?##?#?#..???????? 2,3,6
????#?????#?#. 1,4,3
???#??#??#??#?? 8,1,3
?##?#?#????#?#?#??.? 2,10,3,1
??.?.??#????#?#??. 1,1,2,4,3
?#???#??#???.#??? 2,1,5,1,1
???#??.??###.?#? 3,5,2
?????.??#??. 2,1,3
#?.??#????????? 2,5,1,1
???.???#?####??#?? 2,1,9
#..#.??????#?#??.??? 1,1,10,3
?###??#??????#?#?.? 8,4,2,1
?????#???..????##?#? 6,5
#??#??.?#?##?????? 2,1,1,2,5,1
??????.??? 1,3,1
?#..#..?????..?.? 2,1,5,1,1
.#??#???###??????.#. 14,1
?.?#???#?????????#? 1,1,3,1,1,6
???##????#???#??#?# 5,1,1,6,1
?#????.?????##??.? 4,2
????.???###??#?????? 1,1,15
#??#???#.???? 8,2
##?#???#?.?? 5,2,2
.?#??##.?. 2,2
?#??##???? 5,2
??#?????#?##??????? 12,2
??????#?#..????? 1,6,4
.?#??#??????? 2,1,1,1
????#?#??#???#????? 1,3,9,1
??????#?????.#?# 10,1,1
??#?.??##?###?? 3,8
??#.??##?##??#? 2,1,8
??#??#??.???? 1,3,1,1
#?..?#??##??????.?? 1,6,1,1,1
?#?????##????#?? 5,4,4
??#????#.?? 2,2,1
??#?.???##..##?? 1,2,5,2,1
???#?.????????#. 1,6,1
????##?#?##?#.??.? 2,2,4,1,2,1
???#?.??#?#??#. 1,1,3,1,1
.?#.?#???.#? 1,1,2
?#.??####?#????? 1,11
??????.??.##?#??? 4,1,6
?.??#??.???. 5,1
??.#?##.?????##?# 1,1,2,2,6
????.????#?#? 4,6
#??????##?????.#?. 12,1
.#?????#?.#????##??? 8,4,3
????#??#?##??. 11,1
??..#??#??##??#?. 1,1,1,4,1
?#????????.#???.? 4,3,3
??.#????.?#??????. 1,4,1,1,2
??????#??#?##???. 6,5
.????????#?#? 4,4
.##..?????# 2,5
??????????.? 1,5,1,1
?????#?????.?.?? 1,4,2,1,2
.????#?.??#?#???? 2,2,8
#?????#?##.???? 1,1,6,1,1
?????????? 1,4,1
.#?.#?????????? 1,3,6
#.?.????#?? 1,5
??.##?????.?#?#??#? 1,3,3,1,2,2
#?#?..#?#????? 4,3,1
????.?.?#????????#? 3,11
##????????.. 2,2,1
???#??###.?##???#?? 6,6
??...?????? 1,4
?#.?.?.??? 2,1,1
??#?.???.?.?#.??.?? 2,3,1,1,1,1
.??..??????#???????? 2,6
?????.???.#?..?# 4,1,1,2
?#??#????? 5,2
??????.????##?.????? 4,1,2,3
??#?#?#?.#???? 3,1,2,1
.?##.?#.?.??# 3,1,1,3
??#?????#?#.??? 9,2
.????#???#? 1,3,1
..??.????..?#?? 1,2,1
??#?.????# 1,1,5
?.????.???##?#? 1,1,2,5,1
?????####?#?#??.??.? 12,1
#.?.?????????.#???? 1,1,3,1,1,2
.?#??##?????.#?? 10,1
????????#??????..??? 1,1,6,1,1,1
??#??#???#?.?.?#. 8,2
????..??????.??. 3,4,1
??.???##???.?..???? 6,2
????##..??? 1,2,3
.#?..???????. 1,1
?????#????#??????? 1,1,1,1,6,1
?.????#???? 3,2,2
?????????.??. 3,1,1,1
.?????.??? 3,1
???.?????????? 1,1,6
#???#?#.####?#??#?#? 1,4,11
..??#??#?...?.??.?? 6,2
?#???..?#?#????? 2,1,4,1,1
????????####???? 2,9
??.#?..#????#?..???? 1,2,1,2,2,2
??.#?.???? 1,1,2
?????#?.?.#?? 4,1,2
?????????.?# 1,1,1,1
?????#####??? 1,8,1
.????#?#??#.???? 8,1,1
???##??#.???? 4,1,3
.??????#??? 1,5
.??#?.??..#?? 2,3
??##?#?????#???? 6,1,1,1,1
??#?##.#??#?.?. 5,4,1
#??##????.??#?? 5,1,1,1,1
?##???###.# 4,3,1
???#?????????#?.? 4,2,5
#.?.?.?#??.?? 1,2
.?#.?..????? 2,2,1
?.#?????????? 1,1,3,2
???????.??? 2,1
?##?????????????.. 2,4,4
???#?#?....????.?.. 3,1
##?#???.??? 4,1,1
.???#????.#?????# 4,2,2
.?##?.?#???#??????? 3,4,1,1,1,1
???.#?#?????#?. 1,5,1
??????????##??###.. 6,4,3
??.?#???????.#? 3,1,1,2
#??#?#????#?#???#.? 2,6,1,2,1,1
#?#???????#???#.???? 1,5,1,2,1,1
?#????#????##??#? 2,1,7,3
?.#??.?###?.#? 2,5,1
??##???????????#?# 1,3,1,2,1,1
????????.??# 4,2
????.??#??? 3,2,1
??#?#?..??#?# 1,4,3
#????#??.##.????## 1,1,3,2,6
??..?#???.??#?. 1,1,1,1,2
???.??#?????? 4,1
???#???..? 1,2,2
???.????????????? 2,7,2,1
??#.?????? 1,1,3
??#?????#?##????#? 1,1,2,1,8
??.#?.?#?.?. 1,2
.?##???##????? 3,4,1,1
??#??###?.?? 2,4,1
?#?.????#?????#? 2,5,6
#?.?????#?#?#???#?#. 1,3,12
??.??.##?.? 1,1,2
??#??#?#?#.?? 3,3,1,2
.?##??.?????.? 4,2
??????#?#????#.. 1,8,1
??#?.?.???.#????. 2,3,4
.?#?.??#??? 1,4,1
?.???#???.?#. 1,6,1
?.?.?????#.##?##?. 1,1,3,6
?.???#?#?#??????#??. 11,4
?????????????.#?#??? 1,5,1,1,4,1
??#..????##?#???.?? 1,1,4,4,1
??.???????. 2,5
?.?###??..??.?##???? 6,1,5
???#?????? 2,1
?..?..??????.?##???? 1,3,2,1
???#????#?.???#??#. 6,3,2,4
?#?##???#???#?.#??. 13,3
?#???.?????? 5,1,1
????#???###?.#?? 1,5,3,1,1
???#??##??????##???# 1,2,3,1,1,6
..?###???????.??## 8,1,2
#?.??..??? 1,1,1
.?#?.??.?#? 2,1,3
??????.#?????..???. 2,2,2,1,1,1
.?.???####??# 5,1
##.?..??.?. 2,2
???#??#??..#?????? 4,1,1,1,2,1
?#?.??#?#?? 1,5
?#?#???##?#?..?#?.. 12,1
??#????##???.??.???? 11,1,1
?#???????#??????##?? 3,5,6
???????????.?? 2,2,4,1
???????.??? 2,1,2
?.#?#??.?? 3,1
??#.??#??.???#????. 2,1,3,1,3,1
#.#??????..? 1,4,1
?#?##???#????#???? 1,3,1,1,4
??????#?#?#??????? 9,2
.??#.#???#?#. 1,1,1,4
#????##??#?.??????#? 11,7
?##.??#?##. 2,6
????.?..??? 2,1,1
??.????.????. 3,1
???????.?# 4,1,2
?#.?#???.????.?? 1,3,1,1,1
??#??##?.? 1,4
.?##????????.?.??? 10,2
???#?????.? 4,1
???????#?.??##? 1,1,1,3
.???#?.?????? 2,5
??#?????##?##???? 5,4,5
???.?#????#???#?#? 2,1,2,1,5
?.?????.??#??? 5,5
.??????###?.??##? 4,3,4
.???..???.?##?????#. 2,2,6,2
???##????###? 1,3,5
#??#?#??#?#.??.? 2,8,1
??#?.?..##???#?? 1,1,4,3
?????.???? 1,1,1
.??????##??? 1,2,4
????####??#?.? 1,7
???##???????????.??? 2,1,8,1
?#?.?#??.??????#?. 2,1,7
.#?#??#???? 7,1
???..?.?#?#?###?? 1,1,9
.####??##???????.?# 4,3,5,1
??#..???#???#..? 1,8,1
??#.??.??##????#. 2,1,7
???#???..#??. 3,1,1
??##?###?#?#?#???? 14,1
?#?.??#??? 1,3,1
?.#?????#???.?#???. 8,5
?###??#???#????? 7,1,1,1
..?????????.??.?? 3,5,1
????#?????? 2,4,1
??#?#?????????? 6,1,1,1
?.?#??.###?#?????# 1,3,3,2,1,1
#??#..??#? 2,1,4
...?#??#??? 1,5
.??#?##??????. 5,2
?#.????#?.?.?? 1,4,1,2
#??#???#??#???????# 2,1,3,3,2,1
??#??#.??? 3,1,1
??#??..??#?.??## 2,1,4,3
????????#???? 6,2
??##??..?#?#??# 4,1,7
?..#????????? 1,3,2
??#?????????.???? 4,1,1,2,1
?.???????? 1,1,1
##??#?????? 2,2,3
???#????#??. 3,1,1,1
???#??.??? 2,2
?.???#??????? 1,2,1,2
#?#..?#.???#?? 3,1,3
.??####?#???# 4,5
?#???.???#?? 1,1,1,2
###??#?.##??? 3,3,2,1
?.?????.?????.??.? 1,1,1,1,3,2
???#????#?###?#? 4,1,8
?##?.####??#?. 4,8
??.?##??.?????###? 1,4,8
??.?????#??#??????? 2,1,3,1,5
?#.????..???????##?? 2,1,1,4,3
?#??#.???#??###??#? 2,1,1,1,3,3
..?##?????#????? 4,5
???##?.?###?.? 2,4
????.??..?? 2,1,1
?#????#??#.?????? 2,6,1,1,1
??#?..??.?#?#? 1,1,1,3
????#?#????#?????#?? 14,3
???.#?.??????.??.# 3,1,1,1,1,1
..#.#?#???#??# 1,10
?#??..????????# 2,1,3,1,1
?#..??#??? 2,3
?.###???#.. 3,2
.#??#??##???#? 1,10
????????##?.?###? 8,4
.???###..?..?# 4,1,2
??#?#??.??.#??? 6,4
??##????#??### 3,1,4
#?..##?.?????###???? 2,2,8,1
#????????##..??## 3,1,4,4
????#??#???.? 1,7
?#??###?#??.? 1,6,1
???#??.?#??##?#?. 3,9
.?????????#.????.#? 2,5,1,1,2
?.?????#?????#?? 1,2,1,3
?##???###?#??. 11,1
????#????? 1,2,1
.??.?.??#?#??? 1,7
?..??...?????????.? 1,6
?#?...#??? 2,3
.?#..#?#..? 1,3
?.??????..??#??? 3,5
?.??..???????##??#? 1,1,13
##??#.???????.?##??# 5,5,1,2,1
???#??#?#.???#???.?? 8,2,1,1,1
?????..?####?#?? 3,8
??#??.?#??..??.# 1,1,3,1,1
?????#??##??#?#??#?# 8,8
???.?##?..? 2,3
.??.??#?#??#??.#? 1,1,8,1
###????###???? 3,7
???##..#?. 2,2
..?????#??.?.? 3,3,1,1
#.??.?.#??###??#?#?? 1,1,1,9,1,1
?.?.#.??????##???# 1,1,2,1,6
.??#??#??????? 1,4,1,1
?#?.??#?.?#?#???? 1,4,2,1,3
???.??????#? 1,5
?????..?.???? 3,1,1,1
.????####?.?????. 6,3
???#??#??.?#???? 6,5
#..??####??? 1,4
?#??#?#?##?????.#? 11,1,1
??#?#..?#??# 3,4
??#.???..??#??????#? 3,1,1,1,1,3
?#??.????#????? 4,1,1,4
.#?#?#?#?#??.#????#? 11,1,1
#?#??.???.. 4,2
??#??##?.. 2,3
????.?.##???? 1,1,1,6
.?????.????????.?#? 1,1,6,1,1
??#??.?#?##??? 3,1,6
?.???#???##?##.? 1,1,7
?????.#????#?#?? 2,2,9
#???#?????.#??? 3,1,1,4
???#.#?.??.#??? 3,1,1,2,1
???#????????????? 9,4
?.#.??.??? 1,1,2
.?#??.??#.? 3,1,1
??????.#??.. 2,2
?##???##??#? 4,5
????###?##??#? 1,10";

    println!("Ans: {}", run(input))
}

fn run(input: &str) -> u32 {
    let mut arrangement_count_sum: u32 = 0;

    let lines: Vec<&str> = input.lines().collect();
    let parsed_lines: Vec<(&str, Vec<u32>)> = lines.into_iter().map(
        |x| {
            let split_line: Vec<&str> = x.split(" ").collect();
            let groups: Vec<u32> = split_line[1].split(",").into_iter().map(
                |y| y.parse::<u32>().unwrap()
            ).collect();

            (split_line[0], groups)
        }
    ).collect();

    for line in parsed_lines {
        let mut arrangement_count: u32 = 0;

        let question_marks_num = line.0.matches("?").collect::<Vec<&str>>().len();

        let mut source: String = String::from(r"^\.*");

        for i in line.1 {
            for _j in 0..i {
                source += r"#";
            }

            source += r"\.+"
        }
        source.truncate(source.len() - 3);
        source += r"[^#]*$";

        let re = Regex::new(&source).unwrap();

        let permutations = (1..=question_marks_num)
                             .map(|_| 0..=1)
                             .multi_cartesian_product()
                             .into_iter()
                             .collect::<Vec<_>>();

        for p in permutations {
            let mut subbed_string: String = String::from(line.0.clone());

            for i in p.into_iter() {
                match i {
                    0 => {
                        subbed_string = subbed_string.as_str().replacen("?", r".", 1);
                    },
                    1 => {
                        subbed_string = subbed_string.as_str().replacen("?", r"#", 1);
                    },
                    _ => (),
                }
            }

            if re.is_match(subbed_string.as_str()) {
                arrangement_count += 1;
            }
        }

        arrangement_count_sum += arrangement_count;
    }

    return arrangement_count_sum
}

#[cfg(test)]
mod tests {
    use crate::run;

    #[test]
    fn example_1() {
        let input: &str = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

        let result: u32 = run(input);

        assert_eq!(result, 525152);
    }
}