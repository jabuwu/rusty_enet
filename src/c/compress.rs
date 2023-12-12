use crate::{enet_free, enet_malloc, os::c_void, ENetBuffer};

pub(crate) type ENetRangeCoder = _ENetRangeCoder;
#[derive(Copy, Clone)]
#[repr(C)]
pub(crate) struct _ENetRangeCoder {
    pub(crate) symbols: [ENetSymbol; 4096],
}
pub(crate) type ENetSymbol = _ENetSymbol;
#[derive(Copy, Clone)]
#[repr(C)]
pub(crate) struct _ENetSymbol {
    pub(crate) value: u8,
    pub(crate) count: u8,
    pub(crate) under: u16,
    pub(crate) left: u16,
    pub(crate) right: u16,
    pub(crate) symbols: u16,
    pub(crate) escapes: u16,
    pub(crate) total: u16,
    pub(crate) parent: u16,
}
pub(crate) const ENET_CONTEXT_SYMBOL_MINIMUM: C2RustUnnamed_3 = 1;
pub(crate) const ENET_CONTEXT_ESCAPE_MINIMUM: C2RustUnnamed_3 = 1;
pub(crate) const ENET_SUBCONTEXT_ORDER: C2RustUnnamed_3 = 2;
pub(crate) const ENET_RANGE_CODER_BOTTOM: C2RustUnnamed_3 = 65536;
pub(crate) const ENET_SUBCONTEXT_SYMBOL_DELTA: C2RustUnnamed_3 = 2;
pub(crate) const ENET_SUBCONTEXT_ESCAPE_DELTA: C2RustUnnamed_3 = 5;
pub(crate) const ENET_CONTEXT_SYMBOL_DELTA: C2RustUnnamed_3 = 3;
pub(crate) const ENET_RANGE_CODER_TOP: C2RustUnnamed_3 = 16777216;
pub(crate) type C2RustUnnamed_3 = u32;
pub(crate) unsafe fn enet_range_coder_create() -> *mut c_void {
    let rangeCoder: *mut ENetRangeCoder =
        enet_malloc(::core::mem::size_of::<ENetRangeCoder>()) as *mut ENetRangeCoder;
    if rangeCoder.is_null() {
        return std::ptr::null_mut();
    }
    rangeCoder as *mut c_void
}
pub(crate) unsafe fn enet_range_coder_destroy(context: *mut c_void) {
    let rangeCoder: *mut ENetRangeCoder = context as *mut ENetRangeCoder;
    if rangeCoder.is_null() {
        return;
    }
    enet_free(rangeCoder as *mut c_void);
}
unsafe fn enet_symbol_rescale(mut symbol: *mut ENetSymbol) -> u16 {
    let mut total: u16 = 0_i32 as u16;
    loop {
        (*symbol).count = ((*symbol).count as i32 - ((*symbol).count as i32 >> 1_i32)) as u8;
        (*symbol).under = (*symbol).count as u16;
        if (*symbol).left != 0 {
            (*symbol).under = ((*symbol).under as i32
                + enet_symbol_rescale(symbol.offset((*symbol).left as i32 as isize)) as i32)
                as u16;
        }
        total = (total as i32 + (*symbol).under as i32) as u16;
        if (*symbol).right == 0 {
            break;
        }
        symbol = symbol.offset((*symbol).right as i32 as isize);
    }
    total
}
pub(crate) unsafe fn enet_range_coder_compress(
    context: *mut c_void,
    mut inBuffers: *const ENetBuffer,
    mut inBufferCount: usize,
    inLimit: usize,
    mut outData: *mut u8,
    outLimit: usize,
) -> usize {
    let rangeCoder: *mut ENetRangeCoder = context as *mut ENetRangeCoder;
    let outStart: *mut u8 = outData;
    let outEnd: *mut u8 = &mut *outData.add(outLimit) as *mut u8;
    let mut inData: *const u8;
    let mut inEnd: *const u8;
    let mut encodeLow: u32 = 0_i32 as u32;
    let mut encodeRange: u32 = !0_i32 as u32;
    let mut root: *mut ENetSymbol;
    let mut predicted: u16 = 0_i32 as u16;
    let mut order: usize = 0_i32 as usize;
    let mut nextSymbol: usize = 0_i32 as usize;
    if rangeCoder.is_null() || inBufferCount <= 0_i32 as usize || inLimit <= 0_i32 as usize {
        return 0_i32 as usize;
    }
    inData = (*inBuffers).data as *const u8;
    inEnd = &*inData.add((*inBuffers).dataLength) as *const u8;
    inBuffers = inBuffers.offset(1);
    inBufferCount = inBufferCount.wrapping_sub(1);
    let fresh0 = nextSymbol;
    nextSymbol = nextSymbol.wrapping_add(1);
    root = &mut *((*rangeCoder).symbols).as_mut_ptr().add(fresh0) as *mut ENetSymbol;
    (*root).value = 0_i32 as u8;
    (*root).count = 0_i32 as u8;
    (*root).under = 0_i32 as u16;
    (*root).left = 0_i32 as u16;
    (*root).right = 0_i32 as u16;
    (*root).symbols = 0_i32 as u16;
    (*root).escapes = 0_i32 as u16;
    (*root).total = 0_i32 as u16;
    (*root).parent = 0_i32 as u16;
    (*root).escapes = ENET_CONTEXT_ESCAPE_MINIMUM as i32 as u16;
    (*root).total =
        (ENET_CONTEXT_ESCAPE_MINIMUM as i32 + 256_i32 * ENET_CONTEXT_SYMBOL_MINIMUM as i32) as u16;
    (*root).symbols = 0_i32 as u16;
    let mut current_block_237: u64;
    loop {
        let mut subcontext: *mut ENetSymbol;
        let mut symbol: *mut ENetSymbol;
        let mut count: u16;
        let mut under: u16;
        let mut parent: *mut u16 = &mut predicted;
        let mut total: u16;
        if inData >= inEnd {
            if inBufferCount <= 0_i32 as usize {
                break;
            }
            inData = (*inBuffers).data as *const u8;
            inEnd = &*inData.add((*inBuffers).dataLength) as *const u8;
            inBuffers = inBuffers.offset(1);
            inBufferCount = inBufferCount.wrapping_sub(1);
        }
        let fresh1 = inData;
        inData = inData.offset(1);
        let value = *fresh1;
        subcontext = &mut *((*rangeCoder).symbols)
            .as_mut_ptr()
            .offset(predicted as isize) as *mut ENetSymbol;
        loop {
            if subcontext == root {
                current_block_237 = 2463987395154258233;
                break;
            }
            under = 0_u16;
            count = 0_i32 as u16;
            if (*subcontext).symbols == 0 {
                let fresh2 = nextSymbol;
                nextSymbol = nextSymbol.wrapping_add(1);
                symbol = &mut *((*rangeCoder).symbols).as_mut_ptr().add(fresh2) as *mut ENetSymbol;
                (*symbol).value = value;
                (*symbol).count = ENET_SUBCONTEXT_SYMBOL_DELTA as i32 as u8;
                (*symbol).under = ENET_SUBCONTEXT_SYMBOL_DELTA as i32 as u16;
                (*symbol).left = 0_i32 as u16;
                (*symbol).right = 0_i32 as u16;
                (*symbol).symbols = 0_i32 as u16;
                (*symbol).escapes = 0_i32 as u16;
                (*symbol).total = 0_i32 as u16;
                (*symbol).parent = 0_i32 as u16;
                (*subcontext).symbols = symbol.offset_from(subcontext) as i64 as u16;
            } else {
                let mut node: *mut ENetSymbol =
                    subcontext.offset((*subcontext).symbols as i32 as isize);
                loop {
                    if (value as i32) < (*node).value as i32 {
                        (*node).under =
                            ((*node).under as i32 + ENET_SUBCONTEXT_SYMBOL_DELTA as i32) as u16;
                        if (*node).left != 0 {
                            node = node.offset((*node).left as i32 as isize);
                        } else {
                            let fresh3 = nextSymbol;
                            nextSymbol = nextSymbol.wrapping_add(1);
                            symbol = &mut *((*rangeCoder).symbols).as_mut_ptr().add(fresh3)
                                as *mut ENetSymbol;
                            (*symbol).value = value;
                            (*symbol).count = ENET_SUBCONTEXT_SYMBOL_DELTA as i32 as u8;
                            (*symbol).under = ENET_SUBCONTEXT_SYMBOL_DELTA as i32 as u16;
                            (*symbol).left = 0_i32 as u16;
                            (*symbol).right = 0_i32 as u16;
                            (*symbol).symbols = 0_i32 as u16;
                            (*symbol).escapes = 0_i32 as u16;
                            (*symbol).total = 0_i32 as u16;
                            (*symbol).parent = 0_i32 as u16;
                            (*node).left = symbol.offset_from(node) as i64 as u16;
                            break;
                        }
                    } else if value as i32 > (*node).value as i32 {
                        under = (under as i32 + (*node).under as i32) as u16;
                        if (*node).right != 0 {
                            node = node.offset((*node).right as i32 as isize);
                        } else {
                            let fresh4 = nextSymbol;
                            nextSymbol = nextSymbol.wrapping_add(1);
                            symbol = &mut *((*rangeCoder).symbols).as_mut_ptr().add(fresh4)
                                as *mut ENetSymbol;
                            (*symbol).value = value;
                            (*symbol).count = ENET_SUBCONTEXT_SYMBOL_DELTA as i32 as u8;
                            (*symbol).under = ENET_SUBCONTEXT_SYMBOL_DELTA as i32 as u16;
                            (*symbol).left = 0_i32 as u16;
                            (*symbol).right = 0_i32 as u16;
                            (*symbol).symbols = 0_i32 as u16;
                            (*symbol).escapes = 0_i32 as u16;
                            (*symbol).total = 0_i32 as u16;
                            (*symbol).parent = 0_i32 as u16;
                            (*node).right = symbol.offset_from(node) as i64 as u16;
                            break;
                        }
                    } else {
                        count = (count as i32 + (*node).count as i32) as u16;
                        under =
                            (under as i32 + ((*node).under as i32 - (*node).count as i32)) as u16;
                        (*node).under =
                            ((*node).under as i32 + ENET_SUBCONTEXT_SYMBOL_DELTA as i32) as u16;
                        (*node).count =
                            ((*node).count as i32 + ENET_SUBCONTEXT_SYMBOL_DELTA as i32) as u8;
                        symbol = node;
                        break;
                    }
                }
            }
            *parent = symbol.offset_from(((*rangeCoder).symbols).as_mut_ptr()) as i64 as u16;
            parent = &mut (*symbol).parent;
            total = (*subcontext).total;
            if count as i32 > 0_i32 {
                encodeRange = encodeRange.wrapping_div(total as u32);
                encodeLow = encodeLow.wrapping_add(
                    (((*subcontext).escapes as i32 + under as i32) as u32)
                        .wrapping_mul(encodeRange),
                );
                encodeRange = encodeRange.wrapping_mul(count as u32);
                loop {
                    if encodeLow ^ encodeLow.wrapping_add(encodeRange)
                        >= ENET_RANGE_CODER_TOP as i32 as u32
                    {
                        if encodeRange >= ENET_RANGE_CODER_BOTTOM as i32 as u32 {
                            break;
                        }
                        encodeRange = encodeLow.wrapping_neg()
                            & (ENET_RANGE_CODER_BOTTOM as i32 - 1_i32) as u32;
                    }
                    if outData >= outEnd {
                        return 0_i32 as usize;
                    }
                    let fresh5 = outData;
                    outData = outData.offset(1);
                    *fresh5 = (encodeLow >> 24_i32) as u8;
                    encodeRange <<= 8_i32;
                    encodeLow <<= 8_i32;
                }
            } else {
                if (*subcontext).escapes as i32 > 0_i32
                    && ((*subcontext).escapes as i32) < total as i32
                {
                    encodeRange = encodeRange.wrapping_div(total as u32);
                    encodeLow = encodeLow.wrapping_add((0_i32 as u32).wrapping_mul(encodeRange));
                    encodeRange = encodeRange.wrapping_mul((*subcontext).escapes as u32);
                    loop {
                        if encodeLow ^ encodeLow.wrapping_add(encodeRange)
                            >= ENET_RANGE_CODER_TOP as i32 as u32
                        {
                            if encodeRange >= ENET_RANGE_CODER_BOTTOM as i32 as u32 {
                                break;
                            }
                            encodeRange = encodeLow.wrapping_neg()
                                & (ENET_RANGE_CODER_BOTTOM as i32 - 1_i32) as u32;
                        }
                        if outData >= outEnd {
                            return 0_i32 as usize;
                        }
                        let fresh6 = outData;
                        outData = outData.offset(1);
                        *fresh6 = (encodeLow >> 24_i32) as u8;
                        encodeRange <<= 8_i32;
                        encodeLow <<= 8_i32;
                    }
                }
                (*subcontext).escapes =
                    ((*subcontext).escapes as i32 + ENET_SUBCONTEXT_ESCAPE_DELTA as i32) as u16;
                (*subcontext).total =
                    ((*subcontext).total as i32 + ENET_SUBCONTEXT_ESCAPE_DELTA as i32) as u16;
            }
            (*subcontext).total =
                ((*subcontext).total as i32 + ENET_SUBCONTEXT_SYMBOL_DELTA as i32) as u16;
            if count as i32 > 0xff_i32 - 2_i32 * ENET_SUBCONTEXT_SYMBOL_DELTA as i32
                || (*subcontext).total as i32 > ENET_RANGE_CODER_BOTTOM as i32 - 0x100_i32
            {
                (*subcontext).total = (if (*subcontext).symbols as i32 != 0 {
                    enet_symbol_rescale(subcontext.offset((*subcontext).symbols as i32 as isize))
                        as i32
                } else {
                    0_i32
                }) as u16;
                (*subcontext).escapes =
                    ((*subcontext).escapes as i32 - ((*subcontext).escapes as i32 >> 1_i32)) as u16;
                (*subcontext).total =
                    ((*subcontext).total as i32 + (*subcontext).escapes as i32) as u16;
            }
            if count as i32 > 0_i32 {
                current_block_237 = 836937598693885467;
                break;
            }
            subcontext = &mut *((*rangeCoder).symbols)
                .as_mut_ptr()
                .offset((*subcontext).parent as isize) as *mut ENetSymbol;
        }
        if let 2463987395154258233 = current_block_237 {
            under = (value as i32 * ENET_CONTEXT_SYMBOL_MINIMUM as i32) as u16;
            count = ENET_CONTEXT_SYMBOL_MINIMUM as i32 as u16;
            if (*root).symbols == 0 {
                let fresh7 = nextSymbol;
                nextSymbol = nextSymbol.wrapping_add(1);
                symbol = &mut *((*rangeCoder).symbols).as_mut_ptr().add(fresh7) as *mut ENetSymbol;
                (*symbol).value = value;
                (*symbol).count = ENET_CONTEXT_SYMBOL_DELTA as i32 as u8;
                (*symbol).under = ENET_CONTEXT_SYMBOL_DELTA as i32 as u16;
                (*symbol).left = 0_i32 as u16;
                (*symbol).right = 0_i32 as u16;
                (*symbol).symbols = 0_i32 as u16;
                (*symbol).escapes = 0_i32 as u16;
                (*symbol).total = 0_i32 as u16;
                (*symbol).parent = 0_i32 as u16;
                (*root).symbols = symbol.offset_from(root) as i64 as u16;
            } else {
                let mut node_0: *mut ENetSymbol = root.offset((*root).symbols as i32 as isize);
                loop {
                    if (value as i32) < (*node_0).value as i32 {
                        (*node_0).under =
                            ((*node_0).under as i32 + ENET_CONTEXT_SYMBOL_DELTA as i32) as u16;
                        if (*node_0).left != 0 {
                            node_0 = node_0.offset((*node_0).left as i32 as isize);
                        } else {
                            let fresh8 = nextSymbol;
                            nextSymbol = nextSymbol.wrapping_add(1);
                            symbol = &mut *((*rangeCoder).symbols).as_mut_ptr().add(fresh8)
                                as *mut ENetSymbol;
                            (*symbol).value = value;
                            (*symbol).count = ENET_CONTEXT_SYMBOL_DELTA as i32 as u8;
                            (*symbol).under = ENET_CONTEXT_SYMBOL_DELTA as i32 as u16;
                            (*symbol).left = 0_i32 as u16;
                            (*symbol).right = 0_i32 as u16;
                            (*symbol).symbols = 0_i32 as u16;
                            (*symbol).escapes = 0_i32 as u16;
                            (*symbol).total = 0_i32 as u16;
                            (*symbol).parent = 0_i32 as u16;
                            (*node_0).left = symbol.offset_from(node_0) as i64 as u16;
                            break;
                        }
                    } else if value as i32 > (*node_0).value as i32 {
                        under = (under as i32 + (*node_0).under as i32) as u16;
                        if (*node_0).right != 0 {
                            node_0 = node_0.offset((*node_0).right as i32 as isize);
                        } else {
                            let fresh9 = nextSymbol;
                            nextSymbol = nextSymbol.wrapping_add(1);
                            symbol = &mut *((*rangeCoder).symbols).as_mut_ptr().add(fresh9)
                                as *mut ENetSymbol;
                            (*symbol).value = value;
                            (*symbol).count = ENET_CONTEXT_SYMBOL_DELTA as i32 as u8;
                            (*symbol).under = ENET_CONTEXT_SYMBOL_DELTA as i32 as u16;
                            (*symbol).left = 0_i32 as u16;
                            (*symbol).right = 0_i32 as u16;
                            (*symbol).symbols = 0_i32 as u16;
                            (*symbol).escapes = 0_i32 as u16;
                            (*symbol).total = 0_i32 as u16;
                            (*symbol).parent = 0_i32 as u16;
                            (*node_0).right = symbol.offset_from(node_0) as i64 as u16;
                            break;
                        }
                    } else {
                        count = (count as i32 + (*node_0).count as i32) as u16;
                        under = (under as i32 + ((*node_0).under as i32 - (*node_0).count as i32))
                            as u16;
                        (*node_0).under =
                            ((*node_0).under as i32 + ENET_CONTEXT_SYMBOL_DELTA as i32) as u16;
                        (*node_0).count =
                            ((*node_0).count as i32 + ENET_CONTEXT_SYMBOL_DELTA as i32) as u8;
                        symbol = node_0;
                        break;
                    }
                }
            }
            *parent = symbol.offset_from(((*rangeCoder).symbols).as_mut_ptr()) as i64 as u16;
            total = (*root).total;
            encodeRange = encodeRange.wrapping_div(total as u32);
            encodeLow = encodeLow.wrapping_add(
                (((*root).escapes as i32 + under as i32) as u32).wrapping_mul(encodeRange),
            );
            encodeRange = encodeRange.wrapping_mul(count as u32);
            loop {
                if encodeLow ^ encodeLow.wrapping_add(encodeRange)
                    >= ENET_RANGE_CODER_TOP as i32 as u32
                {
                    if encodeRange >= ENET_RANGE_CODER_BOTTOM as i32 as u32 {
                        break;
                    }
                    encodeRange =
                        encodeLow.wrapping_neg() & (ENET_RANGE_CODER_BOTTOM as i32 - 1_i32) as u32;
                }
                if outData >= outEnd {
                    return 0_i32 as usize;
                }
                let fresh10 = outData;
                outData = outData.offset(1);
                *fresh10 = (encodeLow >> 24_i32) as u8;
                encodeRange <<= 8_i32;
                encodeLow <<= 8_i32;
            }
            (*root).total = ((*root).total as i32 + ENET_CONTEXT_SYMBOL_DELTA as i32) as u16;
            if count as i32
                > 0xff_i32 - 2_i32 * ENET_CONTEXT_SYMBOL_DELTA as i32
                    + ENET_CONTEXT_SYMBOL_MINIMUM as i32
                || (*root).total as i32 > ENET_RANGE_CODER_BOTTOM as i32 - 0x100_i32
            {
                (*root).total = (if (*root).symbols as i32 != 0 {
                    enet_symbol_rescale(root.offset((*root).symbols as i32 as isize)) as i32
                } else {
                    0_i32
                }) as u16;
                (*root).escapes =
                    ((*root).escapes as i32 - ((*root).escapes as i32 >> 1_i32)) as u16;
                (*root).total = ((*root).total as i32
                    + ((*root).escapes as i32 + 256_i32 * ENET_CONTEXT_SYMBOL_MINIMUM as i32))
                    as u16;
            }
        }
        if order >= ENET_SUBCONTEXT_ORDER as i32 as usize {
            predicted = (*rangeCoder).symbols[predicted as usize].parent;
        } else {
            order = order.wrapping_add(1);
        }
        if nextSymbol
            >= ::core::mem::size_of::<[ENetSymbol; 4096]>()
                .wrapping_div(::core::mem::size_of::<ENetSymbol>())
                .wrapping_sub(ENET_SUBCONTEXT_ORDER as i32 as usize)
        {
            nextSymbol = 0_i32 as usize;
            let fresh11 = nextSymbol;
            nextSymbol = nextSymbol.wrapping_add(1);
            root = &mut *((*rangeCoder).symbols).as_mut_ptr().add(fresh11) as *mut ENetSymbol;
            (*root).value = 0_i32 as u8;
            (*root).count = 0_i32 as u8;
            (*root).under = 0_i32 as u16;
            (*root).left = 0_i32 as u16;
            (*root).right = 0_i32 as u16;
            (*root).symbols = 0_i32 as u16;
            (*root).escapes = 0_i32 as u16;
            (*root).total = 0_i32 as u16;
            (*root).parent = 0_i32 as u16;
            (*root).escapes = ENET_CONTEXT_ESCAPE_MINIMUM as i32 as u16;
            (*root).total = (ENET_CONTEXT_ESCAPE_MINIMUM as i32
                + 256_i32 * ENET_CONTEXT_SYMBOL_MINIMUM as i32) as u16;
            (*root).symbols = 0_i32 as u16;
            predicted = 0_i32 as u16;
            order = 0_i32 as usize;
        }
    }
    while encodeLow != 0 {
        if outData >= outEnd {
            return 0_i32 as usize;
        }
        let fresh12 = outData;
        outData = outData.offset(1);
        *fresh12 = (encodeLow >> 24_i32) as u8;
        encodeLow <<= 8_i32;
    }
    outData.offset_from(outStart) as i64 as usize
}
pub(crate) unsafe fn enet_range_coder_decompress(
    context: *mut c_void,
    mut inData: *const u8,
    inLimit: usize,
    mut outData: *mut u8,
    outLimit: usize,
) -> usize {
    let rangeCoder: *mut ENetRangeCoder = context as *mut ENetRangeCoder;
    let outStart: *mut u8 = outData;
    let outEnd: *mut u8 = &mut *outData.add(outLimit) as *mut u8;
    let inEnd: *const u8 = &*inData.add(inLimit) as *const u8;
    let mut decodeLow: u32 = 0_i32 as u32;
    let mut decodeCode: u32 = 0_i32 as u32;
    let mut decodeRange: u32 = !0_i32 as u32;
    let mut root: *mut ENetSymbol;
    let mut predicted: u16 = 0_i32 as u16;
    let mut order: usize = 0_i32 as usize;
    let mut nextSymbol: usize = 0_i32 as usize;
    if rangeCoder.is_null() || inLimit <= 0_i32 as usize {
        return 0_i32 as usize;
    }
    let fresh13 = nextSymbol;
    nextSymbol = nextSymbol.wrapping_add(1);
    root = &mut *((*rangeCoder).symbols).as_mut_ptr().add(fresh13) as *mut ENetSymbol;
    (*root).value = 0_i32 as u8;
    (*root).count = 0_i32 as u8;
    (*root).under = 0_i32 as u16;
    (*root).left = 0_i32 as u16;
    (*root).right = 0_i32 as u16;
    (*root).symbols = 0_i32 as u16;
    (*root).escapes = 0_i32 as u16;
    (*root).total = 0_i32 as u16;
    (*root).parent = 0_i32 as u16;
    (*root).escapes = ENET_CONTEXT_ESCAPE_MINIMUM as i32 as u16;
    (*root).total =
        (ENET_CONTEXT_ESCAPE_MINIMUM as i32 + 256_i32 * ENET_CONTEXT_SYMBOL_MINIMUM as i32) as u16;
    (*root).symbols = 0_i32 as u16;
    if inData < inEnd {
        let fresh14 = inData;
        inData = inData.offset(1);
        decodeCode |= ((*fresh14 as i32) << 24_i32) as u32;
    }
    if inData < inEnd {
        let fresh15 = inData;
        inData = inData.offset(1);
        decodeCode |= ((*fresh15 as i32) << 16_i32) as u32;
    }
    if inData < inEnd {
        let fresh16 = inData;
        inData = inData.offset(1);
        decodeCode |= ((*fresh16 as i32) << 8_i32) as u32;
    }
    if inData < inEnd {
        let fresh17 = inData;
        inData = inData.offset(1);
        decodeCode |= *fresh17 as u32;
    }
    let mut current_block_297: u64;
    loop {
        let mut subcontext: *mut ENetSymbol;
        let mut symbol: *mut ENetSymbol;
        let mut patch: *mut ENetSymbol;
        let mut value: u8 = 0_i32 as u8;
        let mut code: u16;
        let mut under: u16;
        let mut count: u16;
        let mut bottom: u16 = 0;
        let mut parent: *mut u16 = &mut predicted;
        let mut total: u16;
        subcontext = &mut *((*rangeCoder).symbols)
            .as_mut_ptr()
            .offset(predicted as isize) as *mut ENetSymbol;
        loop {
            if subcontext == root {
                current_block_297 = 18325745679564279244;
                break;
            }
            if (*subcontext).escapes as i32 > 0_i32 {
                total = (*subcontext).total;
                if ((*subcontext).escapes as i32) < total as i32 {
                    decodeRange = decodeRange.wrapping_div(total as u32);
                    code = decodeCode.wrapping_sub(decodeLow).wrapping_div(decodeRange) as u16;
                    if (code as i32) < (*subcontext).escapes as i32 {
                        decodeLow =
                            decodeLow.wrapping_add((0_i32 as u32).wrapping_mul(decodeRange));
                        decodeRange = decodeRange.wrapping_mul((*subcontext).escapes as u32);
                        loop {
                            if decodeLow ^ decodeLow.wrapping_add(decodeRange)
                                >= ENET_RANGE_CODER_TOP as i32 as u32
                            {
                                if decodeRange >= ENET_RANGE_CODER_BOTTOM as i32 as u32 {
                                    break;
                                }
                                decodeRange = decodeLow.wrapping_neg()
                                    & (ENET_RANGE_CODER_BOTTOM as i32 - 1_i32) as u32;
                            }
                            decodeCode <<= 8_i32;
                            if inData < inEnd {
                                let fresh18 = inData;
                                inData = inData.offset(1);
                                decodeCode |= *fresh18 as u32;
                            }
                            decodeRange <<= 8_i32;
                            decodeLow <<= 8_i32;
                        }
                    } else {
                        code = (code as i32 - (*subcontext).escapes as i32) as u16;
                        under = 0_i32 as u16;
                        count = 0_i32 as u16;
                        if (*subcontext).symbols == 0 {
                            return 0_i32 as usize;
                        } else {
                            let mut node: *mut ENetSymbol =
                                subcontext.offset((*subcontext).symbols as i32 as isize);
                            loop {
                                let after: u16 = (under as i32 + (*node).under as i32) as u16;
                                let before: u16 = (*node).count as i32 as u16;
                                if code as i32 >= after as i32 {
                                    under = (under as i32 + (*node).under as i32) as u16;
                                    if (*node).right != 0 {
                                        node = node.offset((*node).right as i32 as isize);
                                    } else {
                                        return 0_i32 as usize;
                                    }
                                } else if (code as i32) < after as i32 - before as i32 {
                                    (*node).under = ((*node).under as i32
                                        + ENET_SUBCONTEXT_SYMBOL_DELTA as i32)
                                        as u16;
                                    if (*node).left != 0 {
                                        node = node.offset((*node).left as i32 as isize);
                                    } else {
                                        return 0_i32 as usize;
                                    }
                                } else {
                                    value = (*node).value;
                                    count = (count as i32 + (*node).count as i32) as u16;
                                    under = (after as i32 - before as i32) as u16;
                                    (*node).under = ((*node).under as i32
                                        + ENET_SUBCONTEXT_SYMBOL_DELTA as i32)
                                        as u16;
                                    (*node).count = ((*node).count as i32
                                        + ENET_SUBCONTEXT_SYMBOL_DELTA as i32)
                                        as u8;
                                    symbol = node;
                                    break;
                                }
                            }
                        }
                        bottom =
                            symbol.offset_from(((*rangeCoder).symbols).as_mut_ptr()) as i64 as u16;
                        decodeLow = decodeLow.wrapping_add(
                            (((*subcontext).escapes as i32 + under as i32) as u32)
                                .wrapping_mul(decodeRange),
                        );
                        decodeRange = decodeRange.wrapping_mul(count as u32);
                        loop {
                            if decodeLow ^ decodeLow.wrapping_add(decodeRange)
                                >= ENET_RANGE_CODER_TOP as i32 as u32
                            {
                                if decodeRange >= ENET_RANGE_CODER_BOTTOM as i32 as u32 {
                                    break;
                                }
                                decodeRange = decodeLow.wrapping_neg()
                                    & (ENET_RANGE_CODER_BOTTOM as i32 - 1_i32) as u32;
                            }
                            decodeCode <<= 8_i32;
                            if inData < inEnd {
                                let fresh19 = inData;
                                inData = inData.offset(1);
                                decodeCode |= *fresh19 as u32;
                            }
                            decodeRange <<= 8_i32;
                            decodeLow <<= 8_i32;
                        }
                        (*subcontext).total = ((*subcontext).total as i32
                            + ENET_SUBCONTEXT_SYMBOL_DELTA as i32)
                            as u16;
                        if count as i32 > 0xff_i32 - 2_i32 * ENET_SUBCONTEXT_SYMBOL_DELTA as i32
                            || (*subcontext).total as i32
                                > ENET_RANGE_CODER_BOTTOM as i32 - 0x100_i32
                        {
                            (*subcontext).total = (if (*subcontext).symbols as i32 != 0 {
                                enet_symbol_rescale(
                                    subcontext.offset((*subcontext).symbols as i32 as isize),
                                ) as i32
                            } else {
                                0_i32
                            }) as u16;
                            (*subcontext).escapes = ((*subcontext).escapes as i32
                                - ((*subcontext).escapes as i32 >> 1_i32))
                                as u16;
                            (*subcontext).total =
                                ((*subcontext).total as i32 + (*subcontext).escapes as i32) as u16;
                        }
                        current_block_297 = 16234561804784670422;
                        break;
                    }
                }
            }
            subcontext = &mut *((*rangeCoder).symbols)
                .as_mut_ptr()
                .offset((*subcontext).parent as isize) as *mut ENetSymbol;
        }
        if let 18325745679564279244 = current_block_297 {
            total = (*root).total;
            decodeRange = decodeRange.wrapping_div(total as u32);
            code = decodeCode.wrapping_sub(decodeLow).wrapping_div(decodeRange) as u16;
            if (code as i32) < (*root).escapes as i32 {
                decodeLow = decodeLow.wrapping_add((0_i32 as u32).wrapping_mul(decodeRange));
                decodeRange = decodeRange.wrapping_mul((*root).escapes as u32);
                loop {
                    if decodeLow ^ decodeLow.wrapping_add(decodeRange)
                        >= ENET_RANGE_CODER_TOP as i32 as u32
                    {
                        if decodeRange >= ENET_RANGE_CODER_BOTTOM as i32 as u32 {
                            break;
                        }
                        decodeRange = decodeLow.wrapping_neg()
                            & (ENET_RANGE_CODER_BOTTOM as i32 - 1_i32) as u32;
                    }
                    decodeCode <<= 8_i32;
                    if inData < inEnd {
                        let fresh20 = inData;
                        inData = inData.offset(1);
                        decodeCode |= *fresh20 as u32;
                    }
                    decodeRange <<= 8_i32;
                    decodeLow <<= 8_i32;
                }
                break;
            } else {
                code = (code as i32 - (*root).escapes as i32) as u16;
                under = 0_i32 as u16;
                count = ENET_CONTEXT_SYMBOL_MINIMUM as i32 as u16;
                if (*root).symbols == 0 {
                    value = (code as i32 / ENET_CONTEXT_SYMBOL_MINIMUM as i32) as u8;
                    under = (code as i32 - code as i32 % ENET_CONTEXT_SYMBOL_MINIMUM as i32) as u16;
                    let fresh21 = nextSymbol;
                    nextSymbol = nextSymbol.wrapping_add(1);
                    symbol =
                        &mut *((*rangeCoder).symbols).as_mut_ptr().add(fresh21) as *mut ENetSymbol;
                    (*symbol).value = value;
                    (*symbol).count = ENET_CONTEXT_SYMBOL_DELTA as i32 as u8;
                    (*symbol).under = ENET_CONTEXT_SYMBOL_DELTA as i32 as u16;
                    (*symbol).left = 0_i32 as u16;
                    (*symbol).right = 0_i32 as u16;
                    (*symbol).symbols = 0_i32 as u16;
                    (*symbol).escapes = 0_i32 as u16;
                    (*symbol).total = 0_i32 as u16;
                    (*symbol).parent = 0_i32 as u16;
                    (*root).symbols = symbol.offset_from(root) as i64 as u16;
                } else {
                    let mut node_0: *mut ENetSymbol = root.offset((*root).symbols as i32 as isize);
                    loop {
                        let after_0: u16 = (under as i32
                            + (*node_0).under as i32
                            + ((*node_0).value as i32 + 1_i32) * ENET_CONTEXT_SYMBOL_MINIMUM as i32)
                            as u16;
                        let before_0: u16 =
                            ((*node_0).count as i32 + ENET_CONTEXT_SYMBOL_MINIMUM as i32) as u16;
                        if code as i32 >= after_0 as i32 {
                            under = (under as i32 + (*node_0).under as i32) as u16;
                            if (*node_0).right != 0 {
                                node_0 = node_0.offset((*node_0).right as i32 as isize);
                            } else {
                                value = ((*node_0).value as i32
                                    + 1_i32
                                    + (code as i32 - after_0 as i32)
                                        / ENET_CONTEXT_SYMBOL_MINIMUM as i32)
                                    as u8;
                                under = (code as i32
                                    - (code as i32 - after_0 as i32)
                                        % ENET_CONTEXT_SYMBOL_MINIMUM as i32)
                                    as u16;
                                let fresh22 = nextSymbol;
                                nextSymbol = nextSymbol.wrapping_add(1);
                                symbol = &mut *((*rangeCoder).symbols).as_mut_ptr().add(fresh22)
                                    as *mut ENetSymbol;
                                (*symbol).value = value;
                                (*symbol).count = ENET_CONTEXT_SYMBOL_DELTA as i32 as u8;
                                (*symbol).under = ENET_CONTEXT_SYMBOL_DELTA as i32 as u16;
                                (*symbol).left = 0_i32 as u16;
                                (*symbol).right = 0_i32 as u16;
                                (*symbol).symbols = 0_i32 as u16;
                                (*symbol).escapes = 0_i32 as u16;
                                (*symbol).total = 0_i32 as u16;
                                (*symbol).parent = 0_i32 as u16;
                                (*node_0).right = symbol.offset_from(node_0) as i64 as u16;
                                break;
                            }
                        } else if (code as i32) < after_0 as i32 - before_0 as i32 {
                            (*node_0).under =
                                ((*node_0).under as i32 + ENET_CONTEXT_SYMBOL_DELTA as i32) as u16;
                            if (*node_0).left != 0 {
                                node_0 = node_0.offset((*node_0).left as i32 as isize);
                            } else {
                                value = ((*node_0).value as i32
                                    - 1_i32
                                    - (after_0 as i32 - before_0 as i32 - code as i32 - 1_i32)
                                        / ENET_CONTEXT_SYMBOL_MINIMUM as i32)
                                    as u8;
                                under = (code as i32
                                    - (after_0 as i32 - before_0 as i32 - code as i32 - 1_i32)
                                        % ENET_CONTEXT_SYMBOL_MINIMUM as i32)
                                    as u16;
                                let fresh23 = nextSymbol;
                                nextSymbol = nextSymbol.wrapping_add(1);
                                symbol = &mut *((*rangeCoder).symbols).as_mut_ptr().add(fresh23)
                                    as *mut ENetSymbol;
                                (*symbol).value = value;
                                (*symbol).count = ENET_CONTEXT_SYMBOL_DELTA as i32 as u8;
                                (*symbol).under = ENET_CONTEXT_SYMBOL_DELTA as i32 as u16;
                                (*symbol).left = 0_i32 as u16;
                                (*symbol).right = 0_i32 as u16;
                                (*symbol).symbols = 0_i32 as u16;
                                (*symbol).escapes = 0_i32 as u16;
                                (*symbol).total = 0_i32 as u16;
                                (*symbol).parent = 0_i32 as u16;
                                (*node_0).left = symbol.offset_from(node_0) as i64 as u16;
                                break;
                            }
                        } else {
                            value = (*node_0).value;
                            count = (count as i32 + (*node_0).count as i32) as u16;
                            under = (after_0 as i32 - before_0 as i32) as u16;
                            (*node_0).under =
                                ((*node_0).under as i32 + ENET_CONTEXT_SYMBOL_DELTA as i32) as u16;
                            (*node_0).count =
                                ((*node_0).count as i32 + ENET_CONTEXT_SYMBOL_DELTA as i32) as u8;
                            symbol = node_0;
                            break;
                        }
                    }
                }
                bottom = symbol.offset_from(((*rangeCoder).symbols).as_mut_ptr()) as i64 as u16;
                decodeLow = decodeLow.wrapping_add(
                    (((*root).escapes as i32 + under as i32) as u32).wrapping_mul(decodeRange),
                );
                decodeRange = decodeRange.wrapping_mul(count as u32);
                loop {
                    if decodeLow ^ decodeLow.wrapping_add(decodeRange)
                        >= ENET_RANGE_CODER_TOP as i32 as u32
                    {
                        if decodeRange >= ENET_RANGE_CODER_BOTTOM as i32 as u32 {
                            break;
                        }
                        decodeRange = decodeLow.wrapping_neg()
                            & (ENET_RANGE_CODER_BOTTOM as i32 - 1_i32) as u32;
                    }
                    decodeCode <<= 8_i32;
                    if inData < inEnd {
                        let fresh24 = inData;
                        inData = inData.offset(1);
                        decodeCode |= *fresh24 as u32;
                    }
                    decodeRange <<= 8_i32;
                    decodeLow <<= 8_i32;
                }
                (*root).total = ((*root).total as i32 + ENET_CONTEXT_SYMBOL_DELTA as i32) as u16;
                if count as i32
                    > 0xff_i32 - 2_i32 * ENET_CONTEXT_SYMBOL_DELTA as i32
                        + ENET_CONTEXT_SYMBOL_MINIMUM as i32
                    || (*root).total as i32 > ENET_RANGE_CODER_BOTTOM as i32 - 0x100_i32
                {
                    (*root).total = (if (*root).symbols as i32 != 0 {
                        enet_symbol_rescale(root.offset((*root).symbols as i32 as isize)) as i32
                    } else {
                        0_i32
                    }) as u16;
                    (*root).escapes =
                        ((*root).escapes as i32 - ((*root).escapes as i32 >> 1_i32)) as u16;
                    (*root).total = ((*root).total as i32
                        + ((*root).escapes as i32 + 256_i32 * ENET_CONTEXT_SYMBOL_MINIMUM as i32))
                        as u16;
                }
            }
        }
        patch = &mut *((*rangeCoder).symbols)
            .as_mut_ptr()
            .offset(predicted as isize) as *mut ENetSymbol;
        while patch != subcontext {
            under = 0_u16;
            count = 0_i32 as u16;
            if (*patch).symbols == 0 {
                let fresh25 = nextSymbol;
                nextSymbol = nextSymbol.wrapping_add(1);
                symbol = &mut *((*rangeCoder).symbols).as_mut_ptr().add(fresh25) as *mut ENetSymbol;
                (*symbol).value = value;
                (*symbol).count = ENET_SUBCONTEXT_SYMBOL_DELTA as i32 as u8;
                (*symbol).under = ENET_SUBCONTEXT_SYMBOL_DELTA as i32 as u16;
                (*symbol).left = 0_i32 as u16;
                (*symbol).right = 0_i32 as u16;
                (*symbol).symbols = 0_i32 as u16;
                (*symbol).escapes = 0_i32 as u16;
                (*symbol).total = 0_i32 as u16;
                (*symbol).parent = 0_i32 as u16;
                (*patch).symbols = symbol.offset_from(patch) as i64 as u16;
            } else {
                let mut node_1: *mut ENetSymbol = patch.offset((*patch).symbols as i32 as isize);
                loop {
                    if (value as i32) < (*node_1).value as i32 {
                        (*node_1).under =
                            ((*node_1).under as i32 + ENET_SUBCONTEXT_SYMBOL_DELTA as i32) as u16;
                        if (*node_1).left != 0 {
                            node_1 = node_1.offset((*node_1).left as i32 as isize);
                        } else {
                            let fresh26 = nextSymbol;
                            nextSymbol = nextSymbol.wrapping_add(1);
                            symbol = &mut *((*rangeCoder).symbols).as_mut_ptr().add(fresh26)
                                as *mut ENetSymbol;
                            (*symbol).value = value;
                            (*symbol).count = ENET_SUBCONTEXT_SYMBOL_DELTA as i32 as u8;
                            (*symbol).under = ENET_SUBCONTEXT_SYMBOL_DELTA as i32 as u16;
                            (*symbol).left = 0_i32 as u16;
                            (*symbol).right = 0_i32 as u16;
                            (*symbol).symbols = 0_i32 as u16;
                            (*symbol).escapes = 0_i32 as u16;
                            (*symbol).total = 0_i32 as u16;
                            (*symbol).parent = 0_i32 as u16;
                            (*node_1).left = symbol.offset_from(node_1) as i64 as u16;
                            break;
                        }
                    } else if value as i32 > (*node_1).value as i32 {
                        under = (under as i32 + (*node_1).under as i32) as u16;
                        if (*node_1).right != 0 {
                            node_1 = node_1.offset((*node_1).right as i32 as isize);
                        } else {
                            let fresh27 = nextSymbol;
                            nextSymbol = nextSymbol.wrapping_add(1);
                            symbol = &mut *((*rangeCoder).symbols).as_mut_ptr().add(fresh27)
                                as *mut ENetSymbol;
                            (*symbol).value = value;
                            (*symbol).count = ENET_SUBCONTEXT_SYMBOL_DELTA as i32 as u8;
                            (*symbol).under = ENET_SUBCONTEXT_SYMBOL_DELTA as i32 as u16;
                            (*symbol).left = 0_i32 as u16;
                            (*symbol).right = 0_i32 as u16;
                            (*symbol).symbols = 0_i32 as u16;
                            (*symbol).escapes = 0_i32 as u16;
                            (*symbol).total = 0_i32 as u16;
                            (*symbol).parent = 0_i32 as u16;
                            (*node_1).right = symbol.offset_from(node_1) as i64 as u16;
                            break;
                        }
                    } else {
                        count = (count as i32 + (*node_1).count as i32) as u16;
                        (*node_1).under =
                            ((*node_1).under as i32 + ENET_SUBCONTEXT_SYMBOL_DELTA as i32) as u16;
                        (*node_1).count =
                            ((*node_1).count as i32 + ENET_SUBCONTEXT_SYMBOL_DELTA as i32) as u8;
                        symbol = node_1;
                        break;
                    }
                }
            }
            *parent = symbol.offset_from(((*rangeCoder).symbols).as_mut_ptr()) as i64 as u16;
            parent = &mut (*symbol).parent;
            if count as i32 <= 0_i32 {
                (*patch).escapes =
                    ((*patch).escapes as i32 + ENET_SUBCONTEXT_ESCAPE_DELTA as i32) as u16;
                (*patch).total =
                    ((*patch).total as i32 + ENET_SUBCONTEXT_ESCAPE_DELTA as i32) as u16;
            }
            (*patch).total = ((*patch).total as i32 + ENET_SUBCONTEXT_SYMBOL_DELTA as i32) as u16;
            if count as i32 > 0xff_i32 - 2_i32 * ENET_SUBCONTEXT_SYMBOL_DELTA as i32
                || (*patch).total as i32 > ENET_RANGE_CODER_BOTTOM as i32 - 0x100_i32
            {
                (*patch).total = (if (*patch).symbols as i32 != 0 {
                    enet_symbol_rescale(patch.offset((*patch).symbols as i32 as isize)) as i32
                } else {
                    0_i32
                }) as u16;
                (*patch).escapes =
                    ((*patch).escapes as i32 - ((*patch).escapes as i32 >> 1_i32)) as u16;
                (*patch).total = ((*patch).total as i32 + (*patch).escapes as i32) as u16;
            }
            patch = &mut *((*rangeCoder).symbols)
                .as_mut_ptr()
                .offset((*patch).parent as isize) as *mut ENetSymbol;
        }
        *parent = bottom;
        if outData >= outEnd {
            return 0_i32 as usize;
        }
        let fresh28 = outData;
        outData = outData.offset(1);
        *fresh28 = value;
        if order >= ENET_SUBCONTEXT_ORDER as i32 as usize {
            predicted = (*rangeCoder).symbols[predicted as usize].parent;
        } else {
            order = order.wrapping_add(1);
        }
        if nextSymbol
            >= ::core::mem::size_of::<[ENetSymbol; 4096]>()
                .wrapping_div(::core::mem::size_of::<ENetSymbol>())
                .wrapping_sub(ENET_SUBCONTEXT_ORDER as i32 as usize)
        {
            nextSymbol = 0_i32 as usize;
            let fresh29 = nextSymbol;
            nextSymbol = nextSymbol.wrapping_add(1);
            root = &mut *((*rangeCoder).symbols).as_mut_ptr().add(fresh29) as *mut ENetSymbol;
            (*root).value = 0_i32 as u8;
            (*root).count = 0_i32 as u8;
            (*root).under = 0_i32 as u16;
            (*root).left = 0_i32 as u16;
            (*root).right = 0_i32 as u16;
            (*root).symbols = 0_i32 as u16;
            (*root).escapes = 0_i32 as u16;
            (*root).total = 0_i32 as u16;
            (*root).parent = 0_i32 as u16;
            (*root).escapes = ENET_CONTEXT_ESCAPE_MINIMUM as i32 as u16;
            (*root).total = (ENET_CONTEXT_ESCAPE_MINIMUM as i32
                + 256_i32 * ENET_CONTEXT_SYMBOL_MINIMUM as i32) as u16;
            (*root).symbols = 0_i32 as u16;
            predicted = 0_i32 as u16;
            order = 0_i32 as usize;
        }
    }
    outData.offset_from(outStart) as i64 as usize
}
