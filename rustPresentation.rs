fn main() {

    println!(" --- Declaration/Initializaion of values STACK / HEAP --- ");
    // Value allocated in the STACK
    let stack_a = 5;
    // Value allocated in the HEAP allocated integer
    let heap_a  = Box::new(7i32);
    
    println!("Value of stack_a : {}", stack_a);
    println!("Value of  heap_a : {}", heap_a);
    
    /* Ownership / Move */
    println!(" --- Move --- ");
    println!("Affecting a to b for values in STACK / HEAP"); 
    let stack_b = stack_a; // Copy stack_a into stack_b
    let  heap_b = heap_a;  // Move heap_a into heap_b

    println!("Value of stack_a : {}", stack_a);
    println!("Value of stack_b : {}", stack_b);

    // We can no longer acces to heap_a as the ownership of the heap momory is possessed by
    // heap_b
    // Error! borrow of moved value 'heap_'
    //println!("Value of heap_a : {}", heap_a); // [TODO] uncomment this line to see the error
    println!("Can not access to heap_a as it is not the owner any more because the affectation worked as a move and heap_a is not the owner anymore");
    println!("Value of heap_b : {}", heap_b);

    // We can't affect a new value to an existing one 
    // Error! cannot assign to '*heap_b' as 'heap_b' is not declared as mutable
    //*heap_b = 8; // [TODO] uncomment this line to see the error

    /* Mutability */
    println!(" --- Mutability --- ");

    println!("Affecting heap_b to a new mutable variable mut_heap_b");
    // Move the box to heap_b to mut_heap_c and make it mutable
    let mut mut_heap_b = heap_b;

    // Again we can't access to heap_b as it is not the owner anymore
    // Error! borrow of moved value: 'heap_b'
    //println!("Value of heap_b : {}", heap_b); [TODO] uncomment this line to see the error
    println!("We can not acces to heap_b again");
    println!("Value of mut_heap_b : {}", mut_heap_b);

    println!("Changing the value of mut_heap_b to 8");
    *mut_heap_b = 8;

    println!("Value of mut_heap_b : {}", mut_heap_b);

    // Borrwoning / Lifetime
    println!(" --- Borrowing ---");
    let borrow_b = &mut_heap_b;

    // We try to modify mut_heap_b
    // Error! cannot assign to '*mut_heap_b' because it is borrowed
    // *mut_heap_b = 9; // [TODO] uncomment this line to see the error
    
    println!("Value of borrow_b : {}", borrow_b);

    println!("Changing the value of mut_heap_b after borrow_b lifetime");
    // We can do it here beacause the life time of borrow_b is over
    *mut_heap_b = 9;
    println!("Value of mut_heap_b : {}", mut_heap_b);
    
    // We cannot borrow an immmutable object as mutable 
    //let mut mut_borrow_b = &mut_heap_b;
    //*mut_borrow_b = 9;
    // println!("Value of mut_borrow_b : {}", mut_borrow_b);
}
