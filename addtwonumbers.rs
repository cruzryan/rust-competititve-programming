// From LeetCode: Add Two Numbers problem

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

struct Solution;

impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    
      println!("Running AddTwoNumbers!");
      

      let mut s1: Vec<i32> = vec![];
      let mut s1_c = 0;
      let mut s2: Vec<i32> = vec![];
      let mut s2_c = 0;

      let mut node = &l1;
      while let Some(cur) = node {
        s1.push(cur.val);
        node = &cur.next;
        s1_c += 1;
      }

      let mut node = &l2;
      while let Some(cur) = node {
        s2.push(cur.val);
        node = &cur.next;
        s2_c +=1;
      }

      let mut n1 = 0 as i64;
      while let Some(pp) = s1.pop(){
        n1 += pp as i64 * i64::pow(10, s1_c-1 as u32);
        s1_c -= 1;
      }

      let mut n2 = 0 as i64;
      while let Some(pp) = s2.pop() {
        n2 += pp as i64 * i64::pow(10, s2_c-1 as u32);
        s2_c -= 1;
      }

      let sum :i64 = n1 + n2;
      println!("n1: {} n2: {}   sum: {}", n1, n2, sum);

      let mut sol = None;
      let mut last = &mut sol;
      
      for c in sum.to_string().chars().rev() {
          let n = c as i32 - '0' as i32;
          println!("pushing {} as {}", c, n);
          let new_node = Some(Box::new(ListNode::new(n)));
          if last.is_none() {
              *last = new_node;
          } else {
              let mut current_node = last.as_mut().unwrap();
              while let Some(ref mut next_node) = current_node.next {
                  current_node = next_node;
              }
              current_node.next = new_node;
          }
      }

      let mut nn = &sol;
      while let Some(n) = nn {
        println!("SOL: {}", n.val);
        nn = &n.next;
      }

      return sol;

    }
}

fn main() {

  // List node 1
  let mut l1 = Some(Box::new(ListNode::new(2)));
  let mut ln2 = Some(Box::new(ListNode::new(4)));
  let ln3 = Some(Box::new(ListNode::new(3)));
  
  ln2.as_mut().unwrap().next = ln3;
  l1.as_mut().unwrap().next = ln2;

  // List node 2 
  let mut r1 = Some(Box::new(ListNode::new(5)));
  let mut rn2 = Some(Box::new(ListNode::new(6)));
  let rn3 = Some(Box::new(ListNode::new(4)));
  
  rn2.as_mut().unwrap().next = rn3;
  r1.as_mut().unwrap().next = rn2;

  //Call add_two_numbers
  let _result = Solution::add_two_numbers(l1, r1);

}