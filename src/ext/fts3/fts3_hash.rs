extern "C" {
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memcmp(
        __s1: *const ::core::ffi::c_void,
        __s2: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn strncmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn sqlite3_malloc64(_: sqlite3_uint64) -> *mut ::core::ffi::c_void;
    fn sqlite3_free(_: *mut ::core::ffi::c_void);
}
pub type size_t = usize;
pub type sqlite_int64 = ::core::ffi::c_longlong;
pub type sqlite_uint64 = ::core::ffi::c_ulonglong;
pub type sqlite3_int64 = sqlite_int64;
pub type sqlite3_uint64 = sqlite_uint64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Fts3Hash {
    pub keyClass: ::core::ffi::c_char,
    pub copyKey: ::core::ffi::c_char,
    pub count: ::core::ffi::c_int,
    pub first: *mut Fts3HashElem,
    pub htsize: ::core::ffi::c_int,
    pub ht: *mut _fts3ht,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _fts3ht {
    pub count: ::core::ffi::c_int,
    pub chain: *mut Fts3HashElem,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Fts3HashElem {
    pub next: *mut Fts3HashElem,
    pub prev: *mut Fts3HashElem,
    pub data: *mut ::core::ffi::c_void,
    pub pKey: *mut ::core::ffi::c_void,
    pub nKey: ::core::ffi::c_int,
}
pub const FTS3_HASH_STRING: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
unsafe extern "C" fn fts3HashMalloc(mut n: sqlite3_int64) -> *mut ::core::ffi::c_void {
    let mut p: *mut ::core::ffi::c_void = sqlite3_malloc64(n as sqlite3_uint64);
    if !p.is_null() {
        memset(p, 0 as ::core::ffi::c_int, n as size_t);
    }
    return p;
}
unsafe extern "C" fn fts3HashFree(mut p: *mut ::core::ffi::c_void) {
    sqlite3_free(p);
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3Fts3HashInit(
    mut pNew: *mut Fts3Hash,
    mut keyClass: ::core::ffi::c_char,
    mut copyKey: ::core::ffi::c_char,
) {
    (*pNew).keyClass = keyClass;
    (*pNew).copyKey = copyKey;
    (*pNew).first = ::core::ptr::null_mut::<Fts3HashElem>();
    (*pNew).count = 0 as ::core::ffi::c_int;
    (*pNew).htsize = 0 as ::core::ffi::c_int;
    (*pNew).ht = ::core::ptr::null_mut::<_fts3ht>();
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3Fts3HashClear(mut pH: *mut Fts3Hash) {
    let mut elem: *mut Fts3HashElem = ::core::ptr::null_mut::<Fts3HashElem>();
    elem = (*pH).first;
    (*pH).first = ::core::ptr::null_mut::<Fts3HashElem>();
    fts3HashFree((*pH).ht as *mut ::core::ffi::c_void);
    (*pH).ht = ::core::ptr::null_mut::<_fts3ht>();
    (*pH).htsize = 0 as ::core::ffi::c_int;
    while !elem.is_null() {
        let mut next_elem: *mut Fts3HashElem = (*elem).next;
        if (*pH).copyKey as ::core::ffi::c_int != 0 && !(*elem).pKey.is_null() {
            fts3HashFree((*elem).pKey);
        }
        fts3HashFree(elem as *mut ::core::ffi::c_void);
        elem = next_elem;
    }
    (*pH).count = 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn fts3StrHash(
    mut pKey: *const ::core::ffi::c_void,
    mut nKey: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut z: *const ::core::ffi::c_char = pKey as *const ::core::ffi::c_char;
    let mut h: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    if nKey <= 0 as ::core::ffi::c_int {
        nKey = strlen(z) as ::core::ffi::c_int;
    }
    while nKey > 0 as ::core::ffi::c_int {
        let fresh2 = z;
        z = z.offset(1);
        h = h << 3 as ::core::ffi::c_int ^ h ^ *fresh2 as ::core::ffi::c_uint;
        nKey -= 1;
    }
    return (h & 0x7fffffff as ::core::ffi::c_int as ::core::ffi::c_uint) as ::core::ffi::c_int;
}
unsafe extern "C" fn fts3StrCompare(
    mut pKey1: *const ::core::ffi::c_void,
    mut n1: ::core::ffi::c_int,
    mut pKey2: *const ::core::ffi::c_void,
    mut n2: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if n1 != n2 {
        return 1 as ::core::ffi::c_int;
    }
    return strncmp(
        pKey1 as *const ::core::ffi::c_char,
        pKey2 as *const ::core::ffi::c_char,
        n1 as size_t,
    );
}
unsafe extern "C" fn fts3BinHash(
    mut pKey: *const ::core::ffi::c_void,
    mut nKey: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut h: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut z: *const ::core::ffi::c_char = pKey as *const ::core::ffi::c_char;
    loop {
        let fresh0 = nKey;
        nKey = nKey - 1;
        if !(fresh0 > 0 as ::core::ffi::c_int) {
            break;
        }
        let fresh1 = z;
        z = z.offset(1);
        h = h << 3 as ::core::ffi::c_int ^ h ^ *fresh1 as ::core::ffi::c_int;
    }
    return h & 0x7fffffff as ::core::ffi::c_int;
}
unsafe extern "C" fn fts3BinCompare(
    mut pKey1: *const ::core::ffi::c_void,
    mut n1: ::core::ffi::c_int,
    mut pKey2: *const ::core::ffi::c_void,
    mut n2: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if n1 != n2 {
        return 1 as ::core::ffi::c_int;
    }
    return memcmp(pKey1, pKey2, n1 as size_t);
}
unsafe extern "C" fn ftsHashFunction(
    mut keyClass: ::core::ffi::c_int,
) -> Option<
    unsafe extern "C" fn(*const ::core::ffi::c_void, ::core::ffi::c_int) -> ::core::ffi::c_int,
> {
    if keyClass == FTS3_HASH_STRING {
        return Some(
            fts3StrHash
                as unsafe extern "C" fn(
                    *const ::core::ffi::c_void,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        );
    } else {
        return Some(
            fts3BinHash
                as unsafe extern "C" fn(
                    *const ::core::ffi::c_void,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        );
    };
}
unsafe extern "C" fn ftsCompareFunction(
    mut keyClass: ::core::ffi::c_int,
) -> Option<
    unsafe extern "C" fn(
        *const ::core::ffi::c_void,
        ::core::ffi::c_int,
        *const ::core::ffi::c_void,
        ::core::ffi::c_int,
    ) -> ::core::ffi::c_int,
> {
    if keyClass == FTS3_HASH_STRING {
        return Some(
            fts3StrCompare
                as unsafe extern "C" fn(
                    *const ::core::ffi::c_void,
                    ::core::ffi::c_int,
                    *const ::core::ffi::c_void,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        );
    } else {
        return Some(
            fts3BinCompare
                as unsafe extern "C" fn(
                    *const ::core::ffi::c_void,
                    ::core::ffi::c_int,
                    *const ::core::ffi::c_void,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        );
    };
}
unsafe extern "C" fn fts3HashInsertElement(
    mut pH: *mut Fts3Hash,
    mut pEntry: *mut _fts3ht,
    mut pNew: *mut Fts3HashElem,
) {
    let mut pHead: *mut Fts3HashElem = ::core::ptr::null_mut::<Fts3HashElem>();
    pHead = (*pEntry).chain;
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
        (*pNew).prev = ::core::ptr::null_mut::<Fts3HashElem>();
        (*pH).first = pNew;
    }
    (*pEntry).count += 1;
    (*pEntry).chain = pNew;
}
unsafe extern "C" fn fts3Rehash(
    mut pH: *mut Fts3Hash,
    mut new_size: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut new_ht: *mut _fts3ht = ::core::ptr::null_mut::<_fts3ht>();
    let mut elem: *mut Fts3HashElem = ::core::ptr::null_mut::<Fts3HashElem>();
    let mut next_elem: *mut Fts3HashElem = ::core::ptr::null_mut::<Fts3HashElem>();
    let mut xHash: Option<
        unsafe extern "C" fn(*const ::core::ffi::c_void, ::core::ffi::c_int) -> ::core::ffi::c_int,
    > = None;
    new_ht = fts3HashMalloc(
        (new_size as usize).wrapping_mul(::core::mem::size_of::<_fts3ht>() as usize)
            as sqlite3_int64,
    ) as *mut _fts3ht;
    if new_ht.is_null() {
        return 1 as ::core::ffi::c_int;
    }
    fts3HashFree((*pH).ht as *mut ::core::ffi::c_void);
    (*pH).ht = new_ht as *mut _fts3ht;
    (*pH).htsize = new_size;
    xHash = ftsHashFunction((*pH).keyClass as ::core::ffi::c_int);
    elem = (*pH).first;
    (*pH).first = ::core::ptr::null_mut::<Fts3HashElem>();
    while !elem.is_null() {
        let mut h: ::core::ffi::c_int = Some(xHash.expect("non-null function pointer"))
            .expect("non-null function pointer")(
            (*elem).pKey, (*elem).nKey
        ) & new_size - 1 as ::core::ffi::c_int;
        next_elem = (*elem).next;
        fts3HashInsertElement(pH, new_ht.offset(h as isize) as *mut _fts3ht, elem);
        elem = next_elem;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn fts3FindElementByHash(
    mut pH: *const Fts3Hash,
    mut pKey: *const ::core::ffi::c_void,
    mut nKey: ::core::ffi::c_int,
    mut h: ::core::ffi::c_int,
) -> *mut Fts3HashElem {
    let mut elem: *mut Fts3HashElem = ::core::ptr::null_mut::<Fts3HashElem>();
    let mut count: ::core::ffi::c_int = 0;
    let mut xCompare: Option<
        unsafe extern "C" fn(
            *const ::core::ffi::c_void,
            ::core::ffi::c_int,
            *const ::core::ffi::c_void,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    > = None;
    if !(*pH).ht.is_null() {
        let mut pEntry: *mut _fts3ht = (*pH).ht.offset(h as isize) as *mut _fts3ht;
        elem = (*pEntry).chain;
        count = (*pEntry).count;
        xCompare = ftsCompareFunction((*pH).keyClass as ::core::ffi::c_int);
        loop {
            let fresh3 = count;
            count = count - 1;
            if !(fresh3 != 0 && !elem.is_null()) {
                break;
            }
            if Some(xCompare.expect("non-null function pointer"))
                .expect("non-null function pointer")(
                (*elem).pKey, (*elem).nKey, pKey, nKey
            ) == 0 as ::core::ffi::c_int
            {
                return elem;
            }
            elem = (*elem).next;
        }
    }
    return ::core::ptr::null_mut::<Fts3HashElem>();
}
unsafe extern "C" fn fts3RemoveElementByHash(
    mut pH: *mut Fts3Hash,
    mut elem: *mut Fts3HashElem,
    mut h: ::core::ffi::c_int,
) {
    let mut pEntry: *mut _fts3ht = ::core::ptr::null_mut::<_fts3ht>();
    if !(*elem).prev.is_null() {
        (*(*elem).prev).next = (*elem).next;
    } else {
        (*pH).first = (*elem).next;
    }
    if !(*elem).next.is_null() {
        (*(*elem).next).prev = (*elem).prev;
    }
    pEntry = (*pH).ht.offset(h as isize) as *mut _fts3ht as *mut _fts3ht;
    if (*pEntry).chain == elem {
        (*pEntry).chain = (*elem).next;
    }
    (*pEntry).count -= 1;
    if (*pEntry).count <= 0 as ::core::ffi::c_int {
        (*pEntry).chain = ::core::ptr::null_mut::<Fts3HashElem>();
    }
    if (*pH).copyKey as ::core::ffi::c_int != 0 && !(*elem).pKey.is_null() {
        fts3HashFree((*elem).pKey);
    }
    fts3HashFree(elem as *mut ::core::ffi::c_void);
    (*pH).count -= 1;
    if (*pH).count <= 0 as ::core::ffi::c_int {
        sqlite3Fts3HashClear(pH);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3Fts3HashFindElem(
    mut pH: *const Fts3Hash,
    mut pKey: *const ::core::ffi::c_void,
    mut nKey: ::core::ffi::c_int,
) -> *mut Fts3HashElem {
    let mut h: ::core::ffi::c_int = 0;
    let mut xHash: Option<
        unsafe extern "C" fn(*const ::core::ffi::c_void, ::core::ffi::c_int) -> ::core::ffi::c_int,
    > = None;
    if pH.is_null() || (*pH).ht.is_null() {
        return ::core::ptr::null_mut::<Fts3HashElem>();
    }
    xHash = ftsHashFunction((*pH).keyClass as ::core::ffi::c_int);
    h = Some(xHash.expect("non-null function pointer")).expect("non-null function pointer")(
        pKey, nKey,
    );
    return fts3FindElementByHash(pH, pKey, nKey, h & (*pH).htsize - 1 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3Fts3HashFind(
    mut pH: *const Fts3Hash,
    mut pKey: *const ::core::ffi::c_void,
    mut nKey: ::core::ffi::c_int,
) -> *mut ::core::ffi::c_void {
    let mut pElem: *mut Fts3HashElem = ::core::ptr::null_mut::<Fts3HashElem>();
    pElem = sqlite3Fts3HashFindElem(pH, pKey, nKey);
    return if !pElem.is_null() {
        (*pElem).data
    } else {
        ::core::ptr::null_mut::<::core::ffi::c_void>()
    };
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3Fts3HashInsert(
    mut pH: *mut Fts3Hash,
    mut pKey: *const ::core::ffi::c_void,
    mut nKey: ::core::ffi::c_int,
    mut data: *mut ::core::ffi::c_void,
) -> *mut ::core::ffi::c_void {
    let mut hraw: ::core::ffi::c_int = 0;
    let mut h: ::core::ffi::c_int = 0;
    let mut elem: *mut Fts3HashElem = ::core::ptr::null_mut::<Fts3HashElem>();
    let mut new_elem: *mut Fts3HashElem = ::core::ptr::null_mut::<Fts3HashElem>();
    let mut xHash: Option<
        unsafe extern "C" fn(*const ::core::ffi::c_void, ::core::ffi::c_int) -> ::core::ffi::c_int,
    > = None;
    xHash = ftsHashFunction((*pH).keyClass as ::core::ffi::c_int);
    hraw = Some(xHash.expect("non-null function pointer")).expect("non-null function pointer")(
        pKey, nKey,
    );
    h = hraw & (*pH).htsize - 1 as ::core::ffi::c_int;
    elem = fts3FindElementByHash(pH, pKey, nKey, h);
    if !elem.is_null() {
        let mut old_data: *mut ::core::ffi::c_void = (*elem).data;
        if data.is_null() {
            fts3RemoveElementByHash(pH, elem, h);
        } else {
            (*elem).data = data;
        }
        return old_data;
    }
    if data.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    if (*pH).htsize == 0 as ::core::ffi::c_int && fts3Rehash(pH, 8 as ::core::ffi::c_int) != 0
        || (*pH).count >= (*pH).htsize
            && fts3Rehash(pH, (*pH).htsize * 2 as ::core::ffi::c_int) != 0
    {
        (*pH).count = 0 as ::core::ffi::c_int;
        return data;
    }
    new_elem = fts3HashMalloc(::core::mem::size_of::<Fts3HashElem>() as sqlite3_int64)
        as *mut Fts3HashElem;
    if new_elem.is_null() {
        return data;
    }
    if (*pH).copyKey as ::core::ffi::c_int != 0 && !pKey.is_null() {
        (*new_elem).pKey = fts3HashMalloc(nKey as sqlite3_int64);
        if (*new_elem).pKey.is_null() {
            fts3HashFree(new_elem as *mut ::core::ffi::c_void);
            return data;
        }
        memcpy((*new_elem).pKey, pKey, nKey as size_t);
    } else {
        (*new_elem).pKey = pKey as *mut ::core::ffi::c_void;
    }
    (*new_elem).nKey = nKey;
    (*pH).count += 1;
    h = hraw & (*pH).htsize - 1 as ::core::ffi::c_int;
    fts3HashInsertElement(pH, (*pH).ht.offset(h as isize) as *mut _fts3ht, new_elem);
    (*new_elem).data = data;
    return ::core::ptr::null_mut::<::core::ffi::c_void>();
}
