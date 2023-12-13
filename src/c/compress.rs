use std::cmp::Ordering;

use crate::{enet_free, enet_malloc, ENetBuffer};

#[derive(Copy, Clone)]
#[repr(C)]
pub(crate) struct ENetRangeCoder {
    pub(crate) symbols: [ENetSymbol; 4096],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub(crate) struct ENetSymbol {
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
pub(crate) const ENET_CONTEXT_SYMBOL_MINIMUM: u32 = 1;
pub(crate) const ENET_CONTEXT_ESCAPE_MINIMUM: u32 = 1;
pub(crate) const ENET_SUBCONTEXT_ORDER: u32 = 2;
pub(crate) const ENET_RANGE_CODER_BOTTOM: u32 = 65536;
pub(crate) const ENET_SUBCONTEXT_SYMBOL_DELTA: u32 = 2;
pub(crate) const ENET_SUBCONTEXT_ESCAPE_DELTA: u32 = 5;
pub(crate) const ENET_CONTEXT_SYMBOL_DELTA: u32 = 3;
pub(crate) const ENET_RANGE_CODER_TOP: u32 = 16777216;
pub(crate) unsafe fn enet_range_coder_create() -> *mut u8 {
    let range_coder: *mut ENetRangeCoder =
        enet_malloc(::core::mem::size_of::<ENetRangeCoder>()) as *mut ENetRangeCoder;
    if range_coder.is_null() {
        return std::ptr::null_mut();
    }
    range_coder as *mut u8
}
pub(crate) unsafe fn enet_range_coder_destroy(context: *mut u8) {
    let range_coder: *mut ENetRangeCoder = context as *mut ENetRangeCoder;
    if range_coder.is_null() {
        return;
    }
    enet_free(range_coder as *mut u8);
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
    context: *mut u8,
    mut in_buffers: *const ENetBuffer,
    mut in_buffer_count: usize,
    in_limit: usize,
    mut out_data: *mut u8,
    out_limit: usize,
) -> usize {
    let range_coder: *mut ENetRangeCoder = context as *mut ENetRangeCoder;
    let out_start: *mut u8 = out_data;
    let out_end: *mut u8 = out_data.add(out_limit);
    let mut in_data: *const u8;
    let mut in_end: *const u8;
    let mut encode_low: u32 = 0_i32 as u32;
    let mut encode_range: u32 = !0_i32 as u32;
    let mut root: *mut ENetSymbol;
    let mut predicted: u16 = 0_i32 as u16;
    let mut order: usize = 0_i32 as usize;
    let mut next_symbol: usize = 0_i32 as usize;
    if range_coder.is_null() || in_buffer_count <= 0_i32 as usize || in_limit <= 0_i32 as usize {
        return 0_i32 as usize;
    }
    in_data = (*in_buffers).data as *const u8;
    in_end = in_data.add((*in_buffers).data_length);
    in_buffers = in_buffers.offset(1);
    in_buffer_count = in_buffer_count.wrapping_sub(1);
    let fresh0 = next_symbol;
    next_symbol = next_symbol.wrapping_add(1);
    root = ((*range_coder).symbols).as_mut_ptr().add(fresh0);
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
        if in_data >= in_end {
            if in_buffer_count <= 0_i32 as usize {
                break;
            }
            in_data = (*in_buffers).data as *const u8;
            in_end = in_data.add((*in_buffers).data_length);
            in_buffers = in_buffers.offset(1);
            in_buffer_count = in_buffer_count.wrapping_sub(1);
        }
        let fresh1 = in_data;
        in_data = in_data.offset(1);
        let value = *fresh1;
        subcontext = ((*range_coder).symbols)
            .as_mut_ptr()
            .offset(predicted as isize);
        loop {
            if subcontext == root {
                current_block_237 = 2463987395154258233;
                break;
            }
            under = 0_u16;
            count = 0_i32 as u16;
            if (*subcontext).symbols == 0 {
                let fresh2 = next_symbol;
                next_symbol = next_symbol.wrapping_add(1);
                symbol = ((*range_coder).symbols).as_mut_ptr().add(fresh2);
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
                    match (value as i32).cmp(&((*node).value as i32)) {
                        Ordering::Less => {
                            (*node).under =
                                ((*node).under as i32 + ENET_SUBCONTEXT_SYMBOL_DELTA as i32) as u16;
                            if (*node).left != 0 {
                                node = node.offset((*node).left as i32 as isize);
                            } else {
                                let fresh3 = next_symbol;
                                next_symbol = next_symbol.wrapping_add(1);
                                symbol = ((*range_coder).symbols).as_mut_ptr().add(fresh3);
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
                        }
                        Ordering::Greater => {
                            under = (under as i32 + (*node).under as i32) as u16;
                            if (*node).right != 0 {
                                node = node.offset((*node).right as i32 as isize);
                            } else {
                                let fresh4 = next_symbol;
                                next_symbol = next_symbol.wrapping_add(1);
                                symbol = ((*range_coder).symbols).as_mut_ptr().add(fresh4);
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
                        }
                        Ordering::Equal => {
                            count = (count as i32 + (*node).count as i32) as u16;
                            under = (under as i32 + ((*node).under as i32 - (*node).count as i32))
                                as u16;
                            (*node).under =
                                ((*node).under as i32 + ENET_SUBCONTEXT_SYMBOL_DELTA as i32) as u16;
                            (*node).count =
                                ((*node).count as i32 + ENET_SUBCONTEXT_SYMBOL_DELTA as i32) as u8;
                            symbol = node;
                            break;
                        }
                    }
                }
            }
            *parent = symbol.offset_from(((*range_coder).symbols).as_mut_ptr()) as i64 as u16;
            parent = &mut (*symbol).parent;
            total = (*subcontext).total;
            if count as i32 > 0_i32 {
                encode_range = encode_range.wrapping_div(total as u32);
                encode_low = encode_low.wrapping_add(
                    (((*subcontext).escapes as i32 + under as i32) as u32)
                        .wrapping_mul(encode_range),
                );
                encode_range = encode_range.wrapping_mul(count as u32);
                loop {
                    if encode_low ^ encode_low.wrapping_add(encode_range)
                        >= ENET_RANGE_CODER_TOP as i32 as u32
                    {
                        if encode_range >= ENET_RANGE_CODER_BOTTOM as i32 as u32 {
                            break;
                        }
                        encode_range = encode_low.wrapping_neg()
                            & (ENET_RANGE_CODER_BOTTOM as i32 - 1_i32) as u32;
                    }
                    if out_data >= out_end {
                        return 0_i32 as usize;
                    }
                    let fresh5 = out_data;
                    out_data = out_data.offset(1);
                    *fresh5 = (encode_low >> 24_i32) as u8;
                    encode_range <<= 8_i32;
                    encode_low <<= 8_i32;
                }
            } else {
                if (*subcontext).escapes as i32 > 0_i32
                    && ((*subcontext).escapes as i32) < total as i32
                {
                    encode_range = encode_range.wrapping_div(total as u32);
                    encode_low = encode_low.wrapping_add((0_i32 as u32).wrapping_mul(encode_range));
                    encode_range = encode_range.wrapping_mul((*subcontext).escapes as u32);
                    loop {
                        if encode_low ^ encode_low.wrapping_add(encode_range)
                            >= ENET_RANGE_CODER_TOP as i32 as u32
                        {
                            if encode_range >= ENET_RANGE_CODER_BOTTOM as i32 as u32 {
                                break;
                            }
                            encode_range = encode_low.wrapping_neg()
                                & (ENET_RANGE_CODER_BOTTOM as i32 - 1_i32) as u32;
                        }
                        if out_data >= out_end {
                            return 0_i32 as usize;
                        }
                        let fresh6 = out_data;
                        out_data = out_data.offset(1);
                        *fresh6 = (encode_low >> 24_i32) as u8;
                        encode_range <<= 8_i32;
                        encode_low <<= 8_i32;
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
            subcontext = ((*range_coder).symbols)
                .as_mut_ptr()
                .offset((*subcontext).parent as isize);
        }
        if let 2463987395154258233 = current_block_237 {
            under = (value as i32 * ENET_CONTEXT_SYMBOL_MINIMUM as i32) as u16;
            count = ENET_CONTEXT_SYMBOL_MINIMUM as i32 as u16;
            if (*root).symbols == 0 {
                let fresh7 = next_symbol;
                next_symbol = next_symbol.wrapping_add(1);
                symbol = ((*range_coder).symbols).as_mut_ptr().add(fresh7);
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
                    match (value as i32).cmp(&((*node_0).value as i32)) {
                        Ordering::Less => {
                            (*node_0).under =
                                ((*node_0).under as i32 + ENET_CONTEXT_SYMBOL_DELTA as i32) as u16;
                            if (*node_0).left != 0 {
                                node_0 = node_0.offset((*node_0).left as i32 as isize);
                            } else {
                                let fresh8 = next_symbol;
                                next_symbol = next_symbol.wrapping_add(1);
                                symbol = ((*range_coder).symbols).as_mut_ptr().add(fresh8);
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
                        }
                        Ordering::Greater => {
                            under = (under as i32 + (*node_0).under as i32) as u16;
                            if (*node_0).right != 0 {
                                node_0 = node_0.offset((*node_0).right as i32 as isize);
                            } else {
                                let fresh9 = next_symbol;
                                next_symbol = next_symbol.wrapping_add(1);
                                symbol = ((*range_coder).symbols).as_mut_ptr().add(fresh9);
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
                        }
                        Ordering::Equal => {
                            count = (count as i32 + (*node_0).count as i32) as u16;
                            under = (under as i32
                                + ((*node_0).under as i32 - (*node_0).count as i32))
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
            }
            *parent = symbol.offset_from(((*range_coder).symbols).as_mut_ptr()) as i64 as u16;
            total = (*root).total;
            encode_range = encode_range.wrapping_div(total as u32);
            encode_low = encode_low.wrapping_add(
                (((*root).escapes as i32 + under as i32) as u32).wrapping_mul(encode_range),
            );
            encode_range = encode_range.wrapping_mul(count as u32);
            loop {
                if encode_low ^ encode_low.wrapping_add(encode_range)
                    >= ENET_RANGE_CODER_TOP as i32 as u32
                {
                    if encode_range >= ENET_RANGE_CODER_BOTTOM as i32 as u32 {
                        break;
                    }
                    encode_range =
                        encode_low.wrapping_neg() & (ENET_RANGE_CODER_BOTTOM as i32 - 1_i32) as u32;
                }
                if out_data >= out_end {
                    return 0_i32 as usize;
                }
                let fresh10 = out_data;
                out_data = out_data.offset(1);
                *fresh10 = (encode_low >> 24_i32) as u8;
                encode_range <<= 8_i32;
                encode_low <<= 8_i32;
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
            predicted = (*range_coder).symbols[predicted as usize].parent;
        } else {
            order = order.wrapping_add(1);
        }
        if next_symbol
            >= ::core::mem::size_of::<[ENetSymbol; 4096]>()
                .wrapping_div(::core::mem::size_of::<ENetSymbol>())
                .wrapping_sub(ENET_SUBCONTEXT_ORDER as i32 as usize)
        {
            next_symbol = 0_i32 as usize;
            let fresh11 = next_symbol;
            next_symbol = next_symbol.wrapping_add(1);
            root = ((*range_coder).symbols).as_mut_ptr().add(fresh11);
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
    while encode_low != 0 {
        if out_data >= out_end {
            return 0_i32 as usize;
        }
        let fresh12 = out_data;
        out_data = out_data.offset(1);
        *fresh12 = (encode_low >> 24_i32) as u8;
        encode_low <<= 8_i32;
    }
    out_data.offset_from(out_start) as i64 as usize
}
pub(crate) unsafe fn enet_range_coder_decompress(
    context: *mut u8,
    mut in_data: *const u8,
    in_limit: usize,
    mut out_data: *mut u8,
    out_limit: usize,
) -> usize {
    let range_coder: *mut ENetRangeCoder = context as *mut ENetRangeCoder;
    let out_start: *mut u8 = out_data;
    let out_end: *mut u8 = out_data.add(out_limit);
    let in_end: *const u8 = in_data.add(in_limit);
    let mut decode_low: u32 = 0_i32 as u32;
    let mut decode_code: u32 = 0_i32 as u32;
    let mut decode_range: u32 = !0_i32 as u32;
    let mut root: *mut ENetSymbol;
    let mut predicted: u16 = 0_i32 as u16;
    let mut order: usize = 0_i32 as usize;
    let mut next_symbol: usize = 0_i32 as usize;
    if range_coder.is_null() || in_limit <= 0_i32 as usize {
        return 0_i32 as usize;
    }
    let fresh13 = next_symbol;
    next_symbol = next_symbol.wrapping_add(1);
    root = ((*range_coder).symbols).as_mut_ptr().add(fresh13);
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
    if in_data < in_end {
        let fresh14 = in_data;
        in_data = in_data.offset(1);
        decode_code |= ((*fresh14 as i32) << 24_i32) as u32;
    }
    if in_data < in_end {
        let fresh15 = in_data;
        in_data = in_data.offset(1);
        decode_code |= ((*fresh15 as i32) << 16_i32) as u32;
    }
    if in_data < in_end {
        let fresh16 = in_data;
        in_data = in_data.offset(1);
        decode_code |= ((*fresh16 as i32) << 8_i32) as u32;
    }
    if in_data < in_end {
        let fresh17 = in_data;
        in_data = in_data.offset(1);
        decode_code |= *fresh17 as u32;
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
        subcontext = ((*range_coder).symbols)
            .as_mut_ptr()
            .offset(predicted as isize);
        loop {
            if subcontext == root {
                current_block_297 = 18325745679564279244;
                break;
            }
            if (*subcontext).escapes as i32 > 0_i32 {
                total = (*subcontext).total;
                if ((*subcontext).escapes as i32) < total as i32 {
                    decode_range = decode_range.wrapping_div(total as u32);
                    code = decode_code
                        .wrapping_sub(decode_low)
                        .wrapping_div(decode_range) as u16;
                    if (code as i32) < (*subcontext).escapes as i32 {
                        decode_low =
                            decode_low.wrapping_add((0_i32 as u32).wrapping_mul(decode_range));
                        decode_range = decode_range.wrapping_mul((*subcontext).escapes as u32);
                        loop {
                            if decode_low ^ decode_low.wrapping_add(decode_range)
                                >= ENET_RANGE_CODER_TOP as i32 as u32
                            {
                                if decode_range >= ENET_RANGE_CODER_BOTTOM as i32 as u32 {
                                    break;
                                }
                                decode_range = decode_low.wrapping_neg()
                                    & (ENET_RANGE_CODER_BOTTOM as i32 - 1_i32) as u32;
                            }
                            decode_code <<= 8_i32;
                            if in_data < in_end {
                                let fresh18 = in_data;
                                in_data = in_data.offset(1);
                                decode_code |= *fresh18 as u32;
                            }
                            decode_range <<= 8_i32;
                            decode_low <<= 8_i32;
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
                            symbol.offset_from(((*range_coder).symbols).as_mut_ptr()) as i64 as u16;
                        decode_low = decode_low.wrapping_add(
                            (((*subcontext).escapes as i32 + under as i32) as u32)
                                .wrapping_mul(decode_range),
                        );
                        decode_range = decode_range.wrapping_mul(count as u32);
                        loop {
                            if decode_low ^ decode_low.wrapping_add(decode_range)
                                >= ENET_RANGE_CODER_TOP as i32 as u32
                            {
                                if decode_range >= ENET_RANGE_CODER_BOTTOM as i32 as u32 {
                                    break;
                                }
                                decode_range = decode_low.wrapping_neg()
                                    & (ENET_RANGE_CODER_BOTTOM as i32 - 1_i32) as u32;
                            }
                            decode_code <<= 8_i32;
                            if in_data < in_end {
                                let fresh19 = in_data;
                                in_data = in_data.offset(1);
                                decode_code |= *fresh19 as u32;
                            }
                            decode_range <<= 8_i32;
                            decode_low <<= 8_i32;
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
            subcontext = ((*range_coder).symbols)
                .as_mut_ptr()
                .offset((*subcontext).parent as isize);
        }
        if let 18325745679564279244 = current_block_297 {
            total = (*root).total;
            decode_range = decode_range.wrapping_div(total as u32);
            code = decode_code
                .wrapping_sub(decode_low)
                .wrapping_div(decode_range) as u16;
            if (code as i32) < (*root).escapes as i32 {
                decode_low = decode_low.wrapping_add((0_i32 as u32).wrapping_mul(decode_range));
                decode_range = decode_range.wrapping_mul((*root).escapes as u32);
                loop {
                    if decode_low ^ decode_low.wrapping_add(decode_range)
                        >= ENET_RANGE_CODER_TOP as i32 as u32
                    {
                        if decode_range >= ENET_RANGE_CODER_BOTTOM as i32 as u32 {
                            break;
                        }
                        decode_range = decode_low.wrapping_neg()
                            & (ENET_RANGE_CODER_BOTTOM as i32 - 1_i32) as u32;
                    }
                    decode_code <<= 8_i32;
                    if in_data < in_end {
                        let fresh20 = in_data;
                        in_data = in_data.offset(1);
                        decode_code |= *fresh20 as u32;
                    }
                    decode_range <<= 8_i32;
                    decode_low <<= 8_i32;
                }
                break;
            } else {
                code = (code as i32 - (*root).escapes as i32) as u16;
                under = 0_i32 as u16;
                count = ENET_CONTEXT_SYMBOL_MINIMUM as i32 as u16;
                if (*root).symbols == 0 {
                    value = (code as i32 / ENET_CONTEXT_SYMBOL_MINIMUM as i32) as u8;
                    under = (code as i32 - code as i32 % ENET_CONTEXT_SYMBOL_MINIMUM as i32) as u16;
                    let fresh21 = next_symbol;
                    next_symbol = next_symbol.wrapping_add(1);
                    symbol = ((*range_coder).symbols).as_mut_ptr().add(fresh21);
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
                                let fresh22 = next_symbol;
                                next_symbol = next_symbol.wrapping_add(1);
                                symbol = ((*range_coder).symbols).as_mut_ptr().add(fresh22);
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
                                let fresh23 = next_symbol;
                                next_symbol = next_symbol.wrapping_add(1);
                                symbol = ((*range_coder).symbols).as_mut_ptr().add(fresh23);
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
                bottom = symbol.offset_from(((*range_coder).symbols).as_mut_ptr()) as i64 as u16;
                decode_low = decode_low.wrapping_add(
                    (((*root).escapes as i32 + under as i32) as u32).wrapping_mul(decode_range),
                );
                decode_range = decode_range.wrapping_mul(count as u32);
                loop {
                    if decode_low ^ decode_low.wrapping_add(decode_range)
                        >= ENET_RANGE_CODER_TOP as i32 as u32
                    {
                        if decode_range >= ENET_RANGE_CODER_BOTTOM as i32 as u32 {
                            break;
                        }
                        decode_range = decode_low.wrapping_neg()
                            & (ENET_RANGE_CODER_BOTTOM as i32 - 1_i32) as u32;
                    }
                    decode_code <<= 8_i32;
                    if in_data < in_end {
                        let fresh24 = in_data;
                        in_data = in_data.offset(1);
                        decode_code |= *fresh24 as u32;
                    }
                    decode_range <<= 8_i32;
                    decode_low <<= 8_i32;
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
        patch = ((*range_coder).symbols)
            .as_mut_ptr()
            .offset(predicted as isize);
        while patch != subcontext {
            under = 0_u16;
            count = 0_i32 as u16;
            if (*patch).symbols == 0 {
                let fresh25 = next_symbol;
                next_symbol = next_symbol.wrapping_add(1);
                symbol = ((*range_coder).symbols).as_mut_ptr().add(fresh25);
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
                    match (value as i32).cmp(&((*node_1).value as i32)) {
                        Ordering::Less => {
                            (*node_1).under = ((*node_1).under as i32
                                + ENET_SUBCONTEXT_SYMBOL_DELTA as i32)
                                as u16;
                            if (*node_1).left != 0 {
                                node_1 = node_1.offset((*node_1).left as i32 as isize);
                            } else {
                                let fresh26 = next_symbol;
                                next_symbol = next_symbol.wrapping_add(1);
                                symbol = ((*range_coder).symbols).as_mut_ptr().add(fresh26);
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
                        }
                        Ordering::Greater => {
                            under = (under as i32 + (*node_1).under as i32) as u16;
                            if (*node_1).right != 0 {
                                node_1 = node_1.offset((*node_1).right as i32 as isize);
                            } else {
                                let fresh27 = next_symbol;
                                next_symbol = next_symbol.wrapping_add(1);
                                symbol = ((*range_coder).symbols).as_mut_ptr().add(fresh27);
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
                        }
                        Ordering::Equal => {
                            count = (count as i32 + (*node_1).count as i32) as u16;
                            (*node_1).under = ((*node_1).under as i32
                                + ENET_SUBCONTEXT_SYMBOL_DELTA as i32)
                                as u16;
                            (*node_1).count = ((*node_1).count as i32
                                + ENET_SUBCONTEXT_SYMBOL_DELTA as i32)
                                as u8;
                            symbol = node_1;
                            break;
                        }
                    }
                }
            }
            *parent = symbol.offset_from(((*range_coder).symbols).as_mut_ptr()) as i64 as u16;
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
            patch = ((*range_coder).symbols)
                .as_mut_ptr()
                .offset((*patch).parent as isize);
        }
        *parent = bottom;
        if out_data >= out_end {
            return 0_i32 as usize;
        }
        let fresh28 = out_data;
        out_data = out_data.offset(1);
        *fresh28 = value;
        if order >= ENET_SUBCONTEXT_ORDER as i32 as usize {
            predicted = (*range_coder).symbols[predicted as usize].parent;
        } else {
            order = order.wrapping_add(1);
        }
        if next_symbol
            >= ::core::mem::size_of::<[ENetSymbol; 4096]>()
                .wrapping_div(::core::mem::size_of::<ENetSymbol>())
                .wrapping_sub(ENET_SUBCONTEXT_ORDER as i32 as usize)
        {
            next_symbol = 0_i32 as usize;
            let fresh29 = next_symbol;
            next_symbol = next_symbol.wrapping_add(1);
            root = ((*range_coder).symbols).as_mut_ptr().add(fresh29);
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
    out_data.offset_from(out_start) as i64 as usize
}
