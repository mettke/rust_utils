var N = null;var searchIndex = {};
searchIndex["rust_utils"]={"doc":"Rust utilities with various implementations of classes which cannot be found in the rust standard library","items":[[3,"BTrieMap","rust_utils","A TrieMap with owned nodes.",N,N],[3,"XorLinkedList","","A doubly-linked list with owned nodes",N,N],[0,"btrie","","A TrieMap with owned nodes.",N,N],[3,"BTrieMap","rust_utils::btrie","A TrieMap with owned nodes.",N,N],[11,"fmt","","",0,[[["self"],["formatter"]],["result"]]],[11,"clone","","",0,[[["self"]],["btriemap"]]],[11,"default","","",0,[[],["self"]]],[11,"new","","Creates an empty `BTrieMap`",0,[[],["self"]]],[11,"insert","","Inserts a given value into the `BTrieMap`. An existing Value with the given key will be overridden",0,[[["self"],["i"],["v"]]]],[11,"contains","","Returns `true` if the `BTrieMap` contains an element equal to the given value",0,[[["self"],["i"]],["bool"]]],[11,"get","","Returns the value available in the `BTrieMap` under the given key",0,[[["self"],["i"]],["option"]]],[11,"get_with_prefix","","Returns all values available in the `BTrieMap` under a given key prefix",0,[[["self"],["i"]],["vec"]]],[0,"xor_linked_list","rust_utils","A doubly-linked list with owned nodes",N,N],[3,"XorLinkedList","rust_utils::xor_linked_list","A doubly-linked list with owned nodes",N,N],[3,"Iter","","An iterator over the elements of a `XorLinkedList`.",N,N],[3,"IterMut","","A mutable iterator over the elements of a `XorLinkedList`.",N,N],[3,"IntoIter","","An owning iterator over the elements of a `XorLinkedList`.",N,N],[11,"clone","","",1,[[["self"]],["iter"]]],[11,"fmt","","",1,[[["self"],["formatter"]],["result"]]],[11,"fmt","","",2,[[["self"],["formatter"]],["result"]]],[11,"fmt","","",3,[[["self"],["formatter"]],["result"]]],[11,"default","","Creates an empty `XorLinkedList<T>`",4,[[],["self"]]],[11,"new","","Creates an empty `XorLinkedList`",4,[[],["self"]]],[11,"append","","Moves all elements from `other` to the end of the list.",4,[[["self"],["self"]]]],[11,"iter","","Provides a forward iterator",4,[[["self"]],["iter"]]],[11,"iter_mut","","Provides a forward iterator with mutable references",4,[[["self"]],["itermut"]]],[11,"is_empty","","Returns `true` if the `XorLinkedList` is empty",4,[[["self"]],["bool"]]],[11,"len","","Returns the length of the `XorLinkedList`",4,[[["self"]],["usize"]]],[11,"clear","","Removes all elements from the `XorLinkedList`.",4,[[["self"]]]],[11,"contains","","Returns `true` if the `XorLinkedList` contains an element equal to the given value",4,[[["self"],["t"]],["bool"]]],[11,"front","","Provides a reference to the front element, or `None` if the list is empty",4,[[["self"]],["option"]]],[11,"front_mut","","Provides a mutable reference to the front element, or `None` if the list is empty",4,[[["self"]],["option"]]],[11,"back","","Provides a reference to the back element, or `None` if the list is empty",4,[[["self"]],["option"]]],[11,"back_mut","","Provides a mutable reference to the back element, or `None` if the list is empty",4,[[["self"]],["option"]]],[11,"push_front","","Adds an element first in the list.",4,[[["self"],["t"]]]],[11,"pop_front","","Removes the first element and returns it, or `None` if the list is empty.",4,[[["self"]],["option"]]],[11,"push_back","","Appends an element to the back of a list",4,[[["self"],["t"]]]],[11,"pop_back","","Removes the last element from a list and returns it, or `None` if it is empty",4,[[["self"]],["option"]]],[11,"split_off","","Splits the list into two at the given index. Returns everything after the given index, including the index",4,[[["self"],["usize"]],["xorlinkedlist"]]],[11,"drop","","",4,[[["self"]]]],[11,"next","","",1,[[["self"]],["option"]]],[11,"size_hint","","",1,N],[11,"next_back","","",1,[[["self"]],["option"]]],[11,"next","","",2,[[["self"]],["option"]]],[11,"size_hint","","",2,N],[11,"next_back","","",2,[[["self"]],["option"]]],[11,"next","","",3,[[["self"]],["option"]]],[11,"size_hint","","",3,N],[11,"next_back","","",3,[[["self"]],["option"]]],[11,"from_iter","","",4,[[["i"]],["self"]]],[11,"into_iter","","Consumes the list into an iterator yielding elements by value",4,[[["self"]],["intoiter"]]],[11,"extend","","",4,[[["self"],["i"]]]],[11,"extend","","",4,[[["self"],["i"]]]],[11,"eq","","",4,[[["self"],["self"]],["bool"]]],[11,"partial_cmp","","",4,[[["self"],["self"]],["option",["ordering"]]]],[11,"cmp","","",4,[[["self"],["self"]],["ordering"]]],[11,"clone","","",4,[[["self"]],["self"]]],[11,"fmt","","",4,[[["self"],["formatter"]],["result"]]],[11,"hash","","",4,[[["self"],["h"]]]],[11,"into","rust_utils::btrie","",0,[[["self"]],["u"]]],[11,"to_owned","","",0,[[["self"]],["t"]]],[11,"clone_into","","",0,N],[11,"from","","",0,[[["t"]],["t"]]],[11,"try_from","","",0,[[["u"]],["result"]]],[11,"borrow","","",0,[[["self"]],["t"]]],[11,"borrow_mut","","",0,[[["self"]],["t"]]],[11,"try_into","","",0,[[["self"]],["result"]]],[11,"get_type_id","","",0,[[["self"]],["typeid"]]],[11,"into","rust_utils::xor_linked_list","",4,[[["self"]],["u"]]],[11,"to_owned","","",4,[[["self"]],["t"]]],[11,"clone_into","","",4,N],[11,"from","","",4,[[["t"]],["t"]]],[11,"into_iter","","",4,[[["self"]],["i"]]],[11,"try_from","","",4,[[["u"]],["result"]]],[11,"borrow","","",4,[[["self"]],["t"]]],[11,"borrow_mut","","",4,[[["self"]],["t"]]],[11,"try_into","","",4,[[["self"]],["result"]]],[11,"get_type_id","","",4,[[["self"]],["typeid"]]],[11,"into","","",1,[[["self"]],["u"]]],[11,"to_owned","","",1,[[["self"]],["t"]]],[11,"clone_into","","",1,N],[11,"from","","",1,[[["t"]],["t"]]],[11,"into_iter","","",1,[[["self"]],["i"]]],[11,"try_from","","",1,[[["u"]],["result"]]],[11,"borrow","","",1,[[["self"]],["t"]]],[11,"borrow_mut","","",1,[[["self"]],["t"]]],[11,"try_into","","",1,[[["self"]],["result"]]],[11,"get_type_id","","",1,[[["self"]],["typeid"]]],[11,"into","","",2,[[["self"]],["u"]]],[11,"from","","",2,[[["t"]],["t"]]],[11,"into_iter","","",2,[[["self"]],["i"]]],[11,"try_from","","",2,[[["u"]],["result"]]],[11,"borrow","","",2,[[["self"]],["t"]]],[11,"borrow_mut","","",2,[[["self"]],["t"]]],[11,"try_into","","",2,[[["self"]],["result"]]],[11,"get_type_id","","",2,[[["self"]],["typeid"]]],[11,"into","","",3,[[["self"]],["u"]]],[11,"from","","",3,[[["t"]],["t"]]],[11,"into_iter","","",3,[[["self"]],["i"]]],[11,"try_from","","",3,[[["u"]],["result"]]],[11,"borrow","","",3,[[["self"]],["t"]]],[11,"borrow_mut","","",3,[[["self"]],["t"]]],[11,"try_into","","",3,[[["self"]],["result"]]],[11,"get_type_id","","",3,[[["self"]],["typeid"]]]],"paths":[[3,"BTrieMap"],[3,"Iter"],[3,"IterMut"],[3,"IntoIter"],[3,"XorLinkedList"]]};
initSearch(searchIndex);