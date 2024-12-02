
/* TYPES    
    1. .into_iter() = consumes and takes the ownership of the ds.
    2. .iter() = borrows the ds immutably. Ownership is not transferred.
    3. .iter_mut() = borrows the ds mutably. Ownership is not transferred.
    4. iter.next() = advances iterator state. Need to create a reusable mutable iterator separately.*/

fn main() {
    let mut vec1 = vec![1,2,3];
    
    //for val in vec1{} consumes the vec1 completely thus making is unusable later. It uses .into_iter() under the hood.
    
    for val in vec1.iter(){
        println!("{}", val)     //using .iter() explicitly only borrows the vec immmutably
    }

    for val in vec1.iter_mut() { // borrows the ds mutably. => only that borrow may exist during it's lifetime.
        *val = *val + 1;    //dereferencing the address of the value using *
        println!("{}", val);
    }

    let mut iter = vec1.iter();
    while let Some(val) = iter.next() {   //another way to loop a ds. 
        println!("{}", val);                    // .iter().next() returns an option enum.
    }                                           // the while loop is exited when None is obtained
}

