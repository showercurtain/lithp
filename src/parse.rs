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
        if esc {
          in_word = true;
          word.push(n);
          esc = false;
        } else if in_str {
          if n == '"' {
            in_str = false;
          }
          word.push(n);
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
            ' ' | '\n' => {
              if in_word {
                in_word = false;
                out.add_value(word);
                word = String::new();
              }
            },
            '"' => {word.push(n); in_word = true; in_str = true;},
            '\\' => {esc = true;},
            _ => {word.push(n); in_word = true;},
          }
        }
      }
    }
    i += 1;
  }
}