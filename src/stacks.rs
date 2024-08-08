pub fn is_valid_parentheses(s: String) -> bool {
    let mut stack = Vec::with_capacity(s.len());
    for c in s.chars() {
        if c == '(' || c == '{' || c == '[' {
            stack.push(c);
            continue;
        }
        let Some(last) = stack.last() else {
            return false;
        };
        if (*last == '[' && c == ']') || (*last == '{' && c == '}') || (*last == '(' && c == ')') {
            stack.pop();
        } else {
            stack.push(c);
        }
    }
    stack.len() == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn is_valid_parentheses_tests() {
        assert_eq!(is_valid_parentheses("()".to_string()), true);
        assert_eq!(is_valid_parentheses("()[]{}".to_string()), true);
        assert_eq!(is_valid_parentheses("(]".to_string()), false);
        assert_eq!(is_valid_parentheses("(])".to_string()), false);
    }
}
