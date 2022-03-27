use super::tree::Tree;

pub fn parse(data: &String) -> Result<Tree<String>,String> {
  let mut out: Tree<String> = Tree::new();

  let mut copy = data.clone();
  copy.push(')');

  parse_rec(&copy, 0, &mut out)?;

  Ok(out)
}

fn parse_rec(data: &String, start: usize, out: &mut Tree<String>) -> Result<usize,String> {
  let mut i: usize = start;
  let mut tmp = String::new();
  let mut tmp2 = false;
  let mut tmp3 = false;
  let mut tmp4 = false;
  loop {
    match data.chars().nth(i) {
      None => {return Err(String::from("Reached end of buffer"))},
      Some(n) => {
        if tmp4 {
          tmp.push(n);
          tmp4 = false;
        } else if tmp3 {
          if n == '"' {
            tmp3 = false;
          }
          tmp.push(n);
        } else {
          match n {
            '(' | '[' => {
              if tmp == "" {
                i = parse_rec(data, i+1, out.add_branch())?;
                tmp2 = true;
              } else {
                i = parse_rec(data, i+1, out.add_branch_labeled(tmp))?;
                tmp2 = true;
                tmp = String::new();
              }
            },
            ')' | ']' => {
              if !tmp2 {
                out.add_value(tmp);
              }
              return Ok(i)
            },
            ' ' | '\n' => {
              if !tmp2 {
                tmp2 = true;
                out.add_value(tmp);
                tmp = String::new();
              }
            },
            '"' => {tmp.push(n); tmp2 = false; tmp3 = true;},
            '\\' => {tmp4 = true;},
            _ => {tmp.push(n); tmp2 = false;},
          }
        }
      }
    }
    i += 1;
  }
}