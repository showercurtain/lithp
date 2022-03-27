mod tree;
mod parse;
use tree::Tree;
use tree::TreeElement::Branch;
use tree::TreeElement::Leaf;
use parse::parse;

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

fn test(data: &String) -> Result<(),String> {
  let tree = parse(data)?;
  display(&tree,0);
  Ok(())
}

fn main() -> Result<(),String> {
  let mut args: Vec<String> = std::env::args().collect();
  args.remove(0);
  let joined = args.join(" ");
  test(&joined)
}
