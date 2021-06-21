use std::collections::HashMap;

// use std::env;
use std::ops::{
    // IndexMut,
    Index,
};


use dotenv;

pub trait PriorityQueue<Element> {
    /// create a new priority queue.
    fn new() -> Self;
    /// check whether the queue has no elements.
    fn is_empty(&self) -> bool;
    /// returns the highest-priority element but does not modify the queue.
    fn peek(&self) -> Option<Element>;
    /// add an element to the queue with an associated priority.
    fn insert(&mut self, element: Element, priority: u64);
    /// remove the element from the queue that has the highest priority, and return it.
    fn pop(&mut self) -> Option<Element>;
}

pub struct PriorityQueueImpl(HashMap<Vec<u8>, Vec<u8>>);

// Do not modify anything above ^^^


// Below: A function to reduce redundancy.
// if b is higher than a, a = b
fn which_is_higher(a: &mut Vec<u8>, b: &Vec<u8>){

    // if yes is 1, return 1,
    // if no is 1, return other,
    // No is opposite from Yes.
    fn comparison(yes: bool, no: bool, other: u8) -> u8 {
        // This is for debugging only, compiling for production will exclude these assertions
        assert!(!yes || !no, "Yes and No can't be true at the same time");
        assert!(other <= 1, "Other is not binary. Other = {}.\n", other);
        
        // println!("yes: {} {}\n no: {} {}\n", yes, yes as u8, no, no as u8);

        println!("yes: {} no: {} other: {}", yes as u8, no as u8, other);
        // if current byte is equal to the other byte, check the next byte.
        return (yes as u8) | ((1 - (no as u8)) & other);
    }

    // let c: Vec<u8> = Vec::from([0, 0, 0, 0, 0, 0, 0, 0]);
    // Linter gets confused if we call a function with indexes [] directly. These will be optimized away.
    let (a0, a1, a2, a3, a4, a5, a6, a7) = (a[0], a[1], a[2], a[3], a[4], a[5], a[6], a[7]);
    let (b0, b1, b2, b3, b4, b5, b6, b7) = (b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7]);

    // let higher: u8 = (a0 > b0) | ((1- (a0 < b0)) & ((a1 > b1) | ...));
    // if first condition column is true first, becomes 1.
    // if second condition column is true first, becomes 0.
    // println!("a: {:?}\nb: {:?}\n", a, b);
    let higher = comparison(
        a0>b0, a0<b0, comparison(
        a1>b1, a1<b1, comparison(
        a2>b2, a2<b2, comparison(
        a3>b3, a3<b3, comparison(
        a4>b4, a4<b4, comparison(
        a5>b5, a5<b5, comparison(
        a6>b6, a6<b6, comparison(
        a7>b7, a7<b7, 0)
    )))))));
    println!("Higher: {}\n", higher);

    // higher will be 1 or 0
    a[0] = a[0] * higher + b[0] * (1 - higher);
    a[1] = a[1] * higher + b[1] * (1 - higher);
    a[2] = a[2] * higher + b[2] * (1 - higher);
    a[3] = a[3] * higher + b[3] * (1 - higher);
    a[4] = a[4] * higher + b[4] * (1 - higher);
    a[5] = a[5] * higher + b[5] * (1 - higher);
    a[6] = a[6] * higher + b[6] * (1 - higher);
    a[7] = a[7] * higher + b[7] * (1 - higher);
}


impl PriorityQueue<Vec<u8>> for PriorityQueueImpl {
    // TODO: finish the implementation

    /// Actually, the pre-allocated `capacity` value should pass via environment vars which more flexible.
    fn new() -> Self {

        // This function is used only by new(). That's why it's defined here. Compiler will optimize it away.
        fn get_env_capacity() -> Option<usize> {
            // First attempt to get the environment variables from .env file.
            match dotenv::dotenv() {
                Err(_) => return None,
                Ok(_) => {
                    // Then attempt to get capacity environment variable.
                    match dotenv::var("CAPACITY") {
                        Err(_) => return None,
                        Ok(value) => {
                            // Then attempt to parse the String into a u32.
                            match (&value[..]).parse::<usize>() {
                                Err(_) => return None,
                                // If all of this is succesfull, then return it, else return None.
                                Ok(capacity) => return Some(capacity),
                            }
                        },
                    }
                },
            };
        }

        // If getting environment variable failed, use 5.
        let capacity = match get_env_capacity() {
            None => 5,
            Some(value) => value,
        };
        
        // PriorityQueueImpl(HashMap::with_capacity(5))
        PriorityQueueImpl(HashMap::with_capacity(capacity))
    }

    ///
    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// I believe adding some unsafe code with raw pointer accessing will more efficient when
    /// needed.
    /// 
    /// I used which_is_higher function above to check each element. This will be a lot faster than just using if statements.
    /// Can explain why in the interview.
    fn peek(&self) -> Option<Vec<u8>> {
        
        if self.0.is_empty() {
            return None;
        }

        let mut temp_priority: Vec<u8> = Vec::from([0, 0, 0, 0, 0, 0, 0, 0]);
        for key in self.0.keys() {
            // Make a comparison between the key and current highest priority found.
            // There's no if here, only math. It's easier for the compiler and the processor to optimize.
            // This function isn't allocating new memory, just accessing/changing existing ones.
            which_is_higher(&mut temp_priority, key);
        }
        let element = self.0.get(&temp_priority);

        // At this point, we used temp_priority to get the element.
        // We don't need temp_priority anymore.
        // But to save memory allocation, we will reuse this vector.
        match element {
            None => return None,
            Some(value) => {
                // 8 bytes are already allocated, so we just replace the values.
                for i in 0..(std::cmp::min(8, value.len())) {
                    let num = value.index(i);
                    temp_priority[i] = num.clone();
                }

                // if value.len() <= 8. This loop will be ignored.
                // Appends extra bytes remaining.
                for i in 8..(value.len()) {
                    temp_priority.push(value.index(i).clone());
                }

                // This will occur only if value.len() < 8. Removing unused bytes.
                for _ in 0..(8 - value.len()) {
                    temp_priority.pop();
                }

                return Some(temp_priority);
            }
        }
    }

    /// Add element to the given priority or append if priority is used.
    fn insert(&mut self, element: Vec<u8>, priority: u64) {

        /// Receive a priority value and convert it into a vector of 8 bytes.
        fn eight_bytes(mut priority: u64) -> Vec<u8>{
            let mut new_priority: Vec<u8> = Vec::with_capacity(8);
            // one binary value of 11111111 for bitwise operations
            let allone: u64 = 255;


            for _ in 0..7 {
                // This bitwise operation will select only the last 8 bits from the u64.
                let new_value: u8 = (priority & allone) as u8;
                new_priority.push(new_value);

                // Shift 8 bits to the right. Rust panics if this is done more than 7 times.
                // So last push is outside the loop.
                priority = priority >> 8;
            }

            // Last 8 bits remain;
            new_priority.push((priority & allone) as u8);

            new_priority
        }

        // Original line:
        // self.0.insert(vec![priority as u8], element);
        let priority = eight_bytes(priority);


        // Trying to insert it directly won't work, so I decided to get it first. This comment block won't work.
        // let inserting = self.0.insert(priority, element);
        // // Note to self: Use get instead
        // match inserting {
        //     None => {
        //         // This means that insert was successful, Nothing else needed.
        //     },
        //     Some(value) => {
        //         // This means that key is already being used, 
        //         // so we are going to append the byte list to the end of the vector
        //         for i in element{
        //             value.push(i);
        //         }
        //     }
        // }


        // Attempt to get with key priority.
        // If it doesn't exist, assign element to the key.
        // If it exists, append element to the end of the existing vector.
        let got = self.0.get_mut(&priority);
        match got{
            // There's no element for this priority.
            None => {
                // Doesn't implement index for hashmap<vec, vec>. Therefore can't do the following line:
                // self.0[&priority] = element;

                // Calling insert takes ownership of "priority", but we aren't needing it anymore. So zero-copy here plus guaranteed insert success.
                let inserting = self.0.insert(priority, element);

                // Attempt to insert the value after finding out that spot is empty.
                match inserting {
                    None => {
                        // I think it would better for the trait function to return a Result, just to be sure.
                        // But it doesn't. So this success returns nothing.
                    },
                    Some(_) => {
                        // Since we checked first with get, this shouldn't be reachable.
                        unreachable!();
                    },
                };
            },
            // There's already a vector stored for this key.
            Some(value) => {
                // If priority is already taken, append the bytes to the end of the vector.
                for i in element {
                    value.push(i);
                }
            }
        }

    }

    /// Remove an element with highest priority and return it.
    fn pop(&mut self) -> Option<Vec<u8>> {

        if self.0.is_empty() {
            return None;
        }

        let mut temp_priority: Vec<u8> = Vec::from([0, 0, 0, 0, 0, 0, 0, 0]);
        for key in self.0.keys() {
            // Make a comparison between the key and current highest priority found.
            // There's no if here, only math. It's easier for the compiler and the processor to optimize.
            // This function isn't allocating new memory, just accessing/changing existing ones.
            which_is_higher(&mut temp_priority, key);
        }

        let element = self.0.remove(&temp_priority);

        return element;
    }
}

// Forgot to change this into a lib file...
fn main() {
  println!("HI");
}

#[cfg(test)]
mod tests {
    use super::*;

    // I didn't change this function.
    #[test]
    fn it_works() {
        let mut queue = PriorityQueueImpl::new();
        assert!(queue.is_empty());

        queue.insert(vec![0], 5);
        assert!(!queue.is_empty());
        assert_eq!(queue.peek(), Some(vec![0]));

        queue.insert(vec![2], 3);
        queue.insert(vec![1], 10);
        queue.insert(vec![3], 4);
        queue.insert(vec![4], 6);
        assert_eq!(queue.peek(), Some(vec![1]));

        assert_eq!(queue.pop(), Some(vec![1]));
        assert_eq!(queue.pop(), Some(vec![4]));
        assert_eq!(queue.pop(), Some(vec![0]));
        assert_eq!(queue.pop(), Some(vec![3]));
        assert_eq!(queue.pop(), Some(vec![2]));
        assert!(queue.is_empty());
    }

    // TODO: add more tests as appropriate

    // This is a unit test for which_is_higher function.
    #[test]
    fn test_which_is_higher(){
        fn clear_vector(a: &mut Vec<u8>) {
            for i in 0..a.len() {
                a[i] = 0;
            }
        }

        fn checking(a: &mut Vec<u8>, b: &Vec<u8>, equals: &Vec<u8>){
            which_is_higher(a, &b);
            assert_eq!(a, equals, "Expected A to be ({:?}), got ({:?})", equals, a);
        }

        let mut a: Vec<u8> = vec![0, 0, 0, 0, 0, 0, 0, 0];
        let b: Vec<u8> = vec![0, 0, 0, 0, 0, 0, 1, 0];
        let c: Vec<u8> = vec![0, 0, 3, 0, 4, 0, 0, 1];
        let d: Vec<u8> = vec![9, 0, 0, 0, 0, 0, 0, 2];
        let e: Vec<u8> = vec![1, 0, 0, 0, 0, 0, 0, 9];
        let f: Vec<u8> = vec![5, 4, 3, 2, 1, 0, 0, 0];
        let g: Vec<u8> = vec![1, 1, 1, 1, 1, 1, 1, 1];
        let h: Vec<u8> = vec![9, 9, 9, 9, 9, 9, 9, 9];

        // which_is_higher(&mut a, &b);
        // assert_eq!(a, b);

        checking(&mut a, &b, &b);
        println!("a => b OK");
        checking(&mut a, &c, &c);
        println!("b => c OK");
        checking(&mut a, &d, &d);
        println!("c => d OK");
        checking(&mut a, &e, &d);
        println!("d => e OK");
        checking(&mut a, &f, &d);
        println!("d => f OK");
        checking(&mut a, &g, &d);
        println!("d => g OK");
        checking(&mut a, &h, &h);
        println!("d => h OK");
        clear_vector(&mut a);
        assert_eq!(a, vec![0, 0, 0, 0, 0, 0, 0, 0]);
        println!("clear OK");
        checking(&mut a, &g, &g);
        println!("a => g OK");
        checking(&mut a, &f, &f);
        println!("g => f OK");
        checking(&mut a, &h, &h);
        println!("f => h OK");
    }
}
