mod tree;
mod parse;
mod error;
mod file;
//mod types;
use tree::Tree;
use tree::TreeElement::Branch;
use tree::TreeElement::Leaf;
use parse::parse;
use error::Error;
use error::display_error;

fn display(tree: &Tree<String>, depth: usize) {
  for i in &tree.data {
    match i {
      Branch(n) => {
        match &n.label {
          None => {display(n,depth+1)},
          Some(l) => {
            for _ in 0..depth {
              print!("  ");
            }
            println!("{}:",l); display(n,depth+1)
          },
        }
      },
      Leaf(n) => {
        for _ in 0..depth {
          print!("  ");
        }
        println!("{}",n);
      }
    }
  }
}

fn test(filename: &String) {
  let data = match file::load_file(&filename) {
    Ok(s) => s,
    Err(_) => {display_error(&String::from(""), Error::FileNotFound(filename.clone()));return}
  };
  match parse(&data) {
    Ok(tree) => {
      display(&tree,0);
    }
    Err(e) => {
      display_error(&data,e);
    }
  }
}

fn main() -> Result<(),String> {
  let args: Vec<String> = std::env::args().collect();
  
  test(&args[1]);

  Ok(())
}

