//! A doubly-linked list with owned nodes
//!
//! The `XorLinkedList` allows pushing and popping elements at either end
//! in constant time.
//!
//! Almost always it is better to use `Vec` or [`VecDeque`] instead of
//! [`LinkedList`]. In general, array-based containers are faster,
//! more memory efficient and make better use of CPU cache.
//!
//! [`LinkedList`]: ../linked_list/struct.LinkedList.html
//! [`VecDeque`]: ../vec_deque/struct.VecDeque.html

use std::{
    cmp::Ordering,
    fmt,
    hash::{Hash, Hasher},
    iter::{FromIterator, FusedIterator},
    marker::PhantomData,
    mem,
    ptr::{null_mut, NonNull},
};

/// A doubly-linked list with owned nodes
///
/// The `XorLinkedList` allows pushing and popping elements at either end
/// in constant time.
///
/// Almost always it is better to use `Vec` or `VecDeque` instead of
/// `XorLinkedList`. In general, array-based containers are faster,
/// more memory efficient and make better use of CPU cache.
pub struct XorLinkedList<T> {
    head: Option<NonNull<Node<T>>>,
    tail: Option<NonNull<Node<T>>>,
    len: usize,
    marker: PhantomData<Node<T>>,
}

struct Node<T> {
    reference: usize,
    data: T,
}

/// An iterator over the elements of a `XorLinkedList`.
///
/// This `struct` is created by the [`iter`] method on [`XorLinkedList`]. See its
/// documentation for more.
///
/// [`iter`]: struct.XorLinkedList.html#method.iter
/// [`XorLinkedList`]: struct.XorLinkedList.html
#[derive(Clone)]
pub struct Iter<'a, T: 'a> {
    head: Option<NonNull<Node<T>>>,
    tail: Option<NonNull<Node<T>>>,
    last_head: Option<NonNull<Node<T>>>,
    last_tail: Option<NonNull<Node<T>>>,
    len: usize,
    marker: PhantomData<&'a Node<T>>,
}

impl<'a, T: 'a + fmt::Debug> fmt::Debug for Iter<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_tuple("Iter").field(&self.len).finish()
    }
}

/// A mutable iterator over the elements of a `XorLinkedList`.
///
/// This `struct` is created by the [`iter_mut`] method on [`XorLinkedList`]. See its
/// documentation for more.
///
/// [`iter_mut`]: struct.XorLinkedList.html#method.iter_mut
/// [`XorLinkedList`]: struct.XorLinkedList.html
pub struct IterMut<'a, T: 'a> {
    head: Option<NonNull<Node<T>>>,
    tail: Option<NonNull<Node<T>>>,
    last_head: Option<NonNull<Node<T>>>,
    last_tail: Option<NonNull<Node<T>>>,
    len: usize,
    marker: PhantomData<Box<Node<&'a T>>>,
}

impl<'a, T: 'a + fmt::Debug> fmt::Debug for IterMut<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_tuple("IterMut").field(&self.len).finish()
    }
}

/// An owning iterator over the elements of a `XorLinkedList`.
///
/// This `struct` is created by the [`into_iter`] method on [`XorLinkedList`][`XorLinkedList`]
/// (provided by the `IntoIterator` trait). See its documentation for more.
///
/// [`into_iter`]: struct.XorLinkedList.html#method.into_iter
/// [`XorLinkedList`]: struct.XorLinkedList.html
pub struct IntoIter<T> {
    list: XorLinkedList<T>,
}

impl<T: fmt::Debug> fmt::Debug for IntoIter<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_tuple("IntoIter").field(&self.list).finish()
    }
}

impl<T> Node<T> {
    fn new(data: T) -> Node<T> {
        Node { reference: 0, data }
    }

    fn into_data(self) -> T {
        self.data
    }
}

// private methods
impl<T> XorLinkedList<T> {
    fn calculate_reference(
        previous: Option<NonNull<Node<T>>>,
        next: Option<NonNull<Node<T>>>,
    ) -> usize {
        let pr = previous.map(|i| i.as_ptr()).unwrap_or(null_mut());
        let ne = next.map(|i| i.as_ptr()).unwrap_or(null_mut());
        pr as usize ^ ne as usize
    }

    fn get_element(
        previous_or_next: Option<NonNull<Node<T>>>,
        reference: usize,
    ) -> Option<NonNull<Node<T>>> {
        let other = previous_or_next.map(|i| i.as_ptr()).unwrap_or(null_mut());
        NonNull::new((other as usize ^ reference) as *mut Node<T>)
    }

    /// Adds the given node to the front of the list.
    #[inline]
    fn push_front_node(&mut self, mut node: NonNull<Node<T>>) {
        unsafe {
            match self.head {
                None => {
                    node.as_mut().reference = 0;
                    self.tail = Some(node);
                }
                Some(mut head) => {
                    let next_head = Self::get_element(None, head.as_ref().reference);
                    head.as_mut().reference = Self::calculate_reference(Some(node), next_head);
                    node.as_mut().reference = Self::calculate_reference(None, Some(head));
                }
            }
            self.head = Some(node);
            self.len += 1;
        }
    }

    /// Removes and returns the node at the front of the list.
    #[inline]
    fn pop_front_node(&mut self) -> Option<Box<Node<T>>> {
        self.head.map(|node| unsafe {
            if let Some(mut new_head) = Self::get_element(None, node.as_ref().reference) {
                let next_new_head = Self::get_element(Some(node), new_head.as_ref().reference);
                new_head.as_mut().reference = Self::calculate_reference(None, next_new_head);
                self.head = Some(new_head);
            } else {
                self.head = None;
                self.tail = None;
            }
            self.len -= 1;
            Box::from_raw(node.as_ptr())
        })
    }

    /// Adds the given node to the back of the list.
    #[inline]
    fn push_back_node(&mut self, mut node: NonNull<Node<T>>) {
        unsafe {
            match self.tail {
                None => {
                    node.as_mut().reference = 0;
                    self.head = Some(node);
                }
                Some(mut tail) => {
                    let prev_head = Self::get_element(None, tail.as_ref().reference);
                    tail.as_mut().reference = Self::calculate_reference(prev_head, Some(node));
                    node.as_mut().reference = Self::calculate_reference(Some(tail), None);
                }
            }
            self.tail = Some(node);
            self.len += 1;
        }
    }

    /// Removes and returns the node at the back of the list.
    #[inline]
    fn pop_back_node(&mut self) -> Option<Box<Node<T>>> {
        self.tail.map(|node| unsafe {
            if let Some(mut new_tail) = Self::get_element(None, node.as_ref().reference) {
                let next_new_tail = Self::get_element(Some(node), new_tail.as_ref().reference);
                new_tail.as_mut().reference = Self::calculate_reference(next_new_tail, None);
                self.tail = Some(new_tail);
            } else {
                self.head = None;
                self.tail = None;
            }
            self.len -= 1;
            Box::from_raw(node.as_ptr())
        })
    }
}

impl<T> Default for XorLinkedList<T> {
    /// Creates an empty `XorLinkedList<T>`
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl<T> XorLinkedList<T> {
    /// Creates an empty `XorLinkedList`
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_utils::XorLinkedList;
    ///
    /// let list: XorLinkedList<u32> = XorLinkedList::new();
    /// ```
    #[inline]
    pub fn new() -> Self {
        XorLinkedList {
            head: None,
            tail: None,
            len: 0,
            marker: PhantomData,
        }
    }

    /// Moves all elements from `other` to the end of the list.
    ///
    /// This reuses all the nodes from `other` and moves them into `self`. After
    /// this operation, `other` becomes empty.
    ///
    /// This operation should compute in O(1) time and O(1) memory.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_utils::XorLinkedList;
    ///
    /// let mut list1 = XorLinkedList::new();
    /// list1.push_back('a');
    ///
    /// let mut list2 = XorLinkedList::new();
    /// list2.push_back('b');
    /// list2.push_back('c');
    ///
    /// list1.append(&mut list2);
    ///
    /// let mut iter = list1.iter();
    /// assert_eq!(iter.next(), Some(&'a'));
    /// assert_eq!(iter.next(), Some(&'b'));
    /// assert_eq!(iter.next(), Some(&'c'));
    /// assert!(iter.next().is_none());
    ///
    /// assert!(list2.is_empty());
    /// ```
    pub fn append(&mut self, other: &mut Self) {
        match self.tail {
            None => mem::swap(self, other),
            Some(mut tail) => {
                if let Some(mut other_head) = other.head.take() {
                    unsafe {
                        let other_head_next =
                            Self::get_element(None, other_head.as_ref().reference);
                        other_head.as_mut().reference =
                            Self::calculate_reference(Some(tail), other_head_next);

                        let tail_prev = Self::get_element(None, tail.as_ref().reference);
                        tail.as_mut().reference =
                            Self::calculate_reference(tail_prev, Some(other_head));
                    }

                    self.tail = other.tail.take();
                    self.len += mem::replace(&mut other.len, 0);
                }
            }
        }
    }

    /// Provides a forward iterator
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_utils::XorLinkedList;
    ///
    /// let mut list: XorLinkedList<u32> = XorLinkedList::new();
    ///
    /// list.push_back(0);
    /// list.push_back(1);
    /// list.push_back(2);
    ///
    /// let mut iter = list.iter();
    /// assert_eq!(iter.next(), Some(&0));
    /// assert_eq!(iter.next(), Some(&1));
    /// assert_eq!(iter.next(), Some(&2));
    /// assert_eq!(iter.next(), None);
    /// ```
    #[inline]
    pub fn iter(&self) -> Iter<T> {
        Iter {
            head: self.head,
            tail: self.tail,
            last_head: None,
            last_tail: None,
            len: self.len,
            marker: PhantomData,
        }
    }

    /// Provides a forward iterator with mutable references
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_utils::XorLinkedList;
    ///
    /// let mut list: XorLinkedList<u32> = XorLinkedList::new();
    ///
    /// list.push_back(0);
    /// list.push_back(1);
    /// list.push_back(2);
    ///
    /// for element in list.iter_mut() {
    ///     *element += 10;
    /// }
    ///
    /// let mut iter = list.iter();
    /// assert_eq!(iter.next(), Some(&10));
    /// assert_eq!(iter.next(), Some(&11));
    /// assert_eq!(iter.next(), Some(&12));
    /// assert_eq!(iter.next(), None);
    /// ```
    #[inline]
    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut {
            head: self.head,
            tail: self.tail,
            last_head: None,
            last_tail: None,
            len: self.len,
            marker: PhantomData,
        }
    }

    /// Returns `true` if the `XorLinkedList` is empty
    ///
    /// This operation should compute in O(1) time.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_utils::XorLinkedList;
    ///
    /// let mut dl = XorLinkedList::new();
    /// assert!(dl.is_empty());
    ///
    /// dl.push_front("foo");
    /// assert!(!dl.is_empty());
    /// ```
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    /// Returns the length of the `XorLinkedList`
    ///
    /// This operation should compute in O(1) time.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_utils::XorLinkedList;
    ///
    /// let mut dl = XorLinkedList::new();
    ///
    /// dl.push_front(2);
    /// assert_eq!(dl.len(), 1);
    ///
    /// dl.push_front(1);
    /// assert_eq!(dl.len(), 2);
    ///
    /// dl.push_back(3);
    /// assert_eq!(dl.len(), 3);
    /// ```
    #[inline]
    pub fn len(&self) -> usize {
        self.len
    }

    /// Removes all elements from the `XorLinkedList`.
    ///
    /// This operation should compute in O(n) time
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_utils::XorLinkedList;
    ///
    /// let mut dl = XorLinkedList::new();
    ///
    /// dl.push_front(2);
    /// dl.push_front(1);
    /// assert_eq!(dl.len(), 2);
    /// assert_eq!(dl.front(), Some(&1));
    ///
    /// dl.clear();
    /// assert_eq!(dl.len(), 0);
    /// assert_eq!(dl.front(), None);
    /// ```
    #[inline]
    pub fn clear(&mut self) {
        *self = Self::new();
    }

    /// Returns `true` if the `XorLinkedList` contains an element equal to the
    /// given value
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_utils::XorLinkedList;
    ///
    /// let mut list: XorLinkedList<u32> = XorLinkedList::new();
    ///
    /// list.push_back(0);
    /// list.push_back(1);
    /// list.push_back(2);
    ///
    /// assert_eq!(list.contains(&0), true);
    /// assert_eq!(list.contains(&10), false);
    /// ```
    pub fn contains(&self, x: &T) -> bool
    where
        T: PartialEq<T>,
    {
        self.iter().any(|e| e == x)
    }

    /// Provides a reference to the front element, or `None` if the list is
    /// empty
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_utils::XorLinkedList;
    ///
    /// let mut dl = XorLinkedList::new();
    /// assert_eq!(dl.front(), None);
    ///
    /// dl.push_front(1);
    /// assert_eq!(dl.front(), Some(&1));
    /// ```
    #[inline]
    pub fn front(&self) -> Option<&T> {
        unsafe { self.head.as_ref().map(|node| &node.as_ref().data) }
    }

    /// Provides a mutable reference to the front element, or `None` if the list
    /// is empty
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_utils::XorLinkedList;
    ///
    /// let mut dl = XorLinkedList::new();
    /// assert_eq!(dl.front(), None);
    ///
    /// dl.push_front(1);
    /// assert_eq!(dl.front(), Some(&1));
    ///
    /// match dl.front_mut() {
    ///     None => {},
    ///     Some(x) => *x = 5,
    /// }
    /// assert_eq!(dl.front(), Some(&5));
    /// ```
    #[inline]
    pub fn front_mut(&mut self) -> Option<&mut T> {
        unsafe { self.head.as_mut().map(|node| &mut node.as_mut().data) }
    }

    /// Provides a reference to the back element, or `None` if the list is
    /// empty
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_utils::XorLinkedList;
    ///
    /// let mut dl = XorLinkedList::new();
    /// assert_eq!(dl.back(), None);
    ///
    /// dl.push_back(1);
    /// assert_eq!(dl.back(), Some(&1));
    /// ```
    #[inline]
    pub fn back(&self) -> Option<&T> {
        unsafe { self.tail.as_ref().map(|node| &node.as_ref().data) }
    }

    /// Provides a mutable reference to the back element, or `None` if the list
    /// is empty
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_utils::XorLinkedList;
    ///
    /// let mut dl = XorLinkedList::new();
    /// assert_eq!(dl.back(), None);
    ///
    /// dl.push_back(1);
    /// assert_eq!(dl.back(), Some(&1));
    ///
    /// match dl.back_mut() {
    ///     None => {},
    ///     Some(x) => *x = 5,
    /// }
    /// assert_eq!(dl.back(), Some(&5));
    /// ```
    #[inline]
    pub fn back_mut(&mut self) -> Option<&mut T> {
        unsafe { self.tail.as_mut().map(|node| &mut node.as_mut().data) }
    }

    /// Adds an element first in the list.
    ///
    /// This operation should compute in O(1) time.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_utils::XorLinkedList;
    ///
    /// let mut dl = XorLinkedList::new();
    ///
    /// dl.push_front(2);
    /// assert_eq!(dl.front().unwrap(), &2);
    ///
    /// dl.push_front(1);
    /// assert_eq!(dl.front().unwrap(), &1);
    /// ```
    pub fn push_front(&mut self, data: T) {
        unsafe {
            let value = Box::new(Node::new(data));
            self.push_front_node(NonNull::new_unchecked(Box::into_raw(value)));
        }
    }

    /// Removes the first element and returns it, or `None` if the list is
    /// empty.
    ///
    /// This operation should compute in O(1) time.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_utils::XorLinkedList;
    ///
    /// let mut d = XorLinkedList::new();
    /// assert_eq!(d.pop_front(), None);
    ///
    /// d.push_front(1);
    /// d.push_front(3);
    /// assert_eq!(d.pop_front(), Some(3));
    /// assert_eq!(d.pop_front(), Some(1));
    /// assert_eq!(d.pop_front(), None);
    /// ```
    pub fn pop_front(&mut self) -> Option<T> {
        self.pop_front_node().map(|node| node.into_data())
    }

    /// Appends an element to the back of a list
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_utils::XorLinkedList;
    ///
    /// let mut d = XorLinkedList::new();
    /// d.push_back(1);
    /// d.push_back(3);
    /// assert_eq!(3, *d.back().unwrap());
    /// ```
    pub fn push_back(&mut self, data: T) {
        unsafe {
            let value = Box::new(Node::new(data));
            self.push_back_node(NonNull::new_unchecked(Box::into_raw(value)));
        }
    }

    /// Removes the last element from a list and returns it, or `None` if
    /// it is empty
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_utils::XorLinkedList;
    ///
    /// let mut d = XorLinkedList::new();
    /// assert_eq!(d.pop_back(), None);
    /// d.push_back(1);
    /// d.push_back(3);
    /// assert_eq!(d.pop_back(), Some(3));
    /// ```
    pub fn pop_back(&mut self) -> Option<T> {
        self.pop_back_node().map(|node| node.into_data())
    }

    /// Splits the list into two at the given index. Returns everything after the given index,
    /// including the index
    ///
    /// This operation should compute in O(n) time.
    ///
    /// # Panics
    ///
    /// Panics if `at > len`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_utils::XorLinkedList;
    ///
    /// let mut d = XorLinkedList::new();
    ///
    /// d.push_front(1);
    /// d.push_front(2);
    /// d.push_front(3);
    ///
    /// let mut splitted = d.split_off(2);
    ///
    /// assert_eq!(splitted.pop_front(), Some(1));
    /// assert_eq!(splitted.pop_front(), None);
    /// ```
    pub fn split_off(&mut self, at: usize) -> XorLinkedList<T> {
        let len = self.len();
        assert!(at <= len, "Cannot split off at a nonexistent index");
        if at == 0 {
            return mem::replace(self, Self::new());
        } else if at == len {
            return Self::new();
        }

        // Below, we iterate towards the `i-1`th node, either from the start or the end,
        // depending on which would be faster
        let split_node = if at - 1 <= len - 1 - (at - 1) {
            let mut iter = self.iter_mut();
            let mut last = None;
            // instead of skipping using .skip() (which creates a new struct),
            // we skip manually so we can access the head field without
            // depending on implementation details of Skip
            for _ in 0..at - 1 {
                last = iter.head;
                iter.next();
            }
            (iter.head, last)
        } else {
            // better off starting from the end
            let mut iter = self.iter_mut();
            let mut last = None;
            for _ in 0..len - (at - 1) {
                last = iter.tail;
                iter.next_back();
            }
            (last, iter.tail)
        };

        // The split node is the new tail node of the first part and owns
        // the head of the second part
        let mut second_part_head = None;

        unsafe {
            let mut element = split_node.0.unwrap();
            let element_before = split_node.1;
            let next_element =
                XorLinkedList::get_element(element_before, element.as_ref().reference);
            element.as_mut().reference = XorLinkedList::calculate_reference(element_before, None);
            if let Some(mut next_element) = next_element {
                let next_next_element =
                    XorLinkedList::get_element(Some(element), next_element.as_ref().reference);
                next_element.as_mut().reference =
                    XorLinkedList::calculate_reference(None, next_next_element);
                second_part_head = Some(next_element);
            }
        }

        let second_part = XorLinkedList {
            head: second_part_head,
            tail: self.tail,
            len: len - at,
            marker: PhantomData,
        };

        // Fix the tail ptr of the first part
        self.tail = split_node.0;
        self.len = at;

        second_part
    }
}

impl<T> Drop for XorLinkedList<T> {
    fn drop(&mut self) {
        while let Some(_) = Self::pop_back_node(self) {}
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    #[inline]
    fn next(&mut self) -> Option<&'a T> {
        if self.len == 0 {
            None
        } else {
            self.head.map(|node| unsafe {
                if let Some(node_next) =
                    XorLinkedList::get_element(self.last_head, node.as_ref().reference)
                {
                    self.head = Some(node_next);
                } else {
                    self.head = None;
                }
                self.last_head = Some(node);
                let node_local = &*node.as_ptr();
                self.len -= 1;
                &node_local.data
            })
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }
}

impl<'a, T> DoubleEndedIterator for Iter<'a, T> {
    #[inline]
    fn next_back(&mut self) -> Option<&'a T> {
        if self.len == 0 {
            None
        } else {
            self.tail.map(|node| unsafe {
                if let Some(node_prev) =
                    XorLinkedList::get_element(self.last_tail, node.as_ref().reference)
                {
                    self.head = Some(node_prev);
                } else {
                    self.head = None;
                }
                self.last_tail = Some(node);
                let node_local = &*node.as_ptr();
                self.len -= 1;
                &node_local.data
            })
        }
    }
}

impl<'a, T> ExactSizeIterator for Iter<'a, T> {}

impl<'a, T> FusedIterator for Iter<'a, T> {}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    #[inline]
    fn next(&mut self) -> Option<&'a mut T> {
        if self.len == 0 {
            None
        } else {
            self.head.map(|node| unsafe {
                if let Some(node_next) =
                    XorLinkedList::get_element(self.last_head, node.as_ref().reference)
                {
                    self.head = Some(node_next);
                } else {
                    self.head = None;
                }
                self.last_head = Some(node);
                let node_local = &mut *node.as_ptr();
                self.len -= 1;
                &mut node_local.data
            })
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }
}

impl<'a, T> DoubleEndedIterator for IterMut<'a, T> {
    #[inline]
    fn next_back(&mut self) -> Option<&'a mut T> {
        if self.len == 0 {
            None
        } else {
            self.tail.map(|node| unsafe {
                let test = node.as_ref().reference;
                if let Some(node_prev) = XorLinkedList::get_element(self.last_tail, test) {
                    self.tail = Some(node_prev);
                } else {
                    self.tail = None;
                }
                self.last_tail = Some(node);
                let node_local = &mut *node.as_ptr();
                self.len -= 1;
                &mut node_local.data
            })
        }
    }
}

impl<'a, T> ExactSizeIterator for IterMut<'a, T> {}

impl<'a, T> FusedIterator for IterMut<'a, T> {}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<T> {
        self.list.pop_front()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.list.len, Some(self.list.len))
    }
}

impl<T> DoubleEndedIterator for IntoIter<T> {
    #[inline]
    fn next_back(&mut self) -> Option<T> {
        self.list.pop_back()
    }
}

impl<T> ExactSizeIterator for IntoIter<T> {}

impl<T> FusedIterator for IntoIter<T> {}

impl<T> FromIterator<T> for XorLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut list = Self::new();
        list.extend(iter);
        list
    }
}

impl<T> IntoIterator for XorLinkedList<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    /// Consumes the list into an iterator yielding elements by value
    #[inline]
    fn into_iter(self) -> IntoIter<T> {
        IntoIter { list: self }
    }
}

impl<'a, T> IntoIterator for &'a XorLinkedList<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Iter<'a, T> {
        self.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut XorLinkedList<T> {
    type Item = &'a mut T;
    type IntoIter = IterMut<'a, T>;

    fn into_iter(self) -> IterMut<'a, T> {
        self.iter_mut()
    }
}

impl<T> Extend<T> for XorLinkedList<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for elt in iter {
            self.push_back(elt);
        }
    }
}

impl<'a, T: 'a + Copy + fmt::Debug> Extend<&'a T> for XorLinkedList<T> {
    fn extend<I: IntoIterator<Item = &'a T>>(&mut self, iter: I) {
        self.extend(iter.into_iter().cloned());
    }
}

impl<T: PartialEq + fmt::Debug> PartialEq for XorLinkedList<T> {
    fn eq(&self, other: &Self) -> bool {
        self.len() == other.len() && self.iter().eq(other)
    }
}

impl<T: Eq + fmt::Debug> Eq for XorLinkedList<T> {}

impl<T: PartialOrd + fmt::Debug> PartialOrd for XorLinkedList<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.iter().partial_cmp(other)
    }
}

impl<T: Ord + fmt::Debug> Ord for XorLinkedList<T> {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.iter().cmp(other)
    }
}

impl<T: Clone + fmt::Debug> Clone for XorLinkedList<T> {
    fn clone(&self) -> Self {
        self.iter().cloned().collect()
    }
}

impl<T: fmt::Debug> fmt::Debug for XorLinkedList<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_list().entries(self).finish()
    }
}

impl<T: Hash + fmt::Debug> Hash for XorLinkedList<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.len().hash(state);
        for elt in self {
            elt.hash(state);
        }
    }
}

// Ensure that `XorLinkedList` and its read-only iterators are covariant in their type parameters
#[allow(dead_code)]
fn assert_covariance() {
    fn a<'a>(x: XorLinkedList<&'static str>) -> XorLinkedList<&'a str> {
        x
    }
    fn b<'i, 'a>(x: Iter<'i, &'static str>) -> Iter<'i, &'a str> {
        x
    }
    fn c<'a>(x: IntoIter<&'static str>) -> IntoIter<&'a str> {
        x
    }
}

unsafe impl<T: Send + fmt::Debug> Send for XorLinkedList<T> {}

unsafe impl<T: Sync + fmt::Debug> Sync for XorLinkedList<T> {}

unsafe impl<'a, T: Sync + fmt::Debug> Send for Iter<'a, T> {}

unsafe impl<'a, T: Sync + fmt::Debug> Sync for Iter<'a, T> {}

unsafe impl<'a, T: Send + fmt::Debug> Send for IterMut<'a, T> {}

unsafe impl<'a, T: Sync + fmt::Debug> Sync for IterMut<'a, T> {}

#[cfg(test)]
mod tests {
    use super::{Node, XorLinkedList};
    use rand::{thread_rng, RngCore};
    use std::ptr::NonNull;
    use std::thread;
    use std::vec::Vec;

    #[cfg(test)]
    fn list_from<T: Clone>(v: &[T]) -> XorLinkedList<T> {
        v.iter().cloned().collect()
    }

    fn check_links<T>(list: &XorLinkedList<T>) {
        let mut node_ptr: NonNull<Node<T>>;
        let tail_ptr: NonNull<Node<T>>;
        let head_ptr: NonNull<Node<T>>;
        match (list.head, list.tail) {
            (None, None) => {
                assert_eq!(0, list.len);
                return;
            }
            (None, Some(_)) | (Some(_), None) => panic!("tail and head must both be none or some"),
            (Some(head), Some(tail)) => {
                node_ptr = head;
                head_ptr = head;
                tail_ptr = tail;
            }
        }
        let mut last_ptr: Option<NonNull<Node<T>>> = None;
        unsafe {
            for _ in 0..(list.len - 1) {
                let next_element =
                    XorLinkedList::get_element(last_ptr, node_ptr.as_ref().reference)
                        .expect("next link is null, not good");
                last_ptr = Some(node_ptr);
                node_ptr = next_element;
            }
            assert_eq!(node_ptr, tail_ptr);

            last_ptr = None;
            for _ in 0..(list.len - 1) {
                let prev_element =
                    XorLinkedList::get_element(last_ptr, node_ptr.as_ref().reference)
                        .expect("prev link is null, not good");
                last_ptr = Some(node_ptr);
                node_ptr = prev_element;
            }
            assert_eq!(node_ptr, head_ptr);
        }
    }

    #[test]
    fn test_append() {
        // Empty to empty
        {
            let mut m = XorLinkedList::<i32>::new();
            let mut n = XorLinkedList::new();
            m.append(&mut n);
            check_links(&m);
            assert_eq!(m.len(), 0);
            assert_eq!(n.len(), 0);
        }
        // Non-empty to empty
        {
            let mut m = XorLinkedList::new();
            let mut n = XorLinkedList::new();
            n.push_back(2);
            m.append(&mut n);
            check_links(&m);
            assert_eq!(m.len(), 1);
            assert_eq!(m.pop_back(), Some(2));
            assert_eq!(n.len(), 0);
            check_links(&m);
        }
        // Empty to non-empty
        {
            let mut m = XorLinkedList::new();
            let mut n = XorLinkedList::new();
            m.push_back(2);
            m.append(&mut n);
            check_links(&m);
            assert_eq!(m.len(), 1);
            assert_eq!(m.pop_back(), Some(2));
            check_links(&m);
        }

        // Non-empty to non-empty
        let v = vec![1, 2, 3, 4, 5];
        let u = vec![9, 8, 1, 2, 3, 4, 5];
        let mut m = list_from(&v);
        let mut n = list_from(&u);
        m.append(&mut n);
        check_links(&m);
        let mut sum = v;
        sum.extend_from_slice(&u);
        assert_eq!(sum.len(), m.len());
        for elt in sum {
            assert_eq!(m.pop_front(), Some(elt))
        }
        assert_eq!(n.len(), 0);
        // let's make sure it's working properly, since we
        // did some direct changes to private members
        n.push_back(3);
        assert_eq!(n.len(), 1);
        assert_eq!(n.pop_front(), Some(3));
        check_links(&n);
    }

    #[test]
    #[cfg_attr(target_os = "emscripten", ignore)]
    fn test_send() {
        let n = list_from(&[1, 2, 3]);
        thread::spawn(move || {
            check_links(&n);
            let a: &[_] = &[&1, &2, &3];
            assert_eq!(a, &*n.iter().collect::<Vec<_>>());
        }).join()
        .ok()
        .unwrap();
    }

    #[test]
    fn test_fuzz() {
        for _ in 0..25 {
            fuzz_test(3);
            fuzz_test(16);
            fuzz_test(189);
        }
    }

    #[test]
    fn test_26021() {
        // There was a bug in split_off that failed to null out the RHS's head's prev ptr.
        // This caused the RHS's dtor to walk up into the LHS at drop and delete all of
        // its nodes.
        //
        // https://github.com/rust-lang/rust/issues/26021
        let mut v1 = XorLinkedList::new();
        v1.push_front(1);
        v1.push_front(1);
        v1.push_front(1);
        v1.push_front(1);
        let _ = v1.split_off(3); // Dropping this now should not cause laundry consumption
        assert_eq!(v1.len(), 3);

        assert_eq!(v1.iter().len(), 3);
        assert_eq!(v1.iter().collect::<Vec<_>>().len(), 3);
    }

    #[test]
    fn test_split_off() {
        let mut v1 = XorLinkedList::new();
        v1.push_front(1);
        v1.push_front(1);
        v1.push_front(1);
        v1.push_front(1);

        // test all splits
        for ix in 0..1 + v1.len() {
            let mut a = v1.clone();
            let b = a.split_off(ix);
            check_links(&a);
            check_links(&b);
            a.extend(b);
            assert_eq!(v1, a);
        }
    }

    #[cfg(test)]
    fn fuzz_test(sz: i32) {
        let mut m: XorLinkedList<_> = XorLinkedList::new();
        let mut v = vec![];
        for i in 0..sz {
            check_links(&m);
            let r: u8 = thread_rng().next_u32() as u8;
            match r % 6 {
                0 => {
                    m.pop_back();
                    v.pop();
                }
                1 => {
                    if !v.is_empty() {
                        m.pop_front();
                        v.remove(0);
                    }
                }
                2 | 4 => {
                    m.push_front(-i);
                    v.insert(0, -i);
                }
                3 | 5 | _ => {
                    m.push_back(i);
                    v.push(i);
                }
            }
        }

        check_links(&m);

        let mut i = 0;
        for (a, &b) in m.into_iter().zip(&v) {
            i += 1;
            assert_eq!(a, b);
        }
        assert_eq!(i, v.len());
    }

    #[test]
    fn test_pop_front() {
        let mut v1 = XorLinkedList::new();
        v1.push_front(1);
        v1.push_front(2);
        v1.push_front(3);
        v1.push_front(4);

        assert_eq!(Some(4), v1.pop_front());
        assert_eq!(Some(3), v1.pop_front());
        assert_eq!(Some(2), v1.pop_front());
        assert_eq!(Some(1), v1.pop_front());
        assert_eq!(None, v1.pop_front());
    }

    #[test]
    fn test_pop_back() {
        let mut v1 = XorLinkedList::new();
        v1.push_back(1);
        v1.push_back(2);
        v1.push_back(3);
        v1.push_back(4);

        assert_eq!(Some(4), v1.pop_back());
        assert_eq!(Some(3), v1.pop_back());
        assert_eq!(Some(2), v1.pop_back());
        assert_eq!(Some(1), v1.pop_back());
        assert_eq!(None, v1.pop_back());
    }

    #[test]
    fn test_front_and_front_mut() {
        let mut v1 = XorLinkedList::new();
        v1.push_front(1);
        v1.push_front(2);
        v1.push_front(3);
        v1.push_front(4);
        {
            let front_mut = v1.front_mut();
            if let Some(var) = front_mut {
                *var += 2;
            }
        }
        assert_eq!(Some(&(4 + 2)), v1.front());
    }

    #[test]
    fn test_back_and_back_mut() {
        let mut v1 = XorLinkedList::new();
        v1.push_back(1);
        v1.push_back(2);
        v1.push_back(3);
        v1.push_back(4);
        {
            let back_mut = v1.back_mut();
            if let Some(var) = back_mut {
                *var += 2;
            }
        }
        assert_eq!(Some(&(4 + 2)), v1.back());
    }

    #[test]
    fn test_contains() {
        let mut v1 = XorLinkedList::new();
        v1.push_back(1);
        v1.push_back(2);
        v1.push_back(3);
        v1.push_back(4);

        assert!(v1.contains(&1));
        assert!(v1.contains(&2));
        assert!(v1.contains(&3));
        assert!(v1.contains(&4));
        assert!(!v1.contains(&5));
    }
}
