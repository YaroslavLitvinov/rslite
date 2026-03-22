






pub use crate::__stddef_size_t_h::size_t;


pub use crate::sqlite3_h::sqlite3_file;pub use crate::sqlite3_h::sqlite3_filename;pub use crate::src::src::malloc::sqlite3_free;pub use crate::sqlite3_h::sqlite3_int64;pub use crate::sqlite3_h::sqlite3_io_methods;pub use crate::src::src::malloc::sqlite3_malloc;pub use crate::sqlite3_h::sqlite3_syscall_ptr;pub use crate::sqlite3_h::sqlite3_vfs;pub use crate::sqlite3_h::sqlite_int64;pub use crate::sqlite3_h::SQLITE_IOERR;pub use crate::sqlite3_h::SQLITE_IOERR_NOMEM;pub use crate::sqlite3_h::SQLITE_IOERR_SHORT_READ_1;pub use crate::sqlite3_h::SQLITE_OK;pub use crate::src::ext::rtree::rtree::i64_0;pub use crate::src::ext::rtree::rtree::u8_0;pub use crate::sqliteInt_h::SQLITE_IOERR_NOMEM_BKPT;
pub use crate::stdlib::uint8_t;

pub use crate::stdlib::__uint8_t;
#[derive(Copy, Clone)]
#[repr(C)]

pub struct MemJournal {
    pub pMethod: *const crate::sqlite3_h::sqlite3_io_methods,
    pub nChunkSize: ::core::ffi::c_int,
    pub nSpill: ::core::ffi::c_int,
    pub pFirst: *mut FileChunk,
    pub endpoint: FilePoint,
    pub readpoint: FilePoint,
    pub flags: ::core::ffi::c_int,
    pub pVfs: *mut crate::sqlite3_h::sqlite3_vfs,
    pub zJournal: *const ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]

pub struct FilePoint {
    pub iOffset: crate::sqlite3_h::sqlite3_int64,
    pub pChunk: *mut FileChunk,
}
#[derive(Copy, Clone)]
#[repr(C)]

pub struct FileChunk {
    pub pNext: *mut FileChunk,
    pub zChunk: [crate::src::ext::rtree::rtree::u8_0; 8],
}

pub const MEMJOURNAL_DFLT_FILECHUNKSIZE: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;

unsafe extern "C" fn memjrnlRead(
    mut pJfd: *mut crate::sqlite3_h::sqlite3_file,
    mut zBuf: *mut ::core::ffi::c_void,
    mut iAmt: ::core::ffi::c_int,
    mut iOfst: crate::sqlite3_h::sqlite_int64,
) -> ::core::ffi::c_int {
    let mut p: *mut MemJournal = pJfd as *mut MemJournal;
    let mut zOut: *mut crate::src::ext::rtree::rtree::u8_0 = zBuf as *mut crate::src::ext::rtree::rtree::u8_0;
    let mut nRead: ::core::ffi::c_int = iAmt;
    let mut iChunkOffset: ::core::ffi::c_int = 0;
    let mut pChunk: *mut FileChunk = ::core::ptr::null_mut::<FileChunk>();
    if iAmt as crate::sqlite3_h::sqlite_int64 + iOfst > (*p).endpoint.iOffset {
        return crate::sqlite3_h::SQLITE_IOERR_SHORT_READ_1;
    }
    if (*p).readpoint.iOffset != iOfst || iOfst == 0 as crate::sqlite3_h::sqlite_int64 {
        let mut iOff: crate::sqlite3_h::sqlite3_int64 = 0 as crate::sqlite3_h::sqlite3_int64;
        pChunk = (*p).pFirst;
        while !pChunk.is_null() && iOff + (*p).nChunkSize as crate::sqlite3_h::sqlite3_int64 <= iOfst {
            iOff += (*p).nChunkSize as crate::sqlite3_h::sqlite3_int64;
            pChunk = (*pChunk).pNext;
        }
    } else {
        pChunk = (*p).readpoint.pChunk;
    }
    iChunkOffset = (iOfst % (*p).nChunkSize as crate::sqlite3_h::sqlite_int64) as ::core::ffi::c_int;
    loop {
        let mut iSpace: ::core::ffi::c_int = (*p).nChunkSize - iChunkOffset;
        let mut nCopy: ::core::ffi::c_int = if nRead < (*p).nChunkSize - iChunkOffset {
            nRead
        } else {
            (*p).nChunkSize - iChunkOffset
        };
        ::libc::memcpy(
            zOut as *mut ::core::ffi::c_void,
            (&raw mut (*pChunk).zChunk as *mut crate::src::ext::rtree::rtree::u8_0).offset(iChunkOffset as isize)
                as *const ::core::ffi::c_void,
            nCopy as crate::__stddef_size_t_h::size_t,
        );
        zOut = zOut.offset(nCopy as isize);
        nRead -= iSpace;
        iChunkOffset = 0 as ::core::ffi::c_int;
        if !(nRead >= 0 as ::core::ffi::c_int
            && {
                pChunk = (*pChunk).pNext;
                !pChunk.is_null()
            }
            && nRead > 0 as ::core::ffi::c_int)
        {
            break;
        }
    }
    (*p).readpoint.iOffset = (if !pChunk.is_null() {
        iOfst + iAmt as crate::sqlite3_h::sqlite_int64
    } else {
        0 as crate::sqlite3_h::sqlite_int64
    }) as crate::sqlite3_h::sqlite3_int64;
    (*p).readpoint.pChunk = pChunk;
    return crate::sqlite3_h::SQLITE_OK;
}

unsafe extern "C" fn memjrnlFreeChunks(mut pFirst: *mut FileChunk) {
    let mut pIter: *mut FileChunk = ::core::ptr::null_mut::<FileChunk>();
    let mut pNext: *mut FileChunk = ::core::ptr::null_mut::<FileChunk>();
    pIter = pFirst;
    while !pIter.is_null() {
        pNext = (*pIter).pNext;
        crate::src::src::malloc::sqlite3_free(pIter as *mut ::core::ffi::c_void);
        pIter = pNext;
    }
}

unsafe extern "C" fn memjrnlCreateFile(mut p: *mut MemJournal) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = 0;
    let mut pReal: *mut crate::sqlite3_h::sqlite3_file = p as *mut crate::sqlite3_h::sqlite3_file;
    let mut copy: MemJournal = *p;
    ::libc::memset(
        p as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<MemJournal>() as crate::__stddef_size_t_h::size_t,
    );
    rc = crate::src::src::os::sqlite3OsOpen(
        
        copy.pVfs as *mut crate::sqlite3_h::sqlite3_vfs,
        copy.zJournal,
        
        pReal as *mut crate::sqlite3_h::sqlite3_file,
        copy.flags,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
    );
    if rc == crate::sqlite3_h::SQLITE_OK {
        let mut nChunk: ::core::ffi::c_int = copy.nChunkSize;
        let mut iOff: crate::src::ext::rtree::rtree::i64_0 = 0 as crate::src::ext::rtree::rtree::i64_0;
        let mut pIter: *mut FileChunk = ::core::ptr::null_mut::<FileChunk>();
        pIter = copy.pFirst;
        while !pIter.is_null() {
            if iOff + nChunk as crate::src::ext::rtree::rtree::i64_0 > copy.endpoint.iOffset {
                nChunk = (copy.endpoint.iOffset as crate::src::ext::rtree::rtree::i64_0 - iOff) as ::core::ffi::c_int;
            }
            rc = crate::src::src::os::sqlite3OsWrite(
                
                pReal as *mut crate::sqlite3_h::sqlite3_file,
                &raw mut (*pIter).zChunk as *mut crate::src::ext::rtree::rtree::u8_0 as *const ::core::ffi::c_void,
                nChunk,
                iOff,
            );
            if rc != 0 {
                break;
            }
            iOff += nChunk as crate::src::ext::rtree::rtree::i64_0;
            pIter = (*pIter).pNext;
        }
        if rc == crate::sqlite3_h::SQLITE_OK {
            memjrnlFreeChunks(copy.pFirst);
        }
    }
    if rc != crate::sqlite3_h::SQLITE_OK {
        crate::src::src::os::sqlite3OsClose(pReal as *mut crate::sqlite3_h::sqlite3_file);
        *p = copy;
    }
    return rc;
}

unsafe extern "C" fn memjrnlWrite(
    mut pJfd: *mut crate::sqlite3_h::sqlite3_file,
    mut zBuf: *const ::core::ffi::c_void,
    mut iAmt: ::core::ffi::c_int,
    mut iOfst: crate::sqlite3_h::sqlite_int64,
) -> ::core::ffi::c_int {
    let mut p: *mut MemJournal = pJfd as *mut MemJournal;
    let mut nWrite: ::core::ffi::c_int = iAmt;
    let mut zWrite: *mut crate::src::ext::rtree::rtree::u8_0 = zBuf as *mut crate::src::ext::rtree::rtree::u8_0;
    if (*p).nSpill > 0 as ::core::ffi::c_int
        && iAmt as crate::sqlite3_h::sqlite_int64 + iOfst > (*p).nSpill as crate::sqlite3_h::sqlite_int64
    {
        let mut rc: ::core::ffi::c_int = memjrnlCreateFile(p);
        if rc == crate::sqlite3_h::SQLITE_OK {
            rc = crate::src::src::os::sqlite3OsWrite(pJfd as *mut crate::sqlite3_h::sqlite3_file, zBuf, iAmt, iOfst as crate::src::ext::rtree::rtree::i64_0);
        }
        return rc;
    } else {
        if iOfst > 0 as crate::sqlite3_h::sqlite_int64 && iOfst != (*p).endpoint.iOffset {
            memjrnlTruncate(pJfd, iOfst);
        }
        if iOfst == 0 as crate::sqlite3_h::sqlite_int64 && !(*p).pFirst.is_null() {
            ::libc::memcpy(
                &raw mut (*(*p).pFirst).zChunk as *mut crate::src::ext::rtree::rtree::u8_0 as *mut ::core::ffi::c_void,
                zBuf,
                iAmt as crate::__stddef_size_t_h::size_t,
            );
        } else {
            while nWrite > 0 as ::core::ffi::c_int {
                let mut pChunk: *mut FileChunk = (*p).endpoint.pChunk;
                let mut iChunkOffset: ::core::ffi::c_int = ((*p).endpoint.iOffset
                    % (*p).nChunkSize as crate::sqlite3_h::sqlite3_int64)
                    as ::core::ffi::c_int;
                let mut iSpace: ::core::ffi::c_int = if nWrite < (*p).nChunkSize - iChunkOffset {
                    nWrite
                } else {
                    (*p).nChunkSize - iChunkOffset
                };
                if iChunkOffset == 0 as ::core::ffi::c_int {
                    let mut pNew: *mut FileChunk = crate::src::src::malloc::sqlite3_malloc(
                        (::core::mem::size_of::<FileChunk>() as usize)
                            .wrapping_add(((*p).nChunkSize - 8 as ::core::ffi::c_int) as usize)
                            as ::core::ffi::c_int,
                    ) as *mut FileChunk;
                    if pNew.is_null() {
                        return crate::sqliteInt_h::SQLITE_IOERR_NOMEM_BKPT;
                    }
                    (*pNew).pNext = ::core::ptr::null_mut::<FileChunk>();
                    if !pChunk.is_null() {
                        (*pChunk).pNext = pNew;
                    } else {
                        (*p).pFirst = pNew;
                    }
                    (*p).endpoint.pChunk = pNew;
                    pChunk = (*p).endpoint.pChunk;
                }
                ::libc::memcpy(
                    (&raw mut (*pChunk).zChunk as *mut crate::src::ext::rtree::rtree::u8_0).offset(iChunkOffset as isize)
                        as *mut ::core::ffi::c_void,
                    zWrite as *const ::core::ffi::c_void,
                    iSpace as crate::__stddef_size_t_h::size_t,
                );
                zWrite = zWrite.offset(iSpace as isize);
                nWrite -= iSpace;
                (*p).endpoint.iOffset += iSpace as crate::sqlite3_h::sqlite3_int64;
            }
        }
    }
    return crate::sqlite3_h::SQLITE_OK;
}

unsafe extern "C" fn memjrnlTruncate(
    mut pJfd: *mut crate::sqlite3_h::sqlite3_file,
    mut size: crate::sqlite3_h::sqlite_int64,
) -> ::core::ffi::c_int {
    let mut p: *mut MemJournal = pJfd as *mut MemJournal;
    if size < (*p).endpoint.iOffset {
        let mut pIter: *mut FileChunk = ::core::ptr::null_mut::<FileChunk>();
        if size == 0 as crate::sqlite3_h::sqlite_int64 {
            memjrnlFreeChunks((*p).pFirst);
            (*p).pFirst = ::core::ptr::null_mut::<FileChunk>();
        } else {
            let mut iOff: crate::src::ext::rtree::rtree::i64_0 = (*p).nChunkSize as crate::src::ext::rtree::rtree::i64_0;
            pIter = (*p).pFirst;
            while !pIter.is_null() && iOff < size {
                iOff += (*p).nChunkSize as crate::src::ext::rtree::rtree::i64_0;
                pIter = (*pIter).pNext;
            }
            if !pIter.is_null() {
                memjrnlFreeChunks((*pIter).pNext);
                (*pIter).pNext = ::core::ptr::null_mut::<FileChunk>();
            }
        }
        (*p).endpoint.pChunk = pIter;
        (*p).endpoint.iOffset = size as crate::sqlite3_h::sqlite3_int64;
        (*p).readpoint.pChunk = ::core::ptr::null_mut::<FileChunk>();
        (*p).readpoint.iOffset = 0 as crate::sqlite3_h::sqlite3_int64;
    }
    return crate::sqlite3_h::SQLITE_OK;
}

unsafe extern "C" fn memjrnlClose(mut pJfd: *mut crate::sqlite3_h::sqlite3_file) -> ::core::ffi::c_int {
    let mut p: *mut MemJournal = pJfd as *mut MemJournal;
    memjrnlFreeChunks((*p).pFirst);
    return crate::sqlite3_h::SQLITE_OK;
}

unsafe extern "C" fn memjrnlSync(
    mut _pJfd: *mut crate::sqlite3_h::sqlite3_file,
    mut _flags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return crate::sqlite3_h::SQLITE_OK;
}

unsafe extern "C" fn memjrnlFileSize(
    mut pJfd: *mut crate::sqlite3_h::sqlite3_file,
    mut pSize: *mut crate::sqlite3_h::sqlite_int64,
) -> ::core::ffi::c_int {
    let mut p: *mut MemJournal = pJfd as *mut MemJournal;
    *pSize = (*p).endpoint.iOffset;
    return crate::sqlite3_h::SQLITE_OK;
}

static mut MemJournalMethods: crate::sqlite3_h::sqlite3_io_methods = unsafe {
    crate::sqlite3_h::sqlite3_io_methods {
    iVersion:  1 as ::core::ffi::c_int,
    xClose:  Some(memjrnlClose as unsafe extern "C" fn(*mut crate::sqlite3_h::sqlite3_file) -> ::core::ffi::c_int),
    xRead:  Some(
            memjrnlRead
                as unsafe extern "C" fn(
                    *mut crate::sqlite3_h::sqlite3_file,
                    *mut ::core::ffi::c_void,
                    ::core::ffi::c_int,
                    crate::sqlite3_h::sqlite_int64,
                ) -> ::core::ffi::c_int,
        ),
    xWrite:  Some(
            memjrnlWrite
                as unsafe extern "C" fn(
                    *mut crate::sqlite3_h::sqlite3_file,
                    *const ::core::ffi::c_void,
                    ::core::ffi::c_int,
                    crate::sqlite3_h::sqlite_int64,
                ) -> ::core::ffi::c_int,
        ),
    xTruncate:  Some(
            memjrnlTruncate
                as unsafe extern "C" fn(*mut crate::sqlite3_h::sqlite3_file, crate::sqlite3_h::sqlite_int64) -> ::core::ffi::c_int,
        ),
    xSync:  Some(
            memjrnlSync
                as unsafe extern "C" fn(
                    *mut crate::sqlite3_h::sqlite3_file,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
    xFileSize:  Some(
            memjrnlFileSize
                as unsafe extern "C" fn(*mut crate::sqlite3_h::sqlite3_file, *mut crate::sqlite3_h::sqlite_int64) -> ::core::ffi::c_int,
        ),
    xLock:  None,
    xUnlock:  None,
    xCheckReservedLock:  None,
    xFileControl:  None,
    xSectorSize:  None,
    xDeviceCharacteristics:  None,
    xShmMap:  None,
    xShmLock:  None,
    xShmBarrier:  None,
    xShmUnmap:  None,
    xFetch:  None,
    xUnfetch:  None,
}
};
#[no_mangle]

pub unsafe extern "C" fn sqlite3JournalOpen(
    mut pVfs: *mut crate::sqlite3_h::sqlite3_vfs,
    mut zName: *const ::core::ffi::c_char,
    mut pJfd: *mut crate::sqlite3_h::sqlite3_file,
    mut flags: ::core::ffi::c_int,
    mut nSpill: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut p: *mut MemJournal = pJfd as *mut MemJournal;
    ::libc::memset(
        p as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<MemJournal>() as crate::__stddef_size_t_h::size_t,
    );
    if nSpill == 0 as ::core::ffi::c_int {
        return crate::src::src::os::sqlite3OsOpen(
            
            pVfs as *mut crate::sqlite3_h::sqlite3_vfs,
            zName,
            
            pJfd as *mut crate::sqlite3_h::sqlite3_file,
            flags,
            ::core::ptr::null_mut::<::core::ffi::c_int>(),
        );
    }
    if nSpill > 0 as ::core::ffi::c_int {
        (*p).nChunkSize = nSpill;
    } else {
        (*p).nChunkSize = ((8 as ::core::ffi::c_int + MEMJOURNAL_DFLT_FILECHUNKSIZE) as usize)
            .wrapping_sub(::core::mem::size_of::<FileChunk>() as usize)
            as ::core::ffi::c_int;
    }
    (*pJfd).pMethods =
        &raw const MemJournalMethods as *const crate::sqlite3_h::sqlite3_io_methods as *const crate::sqlite3_h::sqlite3_io_methods;
    (*p).nSpill = nSpill;
    (*p).flags = flags;
    (*p).zJournal = zName;
    (*p).pVfs = pVfs;
    return crate::sqlite3_h::SQLITE_OK;
}
#[no_mangle]

pub unsafe extern "C" fn sqlite3MemJournalOpen(mut pJfd: *mut crate::sqlite3_h::sqlite3_file) {
    sqlite3JournalOpen(
        ::core::ptr::null_mut::<crate::sqlite3_h::sqlite3_vfs>(),
        ::core::ptr::null::<::core::ffi::c_char>(),
        pJfd,
        0 as ::core::ffi::c_int,
        -(1 as ::core::ffi::c_int),
    );
}
#[no_mangle]

pub unsafe extern "C" fn sqlite3JournalIsInMemory(mut p: *mut crate::sqlite3_h::sqlite3_file) -> ::core::ffi::c_int {
    return ((*p).pMethods == &raw const MemJournalMethods) as ::core::ffi::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn sqlite3JournalSize(mut pVfs: *mut crate::sqlite3_h::sqlite3_vfs) -> ::core::ffi::c_int {
    return if (*pVfs).szOsFile > ::core::mem::size_of::<MemJournal>() as ::core::ffi::c_int {
        (*pVfs).szOsFile
    } else {
        ::core::mem::size_of::<MemJournal>() as ::core::ffi::c_int
    };
}
