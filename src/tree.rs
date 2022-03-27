pub enum TreeElement<T> {
  Branch(Tree<T>),
  Leaf(T),
}

pub struct Tree<T> {
  pub data: Vec<TreeElement<T>>,
  pub label: Option<String>,
}

impl<T> Tree<T> {
  pub fn with_label(label: String) -> Self {
    Self {data: vec![],label: Some(label)}
  }

  pub fn new() -> Self {
    Self {data: vec![],label: None}
  }

  pub fn add_branch_labeled(&mut self, label: String) -> &mut Self {
    self.data.push(TreeElement::Branch(Self::with_label(label)));
    let i = self.data.len();
    match &mut self.data[i-1] {
      TreeElement::Branch(n) => n,
      TreeElement::Leaf(_) => {panic!("Something is very amiss")}
    }
  }

  pub fn add_branch(&mut self) -> &mut Self {
    self.data.push(TreeElement::Branch(Self::new()));
    let i = self.data.len();
    match &mut self.data[i-1] {
      TreeElement::Branch(n) => n,
      TreeElement::Leaf(_) => {panic!("Something is very amiss")}
    }
  }

  pub fn add_value(&mut self, value: T) {
    self.data.push(TreeElement::Leaf(value))
  }
}

