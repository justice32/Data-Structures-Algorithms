mod data_structures;
fn main() {
    /*  
    For self learning, array we're using when we have:
    -- size of data is fixed
    -- need random access by index (constant time)
    -- memory efficiency is critical 
    Search: O(N) (linear) or O(log n)(binary seach for sorted arrays)
    Delection: O(N)
    Appending: O(1) if array is prealocated, is case it's resizing dynamically, complexity will be O(N)
    */     
    println!("Array Example!");
    data_structures::array::run();


    /*
    Usage:
    Frequent insertions and deletion at the beginning / back / middle of the list.
    Total size is dynamic, avoiding resizing cost.
    
    Search: O(N) must traverse(so thorugh each node) sequentially 
    Delection: O(1) if node is already located, otherwise O(N)
    Appending: O(1) if we have a pointer to tail, otherwise O(N)
     */
    println!("Linked list Example!");
    data_structures::linked_list::run();


    /*
    Usage: 
    LIFO or Last In First Out , for example - append 10,20,30, 
    on top will be 30, so it first gonna out from the stack, then 20, and after 10

    Good to use during depth-first search , undo/redo functionality, expression evaluation
    Search: O(N), as it must traverse from the bottom
    Delection: O(1) , popping the top element
    Appending: O(1), pushing onto the stack
     */
    println!("Stack Example!");
    data_structures::stacks::run();


    /*
    Usage:
    FIFO - First In First Out order 
    real time systems, breadth-first search, scheduling
    Search: O(N) - must traverse through entire query
    Deletion: O(1) - deque from top 
    Appending: O(1) - enque in the back
    */

    print!("Query Example!\n");
    data_structures::queries::run();   



    /*
    Usage:
    need fast key-based lookups, insertions, or deletions
    use cases like caching, dictionaries, or counting ocurrences in elements.

    Search: O(1) or O(N) in the worst case
    Deletion: O(1), average case
    Appending: O(1), average case
     */

    print!("HashMap example!\n");
    data_structures::hash_tables::run();


    /*
    Big O - worst case 
    Big Omega - best case
    Big Theta - average case 
     */
}
