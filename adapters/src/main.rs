/* ADAPTERS
    1. Consuming Adapters --> Consumes an iterator to perform an operation
    2. Iterator Adapters --> They produce a different iterator by changing the original iterator.     
 */

fn main() {
    let v1 = vec![1,2,3];
    let v1_iter = v1.iter();
    let sum: i32 = v1_iter.sum(); //Consumed v1_iter to produce sum. Can't be used further.
    println!("Sum: {}", sum);
    
    let v2 = vec![1,2,3];
    let iter = v2.iter();
    let mapped_iter = iter.map(|x| x + 1); //takes every value and adds 1.
    for x in mapped_iter {
        println!("{} ", x);
    }

    let iter = v2.iter();
    let filtered_iter = iter.filter(|x| *x % 2 == 0); //filters even numbers into a new iterator.
    let final_vec: Vec<i32> = filtered_iter.cloned().collect(); //collects all elements in iter and converts it into a vec.
    println!("{:?}", final_vec);
}
