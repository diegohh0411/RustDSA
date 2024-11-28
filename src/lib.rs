pub struct LinkedList<T> {
  root: Link<T>,
}

impl<T> LinkedList<T> {
  fn new() -> LinkedList<T> {
    LinkedList { root: None }
  }

  fn push(&mut self, element: T) {
    let old_root = self.root.take();
    let new_root = Box::new(Node {
      element,
      next: old_root,
    });

    self.root = Some(new_root);
  }

  fn pop(&mut self) -> Option<T> {
    self.root.take().map(|node| {
      self.root = node.next;
      node.element
    })
  }

  fn peek(&self) -> Option<&T> {
    self.root.as_ref().map(|node| {
      &node.element
    })
  }
}


struct Node<T> {
  element: T,
  next: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let mut list = LinkedList::new();
    list.push(1024);
    list.push(42);
  }
}