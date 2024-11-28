pub struct LinkedList<T> {
  root: Link<T>,
}

impl<T> LinkedList<T> {
  pub fn new() -> Self {
    Self { root: None }
  }

  pub fn push(&mut self, element: T) {
    let new_node = Box::new(Node {
      element,
      next: None,
    });

    match self.root.as_mut() {
      Some(mut current) => {
        while let Some(ref mut next) = current.next {
          current = next;
        }
        current.next = Some(new_node);
      }
      None => {
        self.root = Some(new_node);
      }
    }
  }

  pub fn pop(&mut self) -> Option<T> {
    if self.root.is_none() {
        return None;
    }

    let mut current = self.root.as_mut().unwrap();
    
    // If there's only one element
    if current.next.is_none() {
        return self.root.take().map(|node| node.element);
    }

    // Traverse to the second-to-last node
    while current.next.as_ref().unwrap().next.is_some() {
        current = current.next.as_mut().unwrap();
    }

    // Remove the last node and return its element
    current.next.take().map(|node| node.element)
  }
}

impl<T: std::fmt::Debug> LinkedList<T> {
  pub fn print(&self) {
    print!("LinkedList {{");
    let mut current = &self.root;

    while let Some(node) = current {
      print!(" {:?}", node.element);
      current = &node.next;
    }

    println!(" }}");
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