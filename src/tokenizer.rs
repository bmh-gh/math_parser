#[derive(Debug)]
#[derive(PartialEq)]
pub enum TokenError {
    InvalidToken(String)
}

impl TokenError {
    fn new(invalid: &char) -> TokenError{
        TokenError::InvalidToken("Invalid token:".to_owned() + &invalid.to_string())
    }
}

/// Ignores Whitespaces!!!
pub fn tokenize(input: &str) -> Result<Vec<String>, TokenError> {
    let mut tokens = Vec::new();
    let mut buf = String::new();
    for c in input.chars().filter(|c| !c.is_ascii_whitespace()) {
        // Can contain multiple valid tokens
        if valid_token(&c) {
          // If the valid token is a numerical token or a dot, save it in a buffer
          if c.is_numeric() || c == '.' {
            buf.push(c);
          }
          else {
            // If buf isn't empty there was a number saved 
            if !buf.is_empty() {
              tokens.push(buf.clone());
              buf.clear();
            }
            // And then the valid token can be pushed on the token stack
            tokens.push(c.to_string())
          }
        }
        // or at least one invalid token
        else { 
          return Err(TokenError::new(&c))
          }
      }
      // If the last iteration was a buf, then buf is not empty yet
      if !buf.is_empty() {
        tokens.push(buf.clone());
        buf.clear();
      }
    Ok(tokens)
}

fn valid_token(c: &char) -> bool {
  let valid_tokens = "+-*/^().";
  return valid_tokens.contains(*c) || c.is_ascii_alphanumeric()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenize_ok() {
        let input = "5+ 4.2 - 3^+";
        let out = ["5", "+", "4.2", "-", "3", "^", "+"];
        let tokens = tokenize(input).unwrap();
        assert_eq!(tokens, out);
    }

    #[test]
    fn tokenize_err() {
        let input = "1 - 2.2 / ;3 *4";
        let out = TokenError::new(&';');
        let tokens = tokenize(input).err().unwrap();
        assert_eq!(tokens, out);
    }
}
