mod dsa;

use dsa::LinkedList;
use rand::Rng;

fn main() {
    let mut list = LinkedList::<u32>::new();
    
    let mut rng = rand::thread_rng();
    for _ in 0..15 {
        let num: u32 = rng.gen_range(1..101);
        list.push(num);
    }

    list.print();

    list.pop();
    list.pop();
    list.print();
}
