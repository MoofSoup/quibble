Minimum viable product:
a rust library that has an API that is able to create a single frame stack allocated linked list, push elements to the top, and pop elements from the top. The only situation in which a single frame stack allocated linked list is special is in situations in which the algorithm can store data in a last in first out manner

Implementing a stack allocated linked list in safe Rust is an exercise in understanding the spirit of the borrow checker. The borrow checker doesn't forbid you from implementing the structs, as long as instead of making it so that a node owns all subsequent nodes you reframe it as every node has a read only reference to its implied owner. This reframing means that you have to think of the list backwards, but assuming a node always has a read only reference to its owner, you can be certain  that every node is owned, and thus the list is finite

However, Safe Rust does not allow you to instantiate such a list, because you cant blah...

This leads to a very interesting situation, in which the borrow checker says that the ownership structures of the list are well defined and is confident that upon deletion, the entire list will be properly deleted. But it also means that the Borrow Checker is confident that such a list is impossible to implement. However, such a list *is* possible to implement, *within the confines of safe rust.*

Safe Rust provides a programmer multiple options for memory management, because Rust doesn't guarantee that a destructor will always run. In other words, there are some types of undefined behavior that Rust Programmers have access to, within "safe" rust. We can use this undefined behavior behind the scenes to create a public facing API for a stack allocated linked list with well defined behavior. 

## Step 1: The basis for our list
The basis of our list is simple: 
```
enum Owner
	ListNode(&'a ListNode<'a>)
	ListHead(&'a ListHead<'a>)
struct ListNode<T>
	owner: Owner<'a>
	value: T
```
Within these structures, 2 things are ensured
 - every ListNode has an owner
 - no ListNode will outlive its owner
We are reframing the concept of ownership. In our new framing, ownership does not mean that a node that owns another can manipulate the data stored within the node. It just means that for every node, there is a node responsible for its deletion. Why is it that Rust's Borrow checker allows us to do this?