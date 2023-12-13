#[derive(Copy, Clone)]
#[repr(C)]
pub(crate) struct ENetListNode {
    pub(crate) next: *mut ENetListNode,
    pub(crate) previous: *mut ENetListNode,
}
pub(crate) type ENetListIterator = *mut ENetListNode;
#[derive(Copy, Clone)]
#[repr(C)]
pub(crate) struct ENetList {
    pub(crate) sentinel: ENetListNode,
}
pub(crate) unsafe fn enet_list_clear(list: *mut ENetList) {
    (*list).sentinel.next = &mut (*list).sentinel;
    (*list).sentinel.previous = &mut (*list).sentinel;
}
pub(crate) unsafe fn enet_list_insert(
    position: ENetListIterator,
    data: *mut u8,
) -> ENetListIterator {
    let result: ENetListIterator = data as ENetListIterator;
    (*result).previous = (*position).previous;
    (*result).next = position;
    (*(*result).previous).next = result;
    (*position).previous = result;
    result
}
pub(crate) unsafe fn enet_list_remove(position: ENetListIterator) -> *mut u8 {
    (*(*position).previous).next = (*position).next;
    (*(*position).next).previous = (*position).previous;
    position.cast()
}
pub(crate) unsafe fn enet_list_move(
    position: ENetListIterator,
    data_first: *mut u8,
    data_last: *mut u8,
) -> ENetListIterator {
    let first: ENetListIterator = data_first as ENetListIterator;
    let last: ENetListIterator = data_last as ENetListIterator;
    (*(*first).previous).next = (*last).next;
    (*(*last).next).previous = (*first).previous;
    (*first).previous = (*position).previous;
    (*last).next = position;
    (*(*first).previous).next = first;
    (*position).previous = last;
    first
}
