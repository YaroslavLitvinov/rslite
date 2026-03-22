




// =============== BEGIN hash_h ================
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Hash {
    pub htsize:  ::core::ffi::c_uint,
    pub count:  ::core::ffi::c_uint,
    pub first:  *mut crate::src::src::hash::HashElem,
    pub ht:  *mut crate::src::src::hash::_ht,
}
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct _ht {
        pub count: ::core::ffi::c_uint,
        pub chain: *mut crate::src::src::hash::HashElem,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct HashElem {
        pub next: *mut crate::src::src::hash::HashElem,
        pub prev: *mut crate::src::src::hash::HashElem,
        pub data: *mut ::core::ffi::c_void,
        pub pKey: *const ::core::ffi::c_char,
        pub h: ::core::ffi::c_uint,
    }
pub use crate::__stddef_size_t_h::size_t;


pub use crate::src::src::malloc::sqlite3_free;pub use crate::sqlite3_h::sqlite_uint64;pub use crate::src::src::fault::sqlite3BeginBenignMalloc;pub use crate::src::src::fault::sqlite3EndBenignMalloc;pub use crate::src::src::malloc::sqlite3Malloc;pub use crate::src::src::malloc::sqlite3MallocSize;pub use crate::src::src::util::sqlite3StrICmp;pub use crate::src::ext::rtree::rtree::u64_0;pub use crate::sqliteInt_h::SQLITE_MALLOC_SOFT_LIMIT;

#[no_mangle]

pub unsafe extern "C" fn sqlite3HashInit(mut pNew: *mut crate::src::src::hash::Hash) {
    (*pNew).first = ::core::ptr::null_mut::<crate::src::src::hash::HashElem>();
    (*pNew).count = 0 as ::core::ffi::c_uint;
    (*pNew).htsize = 0 as ::core::ffi::c_uint;
    (*pNew).ht = ::core::ptr::null_mut::<crate::src::src::hash::_ht>();
}
#[no_mangle]

pub unsafe extern "C" fn sqlite3HashClear(mut pH: *mut crate::src::src::hash::Hash) {
    let mut elem: *mut crate::src::src::hash::HashElem = ::core::ptr::null_mut::<crate::src::src::hash::HashElem>();
    elem = (*pH).first;
    (*pH).first = ::core::ptr::null_mut::<crate::src::src::hash::HashElem>();
    crate::src::src::malloc::sqlite3_free((*pH).ht as *mut ::core::ffi::c_void);
    (*pH).ht = ::core::ptr::null_mut::<crate::src::src::hash::_ht>();
    (*pH).htsize = 0 as ::core::ffi::c_uint;
    while !elem.is_null() {
        let mut next_elem: *mut crate::src::src::hash::HashElem = (*elem).next;
        crate::src::src::malloc::sqlite3_free(elem as *mut ::core::ffi::c_void);
        elem = next_elem;
    }
    (*pH).count = 0 as ::core::ffi::c_uint;
}

unsafe extern "C" fn strHash(mut z: *const ::core::ffi::c_char) -> ::core::ffi::c_uint {
    let mut h: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    while *z.offset(0 as ::core::ffi::c_int as isize) != 0 {
        let fresh0 = z;
        z = z.offset(1);
        h = h.wrapping_add(
            (0xdf as ::core::ffi::c_int & *fresh0 as ::core::ffi::c_uchar as ::core::ffi::c_int)
                as ::core::ffi::c_uint,
        );
        h = h.wrapping_mul(0x9e3779b1 as ::core::ffi::c_uint);
    }
    return h;
}

unsafe extern "C" fn insertElement(
    mut pH: *mut crate::src::src::hash::Hash,
    mut pEntry: *mut crate::src::src::hash::_ht,
    mut pNew: *mut crate::src::src::hash::HashElem,
) {
    let mut pHead: *mut crate::src::src::hash::HashElem = ::core::ptr::null_mut::<crate::src::src::hash::HashElem>();
    if !pEntry.is_null() {
        pHead = if (*pEntry).count != 0 {
            (*pEntry).chain
        } else {
            ::core::ptr::null_mut::<crate::src::src::hash::HashElem>()
        };
        (*pEntry).count = (*pEntry).count.wrapping_add(1);
        (*pEntry).chain = pNew;
    } else {
        pHead = ::core::ptr::null_mut::<crate::src::src::hash::HashElem>();
    }
    if !pHead.is_null() {
        (*pNew).next = pHead;
        (*pNew).prev = (*pHead).prev;
        if !(*pHead).prev.is_null() {
            (*(*pHead).prev).next = pNew;
        } else {
            (*pH).first = pNew;
        }
        (*pHead).prev = pNew;
    } else {
        (*pNew).next = (*pH).first;
        if !(*pH).first.is_null() {
            (*(*pH).first).prev = pNew;
        }
        (*pNew).prev = ::core::ptr::null_mut::<crate::src::src::hash::HashElem>();
        (*pH).first = pNew;
    };
}

unsafe extern "C" fn rehash(
    mut pH: *mut crate::src::src::hash::Hash,
    mut new_size: ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    let mut new_ht: *mut crate::src::src::hash::_ht = ::core::ptr::null_mut::<crate::src::src::hash::_ht>();
    let mut elem: *mut crate::src::src::hash::HashElem = ::core::ptr::null_mut::<crate::src::src::hash::HashElem>();
    let mut next_elem: *mut crate::src::src::hash::HashElem = ::core::ptr::null_mut::<crate::src::src::hash::HashElem>();
    if (new_size as usize).wrapping_mul(::core::mem::size_of::<crate::src::src::hash::_ht>() as usize)
        > crate::sqliteInt_h::SQLITE_MALLOC_SOFT_LIMIT as usize
    {
        new_size = (crate::sqliteInt_h::SQLITE_MALLOC_SOFT_LIMIT as usize)
            .wrapping_div(::core::mem::size_of::<crate::src::src::hash::_ht>() as usize)
            as ::core::ffi::c_uint;
    }
    if new_size == (*pH).htsize {
        return 0 as ::core::ffi::c_int;
    }
    crate::src::src::fault::sqlite3BeginBenignMalloc();
    new_ht = crate::src::src::malloc::sqlite3Malloc(
        (new_size as usize).wrapping_mul(::core::mem::size_of::<crate::src::src::hash::_ht>() as usize) as crate::src::ext::rtree::rtree::u64_0,
    ) as *mut crate::src::src::hash::_ht;
    crate::src::src::fault::sqlite3EndBenignMalloc();
    if new_ht.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    crate::src::src::malloc::sqlite3_free((*pH).ht as *mut ::core::ffi::c_void);
    (*pH).ht = new_ht as *mut crate::src::src::hash::_ht;
    new_size = (crate::src::src::malloc::sqlite3MallocSize(new_ht as *const ::core::ffi::c_void) as usize)
        .wrapping_div(::core::mem::size_of::<crate::src::src::hash::_ht>() as usize) as ::core::ffi::c_uint;
    (*pH).htsize = new_size;
    ::libc::memset(
        new_ht as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        (new_size as crate::__stddef_size_t_h::size_t).wrapping_mul(::core::mem::size_of::<crate::src::src::hash::_ht>() as crate::__stddef_size_t_h::size_t),
    );
    elem = (*pH).first;
    (*pH).first = ::core::ptr::null_mut::<crate::src::src::hash::HashElem>();
    while !elem.is_null() {
        next_elem = (*elem).next;
        insertElement(
            pH,
            new_ht.offset((*elem).h.wrapping_rem(new_size) as isize) as *mut crate::src::src::hash::_ht,
            elem,
        );
        elem = next_elem;
    }
    return 1 as ::core::ffi::c_int;
}

unsafe extern "C" fn findElementWithHash(
    mut pH: *const crate::src::src::hash::Hash,
    mut pKey: *const ::core::ffi::c_char,
    mut pHash: *mut ::core::ffi::c_uint,
) -> *mut crate::src::src::hash::HashElem {
    let mut elem: *mut crate::src::src::hash::HashElem = ::core::ptr::null_mut::<crate::src::src::hash::HashElem>();
    let mut count: ::core::ffi::c_uint = 0;
    let mut h: ::core::ffi::c_uint = 0;
    static mut nullElement: crate::src::src::hash::HashElem = crate::src::src::hash::HashElem {
    next:  ::core::ptr::null::<crate::src::src::hash::HashElem>() as *mut crate::src::src::hash::HashElem,
    prev:  ::core::ptr::null::<crate::src::src::hash::HashElem>() as *mut crate::src::src::hash::HashElem,
    data:  ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
    pKey:  ::core::ptr::null::<::core::ffi::c_char>(),
    h:  0 as ::core::ffi::c_uint,
};
    h = strHash(pKey);
    if !(*pH).ht.is_null() {
        let mut pEntry: *mut crate::src::src::hash::_ht = ::core::ptr::null_mut::<crate::src::src::hash::_ht>();
        pEntry = (*pH).ht.offset(h.wrapping_rem((*pH).htsize) as isize) as *mut crate::src::src::hash::_ht as *mut crate::src::src::hash::_ht;
        elem = (*pEntry).chain;
        count = (*pEntry).count;
    } else {
        elem = (*pH).first;
        count = (*pH).count;
    }
    if !pHash.is_null() {
        *pHash = h;
    }
    while count != 0 {
        if h == (*elem).h && crate::src::src::util::sqlite3StrICmp((*elem).pKey, pKey) == 0 as ::core::ffi::c_int {
            return elem;
        }
        elem = (*elem).next;
        count = count.wrapping_sub(1);
    }
    return &raw mut nullElement;
}

unsafe extern "C" fn removeElement(mut pH: *mut crate::src::src::hash::Hash, mut elem: *mut crate::src::src::hash::HashElem) {
    let mut pEntry: *mut crate::src::src::hash::_ht = ::core::ptr::null_mut::<crate::src::src::hash::_ht>();
    if !(*elem).prev.is_null() {
        (*(*elem).prev).next = (*elem).next;
    } else {
        (*pH).first = (*elem).next;
    }
    if !(*elem).next.is_null() {
        (*(*elem).next).prev = (*elem).prev;
    }
    if !(*pH).ht.is_null() {
        pEntry = (*pH)
            .ht
            .offset((*elem).h.wrapping_rem((*pH).htsize) as isize) as *mut crate::src::src::hash::_ht
            as *mut crate::src::src::hash::_ht;
        if (*pEntry).chain == elem {
            (*pEntry).chain = (*elem).next;
        }
        (*pEntry).count = (*pEntry).count.wrapping_sub(1);
    }
    crate::src::src::malloc::sqlite3_free(elem as *mut ::core::ffi::c_void);
    (*pH).count = (*pH).count.wrapping_sub(1);
    if (*pH).count == 0 as ::core::ffi::c_uint {
        sqlite3HashClear(pH);
    }
}
#[no_mangle]

pub unsafe extern "C" fn sqlite3HashFind(
    mut pH: *const crate::src::src::hash::Hash,
    mut pKey: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_void {
    return (*findElementWithHash(pH, pKey, ::core::ptr::null_mut::<::core::ffi::c_uint>())).data;
}
#[no_mangle]

pub unsafe extern "C" fn sqlite3HashInsert(
    mut pH: *mut crate::src::src::hash::Hash,
    mut pKey: *const ::core::ffi::c_char,
    mut data: *mut ::core::ffi::c_void,
) -> *mut ::core::ffi::c_void {
    let mut h: ::core::ffi::c_uint = 0;
    let mut elem: *mut crate::src::src::hash::HashElem = ::core::ptr::null_mut::<crate::src::src::hash::HashElem>();
    let mut new_elem: *mut crate::src::src::hash::HashElem = ::core::ptr::null_mut::<crate::src::src::hash::HashElem>();
    elem = findElementWithHash(pH, pKey, &raw mut h);
    if !(*elem).data.is_null() {
        let mut old_data: *mut ::core::ffi::c_void = (*elem).data;
        if data.is_null() {
            removeElement(pH, elem);
        } else {
            (*elem).data = data;
            (*elem).pKey = pKey;
        }
        return old_data;
    }
    if data.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    new_elem = crate::src::src::malloc::sqlite3Malloc(::core::mem::size_of::<crate::src::src::hash::HashElem>() as crate::src::ext::rtree::rtree::u64_0) as *mut crate::src::src::hash::HashElem;
    if new_elem.is_null() {
        return data;
    }
    (*new_elem).pKey = pKey;
    (*new_elem).h = h;
    (*new_elem).data = data;
    (*pH).count = (*pH).count.wrapping_add(1);
    if (*pH).count >= 5 as ::core::ffi::c_uint
        && (*pH).count > (2 as ::core::ffi::c_uint).wrapping_mul((*pH).htsize)
    {
        rehash(pH, (*pH).count.wrapping_mul(3 as ::core::ffi::c_uint));
    }
    insertElement(
        pH,
        if !(*pH).ht.is_null() {
            (*pH)
                .ht
                .offset((*new_elem).h.wrapping_rem((*pH).htsize) as isize) as *mut crate::src::src::hash::_ht
        } else {
            ::core::ptr::null_mut::<crate::src::src::hash::_ht>()
        },
        new_elem,
    );
    return ::core::ptr::null_mut::<::core::ffi::c_void>();
}
