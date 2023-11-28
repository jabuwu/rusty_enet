use std::{marker::PhantomData, pin::Pin};

use crate::c_void;

#[derive(Copy, Clone)]
#[repr(C)]
pub(crate) struct ENetListNode<T> {
    next: *mut ENetListNode<T>,
    previous: *mut ENetListNode<T>,
}
impl<T> ENetListNode<T> {
    pub const fn zeroed() -> Self {
        ENetListNode {
            next: std::ptr::null_mut(),
            previous: std::ptr::null_mut(),
        }
    }
}

pub(crate) type ENetListIterator<T> = *mut ENetListNode<T>;
#[derive(Clone)]
#[repr(C)]
pub(crate) struct ENetList<T> {
    sentinel: Pin<Box<ENetListNode<T>>>,
    _marker: PhantomData<T>,
}
impl<T> Default for ENetList<T> {
    fn default() -> Self {
        Self {
            sentinel: Box::pin(ENetListNode {
                next: std::ptr::null_mut(),
                previous: std::ptr::null_mut(),
            }),
            _marker: PhantomData,
        }
    }
}

pub(crate) fn enet_list_clear<T>(list: &mut ENetList<T>) {
    list.sentinel.next = &mut *list.sentinel;
    list.sentinel.previous = &mut *list.sentinel;
}

pub(crate) unsafe fn enet_list_insert<T>(
    position: ENetListIterator<T>,
    data: *mut ENetListNode<T>,
) -> ENetListIterator<T> {
    let result: ENetListIterator<T> = data as ENetListIterator<T>;
    (*result).previous = (*position).previous;
    (*result).next = position;
    (*(*result).previous).next = result;
    (*position).previous = result;
    result
}

pub(crate) unsafe fn enet_list_remove<T>(position: ENetListIterator<T>) -> *mut T {
    (*(*position).previous).next = (*position).next;
    (*(*position).next).previous = (*position).previous;
    position as *mut c_void as *mut T
}

pub(crate) unsafe fn enet_list_move<T>(
    position: ENetListIterator<T>,
    data_first: *mut c_void,
    data_last: *mut c_void,
) -> ENetListIterator<T> {
    let first: ENetListIterator<T> = data_first as ENetListIterator<T>;
    let last: ENetListIterator<T> = data_last as ENetListIterator<T>;
    (*(*first).previous).next = (*last).next;
    (*(*last).next).previous = (*first).previous;
    (*first).previous = (*position).previous;
    (*last).next = position;
    (*(*first).previous).next = first;
    (*position).previous = last;
    first
}

pub(crate) unsafe fn enet_list_begin<T>(list: *mut ENetList<T>) -> ENetListIterator<T> {
    (*list).sentinel.next
}

pub(crate) unsafe fn enet_list_end<T>(list: *mut ENetList<T>) -> ENetListIterator<T> {
    &mut *(*list).sentinel
}

pub(crate) unsafe fn enet_list_empty<T>(list: *mut ENetList<T>) -> bool {
    enet_list_begin(list) == enet_list_end(list)
}

pub(crate) unsafe fn enet_list_next<T>(iterator: ENetListIterator<T>) -> ENetListIterator<T> {
    (*iterator).next
}

pub(crate) unsafe fn enet_list_previous<T>(iterator: ENetListIterator<T>) -> ENetListIterator<T> {
    (*iterator).previous
}

pub(crate) unsafe fn enet_list_front<T>(list: *mut ENetList<T>) -> *mut T {
    (*list).sentinel.next as *mut c_void as *mut T
}
