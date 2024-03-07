## Data Structures & Ownership Model

In this project, I design and implement a stack-allocated linked list in rust, carefully navigating rust's borrow checker. A more practical project would be to simply create a bunch of nodes indexed with an array, but thats not fun. Whats fun is lawyering Rust's borrow checker to find an impractical yet still legal implementation of a design pattern that, in order to conform to the spirit of the borrow checker, is generally implemented differently. In order to solve this problem, we must first recontextualize the concept of the linked list into the language of the borrow checker. In order to do this, our core data structures are `Owner` and `ListNode`.

- **Owner:** The `Owner` enum was pivotal in defining the ownership relationship between list nodes. It has variants to represent whether a `ListNode` is owned by the list head or  by another `ListNode`.
- **ListNode:** Each `ListNode` contains a value, an index, and an `Owner`. The index is used differently from an array based list, as the node with the highest index will be the list head, and own all previous nodes. 
- **ListHead:** The List Head is a struct that owns all subsequent members of the list

These structures were adapted to ensure that each node maintains ownership over all subsequent nodes, with a read-only back-reference to its owner, circumventing the need for array-based tracking.

## Lifetime Management & Safety

Rust's lifetime annotations and ownership rules played central roles in safeguarding against unsafe access patterns or dangling references. Lifetime annotations ensured that each node's lifecycle is explicitly tied to the scope of its owner, preventing premature deallocation or invalid reference access. These measures were integral in maintaining the integrity and safety of the linked list structure.

## Push and Pop Implementations

- **Push Implementation:** For adding new elements, we adhered to the LIFO principle. New `ListNode`s assumed the role of the new list head upon creation, with the previous head becoming its subsequent node. This required careful reassignment of ownership while maintaining Rust's strict borrowing rules.
- **Pop Implementation:** Removing elements from the list involved returning the value of the current head, if present, and reassigning its next node as the new head. Strategies such as `std::mem::replace` were crucial for safely altering ownership within immutable contexts.

## Challenges & Solutions

A significant challenge was implementing dynamic list manipulation while strictly adhering to stack allocation and Rust's ownership model. Ensuring safety and efficiency, especially in ownership transfer during push/pop operations, demanded creative solutions.

- **Solution:** Utilizing `std::mem::replace` emerged as a key strategy in overcoming these challenges. It allowed us to safely swap elements and ownership within the list without violating Rust's borrowing rules, facilitating crucial operations like push and pop within our stack-allocated constraints. Using std::mem operations lets us access code that was written in unsafe rust from a safe context, allowing us to leverage rust's borrow checker for memory safety, while avoiding the borrow checker when it gets in the way.

## Learning & Insights

This project fulfilled its goal: It was an effective medium for experimenting with Rust's data structures and memory management semantics. Experimenting with a stack-allocated linked list pushed the boundaries of what can be achieved with safe Rust, offering valuable insights into managing memory and references without relying on the heap or arrays.

#### Insights Gained:

- The intricate balance between ownership and mutability in Rust and how it impacts the design of data structures.
- Creative application of standard library functions like `std::mem::replace` to circumvent typical borrowing constraints.

### Performance Testing
Is this stack allocated linked list actually able to compete with a heap allocated linked list??
## Documentation

This summary encapsulates the journey and technical accomplishments in realizing a stack-allocated linked list in Rust. From structuring and managing data with a focus on ownership and lifetimes to implementing fundamental list operations under strict memory management constraints, the project stands as a testament to Rust's capabilities in creating safe, efficient data structures.

Future enhancements could include exploring ways to incorporate generics for type flexibility, as well as investigating approaches to further optimize memory usage and access patterns. The insights and experience gained from this endeavor will certainly inform and inspire subsequent projects and enhancements, continuing the exploration of Rust's potent features in crafting sophisticated, safe software solutions.

Data structures:
- ListNode
- Listhead
- Owner: an enum which contains a read only reference to a node's owner. Can either be ListHead or another ListNode

Functions: create head, append list, pop, push, delete node,
create head creates a new node as list head
append list substitutes the first node 

The list tail, or the last node that was created, owns every node created before it. Additionally, all the list nodes it owns have the same lifetime as it. This makes deleting the list tail an impossible operation when operating strictly within the confines of the borrow checker, without deleting the rest of the list. In order to delete a section of the list, the pop function needs to transfer ownership of the remaining list to a new list tail.
The list head is the first node created, stores no value, and has an id of 0