pub fn test(mut x:i32) -> i32 {
    println!(" Value of x : {}", x);
    x = 42i32;
    println!(" Value of x : {}", x);
    let a = x;
    a
}

fn main() {
    test(5i32);


    println!(" --- Declaration/Initializaion of values STACK / HEAP --- ");


    // Value allocated in the STACK
    let stack_a = 5;
    // Value allocated in the HEAP allocated integer
    let mut mut_heap_a = Box::new(7i32);
    let mut mut_heap_b = Box::new(42i32);
    let mut mut_heap_a_ptr: *mut i32 = Box::into_raw(mut_heap_a);
    let mut mut_heap_b_ptr: *mut i32 = Box::into_raw(mut_heap_b);

    // Error ! : dereference of a raw pointer
    // [TODO] : uncomment this line to see the error
    //println!("Value of  *mut_heap_a_ptr : {}", *mut_heap_a_ptr); 
    //println!("Value of  *mut_heap_b_ptr : {}", *mut_heap_b_ptr);
    
    // If we don't place the following lines into the unsafe block the code will not compile 
    unsafe {
	println!("Value of  *mut_heap_a_ptr : {}", *mut_heap_a_ptr);
	println!("Value of  *mut_heap_b_ptr : {}", *mut_heap_b_ptr);
    }

    mut_heap_a_ptr = mut_heap_b_ptr;

    unsafe {
	println!("Value of  *mut_heap_a_ptr : {}", *mut_heap_a_ptr);
	println!("Value of  *mut_heap_b_ptr : {}", *mut_heap_b_ptr);
    }


    // What happend ? Is there a memory leack ?

    


    
//   // println!(" Value oof heap_a pointer : {:p}", mut_heap_a); // We do not have acces to mut_heap_a anymore
//   // println!(" Value oof stack_a pointer : {:p}", mut_heap_b); // We do not have acces to mut_heap_b anymore
//   mut_heap_b_ptr = mut_heap_a_ptr;
//   
//   println!("Value of stack_a : {}", stack_a);
//   println!("Value of  mut_heap_a_ptr : {:p}", mut_heap_a_ptr);
//   println!("Value of  mut_heap_b_ptr : {:p}", mut_heap_b_ptr);
//
//   unsafe {
//	println!("Value of  *mut_heap_a_ptr : {}", *mut_heap_a_ptr);
//	println!("Value of  *mut_heap_b_ptr : {}", *mut_heap_b_ptr);
//   }





    
  //  println!("Value of  mut_heap_a_ptr : {}", mut_heap_a_ptr.self());
//    println!("Value of  mut_heap_b_ptr : {}", mut_heap_b_ptr.as_ref());
    
    /* Ownership / Move */
//    println!(" --- Move --- ");
//    println!("Affecting a to b for values in STACK / HEAP"); 
//    let stack_b = stack_a; // Copy stack_a into stack_b
//    let  heap_b = heap_a;  // Move heap_a into heap_b
//
//    println!("Value of stack_a : {}", stack_a);
//    println!("Value of stack_b : {}", stack_b);
//
//    // We can no longer acces to heap_a as the ownership of the heap momory is possessed by
//    // heap_b
//    // Error! borrow of moved value 'heap_'
//    //println!("Value of heap_a : {}", heap_a); // [TODO] uncomment this line to see the error
//    println!("Can not access to heap_a as it is not the owner any more because the affectation worked as a move and heap_a is not the owner anymore");
//    println!("Value of heap_b : {}", heap_b);
//
//    // We can't affect a new value to an existing one 
//    // Error! cannot assign to '*heap_b' as 'heap_b' is not declared as mutable
//    //*heap_b = 8; // [TODO] uncomment this line to see the error
//
//    /* Mutability */
//    println!(" --- Mutability --- ");
//
//    println!("Affecting heap_b to a new mutable variable mut_heap_b");
//    // Move the box to heap_b to mut_heap_c and make it mutable
//    let mut mut_heap_b = heap_b;
//
//    // Again we can't access to heap_b as it is not the owner anymore
//    // Error! borrow of moved value: 'heap_b'
//    //println!("Value of heap_b : {}", heap_b); [TODO] uncomment this line to see the error
//    println!("We can not acces to heap_b again");
//    println!("Value of mut_heap_b : {}", mut_heap_b);
//
//    println!("Changing the value of mut_heap_b to 8");
//    *mut_heap_b = 8;
//
//    println!("Value of mut_heap_b : {}", mut_heap_b);
//
//    // Borrwoning / Lifetime
//    println!(" --- Borrowing ---");
//    let borrow_b = &mut_heap_b;
//
//    // We try to modify mut_heap_b
//    // Error! cannot assign to '*mut_heap_b' because it is borrowed
//    // *mut_heap_b = 9; // [TODO] uncomment this line to see the error
//    
//    println!("Value of borrow_b : {}", borrow_b);
//
//    println!("Changing the value of mut_heap_b after borrow_b lifetime");
//    // We can do it here beacause the life time of borrow_b is over
//    *mut_heap_b = 9;
//    println!("Value of mut_heap_b : {}", mut_heap_b);
//    
//    // We cannot borrow an immmutable object as mutable 
//    //let mut mut_borrow_b = &mut_heap_b;
//    //*mut_borrow_b = 9;
//    // println!("Value of mut_borrow_b : {}", mut_borrow_b);
}
