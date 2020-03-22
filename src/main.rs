use owning_ref::OwningRefMut;
use std::cell::RefCell;

#[test]
fn as_owner_mut_is_unsound() {
    let mut or: OwningRefMut<Box<i32>, i32> = OwningRefMut::new(Box::new(42));
    *or.as_owner_mut() = Box::new(43);
    assert_eq!(*or, 42);
}

#[test]
fn as_owner_is_unsound() {
    // only for OwningRefMut, not OwningRef
    let or: OwningRefMut<Box<RefCell<Box<i32>>>, RefCell<Box<i32>>> =
        OwningRefMut::new(Box::new(RefCell::new(Box::new(42))));
    let or2: OwningRefMut<Box<RefCell<Box<i32>>>, i32> =
        or.map_mut(|ref_cell| &mut **ref_cell.get_mut());
    *(or2.as_owner().borrow_mut()) = Box::new(43);
    assert_eq!(*or2, 42);
}
