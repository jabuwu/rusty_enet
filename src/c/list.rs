use crate::os::c_void;

#[derive(Copy, Clone)]
#[repr(C)]
pub(crate) struct _ENetListNode {
    pub(crate) next: *mut _ENetListNode,
    pub(crate) previous: *mut _ENetListNode,
}
pub(crate) type ENetListNode = _ENetListNode;
pub(crate) type ENetListIterator = *mut ENetListNode;
#[derive(Copy, Clone)]
#[repr(C)]
pub(crate) struct _ENetList {
    pub(crate) sentinel: ENetListNode,
}
pub(crate) type ENetList = _ENetList;
pub(crate) unsafe fn enet_list_clear(list: *mut ENetList) {
    (*list).sentinel.next = &mut (*list).sentinel;
    (*list).sentinel.previous = &mut (*list).sentinel;
}
pub(crate) unsafe fn enet_list_insert(
    position: ENetListIterator,
    data: *mut c_void,
) -> ENetListIterator {
    let result: ENetListIterator = data as ENetListIterator;
    (*result).previous = (*position).previous;
    (*result).next = position;
    (*(*result).previous).next = result;
    (*position).previous = result;
    result
}
pub(crate) unsafe fn enet_list_remove(position: ENetListIterator) -> *mut c_void {
    (*(*position).previous).next = (*position).next;
    (*(*position).next).previous = (*position).previous;
    position as *mut c_void
}
pub(crate) unsafe fn enet_list_move(
    position: ENetListIterator,
    dataFirst: *mut c_void,
    dataLast: *mut c_void,
) -> ENetListIterator {
    let first: ENetListIterator = dataFirst as ENetListIterator;
    let last: ENetListIterator = dataLast as ENetListIterator;
    (*(*first).previous).next = (*last).next;
    (*(*last).next).previous = (*first).previous;
    (*first).previous = (*position).previous;
    (*last).next = position;
    (*(*first).previous).next = first;
    (*position).previous = last;
    first
}
