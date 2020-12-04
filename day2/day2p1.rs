use std::fs;
use std::fmt;

struct PasswordPolicy<'a> {
    password: &'a str,
    required_char: char,
    minimum: u32,
    maximum: u32,
}

impl PasswordPolicy<'_> {
 
    fn parse_password_policy<'a>(candidate: &'a str) -> PasswordPolicy<'a>  {
        // Assuming the input is well formatted
        let s1: Vec<&str> = candidate.split(':').collect();
        let password: &str = s1[1];
        let policy: Vec<&str> = s1[0].split(' ').collect();
        let required_char: &str = policy[1];
        let bounds: Vec<u32> = policy[0].split('-').map(|s| s.parse().unwrap()).collect();

        return PasswordPolicy{password: password, required_char: required_char.chars().next().unwrap(), minimum: bounds[0], maximum: bounds[1]};
    }

    fn is_valid(self: &Self) -> bool {
        let required_contains = self.password.chars().filter(|c| *c == self.required_char).count();
        if self.minimum > required_contains as u32 || self.maximum < required_contains as u32 {
            return false;
        }
        true
    }

    fn is_valid2(self: &Self) -> bool {
        let password_vec: Vec<char> = self.password.chars().collect();
        // No need to subtract 1 because we didnt strip the space from the password
        let first_char = password_vec[self.minimum as usize];
        let second_char = password_vec[self.maximum as usize];
        
        if first_char == self.required_char || second_char == self.required_char {
            if first_char == second_char {
                return false
            }
           
            return true;
        }
        false
    }

}

impl fmt::Display for PasswordPolicy<'_>  {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Password:{}, required char: {} minimum: {}, maximum: {}", 
        self.password, self.required_char, self.minimum, self.maximum)
    }
}

fn main(){
    let contents = fs::read_to_string("input.txt")
    .expect("Something went wrong reading the file");
    let policies: Vec<PasswordPolicy> = contents.lines().map(|s| PasswordPolicy::parse_password_policy(s)).collect();
    let result = policies.iter().filter(|p| p.is_valid()).count();
    println!("Result: {}", result);
    let result2 = policies.iter().filter(|p| p.is_valid2()).count();
    println!("Result2: {}", result2);
}