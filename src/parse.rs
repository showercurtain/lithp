use super::tree::Tree;
use super::error::Error;

pub fn parse(data: &String) -> Result<Tree<String>,Error> {
  let mut out: Tree<String> = Tree::new();

  let mut copy = data.clone();
  copy.push(')');

  let end = parse_rec(&copy, 0, &mut out, false)?;
  if end < data.len() {
    Err(Error::Exception(end,String::from("What is this for?")))
  } else {
    Ok(out)
  }
}

fn get_char(data: &String, start: usize) -> Result<char,Error> {
  match u8::from_str_radix(&data[start..start+2],16) {
    Err(_) => {Err(Error::CharParseError(start))},
    Ok(n) => {Ok(n as char)}
  }
}

fn parse_rec(data: &String, start: usize, out: &mut Tree<String>, bracket: bool) -> Result<usize,Error> {
  let mut i: usize = start;
  let mut word = String::new();
  let mut in_word = false;
  let mut in_str = false;
  let mut esc = false;
  loop {
    match data.chars().nth(i) {
      None => {return Err(Error::EndOfBuffer)},
      Some(n) => {
        if in_str {
          if esc {
            word.push(match n {
              '0' => '\0',
              'n' => '\n',
              'r' => '\r',
              't' => '\t',
              'x' => {i += 2; get_char(data, i-1)?}
              _ => n,
            });
            esc = false;
          } else if n == '\\' {
            esc = true;
          } else if n == '"' {
            in_str = false;
            word.push(n);
          } else {
            word.push(n);
          }
        } else {
          match n {
            '(' | '[' => {
              if word == "" {
                i = parse_rec(data, i+1, out.add_branch(), n=='[')?;
                in_word = false;
              } else {
                i = parse_rec(data, i+1, out.add_branch_labeled(word),n=='[')?;
                in_word = false;
                word = String::new();
              }
            },
            ')' | ']' => {
              if !((n==')') ^ bracket) {
                return Err(Error::MismatchedParens(start-1,i));
              }
              if in_word {
                out.add_value(word);
              }
              return Ok(i)
            },
            ' ' | '\n' | '\r' | '\t' => {
              if in_word {
                if word.contains('\n') {println!("eXCUSE ME!?")}
                in_word = false;
                out.add_value(word);
                word = String::new();
              }
            },
            '"' => {word.push(n); in_word = true; in_str = true;},
            _ => {
              word.push(n);
              in_word = true;
            },
          }
        }
      }
    }
    i += 1;
  }
}