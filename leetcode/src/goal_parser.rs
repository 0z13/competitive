impl Solution {
    pub fn interpret(command: String) -> String {
        let mut xs : String = String::from("");
        let mut chars = command.chars();
        while let Some(c) = chars.next()  {
            match c {
                'G' => xs.push('G'),
                '(' => if chars.next().unwrap() == ')' {
                        xs.push('o');
                        } else {xs.push_str("al"); chars.next(); chars.next();},
                _   => xs.push('w'),
            }
            
        }
        xs
    }
    
}
