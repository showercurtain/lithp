use super::error::Error;

enum Type {
  Exact(Box<(u64,u64,bool)>),
  Inexact(Box<f64>),
  String(Box<String>),
  Pair(Box<(Type,Type)>),
  List(Vec<Type>),
  HashList(Vec<(String,Type)>),
  Error(Error),
  None,
}

impl Clone for Type {
  fn clone(&self) -> Self {
    match self {
      Type::Exact(x) => Type::Exact(Box::new(**x)),
      Type::Inexact(x) => Type::Inexact(Box::new(**x)),
      Type::String(x) => Type::String(Box::new(*x.clone())),
      Type::Pair(x) => Type::Pair(Box::new(*x.clone())),
      Type::List(x) => Type::List(x.clone()),
      Type::HashList(x) => Type::HashList(x.clone()),
      Type::None => Type::None,
    }
  }
}

fn gcd(x: u64, y: u64) -> u64{
  let (mut x2,mut y2);
  if y > x {
    (x2,y2) = (y,x);
  } else {
    (x2,y2) = (x,y);
  }

  loop {
    if  y != 0 {
      (x2,y2)=(y2,x2%y2);
    } else {
      return x2
    }
  }
}

/*
Exact+Exact->Exact
Exact+Inexact->Inexact
String+String->String
List+List->List
HashList->HashList
*/
