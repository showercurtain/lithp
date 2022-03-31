
#[allow(dead_code)]
pub enum Error {
  MismatchedParens(usize,usize),
  EndOfBuffer,
  Exception(usize,String),
  Unknown,
  FileNotFound(String),
}

pub fn display_error(file: &String, error: Error) {
  match error {
    Error::MismatchedParens(start,end) => {error_message(Some(file),start,Some(end),String::from("note: begins here"),Some(String::from("Mismatched parenthesis")), true)},
    Error::EndOfBuffer => {error_message(Some(file),file.len(),None,String::from("Reached end of buffer"),None,false)},
    Error::Exception(start,msg) => {error_message(Some(file),start,None,msg,None,false)},
    Error::FileNotFound(file) => {eprintln!("File not found: {}",file)}
    _ => {error_message(None,0,None,String::from("Unknown error"),None,false)}
  }
}

pub fn error_message(file: Option<&String>, start: usize, end: Option<usize>, msg1: String, msg2: Option<String>, reverse: bool) {
  match file {
    Some(s) => {
      let file = s;
      let mut start = start;
      let mut end = end;
      let mut line1: (usize, &str) = (0,"");
      let mut line2: (usize, &str) = (0,"");
      let spaces = |x: usize| eprint!("{}"," ".repeat(x));

      for (num, line) in file.split('\n').enumerate() {
        if start <= line.len() {
          line1 = (num,line);
          break;
        } else {
          start -= line.len()+1;
        }
      }

      if let Some(mut e) = end {
        for (num, line) in file.split('\n').enumerate() {
          if e <= line.len() + 1 {
            line2 = (num,line);
            break;
          } else {
            e -= line.len()+1;
          }
        }
        end = Some(e);
      }

      let ind1 = line1.0.to_string().len();
      let ind2 = line2.0.to_string().len();
      let max = if ind1 > ind2 {ind1} else {ind2};

      let first = || {
        eprint!("{}",line1.0+1);
        spaces(if ind2 > ind1 {ind2-ind1+1} else {1});
        eprintln!("|{}",line1.1);
        spaces(max+1);
        eprint!("|");
        if start > msg1.len() {
          spaces(start-msg1.len()-1);
          eprintln!("{} ^",msg1);
        } else {
          spaces(start);
          eprintln!("^ {}",msg1);
        }
      };

      let second = || {
        if let (Some(e),Some(msg)) = (end,msg2) {
          eprint!("{}",line2.0+1);
          spaces(if ind1 > ind2 {ind1-ind2+1} else {1});
          eprintln!("|{}",line2.1);
          spaces(max+1);
          eprint!("|");
          if e > msg.len() {
            spaces(e-msg.len()-1);
            eprintln!("{} ^",msg);
          } else {
            spaces(e);
            eprintln!("^ {}",msg);
          }
        }
      };

      if reverse {
        second();
        first();
      } else {
        first();
        second();
      }
    },
    None => {eprint!("{}",msg1)},
  }
}