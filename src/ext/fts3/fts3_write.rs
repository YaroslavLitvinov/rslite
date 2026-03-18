extern "C" {
    pub type sqlite3;
    pub type sqlite3_stmt;
    pub type sqlite3_value;
    pub type sqlite3_context;
    pub type sqlite3_blob;
    pub type MatchinfoBuffer;
    fn strtol(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
        __base: ::core::ffi::c_int,
    ) -> ::core::ffi::c_long;
    fn qsort(
        __base: *mut ::core::ffi::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
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
    fn sqlite3_exec(
        _: *mut sqlite3,
        sql: *const ::core::ffi::c_char,
        callback: Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                ::core::ffi::c_int,
                *mut *mut ::core::ffi::c_char,
                *mut *mut ::core::ffi::c_char,
            ) -> ::core::ffi::c_int,
        >,
        _: *mut ::core::ffi::c_void,
        errmsg: *mut *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sqlite3_last_insert_rowid(_: *mut sqlite3) -> sqlite3_int64;
    fn sqlite3_mprintf(_: *const ::core::ffi::c_char, ...) -> *mut ::core::ffi::c_char;
    fn sqlite3_malloc64(_: sqlite3_uint64) -> *mut ::core::ffi::c_void;
    fn sqlite3_realloc64(
        _: *mut ::core::ffi::c_void,
        _: sqlite3_uint64,
    ) -> *mut ::core::ffi::c_void;
    fn sqlite3_free(_: *mut ::core::ffi::c_void);
    fn sqlite3_prepare_v2(
        db: *mut sqlite3,
        zSql: *const ::core::ffi::c_char,
        nByte: ::core::ffi::c_int,
        ppStmt: *mut *mut sqlite3_stmt,
        pzTail: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sqlite3_prepare_v3(
        db: *mut sqlite3,
        zSql: *const ::core::ffi::c_char,
        nByte: ::core::ffi::c_int,
        prepFlags: ::core::ffi::c_uint,
        ppStmt: *mut *mut sqlite3_stmt,
        pzTail: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sqlite3_bind_blob(
        _: *mut sqlite3_stmt,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_void,
        n: ::core::ffi::c_int,
        _: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    ) -> ::core::ffi::c_int;
    fn sqlite3_bind_int(
        _: *mut sqlite3_stmt,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3_bind_int64(
        _: *mut sqlite3_stmt,
        _: ::core::ffi::c_int,
        _: sqlite3_int64,
    ) -> ::core::ffi::c_int;
    fn sqlite3_bind_null(_: *mut sqlite3_stmt, _: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn sqlite3_bind_text(
        _: *mut sqlite3_stmt,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    ) -> ::core::ffi::c_int;
    fn sqlite3_bind_value(
        _: *mut sqlite3_stmt,
        _: ::core::ffi::c_int,
        _: *const sqlite3_value,
    ) -> ::core::ffi::c_int;
    fn sqlite3_bind_parameter_count(_: *mut sqlite3_stmt) -> ::core::ffi::c_int;
    fn sqlite3_step(_: *mut sqlite3_stmt) -> ::core::ffi::c_int;
    fn sqlite3_column_blob(
        _: *mut sqlite3_stmt,
        iCol: ::core::ffi::c_int,
    ) -> *const ::core::ffi::c_void;
    fn sqlite3_column_int(_: *mut sqlite3_stmt, iCol: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn sqlite3_column_int64(_: *mut sqlite3_stmt, iCol: ::core::ffi::c_int) -> sqlite3_int64;
    fn sqlite3_column_text(
        _: *mut sqlite3_stmt,
        iCol: ::core::ffi::c_int,
    ) -> *const ::core::ffi::c_uchar;
    fn sqlite3_column_bytes(_: *mut sqlite3_stmt, iCol: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn sqlite3_column_type(_: *mut sqlite3_stmt, iCol: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn sqlite3_finalize(pStmt: *mut sqlite3_stmt) -> ::core::ffi::c_int;
    fn sqlite3_reset(pStmt: *mut sqlite3_stmt) -> ::core::ffi::c_int;
    fn sqlite3_value_int(_: *mut sqlite3_value) -> ::core::ffi::c_int;
    fn sqlite3_value_int64(_: *mut sqlite3_value) -> sqlite3_int64;
    fn sqlite3_value_text(_: *mut sqlite3_value) -> *const ::core::ffi::c_uchar;
    fn sqlite3_value_bytes(_: *mut sqlite3_value) -> ::core::ffi::c_int;
    fn sqlite3_value_type(_: *mut sqlite3_value) -> ::core::ffi::c_int;
    fn sqlite3_blob_open(
        _: *mut sqlite3,
        zDb: *const ::core::ffi::c_char,
        zTable: *const ::core::ffi::c_char,
        zColumn: *const ::core::ffi::c_char,
        iRow: sqlite3_int64,
        flags: ::core::ffi::c_int,
        ppBlob: *mut *mut sqlite3_blob,
    ) -> ::core::ffi::c_int;
    fn sqlite3_blob_reopen(_: *mut sqlite3_blob, _: sqlite3_int64) -> ::core::ffi::c_int;
    fn sqlite3_blob_close(_: *mut sqlite3_blob) -> ::core::ffi::c_int;
    fn sqlite3_blob_bytes(_: *mut sqlite3_blob) -> ::core::ffi::c_int;
    fn sqlite3_blob_read(
        _: *mut sqlite3_blob,
        Z: *mut ::core::ffi::c_void,
        N: ::core::ffi::c_int,
        iOffset: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3_strnicmp(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3_vtab_on_conflict(_: *mut sqlite3) -> ::core::ffi::c_int;
    fn sqlite3Fts3HashInsert(
        _: *mut Fts3Hash,
        pKey: *const ::core::ffi::c_void,
        nKey: ::core::ffi::c_int,
        pData: *mut ::core::ffi::c_void,
    ) -> *mut ::core::ffi::c_void;
    fn sqlite3Fts3HashFind(
        _: *const Fts3Hash,
        pKey: *const ::core::ffi::c_void,
        nKey: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_void;
    fn sqlite3Fts3HashClear(_: *mut Fts3Hash);
    fn sqlite3Fts3HashFindElem(
        _: *const Fts3Hash,
        _: *const ::core::ffi::c_void,
        _: ::core::ffi::c_int,
    ) -> *mut Fts3HashElem;
    fn sqlite3Fts3SegReaderCursor(
        _: *mut Fts3Table,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: *mut Fts3MultiSegReader,
    ) -> ::core::ffi::c_int;
    fn sqlite3Fts3PutVarint(_: *mut ::core::ffi::c_char, _: sqlite3_int64) -> ::core::ffi::c_int;
    fn sqlite3Fts3GetVarint(
        _: *const ::core::ffi::c_char,
        _: *mut sqlite_int64,
    ) -> ::core::ffi::c_int;
    fn sqlite3Fts3GetVarintU(
        _: *const ::core::ffi::c_char,
        _: *mut sqlite_uint64,
    ) -> ::core::ffi::c_int;
    fn sqlite3Fts3GetVarint32(
        _: *const ::core::ffi::c_char,
        _: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3Fts3VarintLen(_: sqlite3_uint64) -> ::core::ffi::c_int;
    fn sqlite3Fts3DoclistPrev(
        _: ::core::ffi::c_int,
        _: *mut ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: *mut *mut ::core::ffi::c_char,
        _: *mut sqlite3_int64,
        _: *mut ::core::ffi::c_int,
        _: *mut u8_0,
    );
    fn sqlite3Fts3FirstFilter(
        _: sqlite3_int64,
        _: *mut ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sqlite3Fts3CreateStatTable(_: *mut ::core::ffi::c_int, _: *mut Fts3Table);
    fn sqlite3Fts3OpenTokenizer(
        _: *mut sqlite3_tokenizer,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: *mut *mut sqlite3_tokenizer_cursor,
    ) -> ::core::ffi::c_int;
}
pub type size_t = usize;
pub type __compar_fn_t = Option<
    unsafe extern "C" fn(
        *const ::core::ffi::c_void,
        *const ::core::ffi::c_void,
    ) -> ::core::ffi::c_int,
>;
pub type sqlite_int64 = ::core::ffi::c_longlong;
pub type sqlite_uint64 = ::core::ffi::c_ulonglong;
pub type sqlite3_int64 = sqlite_int64;
pub type sqlite3_uint64 = sqlite_uint64;
pub type sqlite3_destructor_type = Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sqlite3_vtab {
    pub pModule: *const sqlite3_module,
    pub nRef: ::core::ffi::c_int,
    pub zErrMsg: *mut ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sqlite3_module {
    pub iVersion: ::core::ffi::c_int,
    pub xCreate: Option<
        unsafe extern "C" fn(
            *mut sqlite3,
            *mut ::core::ffi::c_void,
            ::core::ffi::c_int,
            *const *const ::core::ffi::c_char,
            *mut *mut sqlite3_vtab,
            *mut *mut ::core::ffi::c_char,
        ) -> ::core::ffi::c_int,
    >,
    pub xConnect: Option<
        unsafe extern "C" fn(
            *mut sqlite3,
            *mut ::core::ffi::c_void,
            ::core::ffi::c_int,
            *const *const ::core::ffi::c_char,
            *mut *mut sqlite3_vtab,
            *mut *mut ::core::ffi::c_char,
        ) -> ::core::ffi::c_int,
    >,
    pub xBestIndex: Option<
        unsafe extern "C" fn(*mut sqlite3_vtab, *mut sqlite3_index_info) -> ::core::ffi::c_int,
    >,
    pub xDisconnect: Option<unsafe extern "C" fn(*mut sqlite3_vtab) -> ::core::ffi::c_int>,
    pub xDestroy: Option<unsafe extern "C" fn(*mut sqlite3_vtab) -> ::core::ffi::c_int>,
    pub xOpen: Option<
        unsafe extern "C" fn(
            *mut sqlite3_vtab,
            *mut *mut sqlite3_vtab_cursor,
        ) -> ::core::ffi::c_int,
    >,
    pub xClose: Option<unsafe extern "C" fn(*mut sqlite3_vtab_cursor) -> ::core::ffi::c_int>,
    pub xFilter: Option<
        unsafe extern "C" fn(
            *mut sqlite3_vtab_cursor,
            ::core::ffi::c_int,
            *const ::core::ffi::c_char,
            ::core::ffi::c_int,
            *mut *mut sqlite3_value,
        ) -> ::core::ffi::c_int,
    >,
    pub xNext: Option<unsafe extern "C" fn(*mut sqlite3_vtab_cursor) -> ::core::ffi::c_int>,
    pub xEof: Option<unsafe extern "C" fn(*mut sqlite3_vtab_cursor) -> ::core::ffi::c_int>,
    pub xColumn: Option<
        unsafe extern "C" fn(
            *mut sqlite3_vtab_cursor,
            *mut sqlite3_context,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub xRowid: Option<
        unsafe extern "C" fn(*mut sqlite3_vtab_cursor, *mut sqlite3_int64) -> ::core::ffi::c_int,
    >,
    pub xUpdate: Option<
        unsafe extern "C" fn(
            *mut sqlite3_vtab,
            ::core::ffi::c_int,
            *mut *mut sqlite3_value,
            *mut sqlite3_int64,
        ) -> ::core::ffi::c_int,
    >,
    pub xBegin: Option<unsafe extern "C" fn(*mut sqlite3_vtab) -> ::core::ffi::c_int>,
    pub xSync: Option<unsafe extern "C" fn(*mut sqlite3_vtab) -> ::core::ffi::c_int>,
    pub xCommit: Option<unsafe extern "C" fn(*mut sqlite3_vtab) -> ::core::ffi::c_int>,
    pub xRollback: Option<unsafe extern "C" fn(*mut sqlite3_vtab) -> ::core::ffi::c_int>,
    pub xFindFunction: Option<
        unsafe extern "C" fn(
            *mut sqlite3_vtab,
            ::core::ffi::c_int,
            *const ::core::ffi::c_char,
            *mut Option<
                unsafe extern "C" fn(
                    *mut sqlite3_context,
                    ::core::ffi::c_int,
                    *mut *mut sqlite3_value,
                ) -> (),
            >,
            *mut *mut ::core::ffi::c_void,
        ) -> ::core::ffi::c_int,
    >,
    pub xRename: Option<
        unsafe extern "C" fn(*mut sqlite3_vtab, *const ::core::ffi::c_char) -> ::core::ffi::c_int,
    >,
    pub xSavepoint:
        Option<unsafe extern "C" fn(*mut sqlite3_vtab, ::core::ffi::c_int) -> ::core::ffi::c_int>,
    pub xRelease:
        Option<unsafe extern "C" fn(*mut sqlite3_vtab, ::core::ffi::c_int) -> ::core::ffi::c_int>,
    pub xRollbackTo:
        Option<unsafe extern "C" fn(*mut sqlite3_vtab, ::core::ffi::c_int) -> ::core::ffi::c_int>,
    pub xShadowName: Option<unsafe extern "C" fn(*const ::core::ffi::c_char) -> ::core::ffi::c_int>,
    pub xIntegrity: Option<
        unsafe extern "C" fn(
            *mut sqlite3_vtab,
            *const ::core::ffi::c_char,
            *const ::core::ffi::c_char,
            ::core::ffi::c_int,
            *mut *mut ::core::ffi::c_char,
        ) -> ::core::ffi::c_int,
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sqlite3_vtab_cursor {
    pub pVtab: *mut sqlite3_vtab,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sqlite3_index_info {
    pub nConstraint: ::core::ffi::c_int,
    pub aConstraint: *mut sqlite3_index_constraint,
    pub nOrderBy: ::core::ffi::c_int,
    pub aOrderBy: *mut sqlite3_index_orderby,
    pub aConstraintUsage: *mut sqlite3_index_constraint_usage,
    pub idxNum: ::core::ffi::c_int,
    pub idxStr: *mut ::core::ffi::c_char,
    pub needToFreeIdxStr: ::core::ffi::c_int,
    pub orderByConsumed: ::core::ffi::c_int,
    pub estimatedCost: ::core::ffi::c_double,
    pub estimatedRows: sqlite3_int64,
    pub idxFlags: ::core::ffi::c_int,
    pub colUsed: sqlite3_uint64,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sqlite3_index_constraint_usage {
    pub argvIndex: ::core::ffi::c_int,
    pub omit: ::core::ffi::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sqlite3_index_orderby {
    pub iColumn: ::core::ffi::c_int,
    pub desc: ::core::ffi::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sqlite3_index_constraint {
    pub iColumn: ::core::ffi::c_int,
    pub op: ::core::ffi::c_uchar,
    pub usable: ::core::ffi::c_uchar,
    pub iTermOffset: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sqlite3_tokenizer_module {
    pub iVersion: ::core::ffi::c_int,
    pub xCreate: Option<
        unsafe extern "C" fn(
            ::core::ffi::c_int,
            *const *const ::core::ffi::c_char,
            *mut *mut sqlite3_tokenizer,
        ) -> ::core::ffi::c_int,
    >,
    pub xDestroy: Option<unsafe extern "C" fn(*mut sqlite3_tokenizer) -> ::core::ffi::c_int>,
    pub xOpen: Option<
        unsafe extern "C" fn(
            *mut sqlite3_tokenizer,
            *const ::core::ffi::c_char,
            ::core::ffi::c_int,
            *mut *mut sqlite3_tokenizer_cursor,
        ) -> ::core::ffi::c_int,
    >,
    pub xClose: Option<unsafe extern "C" fn(*mut sqlite3_tokenizer_cursor) -> ::core::ffi::c_int>,
    pub xNext: Option<
        unsafe extern "C" fn(
            *mut sqlite3_tokenizer_cursor,
            *mut *const ::core::ffi::c_char,
            *mut ::core::ffi::c_int,
            *mut ::core::ffi::c_int,
            *mut ::core::ffi::c_int,
            *mut ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub xLanguageid: Option<
        unsafe extern "C" fn(
            *mut sqlite3_tokenizer_cursor,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sqlite3_tokenizer_cursor {
    pub pTokenizer: *mut sqlite3_tokenizer,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sqlite3_tokenizer {
    pub pModule: *const sqlite3_tokenizer_module,
}
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
pub type u8_0 = ::core::ffi::c_uchar;
pub type i16_0 = ::core::ffi::c_short;
pub type u32_0 = ::core::ffi::c_uint;
pub type u64_0 = sqlite3_uint64;
pub type i64_0 = sqlite3_int64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Fts3Table {
    pub base: sqlite3_vtab,
    pub db: *mut sqlite3,
    pub zDb: *const ::core::ffi::c_char,
    pub zName: *const ::core::ffi::c_char,
    pub nColumn: ::core::ffi::c_int,
    pub azColumn: *mut *mut ::core::ffi::c_char,
    pub abNotindexed: *mut u8_0,
    pub pTokenizer: *mut sqlite3_tokenizer,
    pub zContentTbl: *mut ::core::ffi::c_char,
    pub zLanguageid: *mut ::core::ffi::c_char,
    pub nAutoincrmerge: ::core::ffi::c_int,
    pub nLeafAdd: u32_0,
    pub bLock: ::core::ffi::c_int,
    pub aStmt: [*mut sqlite3_stmt; 40],
    pub pSeekStmt: *mut sqlite3_stmt,
    pub zReadExprlist: *mut ::core::ffi::c_char,
    pub zWriteExprlist: *mut ::core::ffi::c_char,
    pub nNodeSize: ::core::ffi::c_int,
    pub bFts4: u8_0,
    pub bHasStat: u8_0,
    pub bHasDocsize: u8_0,
    pub bDescIdx: u8_0,
    pub bIgnoreSavepoint: u8_0,
    pub nPgsz: ::core::ffi::c_int,
    pub zSegmentsTbl: *mut ::core::ffi::c_char,
    pub pSegments: *mut sqlite3_blob,
    pub iSavepoint: ::core::ffi::c_int,
    pub nIndex: ::core::ffi::c_int,
    pub aIndex: *mut Fts3Index,
    pub nMaxPendingData: ::core::ffi::c_int,
    pub nPendingData: ::core::ffi::c_int,
    pub iPrevDocid: sqlite_int64,
    pub iPrevLangid: ::core::ffi::c_int,
    pub bPrevDelete: ::core::ffi::c_int,
    pub bNoIncrDoclist: ::core::ffi::c_int,
    pub nMergeCount: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Fts3Index {
    pub nPrefix: ::core::ffi::c_int,
    pub hPending: Fts3Hash,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Fts3Cursor {
    pub base: sqlite3_vtab_cursor,
    pub eSearch: i16_0,
    pub isEof: u8_0,
    pub isRequireSeek: u8_0,
    pub bSeekStmt: u8_0,
    pub pStmt: *mut sqlite3_stmt,
    pub pExpr: *mut Fts3Expr,
    pub iLangid: ::core::ffi::c_int,
    pub nPhrase: ::core::ffi::c_int,
    pub pDeferred: *mut Fts3DeferredToken,
    pub iPrevId: sqlite3_int64,
    pub pNextId: *mut ::core::ffi::c_char,
    pub aDoclist: *mut ::core::ffi::c_char,
    pub nDoclist: ::core::ffi::c_int,
    pub bDesc: u8_0,
    pub eEvalmode: ::core::ffi::c_int,
    pub nRowAvg: ::core::ffi::c_int,
    pub nDoc: sqlite3_int64,
    pub iMinDocid: i64_0,
    pub iMaxDocid: i64_0,
    pub isMatchinfoNeeded: ::core::ffi::c_int,
    pub pMIBuffer: *mut MatchinfoBuffer,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Fts3DeferredToken {
    pub pToken: *mut Fts3PhraseToken,
    pub iCol: ::core::ffi::c_int,
    pub pNext: *mut Fts3DeferredToken,
    pub pList: *mut PendingList,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PendingList {
    pub nData: ::core::ffi::c_int,
    pub aData: *mut ::core::ffi::c_char,
    pub nSpace: ::core::ffi::c_int,
    pub iLastDocid: sqlite3_int64,
    pub iLastCol: sqlite3_int64,
    pub iLastPos: sqlite3_int64,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Fts3PhraseToken {
    pub z: *mut ::core::ffi::c_char,
    pub n: ::core::ffi::c_int,
    pub isPrefix: ::core::ffi::c_int,
    pub bFirst: ::core::ffi::c_int,
    pub pDeferred: *mut Fts3DeferredToken,
    pub pSegcsr: *mut Fts3MultiSegReader,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Fts3MultiSegReader {
    pub apSegment: *mut *mut Fts3SegReader,
    pub nSegment: ::core::ffi::c_int,
    pub nAdvance: ::core::ffi::c_int,
    pub pFilter: *mut Fts3SegFilter,
    pub aBuffer: *mut ::core::ffi::c_char,
    pub nBuffer: i64_0,
    pub iColFilter: ::core::ffi::c_int,
    pub bRestart: ::core::ffi::c_int,
    pub nCost: ::core::ffi::c_int,
    pub bLookup: ::core::ffi::c_int,
    pub zTerm: *mut ::core::ffi::c_char,
    pub nTerm: ::core::ffi::c_int,
    pub aDoclist: *mut ::core::ffi::c_char,
    pub nDoclist: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Fts3SegFilter {
    pub zTerm: *const ::core::ffi::c_char,
    pub nTerm: ::core::ffi::c_int,
    pub iCol: ::core::ffi::c_int,
    pub flags: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Fts3SegReader {
    pub iIdx: ::core::ffi::c_int,
    pub bLookup: u8_0,
    pub rootOnly: u8_0,
    pub iStartBlock: sqlite3_int64,
    pub iLeafEndBlock: sqlite3_int64,
    pub iEndBlock: sqlite3_int64,
    pub iCurrentBlock: sqlite3_int64,
    pub aNode: *mut ::core::ffi::c_char,
    pub nNode: ::core::ffi::c_int,
    pub nPopulate: ::core::ffi::c_int,
    pub pBlob: *mut sqlite3_blob,
    pub ppNextElem: *mut *mut Fts3HashElem,
    pub nTerm: ::core::ffi::c_int,
    pub zTerm: *mut ::core::ffi::c_char,
    pub nTermAlloc: ::core::ffi::c_int,
    pub aDoclist: *mut ::core::ffi::c_char,
    pub nDoclist: ::core::ffi::c_int,
    pub pOffsetList: *mut ::core::ffi::c_char,
    pub nOffsetList: ::core::ffi::c_int,
    pub iDocid: sqlite3_int64,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Fts3Expr {
    pub eType: ::core::ffi::c_int,
    pub nNear: ::core::ffi::c_int,
    pub pParent: *mut Fts3Expr,
    pub pLeft: *mut Fts3Expr,
    pub pRight: *mut Fts3Expr,
    pub pPhrase: *mut Fts3Phrase,
    pub iDocid: sqlite3_int64,
    pub bEof: u8_0,
    pub bStart: u8_0,
    pub bDeferred: u8_0,
    pub iPhrase: ::core::ffi::c_int,
    pub aMI: *mut u32_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Fts3Phrase {
    pub doclist: Fts3Doclist,
    pub bIncr: ::core::ffi::c_int,
    pub iDoclistToken: ::core::ffi::c_int,
    pub pOrPoslist: *mut ::core::ffi::c_char,
    pub iOrDocid: i64_0,
    pub nToken: ::core::ffi::c_int,
    pub iColumn: ::core::ffi::c_int,
    pub aToken: [Fts3PhraseToken; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Fts3Doclist {
    pub aAll: *mut ::core::ffi::c_char,
    pub nAll: ::core::ffi::c_int,
    pub pNextDocid: *mut ::core::ffi::c_char,
    pub iDocid: sqlite3_int64,
    pub bFreeList: ::core::ffi::c_int,
    pub pList: *mut ::core::ffi::c_char,
    pub nList: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SegmentWriter {
    pub pTree: *mut SegmentNode,
    pub iFirst: sqlite3_int64,
    pub iFree: sqlite3_int64,
    pub zTerm: *mut ::core::ffi::c_char,
    pub nTerm: ::core::ffi::c_int,
    pub nMalloc: ::core::ffi::c_int,
    pub zMalloc: *mut ::core::ffi::c_char,
    pub nSize: ::core::ffi::c_int,
    pub nData: ::core::ffi::c_int,
    pub aData: *mut ::core::ffi::c_char,
    pub nLeafData: i64_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SegmentNode {
    pub pParent: *mut SegmentNode,
    pub pRight: *mut SegmentNode,
    pub pLeftmost: *mut SegmentNode,
    pub nEntry: ::core::ffi::c_int,
    pub zTerm: *mut ::core::ffi::c_char,
    pub nTerm: ::core::ffi::c_int,
    pub nMalloc: ::core::ffi::c_int,
    pub zMalloc: *mut ::core::ffi::c_char,
    pub nData: ::core::ffi::c_int,
    pub aData: *mut ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Blob {
    pub a: *mut ::core::ffi::c_char,
    pub n: ::core::ffi::c_int,
    pub nAlloc: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IncrmergeWriter {
    pub nLeafEst: i64_0,
    pub nWork: i64_0,
    pub iAbsLevel: sqlite3_int64,
    pub iIdx: ::core::ffi::c_int,
    pub iStart: sqlite3_int64,
    pub iEnd: sqlite3_int64,
    pub nLeafData: sqlite3_int64,
    pub bNoLeafData: u8_0,
    pub aNodeWriter: [NodeWriter; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NodeWriter {
    pub iBlock: sqlite3_int64,
    pub key: Blob,
    pub block: Blob,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NodeReader {
    pub aNode: *const ::core::ffi::c_char,
    pub nNode: ::core::ffi::c_int,
    pub iOff: ::core::ffi::c_int,
    pub iChild: sqlite3_int64,
    pub term: Blob,
    pub aDoclist: *const ::core::ffi::c_char,
    pub nDoclist: ::core::ffi::c_int,
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL_0: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const ::core::ffi::c_char) -> ::core::ffi::c_int {
    return strtol(
        __nptr,
        NULL as *mut *mut ::core::ffi::c_char,
        10 as ::core::ffi::c_int,
    ) as ::core::ffi::c_int;
}
pub const SQLITE_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_ERROR: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SQLITE_NOMEM: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const SQLITE_CORRUPT: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
pub const SQLITE_CONSTRAINT: ::core::ffi::c_int = 19 as ::core::ffi::c_int;
pub const SQLITE_ROW: ::core::ffi::c_int = 100 as ::core::ffi::c_int;
pub const SQLITE_DONE: ::core::ffi::c_int = 101 as ::core::ffi::c_int;
pub const SQLITE_CORRUPT_VTAB: ::core::ffi::c_int =
    SQLITE_CORRUPT | (1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
pub const SQLITE_PREPARE_PERSISTENT: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const SQLITE_PREPARE_NO_VTAB: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const SQLITE_INTEGER: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SQLITE_BLOB: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const SQLITE_NULL: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const SQLITE_STATIC: sqlite3_destructor_type = None;
pub const SQLITE_REPLACE: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const FTS3_MERGE_COUNT: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const FTS3_MAX_PENDING_DATA: ::core::ffi::c_int =
    1 as ::core::ffi::c_int * 1024 as ::core::ffi::c_int * 1024 as ::core::ffi::c_int;
pub const FTS3_VARINT_MAX: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const FTS3_SEGDIR_MAXLEVEL: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;
pub const LARGEST_INT64: i64_0 =
    0xffffffff as i64_0 | (0x7fffffff as ::core::ffi::c_int as i64_0) << 32 as ::core::ffi::c_int;
pub const FTS_CORRUPT_VTAB: ::core::ffi::c_int = SQLITE_CORRUPT_VTAB;
pub const FTS3_SEGCURSOR_PENDING: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
pub const FTS3_SEGCURSOR_ALL: ::core::ffi::c_int = -(2 as ::core::ffi::c_int);
pub const FTS3_SEGMENT_REQUIRE_POS: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const FTS3_SEGMENT_IGNORE_EMPTY: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const FTS3_SEGMENT_COLUMN_FILTER: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const FTS3_SEGMENT_PREFIX: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
pub const FTS3_SEGMENT_SCAN: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
pub const FTS3_SEGMENT_FIRST: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;
pub const FTS_MAX_APPENDABLE_HEIGHT: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const FTS3_NODE_PADDING: ::core::ffi::c_int = FTS3_VARINT_MAX * 2 as ::core::ffi::c_int;
#[no_mangle]
pub static mut test_fts3_node_chunksize: ::core::ffi::c_int =
    4 as ::core::ffi::c_int * 1024 as ::core::ffi::c_int;
#[no_mangle]
pub static mut test_fts3_node_chunk_threshold: ::core::ffi::c_int =
    4 as ::core::ffi::c_int * 1024 as ::core::ffi::c_int * 4 as ::core::ffi::c_int;
pub const FTS_STAT_DOCTOTAL: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const FTS_STAT_INCRMERGEHINT: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const FTS_STAT_AUTOINCRMERGE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const SQL_DELETE_CONTENT: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQL_IS_EMPTY: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SQL_DELETE_ALL_CONTENT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const SQL_DELETE_ALL_SEGMENTS: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const SQL_DELETE_ALL_SEGDIR: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const SQL_DELETE_ALL_DOCSIZE: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const SQL_DELETE_ALL_STAT: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const SQL_SELECT_CONTENT_BY_ROWID: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const SQL_NEXT_SEGMENT_INDEX: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const SQL_INSERT_SEGMENTS: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
pub const SQL_NEXT_SEGMENTS_ID: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const SQL_INSERT_SEGDIR: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
pub const SQL_SELECT_LEVEL: ::core::ffi::c_int = 12 as ::core::ffi::c_int;
pub const SQL_SELECT_LEVEL_RANGE: ::core::ffi::c_int = 13 as ::core::ffi::c_int;
pub const SQL_SELECT_SEGDIR_MAX_LEVEL: ::core::ffi::c_int = 15 as ::core::ffi::c_int;
pub const SQL_DELETE_SEGDIR_LEVEL: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const SQL_DELETE_SEGMENTS_RANGE: ::core::ffi::c_int = 17 as ::core::ffi::c_int;
pub const SQL_CONTENT_INSERT: ::core::ffi::c_int = 18 as ::core::ffi::c_int;
pub const SQL_DELETE_DOCSIZE: ::core::ffi::c_int = 19 as ::core::ffi::c_int;
pub const SQL_REPLACE_DOCSIZE: ::core::ffi::c_int = 20 as ::core::ffi::c_int;
pub const SQL_SELECT_DOCSIZE: ::core::ffi::c_int = 21 as ::core::ffi::c_int;
pub const SQL_SELECT_STAT: ::core::ffi::c_int = 22 as ::core::ffi::c_int;
pub const SQL_REPLACE_STAT: ::core::ffi::c_int = 23 as ::core::ffi::c_int;
pub const SQL_DELETE_SEGDIR_RANGE: ::core::ffi::c_int = 26 as ::core::ffi::c_int;
pub const SQL_SELECT_ALL_LANGID: ::core::ffi::c_int = 27 as ::core::ffi::c_int;
pub const SQL_FIND_MERGE_LEVEL: ::core::ffi::c_int = 28 as ::core::ffi::c_int;
pub const SQL_MAX_LEAF_NODE_ESTIMATE: ::core::ffi::c_int = 29 as ::core::ffi::c_int;
pub const SQL_DELETE_SEGDIR_ENTRY: ::core::ffi::c_int = 30 as ::core::ffi::c_int;
pub const SQL_SHIFT_SEGDIR_ENTRY: ::core::ffi::c_int = 31 as ::core::ffi::c_int;
pub const SQL_SELECT_SEGDIR: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
pub const SQL_CHOMP_SEGDIR: ::core::ffi::c_int = 33 as ::core::ffi::c_int;
pub const SQL_SEGMENT_IS_APPENDABLE: ::core::ffi::c_int = 34 as ::core::ffi::c_int;
pub const SQL_SELECT_INDEXES: ::core::ffi::c_int = 35 as ::core::ffi::c_int;
pub const SQL_SELECT_MXLEVEL: ::core::ffi::c_int = 36 as ::core::ffi::c_int;
pub const SQL_SELECT_LEVEL_RANGE2: ::core::ffi::c_int = 37 as ::core::ffi::c_int;
pub const SQL_UPDATE_LEVEL_IDX: ::core::ffi::c_int = 38 as ::core::ffi::c_int;
pub const SQL_UPDATE_LEVEL: ::core::ffi::c_int = 39 as ::core::ffi::c_int;
unsafe extern "C" fn fts3SqlStmt(
    mut p: *mut Fts3Table,
    mut eStmt: ::core::ffi::c_int,
    mut pp: *mut *mut sqlite3_stmt,
    mut apVal: *mut *mut sqlite3_value,
) -> ::core::ffi::c_int {
    let mut azSql: [*const ::core::ffi::c_char; 40] = [
        b"DELETE FROM %Q.'%q_content' WHERE rowid = ?\0" as *const u8
            as *const ::core::ffi::c_char,
        b"SELECT NOT EXISTS(SELECT docid FROM %Q.'%q_content' WHERE rowid!=?)\0"
            as *const u8 as *const ::core::ffi::c_char,
        b"DELETE FROM %Q.'%q_content'\0" as *const u8 as *const ::core::ffi::c_char,
        b"DELETE FROM %Q.'%q_segments'\0" as *const u8 as *const ::core::ffi::c_char,
        b"DELETE FROM %Q.'%q_segdir'\0" as *const u8 as *const ::core::ffi::c_char,
        b"DELETE FROM %Q.'%q_docsize'\0" as *const u8 as *const ::core::ffi::c_char,
        b"DELETE FROM %Q.'%q_stat'\0" as *const u8 as *const ::core::ffi::c_char,
        b"SELECT %s WHERE rowid=?\0" as *const u8 as *const ::core::ffi::c_char,
        b"SELECT (SELECT max(idx) FROM %Q.'%q_segdir' WHERE level = ?) + 1\0"
            as *const u8 as *const ::core::ffi::c_char,
        b"REPLACE INTO %Q.'%q_segments'(blockid, block) VALUES(?, ?)\0" as *const u8
            as *const ::core::ffi::c_char,
        b"SELECT coalesce((SELECT max(blockid) FROM %Q.'%q_segments') + 1, 1)\0"
            as *const u8 as *const ::core::ffi::c_char,
        b"REPLACE INTO %Q.'%q_segdir' VALUES(?,?,?,?,?,?)\0" as *const u8
            as *const ::core::ffi::c_char,
        b"SELECT idx, start_block, leaves_end_block, end_block, root FROM %Q.'%q_segdir' WHERE level = ? ORDER BY idx ASC\0"
            as *const u8 as *const ::core::ffi::c_char,
        b"SELECT idx, start_block, leaves_end_block, end_block, root FROM %Q.'%q_segdir' WHERE level BETWEEN ? AND ?ORDER BY level DESC, idx ASC\0"
            as *const u8 as *const ::core::ffi::c_char,
        b"SELECT count(*) FROM %Q.'%q_segdir' WHERE level = ?\0" as *const u8
            as *const ::core::ffi::c_char,
        b"SELECT max(level) FROM %Q.'%q_segdir' WHERE level BETWEEN ? AND ?\0"
            as *const u8 as *const ::core::ffi::c_char,
        b"DELETE FROM %Q.'%q_segdir' WHERE level = ?\0" as *const u8
            as *const ::core::ffi::c_char,
        b"DELETE FROM %Q.'%q_segments' WHERE blockid BETWEEN ? AND ?\0" as *const u8
            as *const ::core::ffi::c_char,
        b"INSERT INTO %Q.'%q_content' VALUES(%s)\0" as *const u8
            as *const ::core::ffi::c_char,
        b"DELETE FROM %Q.'%q_docsize' WHERE docid = ?\0" as *const u8
            as *const ::core::ffi::c_char,
        b"REPLACE INTO %Q.'%q_docsize' VALUES(?,?)\0" as *const u8
            as *const ::core::ffi::c_char,
        b"SELECT size FROM %Q.'%q_docsize' WHERE docid=?\0" as *const u8
            as *const ::core::ffi::c_char,
        b"SELECT value FROM %Q.'%q_stat' WHERE id=?\0" as *const u8
            as *const ::core::ffi::c_char,
        b"REPLACE INTO %Q.'%q_stat' VALUES(?,?)\0" as *const u8
            as *const ::core::ffi::c_char,
        b"\0" as *const u8 as *const ::core::ffi::c_char,
        b"\0" as *const u8 as *const ::core::ffi::c_char,
        b"DELETE FROM %Q.'%q_segdir' WHERE level BETWEEN ? AND ?\0" as *const u8
            as *const ::core::ffi::c_char,
        b"SELECT ? UNION SELECT level / (1024 * ?) FROM %Q.'%q_segdir'\0" as *const u8
            as *const ::core::ffi::c_char,
        b"SELECT level, count(*) AS cnt FROM %Q.'%q_segdir'   GROUP BY level HAVING cnt>=?  ORDER BY (level %% 1024) ASC, 2 DESC LIMIT 1\0"
            as *const u8 as *const ::core::ffi::c_char,
        b"SELECT 2 * total(1 + leaves_end_block - start_block)   FROM (SELECT * FROM %Q.'%q_segdir'         WHERE level = ? ORDER BY idx ASC LIMIT ?  )\0"
            as *const u8 as *const ::core::ffi::c_char,
        b"DELETE FROM %Q.'%q_segdir' WHERE level = ? AND idx = ?\0" as *const u8
            as *const ::core::ffi::c_char,
        b"UPDATE %Q.'%q_segdir' SET idx = ? WHERE level=? AND idx=?\0" as *const u8
            as *const ::core::ffi::c_char,
        b"SELECT idx, start_block, leaves_end_block, end_block, root FROM %Q.'%q_segdir' WHERE level = ? AND idx = ?\0"
            as *const u8 as *const ::core::ffi::c_char,
        b"UPDATE %Q.'%q_segdir' SET start_block = ?, root = ?WHERE level = ? AND idx = ?\0"
            as *const u8 as *const ::core::ffi::c_char,
        b"SELECT 1 FROM %Q.'%q_segments' WHERE blockid=? AND block IS NULL\0"
            as *const u8 as *const ::core::ffi::c_char,
        b"SELECT idx FROM %Q.'%q_segdir' WHERE level=? ORDER BY 1 ASC\0" as *const u8
            as *const ::core::ffi::c_char,
        b"SELECT max( level %% 1024 ) FROM %Q.'%q_segdir'\0" as *const u8
            as *const ::core::ffi::c_char,
        b"SELECT level, idx, end_block FROM %Q.'%q_segdir' WHERE level BETWEEN ? AND ? ORDER BY level DESC, idx ASC\0"
            as *const u8 as *const ::core::ffi::c_char,
        b"UPDATE OR FAIL %Q.'%q_segdir' SET level=-1,idx=? WHERE level=? AND idx=?\0"
            as *const u8 as *const ::core::ffi::c_char,
        b"UPDATE OR FAIL %Q.'%q_segdir' SET level=? WHERE level=-1\0" as *const u8
            as *const ::core::ffi::c_char,
    ];
    let mut rc: ::core::ffi::c_int = SQLITE_OK;
    let mut pStmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    pStmt = (*p).aStmt[eStmt as usize];
    if pStmt.is_null() {
        let mut f: ::core::ffi::c_int = SQLITE_PREPARE_PERSISTENT | SQLITE_PREPARE_NO_VTAB;
        let mut zSql: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        if eStmt == SQL_CONTENT_INSERT {
            zSql = sqlite3_mprintf(
                azSql[eStmt as usize],
                (*p).zDb,
                (*p).zName,
                (*p).zWriteExprlist,
            );
        } else if eStmt == SQL_SELECT_CONTENT_BY_ROWID {
            f &= !SQLITE_PREPARE_NO_VTAB;
            zSql = sqlite3_mprintf(azSql[eStmt as usize], (*p).zReadExprlist);
        } else {
            zSql = sqlite3_mprintf(azSql[eStmt as usize], (*p).zDb, (*p).zName);
        }
        if zSql.is_null() {
            rc = SQLITE_NOMEM;
        } else {
            rc = sqlite3_prepare_v3(
                (*p).db,
                zSql,
                -(1 as ::core::ffi::c_int),
                f as ::core::ffi::c_uint,
                &raw mut pStmt,
                ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
            );
            sqlite3_free(zSql as *mut ::core::ffi::c_void);
            (*p).aStmt[eStmt as usize] = pStmt;
        }
    }
    if !apVal.is_null() {
        let mut i: ::core::ffi::c_int = 0;
        let mut nParam: ::core::ffi::c_int = sqlite3_bind_parameter_count(pStmt);
        i = 0 as ::core::ffi::c_int;
        while rc == SQLITE_OK && i < nParam {
            rc = sqlite3_bind_value(
                pStmt,
                i + 1 as ::core::ffi::c_int,
                *apVal.offset(i as isize),
            );
            i += 1;
        }
    }
    *pp = pStmt;
    return rc;
}
unsafe extern "C" fn fts3SelectDocsize(
    mut pTab: *mut Fts3Table,
    mut iDocid: sqlite3_int64,
    mut ppStmt: *mut *mut sqlite3_stmt,
) -> ::core::ffi::c_int {
    let mut pStmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut rc: ::core::ffi::c_int = 0;
    rc = fts3SqlStmt(
        pTab,
        SQL_SELECT_DOCSIZE,
        &raw mut pStmt,
        ::core::ptr::null_mut::<*mut sqlite3_value>(),
    );
    if rc == SQLITE_OK {
        sqlite3_bind_int64(pStmt, 1 as ::core::ffi::c_int, iDocid);
        rc = sqlite3_step(pStmt);
        if rc != SQLITE_ROW || sqlite3_column_type(pStmt, 0 as ::core::ffi::c_int) != SQLITE_BLOB {
            rc = sqlite3_reset(pStmt);
            if rc == SQLITE_OK {
                rc = FTS_CORRUPT_VTAB;
            }
            pStmt = ::core::ptr::null_mut::<sqlite3_stmt>();
        } else {
            rc = SQLITE_OK;
        }
    }
    *ppStmt = pStmt;
    return rc;
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3Fts3SelectDoctotal(
    mut pTab: *mut Fts3Table,
    mut ppStmt: *mut *mut sqlite3_stmt,
) -> ::core::ffi::c_int {
    let mut pStmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut rc: ::core::ffi::c_int = 0;
    rc = fts3SqlStmt(
        pTab,
        SQL_SELECT_STAT,
        &raw mut pStmt,
        ::core::ptr::null_mut::<*mut sqlite3_value>(),
    );
    if rc == SQLITE_OK {
        sqlite3_bind_int(pStmt, 1 as ::core::ffi::c_int, FTS_STAT_DOCTOTAL);
        if sqlite3_step(pStmt) != SQLITE_ROW
            || sqlite3_column_type(pStmt, 0 as ::core::ffi::c_int) != SQLITE_BLOB
        {
            rc = sqlite3_reset(pStmt);
            if rc == SQLITE_OK {
                rc = FTS_CORRUPT_VTAB;
            }
            pStmt = ::core::ptr::null_mut::<sqlite3_stmt>();
        }
    }
    *ppStmt = pStmt;
    return rc;
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3Fts3SelectDocsize(
    mut pTab: *mut Fts3Table,
    mut iDocid: sqlite3_int64,
    mut ppStmt: *mut *mut sqlite3_stmt,
) -> ::core::ffi::c_int {
    return fts3SelectDocsize(pTab, iDocid, ppStmt);
}
unsafe extern "C" fn fts3SqlExec(
    mut pRC: *mut ::core::ffi::c_int,
    mut p: *mut Fts3Table,
    mut eStmt: ::core::ffi::c_int,
    mut apVal: *mut *mut sqlite3_value,
) {
    let mut pStmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut rc: ::core::ffi::c_int = 0;
    if *pRC != 0 {
        return;
    }
    rc = fts3SqlStmt(p, eStmt, &raw mut pStmt, apVal);
    if rc == SQLITE_OK {
        sqlite3_step(pStmt);
        rc = sqlite3_reset(pStmt);
    }
    *pRC = rc;
}
unsafe extern "C" fn fts3Writelock(mut p: *mut Fts3Table) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = SQLITE_OK;
    if (*p).nPendingData == 0 as ::core::ffi::c_int {
        let mut pStmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
        rc = fts3SqlStmt(
            p,
            SQL_DELETE_SEGDIR_LEVEL,
            &raw mut pStmt,
            ::core::ptr::null_mut::<*mut sqlite3_value>(),
        );
        if rc == SQLITE_OK {
            sqlite3_bind_null(pStmt, 1 as ::core::ffi::c_int);
            sqlite3_step(pStmt);
            rc = sqlite3_reset(pStmt);
        }
    }
    return rc;
}
unsafe extern "C" fn getAbsoluteLevel(
    mut p: *mut Fts3Table,
    mut iLangid: ::core::ffi::c_int,
    mut iIndex: ::core::ffi::c_int,
    mut iLevel: ::core::ffi::c_int,
) -> sqlite3_int64 {
    let mut iBase: sqlite3_int64 = 0;
    iBase = (iLangid as sqlite3_int64 * (*p).nIndex as sqlite3_int64 + iIndex as sqlite3_int64)
        * FTS3_SEGDIR_MAXLEVEL as sqlite3_int64;
    return iBase + iLevel as sqlite3_int64;
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3Fts3AllSegdirs(
    mut p: *mut Fts3Table,
    mut iLangid: ::core::ffi::c_int,
    mut iIndex: ::core::ffi::c_int,
    mut iLevel: ::core::ffi::c_int,
    mut ppStmt: *mut *mut sqlite3_stmt,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = 0;
    let mut pStmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    if iLevel < 0 as ::core::ffi::c_int {
        rc = fts3SqlStmt(
            p,
            SQL_SELECT_LEVEL_RANGE,
            &raw mut pStmt,
            ::core::ptr::null_mut::<*mut sqlite3_value>(),
        );
        if rc == SQLITE_OK {
            sqlite3_bind_int64(
                pStmt,
                1 as ::core::ffi::c_int,
                getAbsoluteLevel(p, iLangid, iIndex, 0 as ::core::ffi::c_int),
            );
            sqlite3_bind_int64(
                pStmt,
                2 as ::core::ffi::c_int,
                getAbsoluteLevel(
                    p,
                    iLangid,
                    iIndex,
                    FTS3_SEGDIR_MAXLEVEL - 1 as ::core::ffi::c_int,
                ),
            );
        }
    } else {
        rc = fts3SqlStmt(
            p,
            SQL_SELECT_LEVEL,
            &raw mut pStmt,
            ::core::ptr::null_mut::<*mut sqlite3_value>(),
        );
        if rc == SQLITE_OK {
            sqlite3_bind_int64(
                pStmt,
                1 as ::core::ffi::c_int,
                getAbsoluteLevel(p, iLangid, iIndex, iLevel),
            );
        }
    }
    *ppStmt = pStmt;
    return rc;
}
unsafe extern "C" fn fts3PendingListAppendVarint(
    mut pp: *mut *mut PendingList,
    mut i: sqlite3_int64,
) -> ::core::ffi::c_int {
    let mut p: *mut PendingList = *pp;
    if p.is_null() {
        p = sqlite3_malloc64(
            (::core::mem::size_of::<PendingList>() as usize).wrapping_add(100 as usize)
                as sqlite3_uint64,
        ) as *mut PendingList;
        if p.is_null() {
            return SQLITE_NOMEM;
        }
        (*p).nSpace = 100 as ::core::ffi::c_int;
        (*p).aData = p.offset(1 as ::core::ffi::c_int as isize) as *mut PendingList
            as *mut ::core::ffi::c_char;
        (*p).nData = 0 as ::core::ffi::c_int;
    } else if (*p).nData + FTS3_VARINT_MAX + 1 as ::core::ffi::c_int > (*p).nSpace {
        let mut nNew: i64_0 = ((*p).nSpace * 2 as ::core::ffi::c_int) as i64_0;
        p = sqlite3_realloc64(
            p as *mut ::core::ffi::c_void,
            (::core::mem::size_of::<PendingList>() as sqlite3_uint64)
                .wrapping_add(nNew as sqlite3_uint64),
        ) as *mut PendingList;
        if p.is_null() {
            sqlite3_free(*pp as *mut ::core::ffi::c_void);
            *pp = ::core::ptr::null_mut::<PendingList>();
            return SQLITE_NOMEM;
        }
        (*p).nSpace = nNew as ::core::ffi::c_int;
        (*p).aData = p.offset(1 as ::core::ffi::c_int as isize) as *mut PendingList
            as *mut ::core::ffi::c_char;
    }
    (*p).nData += sqlite3Fts3PutVarint(
        (*p).aData.offset((*p).nData as isize) as *mut ::core::ffi::c_char,
        i,
    );
    *(*p).aData.offset((*p).nData as isize) = '\0' as i32 as ::core::ffi::c_char;
    *pp = p;
    return SQLITE_OK;
}
unsafe extern "C" fn fts3PendingListAppend(
    mut pp: *mut *mut PendingList,
    mut iDocid: sqlite3_int64,
    mut iCol: sqlite3_int64,
    mut iPos: sqlite3_int64,
    mut pRc: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut p: *mut PendingList = *pp;
    let mut rc: ::core::ffi::c_int = SQLITE_OK;
    if p.is_null() || (*p).iLastDocid != iDocid {
        let mut iDelta: u64_0 = (iDocid as u64_0).wrapping_sub(
            (if !p.is_null() {
                (*p).iLastDocid
            } else {
                0 as sqlite3_int64
            }) as u64_0,
        );
        if !p.is_null() {
            (*p).nData += 1;
        }
        rc = fts3PendingListAppendVarint(&raw mut p, iDelta as sqlite3_int64);
        if SQLITE_OK != rc {
            current_block = 17264066162094073856;
        } else {
            (*p).iLastCol = -(1 as ::core::ffi::c_int) as sqlite3_int64;
            (*p).iLastPos = 0 as sqlite3_int64;
            (*p).iLastDocid = iDocid;
            current_block = 3640593987805443782;
        }
    } else {
        current_block = 3640593987805443782;
    }
    match current_block {
        3640593987805443782 => {
            if iCol > 0 as sqlite3_int64 && (*p).iLastCol != iCol {
                rc = fts3PendingListAppendVarint(&raw mut p, 1 as sqlite3_int64);
                if SQLITE_OK != rc || {
                    rc = fts3PendingListAppendVarint(&raw mut p, iCol);
                    SQLITE_OK != rc
                } {
                    current_block = 17264066162094073856;
                } else {
                    (*p).iLastCol = iCol;
                    (*p).iLastPos = 0 as sqlite3_int64;
                    current_block = 4166486009154926805;
                }
            } else {
                current_block = 4166486009154926805;
            }
            match current_block {
                17264066162094073856 => {}
                _ => {
                    if iCol >= 0 as sqlite3_int64 {
                        rc = fts3PendingListAppendVarint(
                            &raw mut p,
                            2 as sqlite3_int64 + iPos - (*p).iLastPos,
                        );
                        if rc == SQLITE_OK {
                            (*p).iLastPos = iPos;
                        }
                    }
                }
            }
        }
        _ => {}
    }
    *pRc = rc;
    if p != *pp {
        *pp = p;
        return 1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn fts3PendingListDelete(mut pList: *mut PendingList) {
    sqlite3_free(pList as *mut ::core::ffi::c_void);
}
unsafe extern "C" fn fts3PendingTermsAddOne(
    mut p: *mut Fts3Table,
    mut iCol: ::core::ffi::c_int,
    mut iPos: ::core::ffi::c_int,
    mut pHash: *mut Fts3Hash,
    mut zToken: *const ::core::ffi::c_char,
    mut nToken: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut pList: *mut PendingList = ::core::ptr::null_mut::<PendingList>();
    let mut rc: ::core::ffi::c_int = SQLITE_OK;
    pList = sqlite3Fts3HashFind(pHash, zToken as *const ::core::ffi::c_void, nToken)
        as *mut PendingList;
    if !pList.is_null() {
        (*p).nPendingData = ((*p).nPendingData as ::core::ffi::c_ulong).wrapping_sub(
            (((*pList).nData + nToken) as usize)
                .wrapping_add(::core::mem::size_of::<Fts3HashElem>() as usize)
                as ::core::ffi::c_ulong,
        ) as ::core::ffi::c_int as ::core::ffi::c_int;
    }
    if fts3PendingListAppend(
        &raw mut pList,
        (*p).iPrevDocid as sqlite3_int64,
        iCol as sqlite3_int64,
        iPos as sqlite3_int64,
        &raw mut rc,
    ) != 0
    {
        if pList
            == sqlite3Fts3HashInsert(
                pHash,
                zToken as *const ::core::ffi::c_void,
                nToken,
                pList as *mut ::core::ffi::c_void,
            ) as *mut PendingList
        {
            sqlite3_free(pList as *mut ::core::ffi::c_void);
            rc = SQLITE_NOMEM;
        }
    }
    if rc == SQLITE_OK {
        (*p).nPendingData = ((*p).nPendingData as ::core::ffi::c_ulong).wrapping_add(
            (((*pList).nData + nToken) as usize)
                .wrapping_add(::core::mem::size_of::<Fts3HashElem>() as usize)
                as ::core::ffi::c_ulong,
        ) as ::core::ffi::c_int as ::core::ffi::c_int;
    }
    return rc;
}
unsafe extern "C" fn fts3PendingTermsAdd(
    mut p: *mut Fts3Table,
    mut iLangid: ::core::ffi::c_int,
    mut zText: *const ::core::ffi::c_char,
    mut iCol: ::core::ffi::c_int,
    mut pnWord: *mut u32_0,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = 0;
    let mut iStart: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut iEnd: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut iPos: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut nWord: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut zToken: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut nToken: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut pTokenizer: *mut sqlite3_tokenizer = (*p).pTokenizer;
    let mut pModule: *const sqlite3_tokenizer_module = (*pTokenizer).pModule;
    let mut pCsr: *mut sqlite3_tokenizer_cursor =
        ::core::ptr::null_mut::<sqlite3_tokenizer_cursor>();
    let mut xNext: Option<
        unsafe extern "C" fn(
            *mut sqlite3_tokenizer_cursor,
            *mut *const ::core::ffi::c_char,
            *mut ::core::ffi::c_int,
            *mut ::core::ffi::c_int,
            *mut ::core::ffi::c_int,
            *mut ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    > = None;
    if zText.is_null() {
        *pnWord = 0 as u32_0;
        return SQLITE_OK;
    }
    rc = sqlite3Fts3OpenTokenizer(
        pTokenizer,
        iLangid,
        zText,
        -(1 as ::core::ffi::c_int),
        &raw mut pCsr,
    );
    if rc != SQLITE_OK {
        return rc;
    }
    xNext = (*pModule).xNext;
    while SQLITE_OK == rc && {
        rc = xNext.expect("non-null function pointer")(
            pCsr,
            &raw mut zToken,
            &raw mut nToken,
            &raw mut iStart,
            &raw mut iEnd,
            &raw mut iPos,
        );
        SQLITE_OK == rc
    } {
        let mut i: ::core::ffi::c_int = 0;
        if iPos >= nWord {
            nWord = iPos + 1 as ::core::ffi::c_int;
        }
        if iPos < 0 as ::core::ffi::c_int || zToken.is_null() || nToken <= 0 as ::core::ffi::c_int {
            rc = SQLITE_ERROR;
            break;
        } else {
            rc = fts3PendingTermsAddOne(
                p,
                iCol,
                iPos,
                &raw mut (*(*p).aIndex.offset(0 as ::core::ffi::c_int as isize)).hPending,
                zToken,
                nToken,
            );
            i = 1 as ::core::ffi::c_int;
            while rc == SQLITE_OK && i < (*p).nIndex {
                let mut pIndex: *mut Fts3Index = (*p).aIndex.offset(i as isize) as *mut Fts3Index;
                if !(nToken < (*pIndex).nPrefix) {
                    rc = fts3PendingTermsAddOne(
                        p,
                        iCol,
                        iPos,
                        &raw mut (*pIndex).hPending,
                        zToken,
                        (*pIndex).nPrefix,
                    );
                }
                i += 1;
            }
        }
    }
    (*pModule).xClose.expect("non-null function pointer")(pCsr);
    *pnWord = (*pnWord).wrapping_add(nWord as u32_0);
    return if rc == SQLITE_DONE { SQLITE_OK } else { rc };
}
unsafe extern "C" fn fts3PendingTermsDocid(
    mut p: *mut Fts3Table,
    mut bDelete: ::core::ffi::c_int,
    mut iLangid: ::core::ffi::c_int,
    mut iDocid: sqlite_int64,
) -> ::core::ffi::c_int {
    if iDocid < (*p).iPrevDocid
        || iDocid == (*p).iPrevDocid && (*p).bPrevDelete == 0 as ::core::ffi::c_int
        || (*p).iPrevLangid != iLangid
        || (*p).nPendingData > (*p).nMaxPendingData
    {
        let mut rc: ::core::ffi::c_int = sqlite3Fts3PendingTermsFlush(p);
        if rc != SQLITE_OK {
            return rc;
        }
    }
    (*p).iPrevDocid = iDocid;
    (*p).iPrevLangid = iLangid;
    (*p).bPrevDelete = bDelete;
    return SQLITE_OK;
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3Fts3PendingTermsClear(mut p: *mut Fts3Table) {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < (*p).nIndex {
        let mut pElem: *mut Fts3HashElem = ::core::ptr::null_mut::<Fts3HashElem>();
        let mut pHash: *mut Fts3Hash = &raw mut (*(*p).aIndex.offset(i as isize)).hPending;
        pElem = (*pHash).first;
        while !pElem.is_null() {
            let mut pList: *mut PendingList = (*pElem).data as *mut PendingList;
            fts3PendingListDelete(pList);
            pElem = (*pElem).next;
        }
        sqlite3Fts3HashClear(pHash);
        i += 1;
    }
    (*p).nPendingData = 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn fts3InsertTerms(
    mut p: *mut Fts3Table,
    mut iLangid: ::core::ffi::c_int,
    mut apVal: *mut *mut sqlite3_value,
    mut aSz: *mut u32_0,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    i = 2 as ::core::ffi::c_int;
    while i < (*p).nColumn + 2 as ::core::ffi::c_int {
        let mut iCol: ::core::ffi::c_int = i - 2 as ::core::ffi::c_int;
        if *(*p).abNotindexed.offset(iCol as isize) as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        {
            let mut zText: *const ::core::ffi::c_char =
                sqlite3_value_text(*apVal.offset(i as isize)) as *const ::core::ffi::c_char;
            let mut rc: ::core::ffi::c_int = fts3PendingTermsAdd(
                p,
                iLangid,
                zText,
                iCol,
                aSz.offset(iCol as isize) as *mut u32_0,
            );
            if rc != SQLITE_OK {
                return rc;
            }
            let ref mut fresh2 = *aSz.offset((*p).nColumn as isize);
            *fresh2 =
                (*fresh2).wrapping_add(sqlite3_value_bytes(*apVal.offset(i as isize)) as u32_0);
        }
        i += 1;
    }
    return SQLITE_OK;
}
unsafe extern "C" fn fts3InsertData(
    mut p: *mut Fts3Table,
    mut apVal: *mut *mut sqlite3_value,
    mut piDocid: *mut sqlite3_int64,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = 0;
    let mut pContentInsert: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    if !(*p).zContentTbl.is_null() {
        let mut pRowid: *mut sqlite3_value =
            *apVal.offset(((*p).nColumn + 3 as ::core::ffi::c_int) as isize);
        if sqlite3_value_type(pRowid) == SQLITE_NULL {
            pRowid = *apVal.offset(1 as ::core::ffi::c_int as isize);
        }
        if sqlite3_value_type(pRowid) != SQLITE_INTEGER {
            return SQLITE_CONSTRAINT;
        }
        *piDocid = sqlite3_value_int64(pRowid);
        return SQLITE_OK;
    }
    rc = fts3SqlStmt(
        p,
        SQL_CONTENT_INSERT,
        &raw mut pContentInsert,
        apVal.offset(1 as ::core::ffi::c_int as isize) as *mut *mut sqlite3_value,
    );
    if rc == SQLITE_OK && !(*p).zLanguageid.is_null() {
        rc = sqlite3_bind_int(
            pContentInsert,
            (*p).nColumn + 2 as ::core::ffi::c_int,
            sqlite3_value_int(*apVal.offset(((*p).nColumn + 4 as ::core::ffi::c_int) as isize)),
        );
    }
    if rc != SQLITE_OK {
        return rc;
    }
    if SQLITE_NULL
        != sqlite3_value_type(*apVal.offset((3 as ::core::ffi::c_int + (*p).nColumn) as isize))
    {
        if SQLITE_NULL == sqlite3_value_type(*apVal.offset(0 as ::core::ffi::c_int as isize))
            && SQLITE_NULL != sqlite3_value_type(*apVal.offset(1 as ::core::ffi::c_int as isize))
        {
            return SQLITE_ERROR;
        }
        rc = sqlite3_bind_value(
            pContentInsert,
            1 as ::core::ffi::c_int,
            *apVal.offset((3 as ::core::ffi::c_int + (*p).nColumn) as isize),
        );
        if rc != SQLITE_OK {
            return rc;
        }
    }
    sqlite3_step(pContentInsert);
    rc = sqlite3_reset(pContentInsert);
    *piDocid = sqlite3_last_insert_rowid((*p).db);
    return rc;
}
unsafe extern "C" fn fts3DeleteAll(
    mut p: *mut Fts3Table,
    mut bContent: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = SQLITE_OK;
    sqlite3Fts3PendingTermsClear(p);
    if bContent != 0 {
        fts3SqlExec(
            &raw mut rc,
            p,
            SQL_DELETE_ALL_CONTENT,
            ::core::ptr::null_mut::<*mut sqlite3_value>(),
        );
    }
    fts3SqlExec(
        &raw mut rc,
        p,
        SQL_DELETE_ALL_SEGMENTS,
        ::core::ptr::null_mut::<*mut sqlite3_value>(),
    );
    fts3SqlExec(
        &raw mut rc,
        p,
        SQL_DELETE_ALL_SEGDIR,
        ::core::ptr::null_mut::<*mut sqlite3_value>(),
    );
    if (*p).bHasDocsize != 0 {
        fts3SqlExec(
            &raw mut rc,
            p,
            SQL_DELETE_ALL_DOCSIZE,
            ::core::ptr::null_mut::<*mut sqlite3_value>(),
        );
    }
    if (*p).bHasStat != 0 {
        fts3SqlExec(
            &raw mut rc,
            p,
            SQL_DELETE_ALL_STAT,
            ::core::ptr::null_mut::<*mut sqlite3_value>(),
        );
    }
    return rc;
}
unsafe extern "C" fn langidFromSelect(
    mut p: *mut Fts3Table,
    mut pSelect: *mut sqlite3_stmt,
) -> ::core::ffi::c_int {
    let mut iLangid: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if !(*p).zLanguageid.is_null() {
        iLangid = sqlite3_column_int(pSelect, (*p).nColumn + 1 as ::core::ffi::c_int);
    }
    return iLangid;
}
unsafe extern "C" fn fts3DeleteTerms(
    mut pRC: *mut ::core::ffi::c_int,
    mut p: *mut Fts3Table,
    mut pRowid: *mut sqlite3_value,
    mut aSz: *mut u32_0,
    mut pbFound: *mut ::core::ffi::c_int,
) {
    let mut rc: ::core::ffi::c_int = 0;
    let mut pSelect: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    if *pRC != 0 {
        return;
    }
    rc = fts3SqlStmt(
        p,
        SQL_SELECT_CONTENT_BY_ROWID,
        &raw mut pSelect,
        &raw mut pRowid,
    );
    if rc == SQLITE_OK {
        if SQLITE_ROW == sqlite3_step(pSelect) {
            let mut i: ::core::ffi::c_int = 0;
            let mut iLangid: ::core::ffi::c_int = langidFromSelect(p, pSelect);
            let mut iDocid: i64_0 = sqlite3_column_int64(pSelect, 0 as ::core::ffi::c_int) as i64_0;
            rc = fts3PendingTermsDocid(p, 1 as ::core::ffi::c_int, iLangid, iDocid as sqlite_int64);
            i = 1 as ::core::ffi::c_int;
            while rc == SQLITE_OK && i <= (*p).nColumn {
                let mut iCol: ::core::ffi::c_int = i - 1 as ::core::ffi::c_int;
                if *(*p).abNotindexed.offset(iCol as isize) as ::core::ffi::c_int
                    == 0 as ::core::ffi::c_int
                {
                    let mut zText: *const ::core::ffi::c_char =
                        sqlite3_column_text(pSelect, i) as *const ::core::ffi::c_char;
                    rc = fts3PendingTermsAdd(
                        p,
                        iLangid,
                        zText,
                        -(1 as ::core::ffi::c_int),
                        aSz.offset(iCol as isize) as *mut u32_0,
                    );
                    let ref mut fresh11 = *aSz.offset((*p).nColumn as isize);
                    *fresh11 = (*fresh11).wrapping_add(sqlite3_column_bytes(pSelect, i) as u32_0);
                }
                i += 1;
            }
            if rc != SQLITE_OK {
                sqlite3_reset(pSelect);
                *pRC = rc;
                return;
            }
            *pbFound = 1 as ::core::ffi::c_int;
        }
        rc = sqlite3_reset(pSelect);
    } else {
        sqlite3_reset(pSelect);
    }
    *pRC = rc;
}
unsafe extern "C" fn fts3AllocateSegdirIdx(
    mut p: *mut Fts3Table,
    mut iLangid: ::core::ffi::c_int,
    mut iIndex: ::core::ffi::c_int,
    mut iLevel: ::core::ffi::c_int,
    mut piIdx: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = 0;
    let mut pNextIdx: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut iNext: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    rc = fts3SqlStmt(
        p,
        SQL_NEXT_SEGMENT_INDEX,
        &raw mut pNextIdx,
        ::core::ptr::null_mut::<*mut sqlite3_value>(),
    );
    if rc == SQLITE_OK {
        sqlite3_bind_int64(
            pNextIdx,
            1 as ::core::ffi::c_int,
            getAbsoluteLevel(p, iLangid, iIndex, iLevel),
        );
        if SQLITE_ROW == sqlite3_step(pNextIdx) {
            iNext = sqlite3_column_int(pNextIdx, 0 as ::core::ffi::c_int);
        }
        rc = sqlite3_reset(pNextIdx);
    }
    if rc == SQLITE_OK {
        if iNext >= (*p).nMergeCount {
            rc = fts3SegmentMerge(p, iLangid, iIndex, iLevel);
            *piIdx = 0 as ::core::ffi::c_int;
        } else {
            *piIdx = iNext;
        }
    }
    return rc;
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3Fts3ReadBlock(
    mut p: *mut Fts3Table,
    mut iBlockid: sqlite3_int64,
    mut paBlob: *mut *mut ::core::ffi::c_char,
    mut pnBlob: *mut ::core::ffi::c_int,
    mut pnLoad: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = 0;
    if !(*p).pSegments.is_null() {
        rc = sqlite3_blob_reopen((*p).pSegments, iBlockid);
    } else {
        if (*p).zSegmentsTbl.is_null() {
            (*p).zSegmentsTbl = sqlite3_mprintf(
                b"%s_segments\0" as *const u8 as *const ::core::ffi::c_char,
                (*p).zName,
            );
            if (*p).zSegmentsTbl.is_null() {
                return SQLITE_NOMEM;
            }
        }
        rc = sqlite3_blob_open(
            (*p).db,
            (*p).zDb,
            (*p).zSegmentsTbl,
            b"block\0" as *const u8 as *const ::core::ffi::c_char,
            iBlockid,
            0 as ::core::ffi::c_int,
            &raw mut (*p).pSegments,
        );
    }
    if rc == SQLITE_OK {
        let mut nByte: ::core::ffi::c_int = sqlite3_blob_bytes((*p).pSegments);
        *pnBlob = nByte;
        if !paBlob.is_null() {
            let mut aByte: *mut ::core::ffi::c_char =
                sqlite3_malloc64((nByte as i64_0 + FTS3_NODE_PADDING as i64_0) as sqlite3_uint64)
                    as *mut ::core::ffi::c_char;
            if aByte.is_null() {
                rc = SQLITE_NOMEM;
            } else {
                if !pnLoad.is_null() && nByte > test_fts3_node_chunk_threshold {
                    nByte = test_fts3_node_chunksize;
                    *pnLoad = nByte;
                }
                rc = sqlite3_blob_read(
                    (*p).pSegments,
                    aByte as *mut ::core::ffi::c_void,
                    nByte,
                    0 as ::core::ffi::c_int,
                );
                memset(
                    aByte.offset(nByte as isize) as *mut ::core::ffi::c_char
                        as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    FTS3_NODE_PADDING as size_t,
                );
                if rc != SQLITE_OK {
                    sqlite3_free(aByte as *mut ::core::ffi::c_void);
                    aByte = ::core::ptr::null_mut::<::core::ffi::c_char>();
                }
            }
            *paBlob = aByte;
        }
    } else if rc == SQLITE_ERROR {
        rc = FTS_CORRUPT_VTAB;
    }
    return rc;
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3Fts3SegmentsClose(mut p: *mut Fts3Table) {
    sqlite3_blob_close((*p).pSegments);
    (*p).pSegments = ::core::ptr::null_mut::<sqlite3_blob>();
}
unsafe extern "C" fn fts3SegReaderIncrRead(mut pReader: *mut Fts3SegReader) -> ::core::ffi::c_int {
    let mut nRead: ::core::ffi::c_int = 0;
    let mut rc: ::core::ffi::c_int = 0;
    nRead = if (*pReader).nNode - (*pReader).nPopulate < test_fts3_node_chunksize {
        (*pReader).nNode - (*pReader).nPopulate
    } else {
        test_fts3_node_chunksize
    };
    rc = sqlite3_blob_read(
        (*pReader).pBlob,
        (*pReader).aNode.offset((*pReader).nPopulate as isize) as *mut ::core::ffi::c_char
            as *mut ::core::ffi::c_void,
        nRead,
        (*pReader).nPopulate,
    );
    if rc == SQLITE_OK {
        (*pReader).nPopulate += nRead;
        memset(
            (*pReader).aNode.offset((*pReader).nPopulate as isize) as *mut ::core::ffi::c_char
                as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            FTS3_NODE_PADDING as size_t,
        );
        if (*pReader).nPopulate == (*pReader).nNode {
            sqlite3_blob_close((*pReader).pBlob);
            (*pReader).pBlob = ::core::ptr::null_mut::<sqlite3_blob>();
            (*pReader).nPopulate = 0 as ::core::ffi::c_int;
        }
    }
    return rc;
}
unsafe extern "C" fn fts3SegReaderRequire(
    mut pReader: *mut Fts3SegReader,
    mut pFrom: *mut ::core::ffi::c_char,
    mut nByte: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = SQLITE_OK;
    while !(*pReader).pBlob.is_null()
        && rc == SQLITE_OK
        && pFrom.offset_from((*pReader).aNode) as ::core::ffi::c_long + nByte as ::core::ffi::c_long
            > (*pReader).nPopulate as ::core::ffi::c_long
    {
        rc = fts3SegReaderIncrRead(pReader);
    }
    return rc;
}
unsafe extern "C" fn fts3SegReaderSetEof(mut pSeg: *mut Fts3SegReader) {
    if !((*pSeg).rootOnly as ::core::ffi::c_int != 0 as ::core::ffi::c_int) {
        sqlite3_free((*pSeg).aNode as *mut ::core::ffi::c_void);
        sqlite3_blob_close((*pSeg).pBlob);
        (*pSeg).pBlob = ::core::ptr::null_mut::<sqlite3_blob>();
    }
    (*pSeg).aNode = ::core::ptr::null_mut::<::core::ffi::c_char>();
}
unsafe extern "C" fn fts3SegReaderNext(
    mut p: *mut Fts3Table,
    mut pReader: *mut Fts3SegReader,
    mut bIncr: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = 0;
    let mut pNext: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut nPrefix: ::core::ffi::c_int = 0;
    let mut nSuffix: ::core::ffi::c_int = 0;
    if (*pReader).aDoclist.is_null() {
        pNext = (*pReader).aNode;
    } else {
        pNext =
            (*pReader).aDoclist.offset((*pReader).nDoclist as isize) as *mut ::core::ffi::c_char;
    }
    if pNext.is_null()
        || pNext >= (*pReader).aNode.offset((*pReader).nNode as isize) as *mut ::core::ffi::c_char
    {
        if !(*pReader).ppNextElem.is_null() {
            let mut pElem: *mut Fts3HashElem = *(*pReader).ppNextElem;
            sqlite3_free((*pReader).aNode as *mut ::core::ffi::c_void);
            (*pReader).aNode = ::core::ptr::null_mut::<::core::ffi::c_char>();
            if !pElem.is_null() {
                let mut aCopy: *mut ::core::ffi::c_char =
                    ::core::ptr::null_mut::<::core::ffi::c_char>();
                let mut pList: *mut PendingList = (*pElem).data as *mut PendingList;
                let mut nCopy: ::core::ffi::c_int = (*pList).nData + 1 as ::core::ffi::c_int;
                let mut nTerm: ::core::ffi::c_int = (*pElem).nKey;
                if nTerm + 1 as ::core::ffi::c_int > (*pReader).nTermAlloc {
                    sqlite3_free((*pReader).zTerm as *mut ::core::ffi::c_void);
                    (*pReader).zTerm = sqlite3_malloc64(
                        ((nTerm as i64_0 + 1 as i64_0) * 2 as i64_0) as sqlite3_uint64,
                    ) as *mut ::core::ffi::c_char;
                    if (*pReader).zTerm.is_null() {
                        return SQLITE_NOMEM;
                    }
                    (*pReader).nTermAlloc =
                        (nTerm + 1 as ::core::ffi::c_int) * 2 as ::core::ffi::c_int;
                }
                memcpy(
                    (*pReader).zTerm as *mut ::core::ffi::c_void,
                    (*pElem).pKey,
                    nTerm as size_t,
                );
                *(*pReader).zTerm.offset(nTerm as isize) = '\0' as i32 as ::core::ffi::c_char;
                (*pReader).nTerm = nTerm;
                aCopy = sqlite3_malloc64(nCopy as sqlite3_uint64) as *mut ::core::ffi::c_char;
                if aCopy.is_null() {
                    return SQLITE_NOMEM;
                }
                memcpy(
                    aCopy as *mut ::core::ffi::c_void,
                    (*pList).aData as *const ::core::ffi::c_void,
                    nCopy as size_t,
                );
                (*pReader).nDoclist = nCopy;
                (*pReader).nNode = (*pReader).nDoclist;
                (*pReader).aDoclist = aCopy;
                (*pReader).aNode = (*pReader).aDoclist;
                (*pReader).ppNextElem = (*pReader).ppNextElem.offset(1);
            }
            return SQLITE_OK;
        }
        fts3SegReaderSetEof(pReader);
        if (*pReader).iCurrentBlock >= (*pReader).iLeafEndBlock {
            return SQLITE_OK;
        }
        (*pReader).iCurrentBlock += 1;
        rc = sqlite3Fts3ReadBlock(
            p,
            (*pReader).iCurrentBlock,
            &raw mut (*pReader).aNode,
            &raw mut (*pReader).nNode,
            if bIncr != 0 {
                &raw mut (*pReader).nPopulate
            } else {
                ::core::ptr::null_mut::<::core::ffi::c_int>()
            },
        );
        if rc != SQLITE_OK {
            return rc;
        }
        if bIncr != 0 && (*pReader).nPopulate < (*pReader).nNode {
            (*pReader).pBlob = (*p).pSegments;
            (*p).pSegments = ::core::ptr::null_mut::<sqlite3_blob>();
        }
        pNext = (*pReader).aNode;
    }
    rc = fts3SegReaderRequire(pReader, pNext, FTS3_VARINT_MAX * 2 as ::core::ffi::c_int);
    if rc != SQLITE_OK {
        return rc;
    }
    pNext = pNext.offset(
        (if *(pNext as *mut u8_0) as ::core::ffi::c_int & 0x80 as ::core::ffi::c_int != 0 {
            sqlite3Fts3GetVarint32(pNext, &raw mut nPrefix)
        } else {
            nPrefix = *(pNext as *mut u8_0) as ::core::ffi::c_int;
            1 as ::core::ffi::c_int
        }) as isize,
    );
    pNext = pNext.offset(
        (if *(pNext as *mut u8_0) as ::core::ffi::c_int & 0x80 as ::core::ffi::c_int != 0 {
            sqlite3Fts3GetVarint32(pNext, &raw mut nSuffix)
        } else {
            nSuffix = *(pNext as *mut u8_0) as ::core::ffi::c_int;
            1 as ::core::ffi::c_int
        }) as isize,
    );
    if nSuffix <= 0 as ::core::ffi::c_int
        || (((*pReader).aNode.offset((*pReader).nNode as isize) as *mut ::core::ffi::c_char)
            .offset_from(pNext) as ::core::ffi::c_long)
            < nSuffix as ::core::ffi::c_long
        || nPrefix > (*pReader).nTerm
    {
        return FTS_CORRUPT_VTAB;
    }
    if nPrefix as i64_0 + nSuffix as i64_0 > (*pReader).nTermAlloc as i64_0 {
        let mut nNew: i64_0 = (nPrefix as i64_0 + nSuffix as i64_0) * 2 as i64_0;
        let mut zNew: *mut ::core::ffi::c_char = sqlite3_realloc64(
            (*pReader).zTerm as *mut ::core::ffi::c_void,
            nNew as sqlite3_uint64,
        ) as *mut ::core::ffi::c_char;
        if zNew.is_null() {
            return SQLITE_NOMEM;
        }
        (*pReader).zTerm = zNew;
        (*pReader).nTermAlloc = nNew as ::core::ffi::c_int;
    }
    rc = fts3SegReaderRequire(pReader, pNext, nSuffix + FTS3_VARINT_MAX);
    if rc != SQLITE_OK {
        return rc;
    }
    memcpy(
        (*pReader).zTerm.offset(nPrefix as isize) as *mut ::core::ffi::c_char
            as *mut ::core::ffi::c_void,
        pNext as *const ::core::ffi::c_void,
        nSuffix as size_t,
    );
    (*pReader).nTerm = nPrefix + nSuffix;
    pNext = pNext.offset(nSuffix as isize);
    pNext = pNext.offset(
        (if *(pNext as *mut u8_0) as ::core::ffi::c_int & 0x80 as ::core::ffi::c_int != 0 {
            sqlite3Fts3GetVarint32(pNext, &raw mut (*pReader).nDoclist)
        } else {
            (*pReader).nDoclist = *(pNext as *mut u8_0) as ::core::ffi::c_int;
            1 as ::core::ffi::c_int
        }) as isize,
    );
    (*pReader).aDoclist = pNext;
    (*pReader).pOffsetList = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if (*pReader).nDoclist as ::core::ffi::c_long
        > (*pReader).nNode as ::core::ffi::c_long
            - (*pReader).aDoclist.offset_from((*pReader).aNode) as ::core::ffi::c_long
        || (*pReader).nPopulate == 0 as ::core::ffi::c_int
            && *(*pReader)
                .aDoclist
                .offset(((*pReader).nDoclist - 1 as ::core::ffi::c_int) as isize)
                as ::core::ffi::c_int
                != 0
        || (*pReader).nDoclist == 0 as ::core::ffi::c_int
    {
        return FTS_CORRUPT_VTAB;
    }
    return SQLITE_OK;
}
unsafe extern "C" fn fts3SegReaderFirstDocid(
    mut pTab: *mut Fts3Table,
    mut pReader: *mut Fts3SegReader,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = SQLITE_OK;
    if (*pTab).bDescIdx as ::core::ffi::c_int != 0 && !(*pReader).ppNextElem.is_null() {
        let mut bEof: u8_0 = 0 as u8_0;
        (*pReader).iDocid = 0 as sqlite3_int64;
        (*pReader).nOffsetList = 0 as ::core::ffi::c_int;
        sqlite3Fts3DoclistPrev(
            0 as ::core::ffi::c_int,
            (*pReader).aDoclist,
            (*pReader).nDoclist,
            &raw mut (*pReader).pOffsetList,
            &raw mut (*pReader).iDocid,
            &raw mut (*pReader).nOffsetList,
            &raw mut bEof,
        );
    } else {
        rc = fts3SegReaderRequire(pReader, (*pReader).aDoclist, FTS3_VARINT_MAX);
        if rc == SQLITE_OK {
            let mut n: ::core::ffi::c_int =
                sqlite3Fts3GetVarint((*pReader).aDoclist, &raw mut (*pReader).iDocid);
            (*pReader).pOffsetList =
                (*pReader).aDoclist.offset(n as isize) as *mut ::core::ffi::c_char;
        }
    }
    return rc;
}
unsafe extern "C" fn fts3SegReaderNextDocid(
    mut pTab: *mut Fts3Table,
    mut pReader: *mut Fts3SegReader,
    mut ppOffsetList: *mut *mut ::core::ffi::c_char,
    mut pnOffsetList: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = SQLITE_OK;
    let mut p: *mut ::core::ffi::c_char = (*pReader).pOffsetList;
    let mut c: ::core::ffi::c_char = 0 as ::core::ffi::c_char;
    if (*pTab).bDescIdx as ::core::ffi::c_int != 0 && !(*pReader).ppNextElem.is_null() {
        let mut bEof: u8_0 = 0 as u8_0;
        if !ppOffsetList.is_null() {
            *ppOffsetList = (*pReader).pOffsetList;
            *pnOffsetList = (*pReader).nOffsetList - 1 as ::core::ffi::c_int;
        }
        sqlite3Fts3DoclistPrev(
            0 as ::core::ffi::c_int,
            (*pReader).aDoclist,
            (*pReader).nDoclist,
            &raw mut p,
            &raw mut (*pReader).iDocid,
            &raw mut (*pReader).nOffsetList,
            &raw mut bEof,
        );
        if bEof != 0 {
            (*pReader).pOffsetList = ::core::ptr::null_mut::<::core::ffi::c_char>();
        } else {
            (*pReader).pOffsetList = p;
        }
    } else {
        let mut pEnd: *mut ::core::ffi::c_char =
            (*pReader).aDoclist.offset((*pReader).nDoclist as isize) as *mut ::core::ffi::c_char;
        loop {
            while *p as ::core::ffi::c_int | c as ::core::ffi::c_int != 0 {
                let fresh10 = p;
                p = p.offset(1);
                c = (*fresh10 as ::core::ffi::c_int & 0x80 as ::core::ffi::c_int)
                    as ::core::ffi::c_char;
            }
            if (*pReader).pBlob.is_null()
                || p < (*pReader).aNode.offset((*pReader).nPopulate as isize)
                    as *mut ::core::ffi::c_char
            {
                break;
            }
            rc = fts3SegReaderIncrRead(pReader);
            if rc != SQLITE_OK {
                return rc;
            }
        }
        p = p.offset(1);
        if !ppOffsetList.is_null() {
            *ppOffsetList = (*pReader).pOffsetList;
            *pnOffsetList = (p.offset_from((*pReader).pOffsetList) as ::core::ffi::c_long
                - 1 as ::core::ffi::c_long) as ::core::ffi::c_int;
        }
        while p < pEnd && *p as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            p = p.offset(1);
        }
        if p >= pEnd {
            (*pReader).pOffsetList = ::core::ptr::null_mut::<::core::ffi::c_char>();
        } else {
            rc = fts3SegReaderRequire(pReader, p, FTS3_VARINT_MAX);
            if rc == SQLITE_OK {
                let mut iDelta: u64_0 = 0;
                (*pReader).pOffsetList =
                    p.offset(sqlite3Fts3GetVarintU(p, &raw mut iDelta) as isize);
                if (*pTab).bDescIdx != 0 {
                    (*pReader).iDocid =
                        ((*pReader).iDocid as u64_0).wrapping_sub(iDelta) as i64_0 as sqlite3_int64;
                } else {
                    (*pReader).iDocid =
                        ((*pReader).iDocid as u64_0).wrapping_add(iDelta) as i64_0 as sqlite3_int64;
                }
            }
        }
    }
    return rc;
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3Fts3MsrOvfl(
    mut pCsr: *mut Fts3Cursor,
    mut pMsr: *mut Fts3MultiSegReader,
    mut pnOvfl: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut p: *mut Fts3Table = (*pCsr).base.pVtab as *mut Fts3Table;
    let mut nOvfl: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut ii: ::core::ffi::c_int = 0;
    let mut rc: ::core::ffi::c_int = SQLITE_OK;
    let mut pgsz: ::core::ffi::c_int = (*p).nPgsz;
    ii = 0 as ::core::ffi::c_int;
    while rc == SQLITE_OK && ii < (*pMsr).nSegment {
        let mut pReader: *mut Fts3SegReader = *(*pMsr).apSegment.offset(ii as isize);
        if (*pReader).ppNextElem.is_null()
            && !((*pReader).rootOnly as ::core::ffi::c_int != 0 as ::core::ffi::c_int)
        {
            let mut jj: sqlite3_int64 = 0;
            jj = (*pReader).iStartBlock;
            while jj <= (*pReader).iLeafEndBlock {
                let mut nBlob: ::core::ffi::c_int = 0;
                rc = sqlite3Fts3ReadBlock(
                    p,
                    jj,
                    ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
                    &raw mut nBlob,
                    ::core::ptr::null_mut::<::core::ffi::c_int>(),
                );
                if rc != SQLITE_OK {
                    break;
                }
                if nBlob + 35 as ::core::ffi::c_int > pgsz {
                    nOvfl += (nBlob + 34 as ::core::ffi::c_int) / pgsz;
                }
                jj += 1;
            }
        }
        ii += 1;
    }
    *pnOvfl = nOvfl;
    return rc;
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3Fts3SegReaderFree(mut pReader: *mut Fts3SegReader) {
    if !pReader.is_null() {
        sqlite3_free((*pReader).zTerm as *mut ::core::ffi::c_void);
        if !((*pReader).rootOnly as ::core::ffi::c_int != 0 as ::core::ffi::c_int) {
            sqlite3_free((*pReader).aNode as *mut ::core::ffi::c_void);
        }
        sqlite3_blob_close((*pReader).pBlob);
    }
    sqlite3_free(pReader as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3Fts3SegReaderNew(
    mut iAge: ::core::ffi::c_int,
    mut bLookup: ::core::ffi::c_int,
    mut iStartLeaf: sqlite3_int64,
    mut iEndLeaf: sqlite3_int64,
    mut iEndBlock: sqlite3_int64,
    mut zRoot: *const ::core::ffi::c_char,
    mut nRoot: ::core::ffi::c_int,
    mut ppReader: *mut *mut Fts3SegReader,
) -> ::core::ffi::c_int {
    let mut pReader: *mut Fts3SegReader = ::core::ptr::null_mut::<Fts3SegReader>();
    let mut nExtra: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if iStartLeaf == 0 as sqlite3_int64 {
        if iEndLeaf != 0 as sqlite3_int64 {
            return FTS_CORRUPT_VTAB;
        }
        nExtra = nRoot + FTS3_NODE_PADDING;
    }
    pReader = sqlite3_malloc64(
        (::core::mem::size_of::<Fts3SegReader>() as usize).wrapping_add(nExtra as usize)
            as sqlite3_uint64,
    ) as *mut Fts3SegReader;
    if pReader.is_null() {
        return SQLITE_NOMEM;
    }
    memset(
        pReader as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<Fts3SegReader>() as size_t,
    );
    (*pReader).iIdx = iAge;
    (*pReader).bLookup = (bLookup != 0 as ::core::ffi::c_int) as ::core::ffi::c_int as u8_0;
    (*pReader).iStartBlock = iStartLeaf;
    (*pReader).iLeafEndBlock = iEndLeaf;
    (*pReader).iEndBlock = iEndBlock;
    if nExtra != 0 {
        (*pReader).aNode = pReader.offset(1 as ::core::ffi::c_int as isize) as *mut Fts3SegReader
            as *mut ::core::ffi::c_char;
        (*pReader).rootOnly = 1 as u8_0;
        (*pReader).nNode = nRoot;
        if nRoot != 0 {
            memcpy(
                (*pReader).aNode as *mut ::core::ffi::c_void,
                zRoot as *const ::core::ffi::c_void,
                nRoot as size_t,
            );
        }
        memset(
            (*pReader).aNode.offset(nRoot as isize) as *mut ::core::ffi::c_char
                as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            FTS3_NODE_PADDING as size_t,
        );
    } else {
        (*pReader).iCurrentBlock = iStartLeaf - 1 as sqlite3_int64;
    }
    *ppReader = pReader;
    return SQLITE_OK;
}
unsafe extern "C" fn fts3CompareElemByTerm(
    mut lhs: *const ::core::ffi::c_void,
    mut rhs: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut z1: *mut ::core::ffi::c_char =
        (**(lhs as *mut *mut Fts3HashElem)).pKey as *mut ::core::ffi::c_char;
    let mut z2: *mut ::core::ffi::c_char =
        (**(rhs as *mut *mut Fts3HashElem)).pKey as *mut ::core::ffi::c_char;
    let mut n1: ::core::ffi::c_int = (**(lhs as *mut *mut Fts3HashElem)).nKey;
    let mut n2: ::core::ffi::c_int = (**(rhs as *mut *mut Fts3HashElem)).nKey;
    let mut n: ::core::ffi::c_int = if n1 < n2 { n1 } else { n2 };
    let mut c: ::core::ffi::c_int = memcmp(
        z1 as *const ::core::ffi::c_void,
        z2 as *const ::core::ffi::c_void,
        n as size_t,
    );
    if c == 0 as ::core::ffi::c_int {
        c = n1 - n2;
    }
    return c;
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3Fts3SegReaderPending(
    mut p: *mut Fts3Table,
    mut iIndex: ::core::ffi::c_int,
    mut zTerm: *const ::core::ffi::c_char,
    mut nTerm: ::core::ffi::c_int,
    mut bPrefix: ::core::ffi::c_int,
    mut ppReader: *mut *mut Fts3SegReader,
) -> ::core::ffi::c_int {
    let mut pReader: *mut Fts3SegReader = ::core::ptr::null_mut::<Fts3SegReader>();
    let mut pE: *mut Fts3HashElem = ::core::ptr::null_mut::<Fts3HashElem>();
    let mut aElem: *mut *mut Fts3HashElem = ::core::ptr::null_mut::<*mut Fts3HashElem>();
    let mut nElem: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut rc: ::core::ffi::c_int = SQLITE_OK;
    let mut pHash: *mut Fts3Hash = ::core::ptr::null_mut::<Fts3Hash>();
    pHash = &raw mut (*(*p).aIndex.offset(iIndex as isize)).hPending;
    if bPrefix != 0 {
        let mut nAlloc: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        pE = (*pHash).first;
        while !pE.is_null() {
            let mut zKey: *mut ::core::ffi::c_char = (*pE).pKey as *mut ::core::ffi::c_char;
            let mut nKey: ::core::ffi::c_int = (*pE).nKey;
            if nTerm == 0 as ::core::ffi::c_int
                || nKey >= nTerm
                    && 0 as ::core::ffi::c_int
                        == memcmp(
                            zKey as *const ::core::ffi::c_void,
                            zTerm as *const ::core::ffi::c_void,
                            nTerm as size_t,
                        )
            {
                if nElem == nAlloc {
                    let mut aElem2: *mut *mut Fts3HashElem =
                        ::core::ptr::null_mut::<*mut Fts3HashElem>();
                    nAlloc += 16 as ::core::ffi::c_int;
                    aElem2 = sqlite3_realloc64(
                        aElem as *mut ::core::ffi::c_void,
                        (nAlloc as usize)
                            .wrapping_mul(::core::mem::size_of::<*mut Fts3HashElem>() as usize)
                            as sqlite3_uint64,
                    ) as *mut *mut Fts3HashElem;
                    if aElem2.is_null() {
                        rc = SQLITE_NOMEM;
                        nElem = 0 as ::core::ffi::c_int;
                        break;
                    } else {
                        aElem = aElem2;
                    }
                }
                let fresh16 = nElem;
                nElem = nElem + 1;
                let ref mut fresh17 = *aElem.offset(fresh16 as isize);
                *fresh17 = pE;
            }
            pE = (*pE).next;
        }
        if nElem > 1 as ::core::ffi::c_int {
            qsort(
                aElem as *mut ::core::ffi::c_void,
                nElem as size_t,
                ::core::mem::size_of::<*mut Fts3HashElem>() as size_t,
                Some(
                    fts3CompareElemByTerm
                        as unsafe extern "C" fn(
                            *const ::core::ffi::c_void,
                            *const ::core::ffi::c_void,
                        ) -> ::core::ffi::c_int,
                ),
            );
        }
    } else {
        pE = sqlite3Fts3HashFindElem(pHash, zTerm as *const ::core::ffi::c_void, nTerm);
        if !pE.is_null() {
            aElem = &raw mut pE;
            nElem = 1 as ::core::ffi::c_int;
        }
    }
    if nElem > 0 as ::core::ffi::c_int {
        let mut nByte: sqlite3_int64 = 0;
        nByte = (::core::mem::size_of::<Fts3SegReader>() as usize).wrapping_add(
            ((nElem + 1 as ::core::ffi::c_int) as usize)
                .wrapping_mul(::core::mem::size_of::<*mut Fts3HashElem>() as usize),
        ) as sqlite3_int64;
        pReader = sqlite3_malloc64(nByte as sqlite3_uint64) as *mut Fts3SegReader;
        if pReader.is_null() {
            rc = SQLITE_NOMEM;
        } else {
            memset(
                pReader as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                nByte as size_t,
            );
            (*pReader).iIdx = 0x7fffffff as ::core::ffi::c_int;
            (*pReader).ppNextElem = pReader.offset(1 as ::core::ffi::c_int as isize)
                as *mut Fts3SegReader as *mut *mut Fts3HashElem;
            memcpy(
                (*pReader).ppNextElem as *mut ::core::ffi::c_void,
                aElem as *const ::core::ffi::c_void,
                (nElem as size_t)
                    .wrapping_mul(::core::mem::size_of::<*mut Fts3HashElem>() as size_t),
            );
        }
    }
    if bPrefix != 0 {
        sqlite3_free(aElem as *mut ::core::ffi::c_void);
    }
    *ppReader = pReader;
    return rc;
}
unsafe extern "C" fn fts3SegReaderCmp(
    mut pLhs: *mut Fts3SegReader,
    mut pRhs: *mut Fts3SegReader,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = 0;
    if !(*pLhs).aNode.is_null() && !(*pRhs).aNode.is_null() {
        let mut rc2: ::core::ffi::c_int = (*pLhs).nTerm - (*pRhs).nTerm;
        if rc2 < 0 as ::core::ffi::c_int {
            rc = memcmp(
                (*pLhs).zTerm as *const ::core::ffi::c_void,
                (*pRhs).zTerm as *const ::core::ffi::c_void,
                (*pLhs).nTerm as size_t,
            );
        } else {
            rc = memcmp(
                (*pLhs).zTerm as *const ::core::ffi::c_void,
                (*pRhs).zTerm as *const ::core::ffi::c_void,
                (*pRhs).nTerm as size_t,
            );
        }
        if rc == 0 as ::core::ffi::c_int {
            rc = rc2;
        }
    } else {
        rc = ((*pLhs).aNode == ::core::ptr::null_mut::<::core::ffi::c_char>())
            as ::core::ffi::c_int
            - ((*pRhs).aNode == ::core::ptr::null_mut::<::core::ffi::c_char>())
                as ::core::ffi::c_int;
    }
    if rc == 0 as ::core::ffi::c_int {
        rc = (*pRhs).iIdx - (*pLhs).iIdx;
    }
    return rc;
}
unsafe extern "C" fn fts3SegReaderDoclistCmp(
    mut pLhs: *mut Fts3SegReader,
    mut pRhs: *mut Fts3SegReader,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = ((*pLhs).pOffsetList
        == ::core::ptr::null_mut::<::core::ffi::c_char>())
        as ::core::ffi::c_int
        - ((*pRhs).pOffsetList == ::core::ptr::null_mut::<::core::ffi::c_char>())
            as ::core::ffi::c_int;
    if rc == 0 as ::core::ffi::c_int {
        if (*pLhs).iDocid == (*pRhs).iDocid {
            rc = (*pRhs).iIdx - (*pLhs).iIdx;
        } else {
            rc = if (*pLhs).iDocid > (*pRhs).iDocid {
                1 as ::core::ffi::c_int
            } else {
                -(1 as ::core::ffi::c_int)
            };
        }
    }
    return rc;
}
unsafe extern "C" fn fts3SegReaderDoclistCmpRev(
    mut pLhs: *mut Fts3SegReader,
    mut pRhs: *mut Fts3SegReader,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = ((*pLhs).pOffsetList
        == ::core::ptr::null_mut::<::core::ffi::c_char>())
        as ::core::ffi::c_int
        - ((*pRhs).pOffsetList == ::core::ptr::null_mut::<::core::ffi::c_char>())
            as ::core::ffi::c_int;
    if rc == 0 as ::core::ffi::c_int {
        if (*pLhs).iDocid == (*pRhs).iDocid {
            rc = (*pRhs).iIdx - (*pLhs).iIdx;
        } else {
            rc = if (*pLhs).iDocid < (*pRhs).iDocid {
                1 as ::core::ffi::c_int
            } else {
                -(1 as ::core::ffi::c_int)
            };
        }
    }
    return rc;
}
unsafe extern "C" fn fts3SegReaderTermCmp(
    mut pSeg: *mut Fts3SegReader,
    mut zTerm: *const ::core::ffi::c_char,
    mut nTerm: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut res: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if !(*pSeg).aNode.is_null() {
        if (*pSeg).nTerm > nTerm {
            res = memcmp(
                (*pSeg).zTerm as *const ::core::ffi::c_void,
                zTerm as *const ::core::ffi::c_void,
                nTerm as size_t,
            );
        } else {
            res = memcmp(
                (*pSeg).zTerm as *const ::core::ffi::c_void,
                zTerm as *const ::core::ffi::c_void,
                (*pSeg).nTerm as size_t,
            );
        }
        if res == 0 as ::core::ffi::c_int {
            res = (*pSeg).nTerm - nTerm;
        }
    }
    return res;
}
unsafe extern "C" fn fts3SegReaderSort(
    mut apSegment: *mut *mut Fts3SegReader,
    mut nSegment: ::core::ffi::c_int,
    mut nSuspect: ::core::ffi::c_int,
    mut xCmp: Option<
        unsafe extern "C" fn(*mut Fts3SegReader, *mut Fts3SegReader) -> ::core::ffi::c_int,
    >,
) {
    let mut i: ::core::ffi::c_int = 0;
    if nSuspect == nSegment {
        nSuspect -= 1;
    }
    i = nSuspect - 1 as ::core::ffi::c_int;
    while i >= 0 as ::core::ffi::c_int {
        let mut j: ::core::ffi::c_int = 0;
        j = i;
        while j < nSegment - 1 as ::core::ffi::c_int {
            let mut pTmp: *mut Fts3SegReader = ::core::ptr::null_mut::<Fts3SegReader>();
            if xCmp.expect("non-null function pointer")(
                *apSegment.offset(j as isize),
                *apSegment.offset((j + 1 as ::core::ffi::c_int) as isize),
            ) < 0 as ::core::ffi::c_int
            {
                break;
            }
            pTmp = *apSegment.offset((j + 1 as ::core::ffi::c_int) as isize);
            let ref mut fresh7 = *apSegment.offset((j + 1 as ::core::ffi::c_int) as isize);
            *fresh7 = *apSegment.offset(j as isize);
            let ref mut fresh8 = *apSegment.offset(j as isize);
            *fresh8 = pTmp;
            j += 1;
        }
        i -= 1;
    }
}
unsafe extern "C" fn fts3WriteSegment(
    mut p: *mut Fts3Table,
    mut iBlock: sqlite3_int64,
    mut z: *mut ::core::ffi::c_char,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut pStmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut rc: ::core::ffi::c_int = fts3SqlStmt(
        p,
        SQL_INSERT_SEGMENTS,
        &raw mut pStmt,
        ::core::ptr::null_mut::<*mut sqlite3_value>(),
    );
    if rc == SQLITE_OK {
        sqlite3_bind_int64(pStmt, 1 as ::core::ffi::c_int, iBlock);
        sqlite3_bind_blob(
            pStmt,
            2 as ::core::ffi::c_int,
            z as *const ::core::ffi::c_void,
            n,
            SQLITE_STATIC,
        );
        sqlite3_step(pStmt);
        rc = sqlite3_reset(pStmt);
        sqlite3_bind_null(pStmt, 2 as ::core::ffi::c_int);
    }
    return rc;
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3Fts3MaxLevel(
    mut p: *mut Fts3Table,
    mut pnMax: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = 0;
    let mut mxLevel: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut pStmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    rc = fts3SqlStmt(
        p,
        SQL_SELECT_MXLEVEL,
        &raw mut pStmt,
        ::core::ptr::null_mut::<*mut sqlite3_value>(),
    );
    if rc == SQLITE_OK {
        if SQLITE_ROW == sqlite3_step(pStmt) {
            mxLevel = sqlite3_column_int(pStmt, 0 as ::core::ffi::c_int);
        }
        rc = sqlite3_reset(pStmt);
    }
    *pnMax = mxLevel;
    return rc;
}
unsafe extern "C" fn fts3WriteSegdir(
    mut p: *mut Fts3Table,
    mut iLevel: sqlite3_int64,
    mut iIdx: ::core::ffi::c_int,
    mut iStartBlock: sqlite3_int64,
    mut iLeafEndBlock: sqlite3_int64,
    mut iEndBlock: sqlite3_int64,
    mut nLeafData: sqlite3_int64,
    mut zRoot: *mut ::core::ffi::c_char,
    mut nRoot: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut pStmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut rc: ::core::ffi::c_int = fts3SqlStmt(
        p,
        SQL_INSERT_SEGDIR,
        &raw mut pStmt,
        ::core::ptr::null_mut::<*mut sqlite3_value>(),
    );
    if rc == SQLITE_OK {
        sqlite3_bind_int64(pStmt, 1 as ::core::ffi::c_int, iLevel);
        sqlite3_bind_int(pStmt, 2 as ::core::ffi::c_int, iIdx);
        sqlite3_bind_int64(pStmt, 3 as ::core::ffi::c_int, iStartBlock);
        sqlite3_bind_int64(pStmt, 4 as ::core::ffi::c_int, iLeafEndBlock);
        if nLeafData == 0 as sqlite3_int64 {
            sqlite3_bind_int64(pStmt, 5 as ::core::ffi::c_int, iEndBlock);
        } else {
            let mut zEnd: *mut ::core::ffi::c_char = sqlite3_mprintf(
                b"%lld %lld\0" as *const u8 as *const ::core::ffi::c_char,
                iEndBlock,
                nLeafData,
            );
            if zEnd.is_null() {
                return SQLITE_NOMEM;
            }
            sqlite3_bind_text(
                pStmt,
                5 as ::core::ffi::c_int,
                zEnd,
                -(1 as ::core::ffi::c_int),
                Some(sqlite3_free as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()),
            );
        }
        sqlite3_bind_blob(
            pStmt,
            6 as ::core::ffi::c_int,
            zRoot as *const ::core::ffi::c_void,
            nRoot,
            SQLITE_STATIC,
        );
        sqlite3_step(pStmt);
        rc = sqlite3_reset(pStmt);
        sqlite3_bind_null(pStmt, 6 as ::core::ffi::c_int);
    }
    return rc;
}
unsafe extern "C" fn fts3PrefixCompress(
    mut zPrev: *const ::core::ffi::c_char,
    mut nPrev: ::core::ffi::c_int,
    mut zNext: *const ::core::ffi::c_char,
    mut nNext: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut n: ::core::ffi::c_int = 0;
    n = 0 as ::core::ffi::c_int;
    while n < nPrev
        && n < nNext
        && *zPrev.offset(n as isize) as ::core::ffi::c_int
            == *zNext.offset(n as isize) as ::core::ffi::c_int
    {
        n += 1;
    }
    return n;
}
unsafe extern "C" fn fts3NodeAddTerm(
    mut p: *mut Fts3Table,
    mut ppTree: *mut *mut SegmentNode,
    mut isCopyTerm: ::core::ffi::c_int,
    mut zTerm: *const ::core::ffi::c_char,
    mut nTerm: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut pTree: *mut SegmentNode = *ppTree;
    let mut rc: ::core::ffi::c_int = 0;
    let mut pNew: *mut SegmentNode = ::core::ptr::null_mut::<SegmentNode>();
    if !pTree.is_null() {
        let mut nData: ::core::ffi::c_int = (*pTree).nData;
        let mut nReq: ::core::ffi::c_int = nData;
        let mut nPrefix: ::core::ffi::c_int = 0;
        let mut nSuffix: ::core::ffi::c_int = 0;
        nPrefix = fts3PrefixCompress((*pTree).zTerm, (*pTree).nTerm, zTerm, nTerm);
        nSuffix = nTerm - nPrefix;
        if nSuffix <= 0 as ::core::ffi::c_int {
            return FTS_CORRUPT_VTAB;
        }
        nReq += sqlite3Fts3VarintLen(nPrefix as sqlite3_uint64)
            + sqlite3Fts3VarintLen(nSuffix as sqlite3_uint64)
            + nSuffix;
        if nReq <= (*p).nNodeSize || (*pTree).zTerm.is_null() {
            if nReq > (*p).nNodeSize {
                (*pTree).aData =
                    sqlite3_malloc64(nReq as sqlite3_uint64) as *mut ::core::ffi::c_char;
                if (*pTree).aData.is_null() {
                    return SQLITE_NOMEM;
                }
            }
            if !(*pTree).zTerm.is_null() {
                nData += sqlite3Fts3PutVarint(
                    (*pTree).aData.offset(nData as isize) as *mut ::core::ffi::c_char,
                    nPrefix as sqlite3_int64,
                );
            }
            nData += sqlite3Fts3PutVarint(
                (*pTree).aData.offset(nData as isize) as *mut ::core::ffi::c_char,
                nSuffix as sqlite3_int64,
            );
            memcpy(
                (*pTree).aData.offset(nData as isize) as *mut ::core::ffi::c_char
                    as *mut ::core::ffi::c_void,
                zTerm.offset(nPrefix as isize) as *const ::core::ffi::c_char
                    as *const ::core::ffi::c_void,
                nSuffix as size_t,
            );
            (*pTree).nData = nData + nSuffix;
            (*pTree).nEntry += 1;
            if isCopyTerm != 0 {
                if (*pTree).nMalloc < nTerm {
                    let mut zNew: *mut ::core::ffi::c_char = sqlite3_realloc64(
                        (*pTree).zMalloc as *mut ::core::ffi::c_void,
                        (nTerm as i64_0 * 2 as i64_0) as sqlite3_uint64,
                    )
                        as *mut ::core::ffi::c_char;
                    if zNew.is_null() {
                        return SQLITE_NOMEM;
                    }
                    (*pTree).nMalloc = nTerm * 2 as ::core::ffi::c_int;
                    (*pTree).zMalloc = zNew;
                }
                (*pTree).zTerm = (*pTree).zMalloc;
                memcpy(
                    (*pTree).zTerm as *mut ::core::ffi::c_void,
                    zTerm as *const ::core::ffi::c_void,
                    nTerm as size_t,
                );
                (*pTree).nTerm = nTerm;
            } else {
                (*pTree).zTerm = zTerm as *mut ::core::ffi::c_char;
                (*pTree).nTerm = nTerm;
            }
            return SQLITE_OK;
        }
    }
    pNew = sqlite3_malloc64(
        (::core::mem::size_of::<SegmentNode>() as usize).wrapping_add((*p).nNodeSize as usize)
            as sqlite3_uint64,
    ) as *mut SegmentNode;
    if pNew.is_null() {
        return SQLITE_NOMEM;
    }
    memset(
        pNew as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<SegmentNode>() as size_t,
    );
    (*pNew).nData = 1 as ::core::ffi::c_int + FTS3_VARINT_MAX;
    (*pNew).aData = pNew.offset(1 as ::core::ffi::c_int as isize) as *mut SegmentNode
        as *mut ::core::ffi::c_char;
    if !pTree.is_null() {
        let mut pParent: *mut SegmentNode = (*pTree).pParent;
        rc = fts3NodeAddTerm(p, &raw mut pParent, isCopyTerm, zTerm, nTerm);
        if (*pTree).pParent.is_null() {
            (*pTree).pParent = pParent;
        }
        (*pTree).pRight = pNew;
        (*pNew).pLeftmost = (*pTree).pLeftmost;
        (*pNew).pParent = pParent;
        (*pNew).zMalloc = (*pTree).zMalloc;
        (*pNew).nMalloc = (*pTree).nMalloc;
        (*pTree).zMalloc = ::core::ptr::null_mut::<::core::ffi::c_char>();
    } else {
        (*pNew).pLeftmost = pNew;
        rc = fts3NodeAddTerm(p, &raw mut pNew, isCopyTerm, zTerm, nTerm);
    }
    *ppTree = pNew;
    return rc;
}
unsafe extern "C" fn fts3TreeFinishNode(
    mut pTree: *mut SegmentNode,
    mut iHeight: ::core::ffi::c_int,
    mut iLeftChild: sqlite3_int64,
) -> ::core::ffi::c_int {
    let mut nStart: ::core::ffi::c_int = 0;
    nStart = FTS3_VARINT_MAX - sqlite3Fts3VarintLen(iLeftChild as sqlite3_uint64);
    *(*pTree).aData.offset(nStart as isize) = iHeight as ::core::ffi::c_char;
    sqlite3Fts3PutVarint(
        (*pTree)
            .aData
            .offset((nStart + 1 as ::core::ffi::c_int) as isize)
            as *mut ::core::ffi::c_char,
        iLeftChild,
    );
    return nStart;
}
unsafe extern "C" fn fts3NodeWrite(
    mut p: *mut Fts3Table,
    mut pTree: *mut SegmentNode,
    mut iHeight: ::core::ffi::c_int,
    mut iLeaf: sqlite3_int64,
    mut iFree: sqlite3_int64,
    mut piLast: *mut sqlite3_int64,
    mut paRoot: *mut *mut ::core::ffi::c_char,
    mut pnRoot: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = SQLITE_OK;
    if (*pTree).pParent.is_null() {
        let mut nStart: ::core::ffi::c_int = fts3TreeFinishNode(pTree, iHeight, iLeaf);
        *piLast = iFree - 1 as sqlite3_int64;
        *pnRoot = (*pTree).nData - nStart;
        *paRoot = (*pTree).aData.offset(nStart as isize) as *mut ::core::ffi::c_char;
    } else {
        let mut pIter: *mut SegmentNode = ::core::ptr::null_mut::<SegmentNode>();
        let mut iNextFree: sqlite3_int64 = iFree;
        let mut iNextLeaf: sqlite3_int64 = iLeaf;
        pIter = (*pTree).pLeftmost;
        while !pIter.is_null() && rc == SQLITE_OK {
            let mut nStart_0: ::core::ffi::c_int = fts3TreeFinishNode(pIter, iHeight, iNextLeaf);
            let mut nWrite: ::core::ffi::c_int = (*pIter).nData - nStart_0;
            rc = fts3WriteSegment(
                p,
                iNextFree,
                (*pIter).aData.offset(nStart_0 as isize) as *mut ::core::ffi::c_char,
                nWrite,
            );
            iNextFree += 1;
            iNextLeaf += ((*pIter).nEntry + 1 as ::core::ffi::c_int) as sqlite3_int64;
            pIter = (*pIter).pRight;
        }
        if rc == SQLITE_OK {
            rc = fts3NodeWrite(
                p,
                (*pTree).pParent,
                iHeight + 1 as ::core::ffi::c_int,
                iFree,
                iNextFree,
                piLast,
                paRoot,
                pnRoot,
            );
        }
    }
    return rc;
}
unsafe extern "C" fn fts3NodeFree(mut pTree: *mut SegmentNode) {
    if !pTree.is_null() {
        let mut p: *mut SegmentNode = (*pTree).pLeftmost;
        fts3NodeFree((*p).pParent);
        while !p.is_null() {
            let mut pRight: *mut SegmentNode = (*p).pRight;
            if (*p).aData
                != p.offset(1 as ::core::ffi::c_int as isize) as *mut SegmentNode
                    as *mut ::core::ffi::c_char
            {
                sqlite3_free((*p).aData as *mut ::core::ffi::c_void);
            }
            sqlite3_free((*p).zMalloc as *mut ::core::ffi::c_void);
            sqlite3_free(p as *mut ::core::ffi::c_void);
            p = pRight;
        }
    }
}
unsafe extern "C" fn fts3SegWriterAdd(
    mut p: *mut Fts3Table,
    mut ppWriter: *mut *mut SegmentWriter,
    mut isCopyTerm: ::core::ffi::c_int,
    mut zTerm: *const ::core::ffi::c_char,
    mut nTerm: ::core::ffi::c_int,
    mut aDoclist: *const ::core::ffi::c_char,
    mut nDoclist: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut nPrefix: ::core::ffi::c_int = 0;
    let mut nSuffix: ::core::ffi::c_int = 0;
    let mut nReq: i64_0 = 0;
    let mut nData: ::core::ffi::c_int = 0;
    let mut pWriter: *mut SegmentWriter = *ppWriter;
    if pWriter.is_null() {
        let mut rc: ::core::ffi::c_int = 0;
        let mut pStmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
        pWriter = sqlite3_malloc64(::core::mem::size_of::<SegmentWriter>() as sqlite3_uint64)
            as *mut SegmentWriter;
        if pWriter.is_null() {
            return SQLITE_NOMEM;
        }
        memset(
            pWriter as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<SegmentWriter>() as size_t,
        );
        *ppWriter = pWriter;
        (*pWriter).aData =
            sqlite3_malloc64((*p).nNodeSize as sqlite3_uint64) as *mut ::core::ffi::c_char;
        if (*pWriter).aData.is_null() {
            return SQLITE_NOMEM;
        }
        (*pWriter).nSize = (*p).nNodeSize;
        rc = fts3SqlStmt(
            p,
            SQL_NEXT_SEGMENTS_ID,
            &raw mut pStmt,
            ::core::ptr::null_mut::<*mut sqlite3_value>(),
        );
        if rc != SQLITE_OK {
            return rc;
        }
        if SQLITE_ROW == sqlite3_step(pStmt) {
            (*pWriter).iFree = sqlite3_column_int64(pStmt, 0 as ::core::ffi::c_int);
            (*pWriter).iFirst = (*pWriter).iFree;
        }
        rc = sqlite3_reset(pStmt);
        if rc != SQLITE_OK {
            return rc;
        }
    }
    nData = (*pWriter).nData;
    nPrefix = fts3PrefixCompress((*pWriter).zTerm, (*pWriter).nTerm, zTerm, nTerm);
    nSuffix = nTerm - nPrefix;
    if nSuffix <= 0 as ::core::ffi::c_int {
        return FTS_CORRUPT_VTAB;
    }
    nReq = (sqlite3Fts3VarintLen(nPrefix as sqlite3_uint64)
        + sqlite3Fts3VarintLen(nSuffix as sqlite3_uint64)
        + nSuffix
        + sqlite3Fts3VarintLen(nDoclist as sqlite3_uint64)
        + nDoclist) as i64_0;
    if nData > 0 as ::core::ffi::c_int && nData as i64_0 + nReq > (*p).nNodeSize as i64_0 {
        let mut rc_0: ::core::ffi::c_int = 0;
        if (*pWriter).iFree == LARGEST_INT64 {
            return FTS_CORRUPT_VTAB;
        }
        let fresh5 = (*pWriter).iFree;
        (*pWriter).iFree = (*pWriter).iFree + 1;
        rc_0 = fts3WriteSegment(p, fresh5, (*pWriter).aData, nData);
        if rc_0 != SQLITE_OK {
            return rc_0;
        }
        (*p).nLeafAdd = (*p).nLeafAdd.wrapping_add(1);
        rc_0 = fts3NodeAddTerm(
            p,
            &raw mut (*pWriter).pTree,
            isCopyTerm,
            zTerm,
            nPrefix + 1 as ::core::ffi::c_int,
        );
        if rc_0 != SQLITE_OK {
            return rc_0;
        }
        nData = 0 as ::core::ffi::c_int;
        (*pWriter).nTerm = 0 as ::core::ffi::c_int;
        nPrefix = 0 as ::core::ffi::c_int;
        nSuffix = nTerm;
        nReq = (1 as ::core::ffi::c_int
            + sqlite3Fts3VarintLen(nTerm as sqlite3_uint64)
            + nTerm
            + sqlite3Fts3VarintLen(nDoclist as sqlite3_uint64)
            + nDoclist) as i64_0;
    }
    (*pWriter).nLeafData += nReq;
    if nReq > (*pWriter).nSize as i64_0 {
        let mut aNew: *mut ::core::ffi::c_char = sqlite3_realloc64(
            (*pWriter).aData as *mut ::core::ffi::c_void,
            nReq as sqlite3_uint64,
        ) as *mut ::core::ffi::c_char;
        if aNew.is_null() {
            return SQLITE_NOMEM;
        }
        (*pWriter).aData = aNew;
        (*pWriter).nSize = nReq as ::core::ffi::c_int;
    }
    nData += sqlite3Fts3PutVarint(
        (*pWriter).aData.offset(nData as isize) as *mut ::core::ffi::c_char,
        nPrefix as sqlite3_int64,
    );
    nData += sqlite3Fts3PutVarint(
        (*pWriter).aData.offset(nData as isize) as *mut ::core::ffi::c_char,
        nSuffix as sqlite3_int64,
    );
    memcpy(
        (*pWriter).aData.offset(nData as isize) as *mut ::core::ffi::c_char
            as *mut ::core::ffi::c_void,
        zTerm.offset(nPrefix as isize) as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
        nSuffix as size_t,
    );
    nData += nSuffix;
    nData += sqlite3Fts3PutVarint(
        (*pWriter).aData.offset(nData as isize) as *mut ::core::ffi::c_char,
        nDoclist as sqlite3_int64,
    );
    memcpy(
        (*pWriter).aData.offset(nData as isize) as *mut ::core::ffi::c_char
            as *mut ::core::ffi::c_void,
        aDoclist as *const ::core::ffi::c_void,
        nDoclist as size_t,
    );
    (*pWriter).nData = nData + nDoclist;
    if isCopyTerm != 0 {
        if nTerm > (*pWriter).nMalloc {
            let mut zNew: *mut ::core::ffi::c_char = sqlite3_realloc64(
                (*pWriter).zMalloc as *mut ::core::ffi::c_void,
                (nTerm as i64_0 * 2 as i64_0) as sqlite3_uint64,
            ) as *mut ::core::ffi::c_char;
            if zNew.is_null() {
                return SQLITE_NOMEM;
            }
            (*pWriter).nMalloc = nTerm * 2 as ::core::ffi::c_int;
            (*pWriter).zMalloc = zNew;
            (*pWriter).zTerm = zNew;
        }
        memcpy(
            (*pWriter).zTerm as *mut ::core::ffi::c_void,
            zTerm as *const ::core::ffi::c_void,
            nTerm as size_t,
        );
    } else {
        (*pWriter).zTerm = zTerm as *mut ::core::ffi::c_char;
    }
    (*pWriter).nTerm = nTerm;
    return SQLITE_OK;
}
unsafe extern "C" fn fts3SegWriterFlush(
    mut p: *mut Fts3Table,
    mut pWriter: *mut SegmentWriter,
    mut iLevel: sqlite3_int64,
    mut iIdx: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = 0;
    if !(*pWriter).pTree.is_null() {
        let mut iLast: sqlite3_int64 = 0 as sqlite3_int64;
        let mut iLastLeaf: sqlite3_int64 = 0;
        let mut zRoot: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut nRoot: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        iLastLeaf = (*pWriter).iFree;
        let fresh4 = (*pWriter).iFree;
        (*pWriter).iFree = (*pWriter).iFree + 1;
        rc = fts3WriteSegment(p, fresh4, (*pWriter).aData, (*pWriter).nData);
        if rc == SQLITE_OK {
            rc = fts3NodeWrite(
                p,
                (*pWriter).pTree,
                1 as ::core::ffi::c_int,
                (*pWriter).iFirst,
                (*pWriter).iFree,
                &raw mut iLast,
                &raw mut zRoot,
                &raw mut nRoot,
            );
        }
        if rc == SQLITE_OK {
            rc = fts3WriteSegdir(
                p,
                iLevel,
                iIdx,
                (*pWriter).iFirst,
                iLastLeaf,
                iLast,
                (*pWriter).nLeafData as sqlite3_int64,
                zRoot,
                nRoot,
            );
        }
    } else {
        rc = fts3WriteSegdir(
            p,
            iLevel,
            iIdx,
            0 as sqlite3_int64,
            0 as sqlite3_int64,
            0 as sqlite3_int64,
            (*pWriter).nLeafData as sqlite3_int64,
            (*pWriter).aData,
            (*pWriter).nData,
        );
    }
    (*p).nLeafAdd = (*p).nLeafAdd.wrapping_add(1);
    return rc;
}
unsafe extern "C" fn fts3SegWriterFree(mut pWriter: *mut SegmentWriter) {
    if !pWriter.is_null() {
        sqlite3_free((*pWriter).aData as *mut ::core::ffi::c_void);
        sqlite3_free((*pWriter).zMalloc as *mut ::core::ffi::c_void);
        fts3NodeFree((*pWriter).pTree);
        sqlite3_free(pWriter as *mut ::core::ffi::c_void);
    }
}
unsafe extern "C" fn fts3IsEmpty(
    mut p: *mut Fts3Table,
    mut pRowid: *mut sqlite3_value,
    mut pisEmpty: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut pStmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut rc: ::core::ffi::c_int = 0;
    if !(*p).zContentTbl.is_null() {
        *pisEmpty = 0 as ::core::ffi::c_int;
        rc = SQLITE_OK;
    } else {
        rc = fts3SqlStmt(p, SQL_IS_EMPTY, &raw mut pStmt, &raw mut pRowid);
        if rc == SQLITE_OK {
            if SQLITE_ROW == sqlite3_step(pStmt) {
                *pisEmpty = sqlite3_column_int(pStmt, 0 as ::core::ffi::c_int);
            }
            rc = sqlite3_reset(pStmt);
        }
    }
    return rc;
}
unsafe extern "C" fn fts3SegmentMaxLevel(
    mut p: *mut Fts3Table,
    mut iLangid: ::core::ffi::c_int,
    mut iIndex: ::core::ffi::c_int,
    mut pnMax: *mut sqlite3_int64,
) -> ::core::ffi::c_int {
    let mut pStmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut rc: ::core::ffi::c_int = 0;
    rc = fts3SqlStmt(
        p,
        SQL_SELECT_SEGDIR_MAX_LEVEL,
        &raw mut pStmt,
        ::core::ptr::null_mut::<*mut sqlite3_value>(),
    );
    if rc != SQLITE_OK {
        return rc;
    }
    sqlite3_bind_int64(
        pStmt,
        1 as ::core::ffi::c_int,
        getAbsoluteLevel(p, iLangid, iIndex, 0 as ::core::ffi::c_int),
    );
    sqlite3_bind_int64(
        pStmt,
        2 as ::core::ffi::c_int,
        getAbsoluteLevel(
            p,
            iLangid,
            iIndex,
            FTS3_SEGDIR_MAXLEVEL - 1 as ::core::ffi::c_int,
        ),
    );
    if SQLITE_ROW == sqlite3_step(pStmt) {
        *pnMax = sqlite3_column_int64(pStmt, 0 as ::core::ffi::c_int);
    }
    return sqlite3_reset(pStmt);
}
unsafe extern "C" fn fts3SegmentIsMaxLevel(
    mut p: *mut Fts3Table,
    mut iAbsLevel: i64_0,
    mut pbMax: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut pStmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut rc: ::core::ffi::c_int = fts3SqlStmt(
        p,
        SQL_SELECT_SEGDIR_MAX_LEVEL,
        &raw mut pStmt,
        ::core::ptr::null_mut::<*mut sqlite3_value>(),
    );
    if rc != SQLITE_OK {
        return rc;
    }
    sqlite3_bind_int64(
        pStmt,
        1 as ::core::ffi::c_int,
        iAbsLevel as sqlite3_int64 + 1 as sqlite3_int64,
    );
    sqlite3_bind_int64(
        pStmt,
        2 as ::core::ffi::c_int,
        (iAbsLevel as u64_0)
            .wrapping_div(FTS3_SEGDIR_MAXLEVEL as u64_0)
            .wrapping_add(1 as u64_0)
            .wrapping_mul(FTS3_SEGDIR_MAXLEVEL as u64_0) as sqlite3_int64,
    );
    *pbMax = 0 as ::core::ffi::c_int;
    if SQLITE_ROW == sqlite3_step(pStmt) {
        *pbMax = (sqlite3_column_type(pStmt, 0 as ::core::ffi::c_int) == SQLITE_NULL)
            as ::core::ffi::c_int;
    }
    return sqlite3_reset(pStmt);
}
unsafe extern "C" fn fts3DeleteSegment(
    mut p: *mut Fts3Table,
    mut pSeg: *mut Fts3SegReader,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = SQLITE_OK;
    if (*pSeg).iStartBlock != 0 {
        let mut pDelete: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
        rc = fts3SqlStmt(
            p,
            SQL_DELETE_SEGMENTS_RANGE,
            &raw mut pDelete,
            ::core::ptr::null_mut::<*mut sqlite3_value>(),
        );
        if rc == SQLITE_OK {
            sqlite3_bind_int64(pDelete, 1 as ::core::ffi::c_int, (*pSeg).iStartBlock);
            sqlite3_bind_int64(pDelete, 2 as ::core::ffi::c_int, (*pSeg).iEndBlock);
            sqlite3_step(pDelete);
            rc = sqlite3_reset(pDelete);
        }
    }
    return rc;
}
unsafe extern "C" fn fts3DeleteSegdir(
    mut p: *mut Fts3Table,
    mut iLangid: ::core::ffi::c_int,
    mut iIndex: ::core::ffi::c_int,
    mut iLevel: ::core::ffi::c_int,
    mut apSegment: *mut *mut Fts3SegReader,
    mut nReader: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = SQLITE_OK;
    let mut i: ::core::ffi::c_int = 0;
    let mut pDelete: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    i = 0 as ::core::ffi::c_int;
    while rc == SQLITE_OK && i < nReader {
        rc = fts3DeleteSegment(p, *apSegment.offset(i as isize));
        i += 1;
    }
    if rc != SQLITE_OK {
        return rc;
    }
    if iLevel == FTS3_SEGCURSOR_ALL {
        rc = fts3SqlStmt(
            p,
            SQL_DELETE_SEGDIR_RANGE,
            &raw mut pDelete,
            ::core::ptr::null_mut::<*mut sqlite3_value>(),
        );
        if rc == SQLITE_OK {
            sqlite3_bind_int64(
                pDelete,
                1 as ::core::ffi::c_int,
                getAbsoluteLevel(p, iLangid, iIndex, 0 as ::core::ffi::c_int),
            );
            sqlite3_bind_int64(
                pDelete,
                2 as ::core::ffi::c_int,
                getAbsoluteLevel(
                    p,
                    iLangid,
                    iIndex,
                    FTS3_SEGDIR_MAXLEVEL - 1 as ::core::ffi::c_int,
                ),
            );
        }
    } else {
        rc = fts3SqlStmt(
            p,
            SQL_DELETE_SEGDIR_LEVEL,
            &raw mut pDelete,
            ::core::ptr::null_mut::<*mut sqlite3_value>(),
        );
        if rc == SQLITE_OK {
            sqlite3_bind_int64(
                pDelete,
                1 as ::core::ffi::c_int,
                getAbsoluteLevel(p, iLangid, iIndex, iLevel),
            );
        }
    }
    if rc == SQLITE_OK {
        sqlite3_step(pDelete);
        rc = sqlite3_reset(pDelete);
    }
    return rc;
}
unsafe extern "C" fn fts3ColumnFilter(
    mut iCol: ::core::ffi::c_int,
    mut bZero: ::core::ffi::c_int,
    mut ppList: *mut *mut ::core::ffi::c_char,
    mut pnList: *mut ::core::ffi::c_int,
) {
    let mut pList: *mut ::core::ffi::c_char = *ppList;
    let mut nList: ::core::ffi::c_int = *pnList;
    let mut pEnd: *mut ::core::ffi::c_char =
        pList.offset(nList as isize) as *mut ::core::ffi::c_char;
    let mut iCurrent: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut p: *mut ::core::ffi::c_char = pList;
    loop {
        let mut c: ::core::ffi::c_char = 0 as ::core::ffi::c_char;
        while p < pEnd
            && (c as ::core::ffi::c_int | *p as ::core::ffi::c_int) & 0xfe as ::core::ffi::c_int
                != 0
        {
            let fresh9 = p;
            p = p.offset(1);
            c = (*fresh9 as ::core::ffi::c_int & 0x80 as ::core::ffi::c_int) as ::core::ffi::c_char;
        }
        if iCol == iCurrent {
            nList = p.offset_from(pList) as ::core::ffi::c_long as ::core::ffi::c_int;
            break;
        } else {
            nList -= p.offset_from(pList) as ::core::ffi::c_long as ::core::ffi::c_int;
            pList = p;
            if nList <= 0 as ::core::ffi::c_int {
                break;
            }
            p = pList.offset(1 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_char;
            p = p.offset(
                (if *(p as *mut u8_0) as ::core::ffi::c_int & 0x80 as ::core::ffi::c_int != 0 {
                    sqlite3Fts3GetVarint32(p, &raw mut iCurrent)
                } else {
                    iCurrent = *(p as *mut u8_0) as ::core::ffi::c_int;
                    1 as ::core::ffi::c_int
                }) as isize,
            );
        }
    }
    if bZero != 0
        && pEnd.offset_from(pList.offset(nList as isize) as *mut ::core::ffi::c_char)
            as ::core::ffi::c_long
            > 0 as ::core::ffi::c_long
    {
        memset(
            pList.offset(nList as isize) as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            pEnd.offset_from(pList.offset(nList as isize) as *mut ::core::ffi::c_char)
                as ::core::ffi::c_long as size_t,
        );
    }
    *ppList = pList;
    *pnList = nList;
}
unsafe extern "C" fn fts3MsrBufferData(
    mut pMsr: *mut Fts3MultiSegReader,
    mut pList: *mut ::core::ffi::c_char,
    mut nList: i64_0,
) -> ::core::ffi::c_int {
    if nList + FTS3_NODE_PADDING as i64_0 > (*pMsr).nBuffer {
        let mut pNew: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut nNew: ::core::ffi::c_int =
            (nList * 2 as i64_0 + FTS3_NODE_PADDING as i64_0) as ::core::ffi::c_int;
        pNew = sqlite3_realloc64(
            (*pMsr).aBuffer as *mut ::core::ffi::c_void,
            nNew as sqlite3_uint64,
        ) as *mut ::core::ffi::c_char;
        if pNew.is_null() {
            return SQLITE_NOMEM;
        }
        (*pMsr).aBuffer = pNew;
        (*pMsr).nBuffer = nNew as i64_0;
    }
    memcpy(
        (*pMsr).aBuffer as *mut ::core::ffi::c_void,
        pList as *const ::core::ffi::c_void,
        nList as size_t,
    );
    memset(
        (*pMsr).aBuffer.offset(nList as isize) as *mut ::core::ffi::c_char
            as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        FTS3_NODE_PADDING as size_t,
    );
    return SQLITE_OK;
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3Fts3MsrIncrNext(
    mut p: *mut Fts3Table,
    mut pMsr: *mut Fts3MultiSegReader,
    mut piDocid: *mut sqlite3_int64,
    mut paPoslist: *mut *mut ::core::ffi::c_char,
    mut pnPoslist: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut nMerge: ::core::ffi::c_int = (*pMsr).nAdvance;
    let mut apSegment: *mut *mut Fts3SegReader = (*pMsr).apSegment;
    let mut xCmp: Option<
        unsafe extern "C" fn(*mut Fts3SegReader, *mut Fts3SegReader) -> ::core::ffi::c_int,
    > = if (*p).bDescIdx as ::core::ffi::c_int != 0 {
        Some(
            fts3SegReaderDoclistCmpRev
                as unsafe extern "C" fn(
                    *mut Fts3SegReader,
                    *mut Fts3SegReader,
                ) -> ::core::ffi::c_int,
        )
    } else {
        Some(
            fts3SegReaderDoclistCmp
                as unsafe extern "C" fn(
                    *mut Fts3SegReader,
                    *mut Fts3SegReader,
                ) -> ::core::ffi::c_int,
        )
    };
    if nMerge == 0 as ::core::ffi::c_int {
        *paPoslist = ::core::ptr::null_mut::<::core::ffi::c_char>();
        return SQLITE_OK;
    }
    loop {
        let mut pSeg: *mut Fts3SegReader = ::core::ptr::null_mut::<Fts3SegReader>();
        pSeg = *(*pMsr).apSegment.offset(0 as ::core::ffi::c_int as isize);
        if (*pSeg).pOffsetList.is_null() {
            *paPoslist = ::core::ptr::null_mut::<::core::ffi::c_char>();
            break;
        } else {
            let mut rc: ::core::ffi::c_int = 0;
            let mut pList: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            let mut nList: ::core::ffi::c_int = 0;
            let mut j: ::core::ffi::c_int = 0;
            let mut iDocid: sqlite3_int64 =
                (**apSegment.offset(0 as ::core::ffi::c_int as isize)).iDocid;
            rc = fts3SegReaderNextDocid(
                p,
                *apSegment.offset(0 as ::core::ffi::c_int as isize),
                &raw mut pList,
                &raw mut nList,
            );
            j = 1 as ::core::ffi::c_int;
            while rc == SQLITE_OK
                && j < nMerge
                && !(**apSegment.offset(j as isize)).pOffsetList.is_null()
                && (**apSegment.offset(j as isize)).iDocid == iDocid
            {
                rc = fts3SegReaderNextDocid(
                    p,
                    *apSegment.offset(j as isize),
                    ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
                    ::core::ptr::null_mut::<::core::ffi::c_int>(),
                );
                j += 1;
            }
            if rc != SQLITE_OK {
                return rc;
            }
            fts3SegReaderSort((*pMsr).apSegment, nMerge, j, xCmp);
            if nList > 0 as ::core::ffi::c_int
                && !(**apSegment.offset(0 as ::core::ffi::c_int as isize))
                    .ppNextElem
                    .is_null()
            {
                rc = fts3MsrBufferData(pMsr, pList, nList as i64_0 + 1 as i64_0);
                if rc != SQLITE_OK {
                    return rc;
                }
                pList = (*pMsr).aBuffer;
            }
            if (*pMsr).iColFilter >= 0 as ::core::ffi::c_int {
                fts3ColumnFilter(
                    (*pMsr).iColFilter,
                    1 as ::core::ffi::c_int,
                    &raw mut pList,
                    &raw mut nList,
                );
            }
            if !(nList > 0 as ::core::ffi::c_int) {
                continue;
            }
            *paPoslist = pList;
            *piDocid = iDocid;
            *pnPoslist = nList;
            break;
        }
    }
    return SQLITE_OK;
}
unsafe extern "C" fn fts3SegReaderStart(
    mut p: *mut Fts3Table,
    mut pCsr: *mut Fts3MultiSegReader,
    mut zTerm: *const ::core::ffi::c_char,
    mut nTerm: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut nSeg: ::core::ffi::c_int = (*pCsr).nSegment;
    i = 0 as ::core::ffi::c_int;
    while (*pCsr).bRestart == 0 as ::core::ffi::c_int && i < (*pCsr).nSegment {
        let mut res: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut pSeg: *mut Fts3SegReader = *(*pCsr).apSegment.offset(i as isize);
        loop {
            let mut rc: ::core::ffi::c_int = fts3SegReaderNext(p, pSeg, 0 as ::core::ffi::c_int);
            if rc != SQLITE_OK {
                return rc;
            }
            if !(!zTerm.is_null() && {
                res = fts3SegReaderTermCmp(pSeg, zTerm, nTerm);
                res < 0 as ::core::ffi::c_int
            }) {
                break;
            }
        }
        if (*pSeg).bLookup as ::core::ffi::c_int != 0 && res != 0 as ::core::ffi::c_int {
            fts3SegReaderSetEof(pSeg);
        }
        i += 1;
    }
    fts3SegReaderSort(
        (*pCsr).apSegment,
        nSeg,
        nSeg,
        Some(
            fts3SegReaderCmp
                as unsafe extern "C" fn(
                    *mut Fts3SegReader,
                    *mut Fts3SegReader,
                ) -> ::core::ffi::c_int,
        ),
    );
    return SQLITE_OK;
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3Fts3SegReaderStart(
    mut p: *mut Fts3Table,
    mut pCsr: *mut Fts3MultiSegReader,
    mut pFilter: *mut Fts3SegFilter,
) -> ::core::ffi::c_int {
    (*pCsr).pFilter = pFilter;
    return fts3SegReaderStart(p, pCsr, (*pFilter).zTerm, (*pFilter).nTerm);
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3Fts3MsrIncrStart(
    mut p: *mut Fts3Table,
    mut pCsr: *mut Fts3MultiSegReader,
    mut iCol: ::core::ffi::c_int,
    mut zTerm: *const ::core::ffi::c_char,
    mut nTerm: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut rc: ::core::ffi::c_int = 0;
    let mut nSegment: ::core::ffi::c_int = (*pCsr).nSegment;
    let mut xCmp: Option<
        unsafe extern "C" fn(*mut Fts3SegReader, *mut Fts3SegReader) -> ::core::ffi::c_int,
    > = if (*p).bDescIdx as ::core::ffi::c_int != 0 {
        Some(
            fts3SegReaderDoclistCmpRev
                as unsafe extern "C" fn(
                    *mut Fts3SegReader,
                    *mut Fts3SegReader,
                ) -> ::core::ffi::c_int,
        )
    } else {
        Some(
            fts3SegReaderDoclistCmp
                as unsafe extern "C" fn(
                    *mut Fts3SegReader,
                    *mut Fts3SegReader,
                ) -> ::core::ffi::c_int,
        )
    };
    rc = fts3SegReaderStart(p, pCsr, zTerm, nTerm);
    if rc != SQLITE_OK {
        return rc;
    }
    i = 0 as ::core::ffi::c_int;
    while i < nSegment {
        let mut pSeg: *mut Fts3SegReader = *(*pCsr).apSegment.offset(i as isize);
        if (*pSeg).aNode.is_null() || fts3SegReaderTermCmp(pSeg, zTerm, nTerm) != 0 {
            break;
        }
        i += 1;
    }
    (*pCsr).nAdvance = i;
    i = 0 as ::core::ffi::c_int;
    while i < (*pCsr).nAdvance {
        rc = fts3SegReaderFirstDocid(p, *(*pCsr).apSegment.offset(i as isize));
        if rc != SQLITE_OK {
            return rc;
        }
        i += 1;
    }
    fts3SegReaderSort((*pCsr).apSegment, i, i, xCmp);
    (*pCsr).iColFilter = iCol;
    return SQLITE_OK;
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3Fts3MsrIncrRestart(
    mut pCsr: *mut Fts3MultiSegReader,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    (*pCsr).nAdvance = 0 as ::core::ffi::c_int;
    (*pCsr).bRestart = 1 as ::core::ffi::c_int;
    i = 0 as ::core::ffi::c_int;
    while i < (*pCsr).nSegment {
        let ref mut fresh18 = (**(*pCsr).apSegment.offset(i as isize)).pOffsetList;
        *fresh18 = ::core::ptr::null_mut::<::core::ffi::c_char>();
        (**(*pCsr).apSegment.offset(i as isize)).nOffsetList = 0 as ::core::ffi::c_int;
        (**(*pCsr).apSegment.offset(i as isize)).iDocid = 0 as sqlite3_int64;
        i += 1;
    }
    return SQLITE_OK;
}
unsafe extern "C" fn fts3GrowSegReaderBuffer(
    mut pCsr: *mut Fts3MultiSegReader,
    mut nReq: i64_0,
) -> ::core::ffi::c_int {
    if nReq > (*pCsr).nBuffer {
        let mut aNew: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        (*pCsr).nBuffer = nReq * 2 as i64_0;
        aNew = sqlite3_realloc64(
            (*pCsr).aBuffer as *mut ::core::ffi::c_void,
            (*pCsr).nBuffer as sqlite3_uint64,
        ) as *mut ::core::ffi::c_char;
        if aNew.is_null() {
            return SQLITE_NOMEM;
        }
        (*pCsr).aBuffer = aNew;
    }
    return SQLITE_OK;
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3Fts3SegReaderStep(
    mut p: *mut Fts3Table,
    mut pCsr: *mut Fts3MultiSegReader,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = SQLITE_OK;
    let mut isIgnoreEmpty: ::core::ffi::c_int =
        (*(*pCsr).pFilter).flags & FTS3_SEGMENT_IGNORE_EMPTY;
    let mut isRequirePos: ::core::ffi::c_int = (*(*pCsr).pFilter).flags & FTS3_SEGMENT_REQUIRE_POS;
    let mut isColFilter: ::core::ffi::c_int = (*(*pCsr).pFilter).flags & FTS3_SEGMENT_COLUMN_FILTER;
    let mut isPrefix: ::core::ffi::c_int = (*(*pCsr).pFilter).flags & FTS3_SEGMENT_PREFIX;
    let mut isScan: ::core::ffi::c_int = (*(*pCsr).pFilter).flags & FTS3_SEGMENT_SCAN;
    let mut isFirst: ::core::ffi::c_int = (*(*pCsr).pFilter).flags & FTS3_SEGMENT_FIRST;
    let mut apSegment: *mut *mut Fts3SegReader = (*pCsr).apSegment;
    let mut nSegment: ::core::ffi::c_int = (*pCsr).nSegment;
    let mut pFilter: *mut Fts3SegFilter = (*pCsr).pFilter;
    let mut xCmp: Option<
        unsafe extern "C" fn(*mut Fts3SegReader, *mut Fts3SegReader) -> ::core::ffi::c_int,
    > = if (*p).bDescIdx as ::core::ffi::c_int != 0 {
        Some(
            fts3SegReaderDoclistCmpRev
                as unsafe extern "C" fn(
                    *mut Fts3SegReader,
                    *mut Fts3SegReader,
                ) -> ::core::ffi::c_int,
        )
    } else {
        Some(
            fts3SegReaderDoclistCmp
                as unsafe extern "C" fn(
                    *mut Fts3SegReader,
                    *mut Fts3SegReader,
                ) -> ::core::ffi::c_int,
        )
    };
    if (*pCsr).nSegment == 0 as ::core::ffi::c_int {
        return SQLITE_OK;
    }
    loop {
        let mut nMerge: ::core::ffi::c_int = 0;
        let mut i: ::core::ffi::c_int = 0;
        i = 0 as ::core::ffi::c_int;
        while i < (*pCsr).nAdvance {
            let mut pSeg: *mut Fts3SegReader = *apSegment.offset(i as isize);
            if (*pSeg).bLookup != 0 {
                fts3SegReaderSetEof(pSeg);
            } else {
                rc = fts3SegReaderNext(p, pSeg, 0 as ::core::ffi::c_int);
            }
            if rc != SQLITE_OK {
                return rc;
            }
            i += 1;
        }
        fts3SegReaderSort(
            apSegment,
            nSegment,
            (*pCsr).nAdvance,
            Some(
                fts3SegReaderCmp
                    as unsafe extern "C" fn(
                        *mut Fts3SegReader,
                        *mut Fts3SegReader,
                    ) -> ::core::ffi::c_int,
            ),
        );
        (*pCsr).nAdvance = 0 as ::core::ffi::c_int;
        if (**apSegment.offset(0 as ::core::ffi::c_int as isize))
            .aNode
            .is_null()
        {
            break;
        }
        (*pCsr).nTerm = (**apSegment.offset(0 as ::core::ffi::c_int as isize)).nTerm;
        (*pCsr).zTerm = (**apSegment.offset(0 as ::core::ffi::c_int as isize)).zTerm;
        if !(*pFilter).zTerm.is_null() && isScan == 0 {
            if (*pCsr).nTerm < (*pFilter).nTerm
                || isPrefix == 0 && (*pCsr).nTerm > (*pFilter).nTerm
                || memcmp(
                    (*pCsr).zTerm as *const ::core::ffi::c_void,
                    (*pFilter).zTerm as *const ::core::ffi::c_void,
                    (*pFilter).nTerm as size_t,
                ) != 0
            {
                break;
            }
        }
        nMerge = 1 as ::core::ffi::c_int;
        while nMerge < nSegment
            && !(**apSegment.offset(nMerge as isize)).aNode.is_null()
            && (**apSegment.offset(nMerge as isize)).nTerm == (*pCsr).nTerm
            && 0 as ::core::ffi::c_int
                == memcmp(
                    (*pCsr).zTerm as *const ::core::ffi::c_void,
                    (**apSegment.offset(nMerge as isize)).zTerm as *const ::core::ffi::c_void,
                    (*pCsr).nTerm as size_t,
                )
        {
            nMerge += 1;
        }
        if nMerge == 1 as ::core::ffi::c_int
            && isIgnoreEmpty == 0
            && isFirst == 0
            && ((*p).bDescIdx as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                || ((**apSegment.offset(0 as ::core::ffi::c_int as isize)).ppNextElem
                    != ::core::ptr::null_mut::<*mut Fts3HashElem>())
                    as ::core::ffi::c_int
                    == 0 as ::core::ffi::c_int)
        {
            (*pCsr).nDoclist = (**apSegment.offset(0 as ::core::ffi::c_int as isize)).nDoclist;
            if !(**apSegment.offset(0 as ::core::ffi::c_int as isize))
                .ppNextElem
                .is_null()
            {
                rc = fts3MsrBufferData(
                    pCsr,
                    (**apSegment.offset(0 as ::core::ffi::c_int as isize)).aDoclist,
                    (*pCsr).nDoclist as i64_0,
                );
                (*pCsr).aDoclist = (*pCsr).aBuffer;
            } else {
                (*pCsr).aDoclist = (**apSegment.offset(0 as ::core::ffi::c_int as isize)).aDoclist;
            }
            if rc == SQLITE_OK {
                rc = SQLITE_ROW;
            }
        } else {
            let mut nDoclist: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let mut iPrev: sqlite3_int64 = 0 as sqlite3_int64;
            i = 0 as ::core::ffi::c_int;
            while i < nMerge {
                fts3SegReaderFirstDocid(p, *apSegment.offset(i as isize));
                i += 1;
            }
            fts3SegReaderSort(apSegment, nMerge, nMerge, xCmp);
            while !(**apSegment.offset(0 as ::core::ffi::c_int as isize))
                .pOffsetList
                .is_null()
            {
                let mut j: ::core::ffi::c_int = 0;
                let mut pList: *mut ::core::ffi::c_char =
                    ::core::ptr::null_mut::<::core::ffi::c_char>();
                let mut nList: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                let mut nByte: ::core::ffi::c_int = 0;
                let mut iDocid: sqlite3_int64 =
                    (**apSegment.offset(0 as ::core::ffi::c_int as isize)).iDocid;
                fts3SegReaderNextDocid(
                    p,
                    *apSegment.offset(0 as ::core::ffi::c_int as isize),
                    &raw mut pList,
                    &raw mut nList,
                );
                j = 1 as ::core::ffi::c_int;
                while j < nMerge
                    && !(**apSegment.offset(j as isize)).pOffsetList.is_null()
                    && (**apSegment.offset(j as isize)).iDocid == iDocid
                {
                    fts3SegReaderNextDocid(
                        p,
                        *apSegment.offset(j as isize),
                        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
                        ::core::ptr::null_mut::<::core::ffi::c_int>(),
                    );
                    j += 1;
                }
                if isColFilter != 0 {
                    fts3ColumnFilter(
                        (*pFilter).iCol,
                        0 as ::core::ffi::c_int,
                        &raw mut pList,
                        &raw mut nList,
                    );
                }
                if isIgnoreEmpty == 0 || nList > 0 as ::core::ffi::c_int {
                    let mut iDelta: sqlite3_int64 = 0;
                    if (*p).bDescIdx as ::core::ffi::c_int != 0
                        && nDoclist > 0 as ::core::ffi::c_int
                    {
                        if iPrev <= iDocid {
                            return FTS_CORRUPT_VTAB;
                        }
                        iDelta = (iPrev as u64_0).wrapping_sub(iDocid as u64_0) as i64_0
                            as sqlite3_int64;
                    } else {
                        if nDoclist > 0 as ::core::ffi::c_int && iPrev >= iDocid {
                            return FTS_CORRUPT_VTAB;
                        }
                        iDelta = (iDocid as u64_0).wrapping_sub(iPrev as u64_0) as i64_0
                            as sqlite3_int64;
                    }
                    nByte = sqlite3Fts3VarintLen(iDelta as sqlite3_uint64)
                        + (if isRequirePos != 0 {
                            nList + 1 as ::core::ffi::c_int
                        } else {
                            0 as ::core::ffi::c_int
                        });
                    rc = fts3GrowSegReaderBuffer(
                        pCsr,
                        nByte as i64_0 + nDoclist as i64_0 + FTS3_NODE_PADDING as i64_0,
                    );
                    if rc != 0 {
                        return rc;
                    }
                    if isFirst != 0 {
                        let mut a: *mut ::core::ffi::c_char =
                            (*pCsr).aBuffer.offset(nDoclist as isize) as *mut ::core::ffi::c_char;
                        let mut nWrite: ::core::ffi::c_int = 0;
                        nWrite = sqlite3Fts3FirstFilter(iDelta, pList, nList, a);
                        if nWrite != 0 {
                            iPrev = iDocid;
                            nDoclist += nWrite;
                        }
                    } else {
                        nDoclist += sqlite3Fts3PutVarint(
                            (*pCsr).aBuffer.offset(nDoclist as isize) as *mut ::core::ffi::c_char,
                            iDelta,
                        );
                        iPrev = iDocid;
                        if isRequirePos != 0 {
                            memcpy(
                                (*pCsr).aBuffer.offset(nDoclist as isize)
                                    as *mut ::core::ffi::c_char
                                    as *mut ::core::ffi::c_void,
                                pList as *const ::core::ffi::c_void,
                                nList as size_t,
                            );
                            nDoclist += nList;
                            let fresh6 = nDoclist;
                            nDoclist = nDoclist + 1;
                            *(*pCsr).aBuffer.offset(fresh6 as isize) =
                                '\0' as i32 as ::core::ffi::c_char;
                        }
                    }
                }
                fts3SegReaderSort(apSegment, nMerge, j, xCmp);
            }
            if nDoclist > 0 as ::core::ffi::c_int {
                rc = fts3GrowSegReaderBuffer(pCsr, nDoclist as i64_0 + FTS3_NODE_PADDING as i64_0);
                if rc != 0 {
                    return rc;
                }
                memset(
                    (*pCsr).aBuffer.offset(nDoclist as isize) as *mut ::core::ffi::c_char
                        as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    FTS3_NODE_PADDING as size_t,
                );
                (*pCsr).aDoclist = (*pCsr).aBuffer;
                (*pCsr).nDoclist = nDoclist;
                rc = SQLITE_ROW;
            }
        }
        (*pCsr).nAdvance = nMerge;
        if !(rc == SQLITE_OK) {
            break;
        }
    }
    return rc;
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3Fts3SegReaderFinish(mut pCsr: *mut Fts3MultiSegReader) {
    if !pCsr.is_null() {
        let mut i: ::core::ffi::c_int = 0;
        i = 0 as ::core::ffi::c_int;
        while i < (*pCsr).nSegment {
            sqlite3Fts3SegReaderFree(*(*pCsr).apSegment.offset(i as isize));
            i += 1;
        }
        sqlite3_free((*pCsr).apSegment as *mut ::core::ffi::c_void);
        sqlite3_free((*pCsr).aBuffer as *mut ::core::ffi::c_void);
        (*pCsr).nSegment = 0 as ::core::ffi::c_int;
        (*pCsr).apSegment = ::core::ptr::null_mut::<*mut Fts3SegReader>();
        (*pCsr).aBuffer = ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
}
unsafe extern "C" fn fts3ReadEndBlockField(
    mut pStmt: *mut sqlite3_stmt,
    mut iCol: ::core::ffi::c_int,
    mut piEndBlock: *mut i64_0,
    mut pnByte: *mut i64_0,
) {
    let mut zText: *const ::core::ffi::c_uchar = sqlite3_column_text(pStmt, iCol);
    if !zText.is_null() {
        let mut i: ::core::ffi::c_int = 0;
        let mut iMul: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
        let mut iVal: u64_0 = 0 as u64_0;
        i = 0 as ::core::ffi::c_int;
        while *zText.offset(i as isize) as ::core::ffi::c_int >= '0' as i32
            && *zText.offset(i as isize) as ::core::ffi::c_int <= '9' as i32
        {
            iVal = iVal.wrapping_mul(10 as u64_0).wrapping_add(
                (*zText.offset(i as isize) as ::core::ffi::c_int - '0' as i32) as u64_0,
            );
            i += 1;
        }
        *piEndBlock = iVal as i64_0;
        while *zText.offset(i as isize) as ::core::ffi::c_int == ' ' as i32 {
            i += 1;
        }
        iVal = 0 as u64_0;
        if *zText.offset(i as isize) as ::core::ffi::c_int == '-' as i32 {
            i += 1;
            iMul = -(1 as ::core::ffi::c_int);
        }
        while *zText.offset(i as isize) as ::core::ffi::c_int >= '0' as i32
            && *zText.offset(i as isize) as ::core::ffi::c_int <= '9' as i32
        {
            iVal = iVal.wrapping_mul(10 as u64_0).wrapping_add(
                (*zText.offset(i as isize) as ::core::ffi::c_int - '0' as i32) as u64_0,
            );
            i += 1;
        }
        *pnByte = iVal as i64_0 * iMul as i64_0;
    }
}
unsafe extern "C" fn fts3PromoteSegments(
    mut p: *mut Fts3Table,
    mut iAbsLevel: sqlite3_int64,
    mut nByte: sqlite3_int64,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = SQLITE_OK;
    let mut pRange: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    rc = fts3SqlStmt(
        p,
        SQL_SELECT_LEVEL_RANGE2,
        &raw mut pRange,
        ::core::ptr::null_mut::<*mut sqlite3_value>(),
    );
    if rc == SQLITE_OK {
        let mut bOk: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut iLast: i64_0 = (iAbsLevel as i64_0 / FTS3_SEGDIR_MAXLEVEL as i64_0 + 1 as i64_0)
            * FTS3_SEGDIR_MAXLEVEL as i64_0
            - 1 as i64_0;
        let mut nLimit: i64_0 = nByte as i64_0 * 3 as i64_0 / 2 as i64_0;
        sqlite3_bind_int64(
            pRange,
            1 as ::core::ffi::c_int,
            iAbsLevel + 1 as sqlite3_int64,
        );
        sqlite3_bind_int64(pRange, 2 as ::core::ffi::c_int, iLast as sqlite3_int64);
        while SQLITE_ROW == sqlite3_step(pRange) {
            let mut nSize: i64_0 = 0 as i64_0;
            let mut dummy: i64_0 = 0;
            fts3ReadEndBlockField(
                pRange,
                2 as ::core::ffi::c_int,
                &raw mut dummy,
                &raw mut nSize,
            );
            if nSize <= 0 as i64_0 || nSize > nLimit {
                bOk = 0 as ::core::ffi::c_int;
                break;
            } else {
                bOk = 1 as ::core::ffi::c_int;
            }
        }
        rc = sqlite3_reset(pRange);
        if bOk != 0 {
            let mut iIdx: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let mut pUpdate1: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
            let mut pUpdate2: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
            if rc == SQLITE_OK {
                rc = fts3SqlStmt(
                    p,
                    SQL_UPDATE_LEVEL_IDX,
                    &raw mut pUpdate1,
                    ::core::ptr::null_mut::<*mut sqlite3_value>(),
                );
            }
            if rc == SQLITE_OK {
                rc = fts3SqlStmt(
                    p,
                    SQL_UPDATE_LEVEL,
                    &raw mut pUpdate2,
                    ::core::ptr::null_mut::<*mut sqlite3_value>(),
                );
            }
            if rc == SQLITE_OK {
                sqlite3_bind_int64(pRange, 1 as ::core::ffi::c_int, iAbsLevel);
                while SQLITE_ROW == sqlite3_step(pRange) {
                    let fresh3 = iIdx;
                    iIdx = iIdx + 1;
                    sqlite3_bind_int(pUpdate1, 1 as ::core::ffi::c_int, fresh3);
                    sqlite3_bind_int(
                        pUpdate1,
                        2 as ::core::ffi::c_int,
                        sqlite3_column_int(pRange, 0 as ::core::ffi::c_int),
                    );
                    sqlite3_bind_int(
                        pUpdate1,
                        3 as ::core::ffi::c_int,
                        sqlite3_column_int(pRange, 1 as ::core::ffi::c_int),
                    );
                    sqlite3_step(pUpdate1);
                    rc = sqlite3_reset(pUpdate1);
                    if !(rc != SQLITE_OK) {
                        continue;
                    }
                    sqlite3_reset(pRange);
                    break;
                }
            }
            if rc == SQLITE_OK {
                rc = sqlite3_reset(pRange);
            }
            if rc == SQLITE_OK {
                sqlite3_bind_int64(pUpdate2, 1 as ::core::ffi::c_int, iAbsLevel);
                sqlite3_step(pUpdate2);
                rc = sqlite3_reset(pUpdate2);
            }
        }
    }
    return rc;
}
unsafe extern "C" fn fts3SegmentMerge(
    mut p: *mut Fts3Table,
    mut iLangid: ::core::ffi::c_int,
    mut iIndex: ::core::ffi::c_int,
    mut iLevel: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut rc: ::core::ffi::c_int = 0;
    let mut iIdx: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut iNewLevel: sqlite3_int64 = 0 as sqlite3_int64;
    let mut pWriter: *mut SegmentWriter = ::core::ptr::null_mut::<SegmentWriter>();
    let mut filter: Fts3SegFilter = Fts3SegFilter {
        zTerm: ::core::ptr::null::<::core::ffi::c_char>(),
        nTerm: 0,
        iCol: 0,
        flags: 0,
    };
    let mut csr: Fts3MultiSegReader = Fts3MultiSegReader {
        apSegment: ::core::ptr::null_mut::<*mut Fts3SegReader>(),
        nSegment: 0,
        nAdvance: 0,
        pFilter: ::core::ptr::null_mut::<Fts3SegFilter>(),
        aBuffer: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        nBuffer: 0,
        iColFilter: 0,
        bRestart: 0,
        nCost: 0,
        bLookup: 0,
        zTerm: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        nTerm: 0,
        aDoclist: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        nDoclist: 0,
    };
    let mut bIgnoreEmpty: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut iMaxLevel: i64_0 = 0 as i64_0;
    rc = sqlite3Fts3SegReaderCursor(
        p,
        iLangid,
        iIndex,
        iLevel,
        ::core::ptr::null::<::core::ffi::c_char>(),
        0 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        &raw mut csr,
    );
    if !(rc != SQLITE_OK || csr.nSegment == 0 as ::core::ffi::c_int) {
        if iLevel != FTS3_SEGCURSOR_PENDING {
            rc = fts3SegmentMaxLevel(p, iLangid, iIndex, &raw mut iMaxLevel);
            if rc != SQLITE_OK {
                current_block = 14255763095006029735;
            } else {
                current_block = 13183875560443969876;
            }
        } else {
            current_block = 13183875560443969876;
        }
        match current_block {
            14255763095006029735 => {}
            _ => {
                if iLevel == FTS3_SEGCURSOR_ALL {
                    if csr.nSegment == 1 as ::core::ffi::c_int
                        && 0 as ::core::ffi::c_int
                            == ((**csr.apSegment.offset(0 as ::core::ffi::c_int as isize))
                                .ppNextElem
                                != ::core::ptr::null_mut::<*mut Fts3HashElem>())
                                as ::core::ffi::c_int
                    {
                        rc = SQLITE_DONE;
                        current_block = 14255763095006029735;
                    } else {
                        iNewLevel = iMaxLevel as sqlite3_int64;
                        bIgnoreEmpty = 1 as ::core::ffi::c_int;
                        current_block = 15652330335145281839;
                    }
                } else {
                    iNewLevel =
                        getAbsoluteLevel(p, iLangid, iIndex, iLevel + 1 as ::core::ffi::c_int);
                    rc = fts3AllocateSegdirIdx(
                        p,
                        iLangid,
                        iIndex,
                        iLevel + 1 as ::core::ffi::c_int,
                        &raw mut iIdx,
                    );
                    bIgnoreEmpty = (iLevel != FTS3_SEGCURSOR_PENDING && iNewLevel > iMaxLevel)
                        as ::core::ffi::c_int;
                    current_block = 15652330335145281839;
                }
                match current_block {
                    14255763095006029735 => {}
                    _ => {
                        if !(rc != SQLITE_OK) {
                            memset(
                                &raw mut filter as *mut ::core::ffi::c_void,
                                0 as ::core::ffi::c_int,
                                ::core::mem::size_of::<Fts3SegFilter>() as size_t,
                            );
                            filter.flags = FTS3_SEGMENT_REQUIRE_POS;
                            filter.flags |= if bIgnoreEmpty != 0 {
                                FTS3_SEGMENT_IGNORE_EMPTY
                            } else {
                                0 as ::core::ffi::c_int
                            };
                            rc = sqlite3Fts3SegReaderStart(p, &raw mut csr, &raw mut filter);
                            while SQLITE_OK == rc {
                                rc = sqlite3Fts3SegReaderStep(p, &raw mut csr);
                                if rc != SQLITE_ROW {
                                    break;
                                }
                                rc = fts3SegWriterAdd(
                                    p,
                                    &raw mut pWriter,
                                    1 as ::core::ffi::c_int,
                                    csr.zTerm,
                                    csr.nTerm,
                                    csr.aDoclist,
                                    csr.nDoclist,
                                );
                            }
                            if !(rc != SQLITE_OK) {
                                if iLevel != FTS3_SEGCURSOR_PENDING {
                                    rc = fts3DeleteSegdir(
                                        p,
                                        iLangid,
                                        iIndex,
                                        iLevel,
                                        csr.apSegment,
                                        csr.nSegment,
                                    );
                                    if rc != SQLITE_OK {
                                        current_block = 14255763095006029735;
                                    } else {
                                        current_block = 2569451025026770673;
                                    }
                                } else {
                                    current_block = 2569451025026770673;
                                }
                                match current_block {
                                    14255763095006029735 => {}
                                    _ => {
                                        if !pWriter.is_null() {
                                            rc = fts3SegWriterFlush(p, pWriter, iNewLevel, iIdx);
                                            if rc == SQLITE_OK {
                                                if iLevel == FTS3_SEGCURSOR_PENDING
                                                    || iNewLevel < iMaxLevel
                                                {
                                                    rc = fts3PromoteSegments(
                                                        p,
                                                        iNewLevel,
                                                        (*pWriter).nLeafData as sqlite3_int64,
                                                    );
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    fts3SegWriterFree(pWriter);
    sqlite3Fts3SegReaderFinish(&raw mut csr);
    return rc;
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3Fts3PendingTermsFlush(mut p: *mut Fts3Table) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = SQLITE_OK;
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while rc == SQLITE_OK && i < (*p).nIndex {
        rc = fts3SegmentMerge(p, (*p).iPrevLangid, i, FTS3_SEGCURSOR_PENDING);
        if rc == SQLITE_DONE {
            rc = SQLITE_OK;
        }
        i += 1;
    }
    if rc == SQLITE_OK
        && (*p).bHasStat as ::core::ffi::c_int != 0
        && (*p).nAutoincrmerge == 0xff as ::core::ffi::c_int
        && (*p).nLeafAdd > 0 as u32_0
    {
        let mut pStmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
        rc = fts3SqlStmt(
            p,
            SQL_SELECT_STAT,
            &raw mut pStmt,
            ::core::ptr::null_mut::<*mut sqlite3_value>(),
        );
        if rc == SQLITE_OK {
            sqlite3_bind_int(pStmt, 1 as ::core::ffi::c_int, FTS_STAT_AUTOINCRMERGE);
            rc = sqlite3_step(pStmt);
            if rc == SQLITE_ROW {
                (*p).nAutoincrmerge = sqlite3_column_int(pStmt, 0 as ::core::ffi::c_int);
                if (*p).nAutoincrmerge == 1 as ::core::ffi::c_int {
                    (*p).nAutoincrmerge = 8 as ::core::ffi::c_int;
                }
            } else if rc == SQLITE_DONE {
                (*p).nAutoincrmerge = 0 as ::core::ffi::c_int;
            }
            rc = sqlite3_reset(pStmt);
        }
    }
    if rc == SQLITE_OK {
        sqlite3Fts3PendingTermsClear(p);
    }
    return rc;
}
unsafe extern "C" fn fts3EncodeIntArray(
    mut N: ::core::ffi::c_int,
    mut a: *mut u32_0,
    mut zBuf: *mut ::core::ffi::c_char,
    mut pNBuf: *mut ::core::ffi::c_int,
) {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    j = 0 as ::core::ffi::c_int;
    i = j;
    while i < N {
        j += sqlite3Fts3PutVarint(
            zBuf.offset(j as isize) as *mut ::core::ffi::c_char,
            *a.offset(i as isize) as sqlite3_int64,
        );
        i += 1;
    }
    *pNBuf = j;
}
unsafe extern "C" fn fts3DecodeIntArray(
    mut N: ::core::ffi::c_int,
    mut a: *mut u32_0,
    mut zBuf: *const ::core::ffi::c_char,
    mut nBuf: ::core::ffi::c_int,
) {
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if nBuf != 0
        && *zBuf.offset((nBuf - 1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
            & 0x80 as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
    {
        let mut j: ::core::ffi::c_int = 0;
        j = 0 as ::core::ffi::c_int;
        i = j;
        while i < N && j < nBuf {
            let mut x: sqlite3_int64 = 0;
            j += sqlite3Fts3GetVarint(
                zBuf.offset(j as isize) as *const ::core::ffi::c_char,
                &raw mut x,
            );
            *a.offset(i as isize) = (x & 0xffffffff as sqlite3_int64) as u32_0;
            i += 1;
        }
    }
    while i < N {
        let fresh1 = i;
        i = i + 1;
        *a.offset(fresh1 as isize) = 0 as u32_0;
    }
}
unsafe extern "C" fn fts3InsertDocsize(
    mut pRC: *mut ::core::ffi::c_int,
    mut p: *mut Fts3Table,
    mut aSz: *mut u32_0,
) {
    let mut pBlob: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut nBlob: ::core::ffi::c_int = 0;
    let mut pStmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut rc: ::core::ffi::c_int = 0;
    if *pRC != 0 {
        return;
    }
    pBlob =
        sqlite3_malloc64((10 as sqlite3_int64 * (*p).nColumn as sqlite3_int64) as sqlite3_uint64)
            as *mut ::core::ffi::c_char;
    if pBlob.is_null() {
        *pRC = SQLITE_NOMEM;
        return;
    }
    fts3EncodeIntArray((*p).nColumn, aSz, pBlob, &raw mut nBlob);
    rc = fts3SqlStmt(
        p,
        SQL_REPLACE_DOCSIZE,
        &raw mut pStmt,
        ::core::ptr::null_mut::<*mut sqlite3_value>(),
    );
    if rc != 0 {
        sqlite3_free(pBlob as *mut ::core::ffi::c_void);
        *pRC = rc;
        return;
    }
    sqlite3_bind_int64(
        pStmt,
        1 as ::core::ffi::c_int,
        (*p).iPrevDocid as sqlite3_int64,
    );
    sqlite3_bind_blob(
        pStmt,
        2 as ::core::ffi::c_int,
        pBlob as *const ::core::ffi::c_void,
        nBlob,
        Some(sqlite3_free as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()),
    );
    sqlite3_step(pStmt);
    *pRC = sqlite3_reset(pStmt);
}
unsafe extern "C" fn fts3UpdateDocTotals(
    mut pRC: *mut ::core::ffi::c_int,
    mut p: *mut Fts3Table,
    mut aSzIns: *mut u32_0,
    mut aSzDel: *mut u32_0,
    mut nChng: ::core::ffi::c_int,
) {
    let mut pBlob: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut nBlob: ::core::ffi::c_int = 0;
    let mut a: *mut u32_0 = ::core::ptr::null_mut::<u32_0>();
    let mut pStmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut i: ::core::ffi::c_int = 0;
    let mut rc: ::core::ffi::c_int = 0;
    let nStat: ::core::ffi::c_int = (*p).nColumn + 2 as ::core::ffi::c_int;
    if *pRC != 0 {
        return;
    }
    a = sqlite3_malloc64(
        ((::core::mem::size_of::<u32_0>() as usize).wrapping_add(10 as usize) as sqlite3_uint64)
            .wrapping_mul(nStat as sqlite3_int64 as sqlite3_uint64),
    ) as *mut u32_0;
    if a.is_null() {
        *pRC = SQLITE_NOMEM;
        return;
    }
    pBlob = a.offset(nStat as isize) as *mut u32_0 as *mut ::core::ffi::c_char;
    rc = fts3SqlStmt(
        p,
        SQL_SELECT_STAT,
        &raw mut pStmt,
        ::core::ptr::null_mut::<*mut sqlite3_value>(),
    );
    if rc != 0 {
        sqlite3_free(a as *mut ::core::ffi::c_void);
        *pRC = rc;
        return;
    }
    sqlite3_bind_int(pStmt, 1 as ::core::ffi::c_int, FTS_STAT_DOCTOTAL);
    if sqlite3_step(pStmt) == SQLITE_ROW {
        fts3DecodeIntArray(
            nStat,
            a,
            sqlite3_column_blob(pStmt, 0 as ::core::ffi::c_int) as *const ::core::ffi::c_char,
            sqlite3_column_bytes(pStmt, 0 as ::core::ffi::c_int),
        );
    } else {
        memset(
            a as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            (::core::mem::size_of::<u32_0>() as size_t).wrapping_mul(nStat as size_t),
        );
    }
    rc = sqlite3_reset(pStmt);
    if rc != SQLITE_OK {
        sqlite3_free(a as *mut ::core::ffi::c_void);
        *pRC = rc;
        return;
    }
    if nChng < 0 as ::core::ffi::c_int
        && *a.offset(0 as ::core::ffi::c_int as isize) < -nChng as u32_0
    {
        *a.offset(0 as ::core::ffi::c_int as isize) = 0 as u32_0;
    } else {
        let ref mut fresh0 = *a.offset(0 as ::core::ffi::c_int as isize);
        *fresh0 = (*fresh0).wrapping_add(nChng as u32_0);
    }
    i = 0 as ::core::ffi::c_int;
    while i < (*p).nColumn + 1 as ::core::ffi::c_int {
        let mut x: u32_0 = *a.offset((i + 1 as ::core::ffi::c_int) as isize);
        if x.wrapping_add(*aSzIns.offset(i as isize)) < *aSzDel.offset(i as isize) {
            x = 0 as u32_0;
        } else {
            x = x
                .wrapping_add(*aSzIns.offset(i as isize))
                .wrapping_sub(*aSzDel.offset(i as isize));
        }
        *a.offset((i + 1 as ::core::ffi::c_int) as isize) = x;
        i += 1;
    }
    fts3EncodeIntArray(nStat, a, pBlob, &raw mut nBlob);
    rc = fts3SqlStmt(
        p,
        SQL_REPLACE_STAT,
        &raw mut pStmt,
        ::core::ptr::null_mut::<*mut sqlite3_value>(),
    );
    if rc != 0 {
        sqlite3_free(a as *mut ::core::ffi::c_void);
        *pRC = rc;
        return;
    }
    sqlite3_bind_int(pStmt, 1 as ::core::ffi::c_int, FTS_STAT_DOCTOTAL);
    sqlite3_bind_blob(
        pStmt,
        2 as ::core::ffi::c_int,
        pBlob as *const ::core::ffi::c_void,
        nBlob,
        SQLITE_STATIC,
    );
    sqlite3_step(pStmt);
    *pRC = sqlite3_reset(pStmt);
    sqlite3_bind_null(pStmt, 2 as ::core::ffi::c_int);
    sqlite3_free(a as *mut ::core::ffi::c_void);
}
unsafe extern "C" fn fts3DoOptimize(
    mut p: *mut Fts3Table,
    mut bReturnDone: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut bSeenDone: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut rc: ::core::ffi::c_int = 0;
    let mut pAllLangid: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    rc = sqlite3Fts3PendingTermsFlush(p);
    if rc == SQLITE_OK {
        rc = fts3SqlStmt(
            p,
            SQL_SELECT_ALL_LANGID,
            &raw mut pAllLangid,
            ::core::ptr::null_mut::<*mut sqlite3_value>(),
        );
    }
    if rc == SQLITE_OK {
        let mut rc2: ::core::ffi::c_int = 0;
        sqlite3_bind_int(pAllLangid, 1 as ::core::ffi::c_int, (*p).iPrevLangid);
        sqlite3_bind_int(pAllLangid, 2 as ::core::ffi::c_int, (*p).nIndex);
        while sqlite3_step(pAllLangid) == SQLITE_ROW {
            let mut i: ::core::ffi::c_int = 0;
            let mut iLangid: ::core::ffi::c_int =
                sqlite3_column_int(pAllLangid, 0 as ::core::ffi::c_int);
            i = 0 as ::core::ffi::c_int;
            while rc == SQLITE_OK && i < (*p).nIndex {
                rc = fts3SegmentMerge(p, iLangid, i, FTS3_SEGCURSOR_ALL);
                if rc == SQLITE_DONE {
                    bSeenDone = 1 as ::core::ffi::c_int;
                    rc = SQLITE_OK;
                }
                i += 1;
            }
        }
        rc2 = sqlite3_reset(pAllLangid);
        if rc == SQLITE_OK {
            rc = rc2;
        }
    }
    sqlite3Fts3SegmentsClose(p);
    return if rc == SQLITE_OK && bReturnDone != 0 && bSeenDone != 0 {
        SQLITE_DONE
    } else {
        rc
    };
}
unsafe extern "C" fn fts3DoRebuild(mut p: *mut Fts3Table) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = 0;
    rc = fts3DeleteAll(p, 0 as ::core::ffi::c_int);
    if rc == SQLITE_OK {
        let mut aSz: *mut u32_0 = ::core::ptr::null_mut::<u32_0>();
        let mut aSzIns: *mut u32_0 = ::core::ptr::null_mut::<u32_0>();
        let mut aSzDel: *mut u32_0 = ::core::ptr::null_mut::<u32_0>();
        let mut pStmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
        let mut nEntry: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut zSql: *mut ::core::ffi::c_char = sqlite3_mprintf(
            b"SELECT %s\0" as *const u8 as *const ::core::ffi::c_char,
            (*p).zReadExprlist,
        );
        if zSql.is_null() {
            rc = SQLITE_NOMEM;
        } else {
            rc = sqlite3_prepare_v2(
                (*p).db,
                zSql,
                -(1 as ::core::ffi::c_int),
                &raw mut pStmt,
                ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
            );
            sqlite3_free(zSql as *mut ::core::ffi::c_void);
        }
        if rc == SQLITE_OK {
            let mut nByte: sqlite3_int64 =
                (::core::mem::size_of::<u32_0>() as ::core::ffi::c_ulonglong)
                    .wrapping_mul(
                        ((*p).nColumn as sqlite3_int64 + 1 as sqlite3_int64)
                            as ::core::ffi::c_ulonglong,
                    )
                    .wrapping_mul(3 as ::core::ffi::c_ulonglong) as sqlite3_int64;
            aSz = sqlite3_malloc64(nByte as sqlite3_uint64) as *mut u32_0;
            if aSz.is_null() {
                rc = SQLITE_NOMEM;
            } else {
                memset(
                    aSz as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    nByte as size_t,
                );
                aSzIns =
                    aSz.offset(((*p).nColumn + 1 as ::core::ffi::c_int) as isize) as *mut u32_0;
                aSzDel =
                    aSzIns.offset(((*p).nColumn + 1 as ::core::ffi::c_int) as isize) as *mut u32_0;
            }
        }
        while rc == SQLITE_OK && SQLITE_ROW == sqlite3_step(pStmt) {
            let mut iCol: ::core::ffi::c_int = 0;
            let mut iLangid: ::core::ffi::c_int = langidFromSelect(p, pStmt);
            rc = fts3PendingTermsDocid(
                p,
                0 as ::core::ffi::c_int,
                iLangid,
                sqlite3_column_int64(pStmt, 0 as ::core::ffi::c_int) as sqlite_int64,
            );
            memset(
                aSz as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                (::core::mem::size_of::<u32_0>() as size_t)
                    .wrapping_mul(((*p).nColumn + 1 as ::core::ffi::c_int) as size_t),
            );
            iCol = 0 as ::core::ffi::c_int;
            while rc == SQLITE_OK && iCol < (*p).nColumn {
                if *(*p).abNotindexed.offset(iCol as isize) as ::core::ffi::c_int
                    == 0 as ::core::ffi::c_int
                {
                    let mut z: *const ::core::ffi::c_char =
                        sqlite3_column_text(pStmt, iCol + 1 as ::core::ffi::c_int)
                            as *const ::core::ffi::c_char;
                    rc = fts3PendingTermsAdd(
                        p,
                        iLangid,
                        z,
                        iCol,
                        aSz.offset(iCol as isize) as *mut u32_0,
                    );
                    let ref mut fresh14 = *aSz.offset((*p).nColumn as isize);
                    *fresh14 = (*fresh14)
                        .wrapping_add(
                            sqlite3_column_bytes(pStmt, iCol + 1 as ::core::ffi::c_int) as u32_0
                        );
                }
                iCol += 1;
            }
            if (*p).bHasDocsize != 0 {
                fts3InsertDocsize(&raw mut rc, p, aSz);
            }
            if rc != SQLITE_OK {
                sqlite3_finalize(pStmt);
                pStmt = ::core::ptr::null_mut::<sqlite3_stmt>();
            } else {
                nEntry += 1;
                iCol = 0 as ::core::ffi::c_int;
                while iCol <= (*p).nColumn {
                    let ref mut fresh15 = *aSzIns.offset(iCol as isize);
                    *fresh15 = (*fresh15).wrapping_add(*aSz.offset(iCol as isize));
                    iCol += 1;
                }
            }
        }
        if (*p).bFts4 != 0 {
            fts3UpdateDocTotals(&raw mut rc, p, aSzIns, aSzDel, nEntry);
        }
        sqlite3_free(aSz as *mut ::core::ffi::c_void);
        if !pStmt.is_null() {
            let mut rc2: ::core::ffi::c_int = sqlite3_finalize(pStmt);
            if rc == SQLITE_OK {
                rc = rc2;
            }
        }
    }
    return rc;
}
unsafe extern "C" fn fts3IncrmergeCsr(
    mut p: *mut Fts3Table,
    mut iAbsLevel: sqlite3_int64,
    mut nSeg: ::core::ffi::c_int,
    mut pCsr: *mut Fts3MultiSegReader,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = 0;
    let mut pStmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut nByte: sqlite3_int64 = 0;
    memset(
        pCsr as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<Fts3MultiSegReader>() as size_t,
    );
    nByte = (::core::mem::size_of::<*mut Fts3SegReader>() as usize).wrapping_mul(nSeg as usize)
        as sqlite3_int64;
    (*pCsr).apSegment = sqlite3_malloc64(nByte as sqlite3_uint64) as *mut *mut Fts3SegReader;
    if (*pCsr).apSegment.is_null() {
        rc = SQLITE_NOMEM;
    } else {
        memset(
            (*pCsr).apSegment as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            nByte as size_t,
        );
        rc = fts3SqlStmt(
            p,
            SQL_SELECT_LEVEL,
            &raw mut pStmt,
            ::core::ptr::null_mut::<*mut sqlite3_value>(),
        );
    }
    if rc == SQLITE_OK {
        let mut i: ::core::ffi::c_int = 0;
        let mut rc2: ::core::ffi::c_int = 0;
        sqlite3_bind_int64(pStmt, 1 as ::core::ffi::c_int, iAbsLevel);
        i = 0 as ::core::ffi::c_int;
        while rc == SQLITE_OK && sqlite3_step(pStmt) == SQLITE_ROW && i < nSeg {
            rc = sqlite3Fts3SegReaderNew(
                i,
                0 as ::core::ffi::c_int,
                sqlite3_column_int64(pStmt, 1 as ::core::ffi::c_int),
                sqlite3_column_int64(pStmt, 2 as ::core::ffi::c_int),
                sqlite3_column_int64(pStmt, 3 as ::core::ffi::c_int),
                sqlite3_column_blob(pStmt, 4 as ::core::ffi::c_int) as *const ::core::ffi::c_char,
                sqlite3_column_bytes(pStmt, 4 as ::core::ffi::c_int),
                (*pCsr).apSegment.offset(i as isize) as *mut *mut Fts3SegReader,
            );
            (*pCsr).nSegment += 1;
            i += 1;
        }
        rc2 = sqlite3_reset(pStmt);
        if rc == SQLITE_OK {
            rc = rc2;
        }
    }
    return rc;
}
unsafe extern "C" fn blobGrowBuffer(
    mut pBlob: *mut Blob,
    mut nMin: ::core::ffi::c_int,
    mut pRc: *mut ::core::ffi::c_int,
) {
    if *pRc == SQLITE_OK && nMin > (*pBlob).nAlloc {
        let mut nAlloc: ::core::ffi::c_int = nMin;
        let mut a: *mut ::core::ffi::c_char = sqlite3_realloc64(
            (*pBlob).a as *mut ::core::ffi::c_void,
            nAlloc as sqlite3_uint64,
        ) as *mut ::core::ffi::c_char;
        if !a.is_null() {
            (*pBlob).nAlloc = nAlloc;
            (*pBlob).a = a;
        } else {
            *pRc = SQLITE_NOMEM;
        }
    }
}
unsafe extern "C" fn nodeReaderNext(mut p: *mut NodeReader) -> ::core::ffi::c_int {
    let mut bFirst: ::core::ffi::c_int =
        ((*p).term.n == 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
    let mut nPrefix: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut nSuffix: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut rc: ::core::ffi::c_int = SQLITE_OK;
    if (*p).iChild != 0 && bFirst == 0 as ::core::ffi::c_int {
        (*p).iChild += 1;
    }
    if (*p).iOff >= (*p).nNode {
        (*p).aNode = ::core::ptr::null::<::core::ffi::c_char>();
    } else {
        if bFirst == 0 as ::core::ffi::c_int {
            (*p).iOff += if *((*p).aNode.offset((*p).iOff as isize) as *const ::core::ffi::c_char
                as *mut u8_0) as ::core::ffi::c_int
                & 0x80 as ::core::ffi::c_int
                != 0
            {
                sqlite3Fts3GetVarint32(
                    (*p).aNode.offset((*p).iOff as isize) as *const ::core::ffi::c_char,
                    &raw mut nPrefix,
                )
            } else {
                nPrefix = *((*p).aNode.offset((*p).iOff as isize) as *const ::core::ffi::c_char
                    as *mut u8_0) as ::core::ffi::c_int;
                1 as ::core::ffi::c_int
            };
        }
        (*p).iOff += if *((*p).aNode.offset((*p).iOff as isize) as *const ::core::ffi::c_char
            as *mut u8_0) as ::core::ffi::c_int
            & 0x80 as ::core::ffi::c_int
            != 0
        {
            sqlite3Fts3GetVarint32(
                (*p).aNode.offset((*p).iOff as isize) as *const ::core::ffi::c_char,
                &raw mut nSuffix,
            )
        } else {
            nSuffix = *((*p).aNode.offset((*p).iOff as isize) as *const ::core::ffi::c_char
                as *mut u8_0) as ::core::ffi::c_int;
            1 as ::core::ffi::c_int
        };
        if nPrefix > (*p).term.n
            || nSuffix > (*p).nNode - (*p).iOff
            || nSuffix == 0 as ::core::ffi::c_int
        {
            return FTS_CORRUPT_VTAB;
        }
        blobGrowBuffer(&raw mut (*p).term, nPrefix + nSuffix, &raw mut rc);
        if rc == SQLITE_OK && !(*p).term.a.is_null() {
            memcpy(
                (*p).term.a.offset(nPrefix as isize) as *mut ::core::ffi::c_char
                    as *mut ::core::ffi::c_void,
                (*p).aNode.offset((*p).iOff as isize) as *const ::core::ffi::c_char
                    as *const ::core::ffi::c_void,
                nSuffix as size_t,
            );
            (*p).term.n = nPrefix + nSuffix;
            (*p).iOff += nSuffix;
            if (*p).iChild == 0 as sqlite3_int64 {
                (*p).iOff += if *((*p).aNode.offset((*p).iOff as isize)
                    as *const ::core::ffi::c_char as *mut u8_0)
                    as ::core::ffi::c_int
                    & 0x80 as ::core::ffi::c_int
                    != 0
                {
                    sqlite3Fts3GetVarint32(
                        (*p).aNode.offset((*p).iOff as isize) as *const ::core::ffi::c_char,
                        &raw mut (*p).nDoclist,
                    )
                } else {
                    (*p).nDoclist = *((*p).aNode.offset((*p).iOff as isize)
                        as *const ::core::ffi::c_char
                        as *mut u8_0) as ::core::ffi::c_int;
                    1 as ::core::ffi::c_int
                };
                if (*p).nNode - (*p).iOff < (*p).nDoclist {
                    return FTS_CORRUPT_VTAB;
                }
                (*p).aDoclist = (*p).aNode.offset((*p).iOff as isize) as *const ::core::ffi::c_char;
                (*p).iOff += (*p).nDoclist;
            }
        }
    }
    return rc;
}
unsafe extern "C" fn nodeReaderRelease(mut p: *mut NodeReader) {
    sqlite3_free((*p).term.a as *mut ::core::ffi::c_void);
}
unsafe extern "C" fn nodeReaderInit(
    mut p: *mut NodeReader,
    mut aNode: *const ::core::ffi::c_char,
    mut nNode: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    memset(
        p as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<NodeReader>() as size_t,
    );
    (*p).aNode = aNode;
    (*p).nNode = nNode;
    if !aNode.is_null()
        && *aNode.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != 0
    {
        (*p).iOff = 1 as ::core::ffi::c_int
            + sqlite3Fts3GetVarint(
                (*p).aNode.offset(1 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_char,
                &raw mut (*p).iChild,
            );
    } else {
        (*p).iOff = 1 as ::core::ffi::c_int;
    }
    return if !aNode.is_null() {
        nodeReaderNext(p)
    } else {
        SQLITE_OK
    };
}
unsafe extern "C" fn fts3IncrmergePush(
    mut p: *mut Fts3Table,
    mut pWriter: *mut IncrmergeWriter,
    mut zTerm: *const ::core::ffi::c_char,
    mut nTerm: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut iPtr: sqlite3_int64 = (*pWriter).aNodeWriter[0 as ::core::ffi::c_int as usize].iBlock;
    let mut iLayer: ::core::ffi::c_int = 0;
    iLayer = 1 as ::core::ffi::c_int;
    while iLayer < 16 as ::core::ffi::c_int {
        let mut iNextPtr: sqlite3_int64 = 0 as sqlite3_int64;
        let mut pNode: *mut NodeWriter = (&raw mut (*pWriter).aNodeWriter as *mut NodeWriter)
            .offset(iLayer as isize) as *mut NodeWriter;
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        let mut nPrefix: ::core::ffi::c_int = 0;
        let mut nSuffix: ::core::ffi::c_int = 0;
        let mut nSpace: ::core::ffi::c_int = 0;
        nPrefix = fts3PrefixCompress((*pNode).key.a, (*pNode).key.n, zTerm, nTerm);
        nSuffix = nTerm - nPrefix;
        if nSuffix <= 0 as ::core::ffi::c_int {
            return FTS_CORRUPT_VTAB;
        }
        nSpace = sqlite3Fts3VarintLen(nPrefix as sqlite3_uint64);
        nSpace += sqlite3Fts3VarintLen(nSuffix as sqlite3_uint64) + nSuffix;
        if (*pNode).key.n == 0 as ::core::ffi::c_int || (*pNode).block.n + nSpace <= (*p).nNodeSize
        {
            let mut pBlk: *mut Blob = &raw mut (*pNode).block;
            if (*pBlk).n == 0 as ::core::ffi::c_int {
                blobGrowBuffer(pBlk, (*p).nNodeSize, &raw mut rc);
                if rc == SQLITE_OK {
                    *(*pBlk).a.offset(0 as ::core::ffi::c_int as isize) =
                        iLayer as ::core::ffi::c_char;
                    (*pBlk).n = 1 as ::core::ffi::c_int
                        + sqlite3Fts3PutVarint(
                            (*pBlk).a.offset(1 as ::core::ffi::c_int as isize)
                                as *mut ::core::ffi::c_char,
                            iPtr,
                        );
                }
            }
            blobGrowBuffer(pBlk, (*pBlk).n + nSpace, &raw mut rc);
            blobGrowBuffer(&raw mut (*pNode).key, nTerm, &raw mut rc);
            if rc == SQLITE_OK {
                if (*pNode).key.n != 0 {
                    (*pBlk).n += sqlite3Fts3PutVarint(
                        (*pBlk).a.offset((*pBlk).n as isize) as *mut ::core::ffi::c_char,
                        nPrefix as sqlite3_int64,
                    );
                }
                (*pBlk).n += sqlite3Fts3PutVarint(
                    (*pBlk).a.offset((*pBlk).n as isize) as *mut ::core::ffi::c_char,
                    nSuffix as sqlite3_int64,
                );
                memcpy(
                    (*pBlk).a.offset((*pBlk).n as isize) as *mut ::core::ffi::c_char
                        as *mut ::core::ffi::c_void,
                    zTerm.offset(nPrefix as isize) as *const ::core::ffi::c_char
                        as *const ::core::ffi::c_void,
                    nSuffix as size_t,
                );
                (*pBlk).n += nSuffix;
                memcpy(
                    (*pNode).key.a as *mut ::core::ffi::c_void,
                    zTerm as *const ::core::ffi::c_void,
                    nTerm as size_t,
                );
                (*pNode).key.n = nTerm;
            }
        } else {
            rc = fts3WriteSegment(p, (*pNode).iBlock, (*pNode).block.a, (*pNode).block.n);
            *(*pNode).block.a.offset(0 as ::core::ffi::c_int as isize) =
                iLayer as ::core::ffi::c_char;
            (*pNode).block.n = 1 as ::core::ffi::c_int
                + sqlite3Fts3PutVarint(
                    (*pNode).block.a.offset(1 as ::core::ffi::c_int as isize)
                        as *mut ::core::ffi::c_char,
                    iPtr + 1 as sqlite3_int64,
                );
            iNextPtr = (*pNode).iBlock;
            (*pNode).iBlock += 1;
            (*pNode).key.n = 0 as ::core::ffi::c_int;
        }
        if rc != SQLITE_OK || iNextPtr == 0 as sqlite3_int64 {
            return rc;
        }
        iPtr = iNextPtr;
        iLayer += 1;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn fts3AppendToNode(
    mut pNode: *mut Blob,
    mut pPrev: *mut Blob,
    mut zTerm: *const ::core::ffi::c_char,
    mut nTerm: ::core::ffi::c_int,
    mut aDoclist: *const ::core::ffi::c_char,
    mut nDoclist: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = SQLITE_OK;
    let mut bFirst: ::core::ffi::c_int =
        ((*pPrev).n == 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
    let mut nPrefix: ::core::ffi::c_int = 0;
    let mut nSuffix: ::core::ffi::c_int = 0;
    blobGrowBuffer(pPrev, nTerm, &raw mut rc);
    if rc != SQLITE_OK {
        return rc;
    }
    nPrefix = fts3PrefixCompress((*pPrev).a, (*pPrev).n, zTerm, nTerm);
    nSuffix = nTerm - nPrefix;
    if nSuffix <= 0 as ::core::ffi::c_int {
        return FTS_CORRUPT_VTAB;
    }
    memcpy(
        (*pPrev).a as *mut ::core::ffi::c_void,
        zTerm as *const ::core::ffi::c_void,
        nTerm as size_t,
    );
    (*pPrev).n = nTerm;
    if bFirst == 0 as ::core::ffi::c_int {
        (*pNode).n += sqlite3Fts3PutVarint(
            (*pNode).a.offset((*pNode).n as isize) as *mut ::core::ffi::c_char,
            nPrefix as sqlite3_int64,
        );
    }
    (*pNode).n += sqlite3Fts3PutVarint(
        (*pNode).a.offset((*pNode).n as isize) as *mut ::core::ffi::c_char,
        nSuffix as sqlite3_int64,
    );
    memcpy(
        (*pNode).a.offset((*pNode).n as isize) as *mut ::core::ffi::c_char
            as *mut ::core::ffi::c_void,
        zTerm.offset(nPrefix as isize) as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
        nSuffix as size_t,
    );
    (*pNode).n += nSuffix;
    if !aDoclist.is_null() {
        (*pNode).n += sqlite3Fts3PutVarint(
            (*pNode).a.offset((*pNode).n as isize) as *mut ::core::ffi::c_char,
            nDoclist as sqlite3_int64,
        );
        memcpy(
            (*pNode).a.offset((*pNode).n as isize) as *mut ::core::ffi::c_char
                as *mut ::core::ffi::c_void,
            aDoclist as *const ::core::ffi::c_void,
            nDoclist as size_t,
        );
        (*pNode).n += nDoclist;
    }
    return SQLITE_OK;
}
unsafe extern "C" fn fts3IncrmergeAppend(
    mut p: *mut Fts3Table,
    mut pWriter: *mut IncrmergeWriter,
    mut pCsr: *mut Fts3MultiSegReader,
) -> ::core::ffi::c_int {
    let mut zTerm: *const ::core::ffi::c_char = (*pCsr).zTerm;
    let mut nTerm: ::core::ffi::c_int = (*pCsr).nTerm;
    let mut aDoclist: *const ::core::ffi::c_char = (*pCsr).aDoclist;
    let mut nDoclist: ::core::ffi::c_int = (*pCsr).nDoclist;
    let mut rc: ::core::ffi::c_int = SQLITE_OK;
    let mut nSpace: ::core::ffi::c_int = 0;
    let mut nPrefix: ::core::ffi::c_int = 0;
    let mut nSuffix: ::core::ffi::c_int = 0;
    let mut pLeaf: *mut NodeWriter = ::core::ptr::null_mut::<NodeWriter>();
    pLeaf = (&raw mut (*pWriter).aNodeWriter as *mut NodeWriter)
        .offset(0 as ::core::ffi::c_int as isize) as *mut NodeWriter;
    nPrefix = fts3PrefixCompress((*pLeaf).key.a, (*pLeaf).key.n, zTerm, nTerm);
    nSuffix = nTerm - nPrefix;
    if nSuffix <= 0 as ::core::ffi::c_int {
        return FTS_CORRUPT_VTAB;
    }
    nSpace = sqlite3Fts3VarintLen(nPrefix as sqlite3_uint64);
    nSpace += sqlite3Fts3VarintLen(nSuffix as sqlite3_uint64) + nSuffix;
    nSpace += sqlite3Fts3VarintLen(nDoclist as sqlite3_uint64) + nDoclist;
    if (*pLeaf).block.n > 0 as ::core::ffi::c_int
        && (*pLeaf).block.n + nSpace > (*p).nNodeSize
        && (*pLeaf).iBlock < (*pWriter).iStart as i64_0 + (*pWriter).nLeafEst
    {
        rc = fts3WriteSegment(p, (*pLeaf).iBlock, (*pLeaf).block.a, (*pLeaf).block.n);
        (*pWriter).nWork += 1;
        if rc == SQLITE_OK {
            rc = fts3IncrmergePush(p, pWriter, zTerm, nPrefix + 1 as ::core::ffi::c_int);
        }
        (*pLeaf).iBlock += 1;
        (*pLeaf).key.n = 0 as ::core::ffi::c_int;
        (*pLeaf).block.n = 0 as ::core::ffi::c_int;
        nSuffix = nTerm;
        nSpace = 1 as ::core::ffi::c_int;
        nSpace += sqlite3Fts3VarintLen(nSuffix as sqlite3_uint64) + nSuffix;
        nSpace += sqlite3Fts3VarintLen(nDoclist as sqlite3_uint64) + nDoclist;
    }
    (*pWriter).nLeafData += nSpace as sqlite3_int64;
    blobGrowBuffer(
        &raw mut (*pLeaf).block,
        (*pLeaf).block.n + nSpace,
        &raw mut rc,
    );
    if rc == SQLITE_OK {
        if (*pLeaf).block.n == 0 as ::core::ffi::c_int {
            (*pLeaf).block.n = 1 as ::core::ffi::c_int;
            *(*pLeaf).block.a.offset(0 as ::core::ffi::c_int as isize) =
                '\0' as i32 as ::core::ffi::c_char;
        }
        rc = fts3AppendToNode(
            &raw mut (*pLeaf).block,
            &raw mut (*pLeaf).key,
            zTerm,
            nTerm,
            aDoclist,
            nDoclist,
        );
    }
    return rc;
}
unsafe extern "C" fn fts3IncrmergeRelease(
    mut p: *mut Fts3Table,
    mut pWriter: *mut IncrmergeWriter,
    mut pRc: *mut ::core::ffi::c_int,
) {
    let mut i: ::core::ffi::c_int = 0;
    let mut iRoot: ::core::ffi::c_int = 0;
    let mut pRoot: *mut NodeWriter = ::core::ptr::null_mut::<NodeWriter>();
    let mut rc: ::core::ffi::c_int = *pRc;
    iRoot = FTS_MAX_APPENDABLE_HEIGHT - 1 as ::core::ffi::c_int;
    while iRoot >= 0 as ::core::ffi::c_int {
        let mut pNode: *mut NodeWriter = (&raw mut (*pWriter).aNodeWriter as *mut NodeWriter)
            .offset(iRoot as isize) as *mut NodeWriter;
        if (*pNode).block.n > 0 as ::core::ffi::c_int {
            break;
        }
        sqlite3_free((*pNode).block.a as *mut ::core::ffi::c_void);
        sqlite3_free((*pNode).key.a as *mut ::core::ffi::c_void);
        iRoot -= 1;
    }
    if iRoot < 0 as ::core::ffi::c_int {
        return;
    }
    if iRoot == 0 as ::core::ffi::c_int {
        let mut pBlock: *mut Blob = &raw mut (*(&raw mut (*pWriter).aNodeWriter
            as *mut NodeWriter)
            .offset(1 as ::core::ffi::c_int as isize))
        .block;
        blobGrowBuffer(
            pBlock,
            1 as ::core::ffi::c_int + FTS3_VARINT_MAX,
            &raw mut rc,
        );
        if rc == SQLITE_OK {
            *(*pBlock).a.offset(0 as ::core::ffi::c_int as isize) = 0x1 as ::core::ffi::c_char;
            (*pBlock).n = 1 as ::core::ffi::c_int
                + sqlite3Fts3PutVarint(
                    (*pBlock).a.offset(1 as ::core::ffi::c_int as isize)
                        as *mut ::core::ffi::c_char,
                    (*pWriter).aNodeWriter[0 as ::core::ffi::c_int as usize].iBlock,
                );
        }
        iRoot = 1 as ::core::ffi::c_int;
    }
    pRoot = (&raw mut (*pWriter).aNodeWriter as *mut NodeWriter).offset(iRoot as isize)
        as *mut NodeWriter;
    i = 0 as ::core::ffi::c_int;
    while i < iRoot {
        let mut pNode_0: *mut NodeWriter = (&raw mut (*pWriter).aNodeWriter as *mut NodeWriter)
            .offset(i as isize) as *mut NodeWriter;
        if (*pNode_0).block.n > 0 as ::core::ffi::c_int && rc == SQLITE_OK {
            rc = fts3WriteSegment(p, (*pNode_0).iBlock, (*pNode_0).block.a, (*pNode_0).block.n);
        }
        sqlite3_free((*pNode_0).block.a as *mut ::core::ffi::c_void);
        sqlite3_free((*pNode_0).key.a as *mut ::core::ffi::c_void);
        i += 1;
    }
    if rc == SQLITE_OK {
        rc = fts3WriteSegdir(
            p,
            (*pWriter).iAbsLevel + 1 as sqlite3_int64,
            (*pWriter).iIdx,
            (*pWriter).iStart,
            (*pWriter).aNodeWriter[0 as ::core::ffi::c_int as usize].iBlock,
            (*pWriter).iEnd,
            if (*pWriter).bNoLeafData as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                (*pWriter).nLeafData
            } else {
                0 as sqlite3_int64
            },
            (*pRoot).block.a,
            (*pRoot).block.n,
        );
    }
    sqlite3_free((*pRoot).block.a as *mut ::core::ffi::c_void);
    sqlite3_free((*pRoot).key.a as *mut ::core::ffi::c_void);
    *pRc = rc;
}
unsafe extern "C" fn fts3TermCmp(
    mut zLhs: *const ::core::ffi::c_char,
    mut nLhs: ::core::ffi::c_int,
    mut zRhs: *const ::core::ffi::c_char,
    mut nRhs: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut nCmp: ::core::ffi::c_int = if nLhs < nRhs { nLhs } else { nRhs };
    let mut res: ::core::ffi::c_int = 0;
    if nCmp != 0 && !zLhs.is_null() && !zRhs.is_null() {
        res = memcmp(
            zLhs as *const ::core::ffi::c_void,
            zRhs as *const ::core::ffi::c_void,
            nCmp as size_t,
        );
    } else {
        res = 0 as ::core::ffi::c_int;
    }
    if res == 0 as ::core::ffi::c_int {
        res = nLhs - nRhs;
    }
    return res;
}
unsafe extern "C" fn fts3IsAppendable(
    mut p: *mut Fts3Table,
    mut iEnd: sqlite3_int64,
    mut pbRes: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut bRes: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut pCheck: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut rc: ::core::ffi::c_int = 0;
    rc = fts3SqlStmt(
        p,
        SQL_SEGMENT_IS_APPENDABLE,
        &raw mut pCheck,
        ::core::ptr::null_mut::<*mut sqlite3_value>(),
    );
    if rc == SQLITE_OK {
        sqlite3_bind_int64(pCheck, 1 as ::core::ffi::c_int, iEnd);
        if SQLITE_ROW == sqlite3_step(pCheck) {
            bRes = 1 as ::core::ffi::c_int;
        }
        rc = sqlite3_reset(pCheck);
    }
    *pbRes = bRes;
    return rc;
}
unsafe extern "C" fn fts3IncrmergeLoad(
    mut p: *mut Fts3Table,
    mut iAbsLevel: sqlite3_int64,
    mut iIdx: ::core::ffi::c_int,
    mut zKey: *const ::core::ffi::c_char,
    mut nKey: ::core::ffi::c_int,
    mut pWriter: *mut IncrmergeWriter,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = 0;
    let mut pSelect: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    rc = fts3SqlStmt(
        p,
        SQL_SELECT_SEGDIR,
        &raw mut pSelect,
        ::core::ptr::null_mut::<*mut sqlite3_value>(),
    );
    if rc == SQLITE_OK {
        let mut iStart: sqlite3_int64 = 0 as sqlite3_int64;
        let mut iLeafEnd: sqlite3_int64 = 0 as sqlite3_int64;
        let mut iEnd: sqlite3_int64 = 0 as sqlite3_int64;
        let mut aRoot: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
        let mut nRoot: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut rc2: ::core::ffi::c_int = 0;
        let mut bAppendable: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        sqlite3_bind_int64(
            pSelect,
            1 as ::core::ffi::c_int,
            iAbsLevel + 1 as sqlite3_int64,
        );
        sqlite3_bind_int(pSelect, 2 as ::core::ffi::c_int, iIdx);
        if sqlite3_step(pSelect) == SQLITE_ROW {
            iStart = sqlite3_column_int64(pSelect, 1 as ::core::ffi::c_int);
            iLeafEnd = sqlite3_column_int64(pSelect, 2 as ::core::ffi::c_int);
            fts3ReadEndBlockField(
                pSelect,
                3 as ::core::ffi::c_int,
                &raw mut iEnd,
                &raw mut (*pWriter).nLeafData,
            );
            if (*pWriter).nLeafData < 0 as sqlite3_int64 {
                (*pWriter).nLeafData =
                    (*pWriter).nLeafData * -(1 as ::core::ffi::c_int) as sqlite3_int64;
            }
            (*pWriter).bNoLeafData =
                ((*pWriter).nLeafData == 0 as sqlite3_int64) as ::core::ffi::c_int as u8_0;
            nRoot = sqlite3_column_bytes(pSelect, 4 as ::core::ffi::c_int);
            aRoot =
                sqlite3_column_blob(pSelect, 4 as ::core::ffi::c_int) as *const ::core::ffi::c_char;
            if aRoot.is_null() {
                sqlite3_reset(pSelect);
                return if nRoot != 0 {
                    SQLITE_NOMEM
                } else {
                    FTS_CORRUPT_VTAB
                };
            }
        } else {
            return sqlite3_reset(pSelect);
        }
        rc = fts3IsAppendable(p, iEnd, &raw mut bAppendable);
        if rc == SQLITE_OK && bAppendable != 0 {
            let mut aLeaf: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            let mut nLeaf: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            rc = sqlite3Fts3ReadBlock(
                p,
                iLeafEnd,
                &raw mut aLeaf,
                &raw mut nLeaf,
                ::core::ptr::null_mut::<::core::ffi::c_int>(),
            );
            if rc == SQLITE_OK {
                let mut reader: NodeReader = NodeReader {
                    aNode: ::core::ptr::null::<::core::ffi::c_char>(),
                    nNode: 0,
                    iOff: 0,
                    iChild: 0,
                    term: Blob {
                        a: ::core::ptr::null_mut::<::core::ffi::c_char>(),
                        n: 0,
                        nAlloc: 0,
                    },
                    aDoclist: ::core::ptr::null::<::core::ffi::c_char>(),
                    nDoclist: 0,
                };
                rc = nodeReaderInit(&raw mut reader, aLeaf, nLeaf);
                while rc == SQLITE_OK && !reader.aNode.is_null() {
                    rc = nodeReaderNext(&raw mut reader);
                }
                if fts3TermCmp(zKey, nKey, reader.term.a, reader.term.n) <= 0 as ::core::ffi::c_int
                {
                    bAppendable = 0 as ::core::ffi::c_int;
                }
                nodeReaderRelease(&raw mut reader);
            }
            sqlite3_free(aLeaf as *mut ::core::ffi::c_void);
        }
        if rc == SQLITE_OK && bAppendable != 0 {
            let mut i: ::core::ffi::c_int = 0;
            let mut nHeight: ::core::ffi::c_int =
                *aRoot.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
            let mut pNode: *mut NodeWriter = ::core::ptr::null_mut::<NodeWriter>();
            if nHeight < 1 as ::core::ffi::c_int || nHeight >= FTS_MAX_APPENDABLE_HEIGHT {
                sqlite3_reset(pSelect);
                return FTS_CORRUPT_VTAB;
            }
            (*pWriter).nLeafEst = ((iEnd - iStart + 1 as sqlite3_int64) as ::core::ffi::c_int
                / FTS_MAX_APPENDABLE_HEIGHT) as i64_0;
            (*pWriter).iStart = iStart;
            (*pWriter).iEnd = iEnd;
            (*pWriter).iAbsLevel = iAbsLevel;
            (*pWriter).iIdx = iIdx;
            i = nHeight + 1 as ::core::ffi::c_int;
            while i < FTS_MAX_APPENDABLE_HEIGHT {
                (*pWriter).aNodeWriter[i as usize].iBlock = ((*pWriter).iStart as i64_0
                    + i as i64_0 * (*pWriter).nLeafEst)
                    as sqlite3_int64;
                i += 1;
            }
            pNode = (&raw mut (*pWriter).aNodeWriter as *mut NodeWriter).offset(nHeight as isize)
                as *mut NodeWriter;
            (*pNode).iBlock = ((*pWriter).iStart as i64_0 + (*pWriter).nLeafEst * nHeight as i64_0)
                as sqlite3_int64;
            blobGrowBuffer(
                &raw mut (*pNode).block,
                (if nRoot > (*p).nNodeSize {
                    nRoot
                } else {
                    (*p).nNodeSize
                }) + FTS3_NODE_PADDING,
                &raw mut rc,
            );
            if rc == SQLITE_OK {
                memcpy(
                    (*pNode).block.a as *mut ::core::ffi::c_void,
                    aRoot as *const ::core::ffi::c_void,
                    nRoot as size_t,
                );
                (*pNode).block.n = nRoot;
                memset(
                    (*pNode).block.a.offset(nRoot as isize) as *mut ::core::ffi::c_char
                        as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    FTS3_NODE_PADDING as size_t,
                );
            }
            i = nHeight;
            while i >= 0 as ::core::ffi::c_int && rc == SQLITE_OK {
                let mut reader_0: NodeReader = NodeReader {
                    aNode: ::core::ptr::null::<::core::ffi::c_char>(),
                    nNode: 0,
                    iOff: 0,
                    iChild: 0,
                    term: Blob {
                        a: ::core::ptr::null_mut::<::core::ffi::c_char>(),
                        n: 0,
                        nAlloc: 0,
                    },
                    aDoclist: ::core::ptr::null::<::core::ffi::c_char>(),
                    nDoclist: 0,
                };
                memset(
                    &raw mut reader_0 as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    ::core::mem::size_of::<NodeReader>() as size_t,
                );
                pNode = (&raw mut (*pWriter).aNodeWriter as *mut NodeWriter).offset(i as isize)
                    as *mut NodeWriter;
                if !(*pNode).block.a.is_null() {
                    rc = nodeReaderInit(&raw mut reader_0, (*pNode).block.a, (*pNode).block.n);
                    while !reader_0.aNode.is_null() && rc == SQLITE_OK {
                        rc = nodeReaderNext(&raw mut reader_0);
                    }
                    blobGrowBuffer(&raw mut (*pNode).key, reader_0.term.n, &raw mut rc);
                    if rc == SQLITE_OK {
                        if reader_0.term.n > 0 as ::core::ffi::c_int {
                            memcpy(
                                (*pNode).key.a as *mut ::core::ffi::c_void,
                                reader_0.term.a as *const ::core::ffi::c_void,
                                reader_0.term.n as size_t,
                            );
                        }
                        (*pNode).key.n = reader_0.term.n;
                        if i > 0 as ::core::ffi::c_int {
                            let mut aBlock: *mut ::core::ffi::c_char =
                                ::core::ptr::null_mut::<::core::ffi::c_char>();
                            let mut nBlock: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                            pNode = (&raw mut (*pWriter).aNodeWriter as *mut NodeWriter)
                                .offset((i - 1 as ::core::ffi::c_int) as isize)
                                as *mut NodeWriter;
                            (*pNode).iBlock = reader_0.iChild;
                            rc = sqlite3Fts3ReadBlock(
                                p,
                                reader_0.iChild,
                                &raw mut aBlock,
                                &raw mut nBlock,
                                ::core::ptr::null_mut::<::core::ffi::c_int>(),
                            );
                            blobGrowBuffer(
                                &raw mut (*pNode).block,
                                (if nBlock > (*p).nNodeSize {
                                    nBlock
                                } else {
                                    (*p).nNodeSize
                                }) + FTS3_NODE_PADDING,
                                &raw mut rc,
                            );
                            if rc == SQLITE_OK {
                                memcpy(
                                    (*pNode).block.a as *mut ::core::ffi::c_void,
                                    aBlock as *const ::core::ffi::c_void,
                                    nBlock as size_t,
                                );
                                (*pNode).block.n = nBlock;
                                memset(
                                    (*pNode).block.a.offset(nBlock as isize)
                                        as *mut ::core::ffi::c_char
                                        as *mut ::core::ffi::c_void,
                                    0 as ::core::ffi::c_int,
                                    FTS3_NODE_PADDING as size_t,
                                );
                            }
                            sqlite3_free(aBlock as *mut ::core::ffi::c_void);
                        }
                    }
                }
                nodeReaderRelease(&raw mut reader_0);
                i -= 1;
            }
        }
        rc2 = sqlite3_reset(pSelect);
        if rc == SQLITE_OK {
            rc = rc2;
        }
    }
    return rc;
}
unsafe extern "C" fn fts3IncrmergeOutputIdx(
    mut p: *mut Fts3Table,
    mut iAbsLevel: sqlite3_int64,
    mut piIdx: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = 0;
    let mut pOutputIdx: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    rc = fts3SqlStmt(
        p,
        SQL_NEXT_SEGMENT_INDEX,
        &raw mut pOutputIdx,
        ::core::ptr::null_mut::<*mut sqlite3_value>(),
    );
    if rc == SQLITE_OK {
        sqlite3_bind_int64(
            pOutputIdx,
            1 as ::core::ffi::c_int,
            iAbsLevel + 1 as sqlite3_int64,
        );
        sqlite3_step(pOutputIdx);
        *piIdx = sqlite3_column_int(pOutputIdx, 0 as ::core::ffi::c_int);
        rc = sqlite3_reset(pOutputIdx);
    }
    return rc;
}
unsafe extern "C" fn fts3IncrmergeWriter(
    mut p: *mut Fts3Table,
    mut iAbsLevel: sqlite3_int64,
    mut iIdx: ::core::ffi::c_int,
    mut pCsr: *mut Fts3MultiSegReader,
    mut pWriter: *mut IncrmergeWriter,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut nLeafEst: i64_0 = 0 as i64_0;
    let mut pLeafEst: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut pFirstBlock: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    rc = fts3SqlStmt(
        p,
        SQL_MAX_LEAF_NODE_ESTIMATE,
        &raw mut pLeafEst,
        ::core::ptr::null_mut::<*mut sqlite3_value>(),
    );
    if rc == SQLITE_OK {
        sqlite3_bind_int64(pLeafEst, 1 as ::core::ffi::c_int, iAbsLevel);
        sqlite3_bind_int64(
            pLeafEst,
            2 as ::core::ffi::c_int,
            (*pCsr).nSegment as sqlite3_int64,
        );
        if SQLITE_ROW == sqlite3_step(pLeafEst) {
            nLeafEst = sqlite3_column_int64(pLeafEst, 0 as ::core::ffi::c_int) as i64_0;
        }
        rc = sqlite3_reset(pLeafEst);
    }
    if rc != SQLITE_OK {
        return rc;
    }
    rc = fts3SqlStmt(
        p,
        SQL_NEXT_SEGMENTS_ID,
        &raw mut pFirstBlock,
        ::core::ptr::null_mut::<*mut sqlite3_value>(),
    );
    if rc == SQLITE_OK {
        if SQLITE_ROW == sqlite3_step(pFirstBlock) {
            (*pWriter).iStart = sqlite3_column_int64(pFirstBlock, 0 as ::core::ffi::c_int);
            (*pWriter).iEnd = (*pWriter).iStart - 1 as sqlite3_int64;
            (*pWriter).iEnd += (nLeafEst * FTS_MAX_APPENDABLE_HEIGHT as i64_0) as sqlite3_int64;
        }
        rc = sqlite3_reset(pFirstBlock);
    }
    if rc != SQLITE_OK {
        return rc;
    }
    rc = fts3WriteSegment(
        p,
        (*pWriter).iEnd,
        ::core::ptr::null_mut::<::core::ffi::c_char>(),
        0 as ::core::ffi::c_int,
    );
    if rc != SQLITE_OK {
        return rc;
    }
    (*pWriter).iAbsLevel = iAbsLevel;
    (*pWriter).nLeafEst = nLeafEst;
    (*pWriter).iIdx = iIdx;
    i = 0 as ::core::ffi::c_int;
    while i < FTS_MAX_APPENDABLE_HEIGHT {
        (*pWriter).aNodeWriter[i as usize].iBlock =
            ((*pWriter).iStart as i64_0 + i as i64_0 * (*pWriter).nLeafEst) as sqlite3_int64;
        i += 1;
    }
    return SQLITE_OK;
}
unsafe extern "C" fn fts3RemoveSegdirEntry(
    mut p: *mut Fts3Table,
    mut iAbsLevel: sqlite3_int64,
    mut iIdx: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = 0;
    let mut pDelete: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    rc = fts3SqlStmt(
        p,
        SQL_DELETE_SEGDIR_ENTRY,
        &raw mut pDelete,
        ::core::ptr::null_mut::<*mut sqlite3_value>(),
    );
    if rc == SQLITE_OK {
        sqlite3_bind_int64(pDelete, 1 as ::core::ffi::c_int, iAbsLevel);
        sqlite3_bind_int(pDelete, 2 as ::core::ffi::c_int, iIdx);
        sqlite3_step(pDelete);
        rc = sqlite3_reset(pDelete);
    }
    return rc;
}
unsafe extern "C" fn fts3RepackSegdirLevel(
    mut p: *mut Fts3Table,
    mut iAbsLevel: sqlite3_int64,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = 0;
    let mut aIdx: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut nIdx: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut nAlloc: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0;
    let mut pSelect: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut pUpdate: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    rc = fts3SqlStmt(
        p,
        SQL_SELECT_INDEXES,
        &raw mut pSelect,
        ::core::ptr::null_mut::<*mut sqlite3_value>(),
    );
    if rc == SQLITE_OK {
        let mut rc2: ::core::ffi::c_int = 0;
        sqlite3_bind_int64(pSelect, 1 as ::core::ffi::c_int, iAbsLevel);
        while SQLITE_ROW == sqlite3_step(pSelect) {
            if nIdx >= nAlloc {
                let mut aNew: *mut ::core::ffi::c_int =
                    ::core::ptr::null_mut::<::core::ffi::c_int>();
                nAlloc += 16 as ::core::ffi::c_int;
                aNew = sqlite3_realloc64(
                    aIdx as *mut ::core::ffi::c_void,
                    (nAlloc as usize)
                        .wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>() as usize)
                        as sqlite3_uint64,
                ) as *mut ::core::ffi::c_int;
                if aNew.is_null() {
                    rc = SQLITE_NOMEM;
                    break;
                } else {
                    aIdx = aNew;
                }
            }
            let fresh13 = nIdx;
            nIdx = nIdx + 1;
            *aIdx.offset(fresh13 as isize) = sqlite3_column_int(pSelect, 0 as ::core::ffi::c_int);
        }
        rc2 = sqlite3_reset(pSelect);
        if rc == SQLITE_OK {
            rc = rc2;
        }
    }
    if rc == SQLITE_OK {
        rc = fts3SqlStmt(
            p,
            SQL_SHIFT_SEGDIR_ENTRY,
            &raw mut pUpdate,
            ::core::ptr::null_mut::<*mut sqlite3_value>(),
        );
    }
    if rc == SQLITE_OK {
        sqlite3_bind_int64(pUpdate, 2 as ::core::ffi::c_int, iAbsLevel);
    }
    (*p).bIgnoreSavepoint = 1 as u8_0;
    i = 0 as ::core::ffi::c_int;
    while rc == SQLITE_OK && i < nIdx {
        if *aIdx.offset(i as isize) != i {
            sqlite3_bind_int(pUpdate, 3 as ::core::ffi::c_int, *aIdx.offset(i as isize));
            sqlite3_bind_int(pUpdate, 1 as ::core::ffi::c_int, i);
            sqlite3_step(pUpdate);
            rc = sqlite3_reset(pUpdate);
        }
        i += 1;
    }
    (*p).bIgnoreSavepoint = 0 as u8_0;
    sqlite3_free(aIdx as *mut ::core::ffi::c_void);
    return rc;
}
unsafe extern "C" fn fts3StartNode(
    mut pNode: *mut Blob,
    mut iHeight: ::core::ffi::c_int,
    mut iChild: sqlite3_int64,
) {
    *(*pNode).a.offset(0 as ::core::ffi::c_int as isize) = iHeight as ::core::ffi::c_char;
    if iChild != 0 {
        (*pNode).n = 1 as ::core::ffi::c_int
            + sqlite3Fts3PutVarint(
                (*pNode).a.offset(1 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_char,
                iChild,
            );
    } else {
        (*pNode).n = 1 as ::core::ffi::c_int;
    };
}
unsafe extern "C" fn fts3TruncateNode(
    mut aNode: *const ::core::ffi::c_char,
    mut nNode: ::core::ffi::c_int,
    mut pNew: *mut Blob,
    mut zTerm: *const ::core::ffi::c_char,
    mut nTerm: ::core::ffi::c_int,
    mut piBlock: *mut sqlite3_int64,
) -> ::core::ffi::c_int {
    let mut reader: NodeReader = NodeReader {
        aNode: ::core::ptr::null::<::core::ffi::c_char>(),
        nNode: 0,
        iOff: 0,
        iChild: 0,
        term: Blob {
            a: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            n: 0,
            nAlloc: 0,
        },
        aDoclist: ::core::ptr::null::<::core::ffi::c_char>(),
        nDoclist: 0,
    };
    let mut prev: Blob = Blob {
        a: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        n: 0 as ::core::ffi::c_int,
        nAlloc: 0 as ::core::ffi::c_int,
    };
    let mut rc: ::core::ffi::c_int = SQLITE_OK;
    let mut bLeaf: ::core::ffi::c_int = 0;
    if nNode < 1 as ::core::ffi::c_int {
        return FTS_CORRUPT_VTAB;
    }
    bLeaf = (*aNode.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '\0' as i32)
        as ::core::ffi::c_int;
    blobGrowBuffer(pNew, nNode, &raw mut rc);
    if rc != SQLITE_OK {
        return rc;
    }
    (*pNew).n = 0 as ::core::ffi::c_int;
    let mut current_block_10: u64;
    rc = nodeReaderInit(&raw mut reader, aNode, nNode);
    while rc == SQLITE_OK && !reader.aNode.is_null() {
        if (*pNew).n == 0 as ::core::ffi::c_int {
            let mut res: ::core::ffi::c_int =
                fts3TermCmp(reader.term.a, reader.term.n, zTerm, nTerm);
            if res < 0 as ::core::ffi::c_int
                || bLeaf == 0 as ::core::ffi::c_int && res == 0 as ::core::ffi::c_int
            {
                current_block_10 = 4906268039856690917;
            } else {
                fts3StartNode(
                    pNew,
                    *aNode.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
                    reader.iChild,
                );
                *piBlock = reader.iChild;
                current_block_10 = 12209867499936983673;
            }
        } else {
            current_block_10 = 12209867499936983673;
        }
        match current_block_10 {
            12209867499936983673 => {
                rc = fts3AppendToNode(
                    pNew,
                    &raw mut prev,
                    reader.term.a,
                    reader.term.n,
                    reader.aDoclist,
                    reader.nDoclist,
                );
                if rc != SQLITE_OK {
                    break;
                }
            }
            _ => {}
        }
        rc = nodeReaderNext(&raw mut reader);
    }
    if (*pNew).n == 0 as ::core::ffi::c_int {
        fts3StartNode(
            pNew,
            *aNode.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
            reader.iChild,
        );
        *piBlock = reader.iChild;
    }
    nodeReaderRelease(&raw mut reader);
    sqlite3_free(prev.a as *mut ::core::ffi::c_void);
    return rc;
}
unsafe extern "C" fn fts3TruncateSegment(
    mut p: *mut Fts3Table,
    mut iAbsLevel: sqlite3_int64,
    mut iIdx: ::core::ffi::c_int,
    mut zTerm: *const ::core::ffi::c_char,
    mut nTerm: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = SQLITE_OK;
    let mut root: Blob = Blob {
        a: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        n: 0 as ::core::ffi::c_int,
        nAlloc: 0 as ::core::ffi::c_int,
    };
    let mut block: Blob = Blob {
        a: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        n: 0 as ::core::ffi::c_int,
        nAlloc: 0 as ::core::ffi::c_int,
    };
    let mut iBlock: sqlite3_int64 = 0 as sqlite3_int64;
    let mut iNewStart: sqlite3_int64 = 0 as sqlite3_int64;
    let mut iOldStart: sqlite3_int64 = 0 as sqlite3_int64;
    let mut pFetch: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    rc = fts3SqlStmt(
        p,
        SQL_SELECT_SEGDIR,
        &raw mut pFetch,
        ::core::ptr::null_mut::<*mut sqlite3_value>(),
    );
    if rc == SQLITE_OK {
        let mut rc2: ::core::ffi::c_int = 0;
        sqlite3_bind_int64(pFetch, 1 as ::core::ffi::c_int, iAbsLevel);
        sqlite3_bind_int(pFetch, 2 as ::core::ffi::c_int, iIdx);
        if SQLITE_ROW == sqlite3_step(pFetch) {
            let mut aRoot: *const ::core::ffi::c_char =
                sqlite3_column_blob(pFetch, 4 as ::core::ffi::c_int) as *const ::core::ffi::c_char;
            let mut nRoot: ::core::ffi::c_int =
                sqlite3_column_bytes(pFetch, 4 as ::core::ffi::c_int);
            iOldStart = sqlite3_column_int64(pFetch, 1 as ::core::ffi::c_int);
            rc = fts3TruncateNode(aRoot, nRoot, &raw mut root, zTerm, nTerm, &raw mut iBlock);
        }
        rc2 = sqlite3_reset(pFetch);
        if rc == SQLITE_OK {
            rc = rc2;
        }
    }
    while rc == SQLITE_OK && iBlock != 0 {
        let mut aBlock: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut nBlock: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        iNewStart = iBlock;
        rc = sqlite3Fts3ReadBlock(
            p,
            iBlock,
            &raw mut aBlock,
            &raw mut nBlock,
            ::core::ptr::null_mut::<::core::ffi::c_int>(),
        );
        if rc == SQLITE_OK {
            rc = fts3TruncateNode(
                aBlock,
                nBlock,
                &raw mut block,
                zTerm,
                nTerm,
                &raw mut iBlock,
            );
        }
        if rc == SQLITE_OK {
            rc = fts3WriteSegment(p, iNewStart, block.a, block.n);
        }
        sqlite3_free(aBlock as *mut ::core::ffi::c_void);
    }
    if rc == SQLITE_OK && iNewStart != 0 {
        let mut pDel: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
        rc = fts3SqlStmt(
            p,
            SQL_DELETE_SEGMENTS_RANGE,
            &raw mut pDel,
            ::core::ptr::null_mut::<*mut sqlite3_value>(),
        );
        if rc == SQLITE_OK {
            sqlite3_bind_int64(pDel, 1 as ::core::ffi::c_int, iOldStart);
            sqlite3_bind_int64(
                pDel,
                2 as ::core::ffi::c_int,
                iNewStart - 1 as sqlite3_int64,
            );
            sqlite3_step(pDel);
            rc = sqlite3_reset(pDel);
        }
    }
    if rc == SQLITE_OK {
        let mut pChomp: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
        rc = fts3SqlStmt(
            p,
            SQL_CHOMP_SEGDIR,
            &raw mut pChomp,
            ::core::ptr::null_mut::<*mut sqlite3_value>(),
        );
        if rc == SQLITE_OK {
            sqlite3_bind_int64(pChomp, 1 as ::core::ffi::c_int, iNewStart);
            sqlite3_bind_blob(
                pChomp,
                2 as ::core::ffi::c_int,
                root.a as *const ::core::ffi::c_void,
                root.n,
                SQLITE_STATIC,
            );
            sqlite3_bind_int64(pChomp, 3 as ::core::ffi::c_int, iAbsLevel);
            sqlite3_bind_int(pChomp, 4 as ::core::ffi::c_int, iIdx);
            sqlite3_step(pChomp);
            rc = sqlite3_reset(pChomp);
            sqlite3_bind_null(pChomp, 2 as ::core::ffi::c_int);
        }
    }
    sqlite3_free(root.a as *mut ::core::ffi::c_void);
    sqlite3_free(block.a as *mut ::core::ffi::c_void);
    return rc;
}
unsafe extern "C" fn fts3IncrmergeChomp(
    mut p: *mut Fts3Table,
    mut iAbsLevel: sqlite3_int64,
    mut pCsr: *mut Fts3MultiSegReader,
    mut pnRem: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut nRem: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut rc: ::core::ffi::c_int = SQLITE_OK;
    i = (*pCsr).nSegment - 1 as ::core::ffi::c_int;
    while i >= 0 as ::core::ffi::c_int && rc == SQLITE_OK {
        let mut pSeg: *mut Fts3SegReader = ::core::ptr::null_mut::<Fts3SegReader>();
        let mut j: ::core::ffi::c_int = 0;
        j = 0 as ::core::ffi::c_int;
        while j < (*pCsr).nSegment {
            pSeg = *(*pCsr).apSegment.offset(j as isize);
            if (*pSeg).iIdx == i {
                break;
            }
            j += 1;
        }
        if (*pSeg).aNode.is_null() {
            rc = fts3DeleteSegment(p, pSeg);
            if rc == SQLITE_OK {
                rc = fts3RemoveSegdirEntry(p, iAbsLevel, (*pSeg).iIdx);
            }
            *pnRem = 0 as ::core::ffi::c_int;
        } else {
            let mut zTerm: *const ::core::ffi::c_char = (*pSeg).zTerm;
            let mut nTerm: ::core::ffi::c_int = (*pSeg).nTerm;
            rc = fts3TruncateSegment(p, iAbsLevel, (*pSeg).iIdx, zTerm, nTerm);
            nRem += 1;
        }
        i -= 1;
    }
    if rc == SQLITE_OK && nRem != (*pCsr).nSegment {
        rc = fts3RepackSegdirLevel(p, iAbsLevel);
    }
    *pnRem = nRem;
    return rc;
}
unsafe extern "C" fn fts3IncrmergeHintStore(
    mut p: *mut Fts3Table,
    mut pHint: *mut Blob,
) -> ::core::ffi::c_int {
    let mut pReplace: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut rc: ::core::ffi::c_int = 0;
    rc = fts3SqlStmt(
        p,
        SQL_REPLACE_STAT,
        &raw mut pReplace,
        ::core::ptr::null_mut::<*mut sqlite3_value>(),
    );
    if rc == SQLITE_OK {
        sqlite3_bind_int(pReplace, 1 as ::core::ffi::c_int, FTS_STAT_INCRMERGEHINT);
        sqlite3_bind_blob(
            pReplace,
            2 as ::core::ffi::c_int,
            (*pHint).a as *const ::core::ffi::c_void,
            (*pHint).n,
            SQLITE_STATIC,
        );
        sqlite3_step(pReplace);
        rc = sqlite3_reset(pReplace);
        sqlite3_bind_null(pReplace, 2 as ::core::ffi::c_int);
    }
    return rc;
}
unsafe extern "C" fn fts3IncrmergeHintLoad(
    mut p: *mut Fts3Table,
    mut pHint: *mut Blob,
) -> ::core::ffi::c_int {
    let mut pSelect: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut rc: ::core::ffi::c_int = 0;
    (*pHint).n = 0 as ::core::ffi::c_int;
    rc = fts3SqlStmt(
        p,
        SQL_SELECT_STAT,
        &raw mut pSelect,
        ::core::ptr::null_mut::<*mut sqlite3_value>(),
    );
    if rc == SQLITE_OK {
        let mut rc2: ::core::ffi::c_int = 0;
        sqlite3_bind_int(pSelect, 1 as ::core::ffi::c_int, FTS_STAT_INCRMERGEHINT);
        if SQLITE_ROW == sqlite3_step(pSelect) {
            let mut aHint: *const ::core::ffi::c_char =
                sqlite3_column_blob(pSelect, 0 as ::core::ffi::c_int) as *const ::core::ffi::c_char;
            let mut nHint: ::core::ffi::c_int =
                sqlite3_column_bytes(pSelect, 0 as ::core::ffi::c_int);
            if !aHint.is_null() {
                blobGrowBuffer(pHint, nHint, &raw mut rc);
                if rc == SQLITE_OK {
                    if !(*pHint).a.is_null() {
                        memcpy(
                            (*pHint).a as *mut ::core::ffi::c_void,
                            aHint as *const ::core::ffi::c_void,
                            nHint as size_t,
                        );
                    }
                    (*pHint).n = nHint;
                }
            }
        }
        rc2 = sqlite3_reset(pSelect);
        if rc == SQLITE_OK {
            rc = rc2;
        }
    }
    return rc;
}
unsafe extern "C" fn fts3IncrmergeHintPush(
    mut pHint: *mut Blob,
    mut iAbsLevel: i64_0,
    mut nInput: ::core::ffi::c_int,
    mut pRc: *mut ::core::ffi::c_int,
) {
    blobGrowBuffer(
        pHint,
        (*pHint).n + 2 as ::core::ffi::c_int * FTS3_VARINT_MAX,
        pRc,
    );
    if *pRc == SQLITE_OK {
        (*pHint).n += sqlite3Fts3PutVarint(
            (*pHint).a.offset((*pHint).n as isize) as *mut ::core::ffi::c_char,
            iAbsLevel as sqlite3_int64,
        );
        (*pHint).n += sqlite3Fts3PutVarint(
            (*pHint).a.offset((*pHint).n as isize) as *mut ::core::ffi::c_char,
            nInput as sqlite3_int64,
        );
    }
}
unsafe extern "C" fn fts3IncrmergeHintPop(
    mut pHint: *mut Blob,
    mut piAbsLevel: *mut i64_0,
    mut pnInput: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let nHint: ::core::ffi::c_int = (*pHint).n;
    let mut i: ::core::ffi::c_int = 0;
    i = (*pHint).n - 1 as ::core::ffi::c_int;
    if *(*pHint).a.offset(i as isize) as ::core::ffi::c_int & 0x80 as ::core::ffi::c_int != 0 {
        return FTS_CORRUPT_VTAB;
    }
    while i > 0 as ::core::ffi::c_int
        && *(*pHint).a.offset((i - 1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
            & 0x80 as ::core::ffi::c_int
            != 0
    {
        i -= 1;
    }
    if i == 0 as ::core::ffi::c_int {
        return FTS_CORRUPT_VTAB;
    }
    i -= 1;
    while i > 0 as ::core::ffi::c_int
        && *(*pHint).a.offset((i - 1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
            & 0x80 as ::core::ffi::c_int
            != 0
    {
        i -= 1;
    }
    (*pHint).n = i;
    i += sqlite3Fts3GetVarint(
        (*pHint).a.offset(i as isize) as *mut ::core::ffi::c_char,
        piAbsLevel as *mut sqlite_int64,
    );
    i += if *((*pHint).a.offset(i as isize) as *mut ::core::ffi::c_char as *mut u8_0)
        as ::core::ffi::c_int
        & 0x80 as ::core::ffi::c_int
        != 0
    {
        sqlite3Fts3GetVarint32(
            (*pHint).a.offset(i as isize) as *mut ::core::ffi::c_char,
            pnInput,
        )
    } else {
        *pnInput = *((*pHint).a.offset(i as isize) as *mut ::core::ffi::c_char as *mut u8_0)
            as ::core::ffi::c_int;
        1 as ::core::ffi::c_int
    };
    if i != nHint {
        return FTS_CORRUPT_VTAB;
    }
    return SQLITE_OK;
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3Fts3Incrmerge(
    mut p: *mut Fts3Table,
    mut nMerge: ::core::ffi::c_int,
    mut nMin: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = 0;
    let mut nRem: ::core::ffi::c_int = nMerge;
    let mut pCsr: *mut Fts3MultiSegReader = ::core::ptr::null_mut::<Fts3MultiSegReader>();
    let mut pFilter: *mut Fts3SegFilter = ::core::ptr::null_mut::<Fts3SegFilter>();
    let mut pWriter: *mut IncrmergeWriter = ::core::ptr::null_mut::<IncrmergeWriter>();
    let mut nSeg: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut iAbsLevel: sqlite3_int64 = 0 as sqlite3_int64;
    let mut hint: Blob = Blob {
        a: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        n: 0 as ::core::ffi::c_int,
        nAlloc: 0 as ::core::ffi::c_int,
    };
    let mut bDirtyHint: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let nAlloc: ::core::ffi::c_int = (::core::mem::size_of::<Fts3MultiSegReader>() as usize)
        .wrapping_add(::core::mem::size_of::<Fts3SegFilter>() as usize)
        .wrapping_add(::core::mem::size_of::<IncrmergeWriter>() as usize)
        as ::core::ffi::c_int;
    pWriter = sqlite3_malloc64(nAlloc as sqlite3_uint64) as *mut IncrmergeWriter;
    if pWriter.is_null() {
        return SQLITE_NOMEM;
    }
    pFilter = pWriter.offset(1 as ::core::ffi::c_int as isize) as *mut IncrmergeWriter
        as *mut Fts3SegFilter;
    pCsr = pFilter.offset(1 as ::core::ffi::c_int as isize) as *mut Fts3SegFilter
        as *mut Fts3MultiSegReader;
    rc = fts3IncrmergeHintLoad(p, &raw mut hint);
    while rc == SQLITE_OK && nRem > 0 as ::core::ffi::c_int {
        let nMod: i64_0 = (FTS3_SEGDIR_MAXLEVEL * (*p).nIndex) as i64_0;
        let mut pFindLevel: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
        let mut bUseHint: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut iIdx: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        rc = fts3SqlStmt(
            p,
            SQL_FIND_MERGE_LEVEL,
            &raw mut pFindLevel,
            ::core::ptr::null_mut::<*mut sqlite3_value>(),
        );
        sqlite3_bind_int(
            pFindLevel,
            1 as ::core::ffi::c_int,
            if 2 as ::core::ffi::c_int > nMin {
                2 as ::core::ffi::c_int
            } else {
                nMin
            },
        );
        if sqlite3_step(pFindLevel) == SQLITE_ROW {
            iAbsLevel = sqlite3_column_int64(pFindLevel, 0 as ::core::ffi::c_int);
            nSeg = sqlite3_column_int(pFindLevel, 1 as ::core::ffi::c_int);
        } else {
            nSeg = -(1 as ::core::ffi::c_int);
        }
        rc = sqlite3_reset(pFindLevel);
        if rc == SQLITE_OK && hint.n != 0 {
            let mut nHint: ::core::ffi::c_int = hint.n;
            let mut iHintAbsLevel: sqlite3_int64 = 0 as sqlite3_int64;
            let mut nHintSeg: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            rc = fts3IncrmergeHintPop(&raw mut hint, &raw mut iHintAbsLevel, &raw mut nHintSeg);
            if nSeg < 0 as ::core::ffi::c_int
                || iAbsLevel as i64_0 % nMod >= iHintAbsLevel as i64_0 % nMod
            {
                iAbsLevel = iHintAbsLevel;
                nSeg = if (if nMin > nSeg { nMin } else { nSeg }) < nHintSeg {
                    if nMin > nSeg {
                        nMin
                    } else {
                        nSeg
                    }
                } else {
                    nHintSeg
                };
                bUseHint = 1 as ::core::ffi::c_int;
                bDirtyHint = 1 as ::core::ffi::c_int;
            } else {
                hint.n = nHint;
            }
        }
        if nSeg <= 0 as ::core::ffi::c_int {
            break;
        }
        if iAbsLevel < 0 as sqlite3_int64 || iAbsLevel > nMod << 32 as ::core::ffi::c_int {
            rc = FTS_CORRUPT_VTAB;
            break;
        } else {
            memset(
                pWriter as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                nAlloc as size_t,
            );
            (*pFilter).flags = FTS3_SEGMENT_REQUIRE_POS;
            if rc == SQLITE_OK {
                rc = fts3IncrmergeOutputIdx(p, iAbsLevel, &raw mut iIdx);
                if iIdx == 0 as ::core::ffi::c_int
                    || bUseHint != 0 && iIdx == 1 as ::core::ffi::c_int
                {
                    let mut bIgnore: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                    rc =
                        fts3SegmentIsMaxLevel(p, iAbsLevel as i64_0 + 1 as i64_0, &raw mut bIgnore);
                    if bIgnore != 0 {
                        (*pFilter).flags |= FTS3_SEGMENT_IGNORE_EMPTY;
                    }
                }
            }
            if rc == SQLITE_OK {
                rc = fts3IncrmergeCsr(p, iAbsLevel, nSeg, pCsr);
            }
            if SQLITE_OK == rc && (*pCsr).nSegment == nSeg && {
                rc = sqlite3Fts3SegReaderStart(p, pCsr, pFilter);
                SQLITE_OK == rc
            } {
                let mut bEmpty: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                rc = sqlite3Fts3SegReaderStep(p, pCsr);
                if rc == SQLITE_OK {
                    bEmpty = 1 as ::core::ffi::c_int;
                } else if rc != SQLITE_ROW {
                    sqlite3Fts3SegReaderFinish(pCsr);
                    break;
                }
                if bUseHint != 0 && iIdx > 0 as ::core::ffi::c_int {
                    let mut zKey: *const ::core::ffi::c_char = (*pCsr).zTerm;
                    let mut nKey: ::core::ffi::c_int = (*pCsr).nTerm;
                    rc = fts3IncrmergeLoad(
                        p,
                        iAbsLevel,
                        iIdx - 1 as ::core::ffi::c_int,
                        zKey,
                        nKey,
                        pWriter,
                    );
                } else {
                    rc = fts3IncrmergeWriter(p, iAbsLevel, iIdx, pCsr, pWriter);
                }
                if rc == SQLITE_OK && (*pWriter).nLeafEst != 0 {
                    if bEmpty == 0 as ::core::ffi::c_int {
                        loop {
                            rc = fts3IncrmergeAppend(p, pWriter, pCsr);
                            if rc == SQLITE_OK {
                                rc = sqlite3Fts3SegReaderStep(p, pCsr);
                            }
                            if (*pWriter).nWork >= nRem as i64_0 && rc == SQLITE_ROW {
                                rc = SQLITE_OK;
                            }
                            if !(rc == SQLITE_ROW) {
                                break;
                            }
                        }
                    }
                    if rc == SQLITE_OK {
                        nRem =
                            (nRem as i64_0 - (1 as i64_0 + (*pWriter).nWork)) as ::core::ffi::c_int;
                        rc = fts3IncrmergeChomp(p, iAbsLevel, pCsr, &raw mut nSeg);
                        if nSeg != 0 as ::core::ffi::c_int {
                            bDirtyHint = 1 as ::core::ffi::c_int;
                            fts3IncrmergeHintPush(
                                &raw mut hint,
                                iAbsLevel as i64_0,
                                nSeg,
                                &raw mut rc,
                            );
                        }
                    }
                }
                if nSeg != 0 as ::core::ffi::c_int {
                    (*pWriter).nLeafData =
                        (*pWriter).nLeafData * -(1 as ::core::ffi::c_int) as sqlite3_int64;
                }
                fts3IncrmergeRelease(p, pWriter, &raw mut rc);
                if nSeg == 0 as ::core::ffi::c_int
                    && (*pWriter).bNoLeafData as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                {
                    fts3PromoteSegments(p, iAbsLevel + 1 as sqlite3_int64, (*pWriter).nLeafData);
                }
            }
            sqlite3Fts3SegReaderFinish(pCsr);
        }
    }
    if bDirtyHint != 0 && rc == SQLITE_OK {
        rc = fts3IncrmergeHintStore(p, &raw mut hint);
    }
    sqlite3_free(pWriter as *mut ::core::ffi::c_void);
    sqlite3_free(hint.a as *mut ::core::ffi::c_void);
    return rc;
}
unsafe extern "C" fn fts3Getint(mut pz: *mut *const ::core::ffi::c_char) -> ::core::ffi::c_int {
    let mut z: *const ::core::ffi::c_char = *pz;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while *z as ::core::ffi::c_int >= '0' as i32
        && *z as ::core::ffi::c_int <= '9' as i32
        && i < 214748363 as ::core::ffi::c_int
    {
        let fresh12 = z;
        z = z.offset(1);
        i = 10 as ::core::ffi::c_int * i + *fresh12 as ::core::ffi::c_int - '0' as i32;
    }
    *pz = z;
    return i;
}
unsafe extern "C" fn fts3DoIncrmerge(
    mut p: *mut Fts3Table,
    mut zParam: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = 0;
    let mut nMin: ::core::ffi::c_int = (*p).nMergeCount / 2 as ::core::ffi::c_int;
    let mut nMerge: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut z: *const ::core::ffi::c_char = zParam;
    nMerge = fts3Getint(&raw mut z);
    if *z.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == ',' as i32
        && *z.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != '\0' as i32
    {
        z = z.offset(1);
        nMin = fts3Getint(&raw mut z);
    }
    if *z.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != '\0' as i32
        || nMin < 2 as ::core::ffi::c_int
    {
        rc = SQLITE_ERROR;
    } else {
        rc = SQLITE_OK;
        if (*p).bHasStat == 0 {
            sqlite3Fts3CreateStatTable(&raw mut rc, p);
        }
        if rc == SQLITE_OK {
            rc = sqlite3Fts3Incrmerge(p, nMerge, nMin);
        }
        sqlite3Fts3SegmentsClose(p);
    }
    return rc;
}
unsafe extern "C" fn fts3DoAutoincrmerge(
    mut p: *mut Fts3Table,
    mut zParam: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = SQLITE_OK;
    let mut pStmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    (*p).nAutoincrmerge = fts3Getint(&raw mut zParam);
    if (*p).nAutoincrmerge == 1 as ::core::ffi::c_int || (*p).nAutoincrmerge > (*p).nMergeCount {
        (*p).nAutoincrmerge = 8 as ::core::ffi::c_int;
    }
    if (*p).bHasStat == 0 {
        sqlite3Fts3CreateStatTable(&raw mut rc, p);
        if rc != 0 {
            return rc;
        }
    }
    rc = fts3SqlStmt(
        p,
        SQL_REPLACE_STAT,
        &raw mut pStmt,
        ::core::ptr::null_mut::<*mut sqlite3_value>(),
    );
    if rc != 0 {
        return rc;
    }
    sqlite3_bind_int(pStmt, 1 as ::core::ffi::c_int, FTS_STAT_AUTOINCRMERGE);
    sqlite3_bind_int(pStmt, 2 as ::core::ffi::c_int, (*p).nAutoincrmerge);
    sqlite3_step(pStmt);
    rc = sqlite3_reset(pStmt);
    return rc;
}
unsafe extern "C" fn fts3ChecksumEntry(
    mut zTerm: *const ::core::ffi::c_char,
    mut nTerm: ::core::ffi::c_int,
    mut iLangid: ::core::ffi::c_int,
    mut iIndex: ::core::ffi::c_int,
    mut iDocid: i64_0,
    mut iCol: ::core::ffi::c_int,
    mut iPos: ::core::ffi::c_int,
) -> u64_0 {
    let mut i: ::core::ffi::c_int = 0;
    let mut ret: u64_0 = iDocid as u64_0;
    ret = ret.wrapping_add((ret << 3 as ::core::ffi::c_int).wrapping_add(iLangid as u64_0));
    ret = ret.wrapping_add((ret << 3 as ::core::ffi::c_int).wrapping_add(iIndex as u64_0));
    ret = ret.wrapping_add((ret << 3 as ::core::ffi::c_int).wrapping_add(iCol as u64_0));
    ret = ret.wrapping_add((ret << 3 as ::core::ffi::c_int).wrapping_add(iPos as u64_0));
    i = 0 as ::core::ffi::c_int;
    while i < nTerm {
        ret = ret.wrapping_add(
            (ret << 3 as ::core::ffi::c_int).wrapping_add(*zTerm.offset(i as isize) as u64_0),
        );
        i += 1;
    }
    return ret;
}
unsafe extern "C" fn fts3ChecksumIndex(
    mut p: *mut Fts3Table,
    mut iLangid: ::core::ffi::c_int,
    mut iIndex: ::core::ffi::c_int,
    mut pRc: *mut ::core::ffi::c_int,
) -> u64_0 {
    let mut filter: Fts3SegFilter = Fts3SegFilter {
        zTerm: ::core::ptr::null::<::core::ffi::c_char>(),
        nTerm: 0,
        iCol: 0,
        flags: 0,
    };
    let mut csr: Fts3MultiSegReader = Fts3MultiSegReader {
        apSegment: ::core::ptr::null_mut::<*mut Fts3SegReader>(),
        nSegment: 0,
        nAdvance: 0,
        pFilter: ::core::ptr::null_mut::<Fts3SegFilter>(),
        aBuffer: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        nBuffer: 0,
        iColFilter: 0,
        bRestart: 0,
        nCost: 0,
        bLookup: 0,
        zTerm: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        nTerm: 0,
        aDoclist: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        nDoclist: 0,
    };
    let mut rc: ::core::ffi::c_int = 0;
    let mut cksum: u64_0 = 0 as u64_0;
    if *pRc != 0 {
        return 0 as u64_0;
    }
    memset(
        &raw mut filter as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<Fts3SegFilter>() as size_t,
    );
    memset(
        &raw mut csr as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<Fts3MultiSegReader>() as size_t,
    );
    filter.flags = FTS3_SEGMENT_REQUIRE_POS | FTS3_SEGMENT_IGNORE_EMPTY;
    filter.flags |= FTS3_SEGMENT_SCAN;
    rc = sqlite3Fts3SegReaderCursor(
        p,
        iLangid,
        iIndex,
        FTS3_SEGCURSOR_ALL,
        ::core::ptr::null::<::core::ffi::c_char>(),
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        &raw mut csr,
    );
    if rc == SQLITE_OK {
        rc = sqlite3Fts3SegReaderStart(p, &raw mut csr, &raw mut filter);
    }
    if rc == SQLITE_OK {
        loop {
            rc = sqlite3Fts3SegReaderStep(p, &raw mut csr);
            if !(SQLITE_ROW == rc) {
                break;
            }
            let mut pCsr: *mut ::core::ffi::c_char = csr.aDoclist;
            let mut pEnd: *mut ::core::ffi::c_char =
                pCsr.offset(csr.nDoclist as isize) as *mut ::core::ffi::c_char;
            let mut iDocid: i64_0 = 0 as i64_0;
            let mut iCol: i64_0 = 0 as i64_0;
            let mut iPos: u64_0 = 0 as u64_0;
            pCsr = pCsr.offset(sqlite3Fts3GetVarint(pCsr, &raw mut iDocid) as isize);
            while pCsr < pEnd {
                let mut iVal: u64_0 = 0 as u64_0;
                pCsr = pCsr.offset(sqlite3Fts3GetVarintU(pCsr, &raw mut iVal) as isize);
                if pCsr < pEnd {
                    if iVal == 0 as u64_0 || iVal == 1 as u64_0 {
                        iCol = 0 as i64_0;
                        iPos = 0 as u64_0;
                        if iVal != 0 {
                            pCsr = pCsr.offset(sqlite3Fts3GetVarint(pCsr, &raw mut iCol) as isize);
                        } else {
                            pCsr = pCsr.offset(sqlite3Fts3GetVarintU(pCsr, &raw mut iVal) as isize);
                            if (*p).bDescIdx != 0 {
                                iDocid = (iDocid as u64_0).wrapping_sub(iVal) as i64_0;
                            } else {
                                iDocid = (iDocid as u64_0).wrapping_add(iVal) as i64_0;
                            }
                        }
                    } else {
                        iPos = iPos.wrapping_add(iVal.wrapping_sub(2 as u64_0));
                        cksum = cksum
                            ^ fts3ChecksumEntry(
                                csr.zTerm,
                                csr.nTerm,
                                iLangid,
                                iIndex,
                                iDocid,
                                iCol as ::core::ffi::c_int,
                                iPos as ::core::ffi::c_int,
                            );
                    }
                }
            }
        }
    }
    sqlite3Fts3SegReaderFinish(&raw mut csr);
    *pRc = rc;
    return cksum;
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3Fts3IntegrityCheck(
    mut p: *mut Fts3Table,
    mut pbOk: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = SQLITE_OK;
    let mut cksum1: u64_0 = 0 as u64_0;
    let mut cksum2: u64_0 = 0 as u64_0;
    let mut pAllLangid: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    rc = fts3SqlStmt(
        p,
        SQL_SELECT_ALL_LANGID,
        &raw mut pAllLangid,
        ::core::ptr::null_mut::<*mut sqlite3_value>(),
    );
    if rc == SQLITE_OK {
        let mut rc2: ::core::ffi::c_int = 0;
        sqlite3_bind_int(pAllLangid, 1 as ::core::ffi::c_int, (*p).iPrevLangid);
        sqlite3_bind_int(pAllLangid, 2 as ::core::ffi::c_int, (*p).nIndex);
        while rc == SQLITE_OK && sqlite3_step(pAllLangid) == SQLITE_ROW {
            let mut iLangid: ::core::ffi::c_int =
                sqlite3_column_int(pAllLangid, 0 as ::core::ffi::c_int);
            let mut i: ::core::ffi::c_int = 0;
            i = 0 as ::core::ffi::c_int;
            while i < (*p).nIndex {
                cksum1 = cksum1 ^ fts3ChecksumIndex(p, iLangid, i, &raw mut rc);
                i += 1;
            }
        }
        rc2 = sqlite3_reset(pAllLangid);
        if rc == SQLITE_OK {
            rc = rc2;
        }
    }
    if rc == SQLITE_OK {
        let mut pModule: *const sqlite3_tokenizer_module = (*(*p).pTokenizer).pModule;
        let mut pStmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
        let mut zSql: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        zSql = sqlite3_mprintf(
            b"SELECT %s\0" as *const u8 as *const ::core::ffi::c_char,
            (*p).zReadExprlist,
        );
        if zSql.is_null() {
            rc = SQLITE_NOMEM;
        } else {
            rc = sqlite3_prepare_v2(
                (*p).db,
                zSql,
                -(1 as ::core::ffi::c_int),
                &raw mut pStmt,
                ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
            );
            sqlite3_free(zSql as *mut ::core::ffi::c_void);
        }
        while rc == SQLITE_OK && SQLITE_ROW == sqlite3_step(pStmt) {
            let mut iDocid: i64_0 = sqlite3_column_int64(pStmt, 0 as ::core::ffi::c_int) as i64_0;
            let mut iLang: ::core::ffi::c_int = langidFromSelect(p, pStmt);
            let mut iCol: ::core::ffi::c_int = 0;
            iCol = 0 as ::core::ffi::c_int;
            while rc == SQLITE_OK && iCol < (*p).nColumn {
                if *(*p).abNotindexed.offset(iCol as isize) as ::core::ffi::c_int
                    == 0 as ::core::ffi::c_int
                {
                    let mut zText: *const ::core::ffi::c_char =
                        sqlite3_column_text(pStmt, iCol + 1 as ::core::ffi::c_int)
                            as *const ::core::ffi::c_char;
                    let mut pT: *mut sqlite3_tokenizer_cursor =
                        ::core::ptr::null_mut::<sqlite3_tokenizer_cursor>();
                    rc = sqlite3Fts3OpenTokenizer(
                        (*p).pTokenizer,
                        iLang,
                        zText,
                        -(1 as ::core::ffi::c_int),
                        &raw mut pT,
                    );
                    while rc == SQLITE_OK {
                        let mut zToken: *const ::core::ffi::c_char =
                            ::core::ptr::null::<::core::ffi::c_char>();
                        let mut nToken: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                        let mut iDum1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                        let mut iDum2: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                        let mut iPos: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                        rc = (*pModule).xNext.expect("non-null function pointer")(
                            pT,
                            &raw mut zToken,
                            &raw mut nToken,
                            &raw mut iDum1,
                            &raw mut iDum2,
                            &raw mut iPos,
                        );
                        if rc == SQLITE_OK {
                            let mut i_0: ::core::ffi::c_int = 0;
                            cksum2 = cksum2
                                ^ fts3ChecksumEntry(
                                    zToken,
                                    nToken,
                                    iLang,
                                    0 as ::core::ffi::c_int,
                                    iDocid,
                                    iCol,
                                    iPos,
                                );
                            i_0 = 1 as ::core::ffi::c_int;
                            while i_0 < (*p).nIndex {
                                if (*(*p).aIndex.offset(i_0 as isize)).nPrefix <= nToken {
                                    cksum2 = cksum2
                                        ^ fts3ChecksumEntry(
                                            zToken,
                                            (*(*p).aIndex.offset(i_0 as isize)).nPrefix,
                                            iLang,
                                            i_0,
                                            iDocid,
                                            iCol,
                                            iPos,
                                        );
                                }
                                i_0 += 1;
                            }
                        }
                    }
                    if !pT.is_null() {
                        (*pModule).xClose.expect("non-null function pointer")(pT);
                    }
                    if rc == SQLITE_DONE {
                        rc = SQLITE_OK;
                    }
                }
                iCol += 1;
            }
        }
        sqlite3_finalize(pStmt);
    }
    if rc == SQLITE_CORRUPT_VTAB {
        rc = SQLITE_OK;
        *pbOk = 0 as ::core::ffi::c_int;
    } else {
        *pbOk = (rc == SQLITE_OK && cksum1 == cksum2) as ::core::ffi::c_int;
    }
    return rc;
}
unsafe extern "C" fn fts3DoIntegrityCheck(mut p: *mut Fts3Table) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = 0;
    let mut bOk: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    rc = sqlite3Fts3IntegrityCheck(p, &raw mut bOk);
    if rc == SQLITE_OK && bOk == 0 as ::core::ffi::c_int {
        rc = FTS_CORRUPT_VTAB;
    }
    return rc;
}
unsafe extern "C" fn fts3SpecialInsert(
    mut p: *mut Fts3Table,
    mut pVal: *mut sqlite3_value,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = SQLITE_ERROR;
    let mut zVal: *const ::core::ffi::c_char =
        sqlite3_value_text(pVal) as *const ::core::ffi::c_char;
    let mut nVal: ::core::ffi::c_int = sqlite3_value_bytes(pVal);
    if zVal.is_null() {
        return SQLITE_NOMEM;
    } else if nVal == 8 as ::core::ffi::c_int
        && 0 as ::core::ffi::c_int
            == sqlite3_strnicmp(
                zVal,
                b"optimize\0" as *const u8 as *const ::core::ffi::c_char,
                8 as ::core::ffi::c_int,
            )
    {
        rc = fts3DoOptimize(p, 0 as ::core::ffi::c_int);
    } else if nVal == 7 as ::core::ffi::c_int
        && 0 as ::core::ffi::c_int
            == sqlite3_strnicmp(
                zVal,
                b"rebuild\0" as *const u8 as *const ::core::ffi::c_char,
                7 as ::core::ffi::c_int,
            )
    {
        rc = fts3DoRebuild(p);
    } else if nVal == 15 as ::core::ffi::c_int
        && 0 as ::core::ffi::c_int
            == sqlite3_strnicmp(
                zVal,
                b"integrity-check\0" as *const u8 as *const ::core::ffi::c_char,
                15 as ::core::ffi::c_int,
            )
    {
        rc = fts3DoIntegrityCheck(p);
    } else if nVal > 6 as ::core::ffi::c_int
        && 0 as ::core::ffi::c_int
            == sqlite3_strnicmp(
                zVal,
                b"merge=\0" as *const u8 as *const ::core::ffi::c_char,
                6 as ::core::ffi::c_int,
            )
    {
        rc = fts3DoIncrmerge(
            p,
            zVal.offset(6 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_char,
        );
    } else if nVal > 10 as ::core::ffi::c_int
        && 0 as ::core::ffi::c_int
            == sqlite3_strnicmp(
                zVal,
                b"automerge=\0" as *const u8 as *const ::core::ffi::c_char,
                10 as ::core::ffi::c_int,
            )
    {
        rc = fts3DoAutoincrmerge(
            p,
            zVal.offset(10 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_char,
        );
    } else if nVal == 5 as ::core::ffi::c_int
        && 0 as ::core::ffi::c_int
            == sqlite3_strnicmp(
                zVal,
                b"flush\0" as *const u8 as *const ::core::ffi::c_char,
                5 as ::core::ffi::c_int,
            )
    {
        rc = sqlite3Fts3PendingTermsFlush(p);
    } else {
        let mut v: ::core::ffi::c_int = 0;
        if nVal > 9 as ::core::ffi::c_int
            && 0 as ::core::ffi::c_int
                == sqlite3_strnicmp(
                    zVal,
                    b"nodesize=\0" as *const u8 as *const ::core::ffi::c_char,
                    9 as ::core::ffi::c_int,
                )
        {
            v = atoi(zVal.offset(9 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_char);
            if v >= 24 as ::core::ffi::c_int && v <= (*p).nPgsz - 35 as ::core::ffi::c_int {
                (*p).nNodeSize = v;
            }
            rc = SQLITE_OK;
        } else if nVal > 11 as ::core::ffi::c_int
            && 0 as ::core::ffi::c_int
                == sqlite3_strnicmp(
                    zVal,
                    b"maxpending=\0" as *const u8 as *const ::core::ffi::c_char,
                    9 as ::core::ffi::c_int,
                )
        {
            v = atoi(zVal.offset(11 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_char);
            if v >= 64 as ::core::ffi::c_int && v <= FTS3_MAX_PENDING_DATA {
                (*p).nMaxPendingData = v;
            }
            rc = SQLITE_OK;
        } else if nVal > 21 as ::core::ffi::c_int
            && 0 as ::core::ffi::c_int
                == sqlite3_strnicmp(
                    zVal,
                    b"test-no-incr-doclist=\0" as *const u8 as *const ::core::ffi::c_char,
                    21 as ::core::ffi::c_int,
                )
        {
            (*p).bNoIncrDoclist =
                atoi(zVal.offset(21 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_char);
            rc = SQLITE_OK;
        } else if nVal > 11 as ::core::ffi::c_int
            && 0 as ::core::ffi::c_int
                == sqlite3_strnicmp(
                    zVal,
                    b"mergecount=\0" as *const u8 as *const ::core::ffi::c_char,
                    11 as ::core::ffi::c_int,
                )
        {
            v = atoi(zVal.offset(11 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_char);
            if v >= 4 as ::core::ffi::c_int
                && v <= FTS3_MERGE_COUNT
                && v & 1 as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            {
                (*p).nMergeCount = v;
            }
            rc = SQLITE_OK;
        }
    }
    return rc;
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3Fts3FreeDeferredDoclists(mut pCsr: *mut Fts3Cursor) {
    let mut pDef: *mut Fts3DeferredToken = ::core::ptr::null_mut::<Fts3DeferredToken>();
    pDef = (*pCsr).pDeferred;
    while !pDef.is_null() {
        fts3PendingListDelete((*pDef).pList);
        (*pDef).pList = ::core::ptr::null_mut::<PendingList>();
        pDef = (*pDef).pNext;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3Fts3FreeDeferredTokens(mut pCsr: *mut Fts3Cursor) {
    let mut pDef: *mut Fts3DeferredToken = ::core::ptr::null_mut::<Fts3DeferredToken>();
    let mut pNext: *mut Fts3DeferredToken = ::core::ptr::null_mut::<Fts3DeferredToken>();
    pDef = (*pCsr).pDeferred;
    while !pDef.is_null() {
        pNext = (*pDef).pNext;
        fts3PendingListDelete((*pDef).pList);
        sqlite3_free(pDef as *mut ::core::ffi::c_void);
        pDef = pNext;
    }
    (*pCsr).pDeferred = ::core::ptr::null_mut::<Fts3DeferredToken>();
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3Fts3CacheDeferredDoclists(
    mut pCsr: *mut Fts3Cursor,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = SQLITE_OK;
    if !(*pCsr).pDeferred.is_null() {
        let mut i: ::core::ffi::c_int = 0;
        let mut iDocid: sqlite3_int64 = 0;
        let mut pDef: *mut Fts3DeferredToken = ::core::ptr::null_mut::<Fts3DeferredToken>();
        let mut p: *mut Fts3Table = (*pCsr).base.pVtab as *mut Fts3Table;
        let mut pT: *mut sqlite3_tokenizer = (*p).pTokenizer;
        let mut pModule: *const sqlite3_tokenizer_module = (*pT).pModule;
        iDocid = sqlite3_column_int64((*pCsr).pStmt, 0 as ::core::ffi::c_int);
        i = 0 as ::core::ffi::c_int;
        while i < (*p).nColumn && rc == SQLITE_OK {
            if *(*p).abNotindexed.offset(i as isize) as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int
            {
                let mut zText: *const ::core::ffi::c_char =
                    sqlite3_column_text((*pCsr).pStmt, i + 1 as ::core::ffi::c_int)
                        as *const ::core::ffi::c_char;
                let mut pTC: *mut sqlite3_tokenizer_cursor =
                    ::core::ptr::null_mut::<sqlite3_tokenizer_cursor>();
                rc = sqlite3Fts3OpenTokenizer(
                    pT,
                    (*pCsr).iLangid,
                    zText,
                    -(1 as ::core::ffi::c_int),
                    &raw mut pTC,
                );
                while rc == SQLITE_OK {
                    let mut zToken: *const ::core::ffi::c_char =
                        ::core::ptr::null::<::core::ffi::c_char>();
                    let mut nToken: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                    let mut iDum1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                    let mut iDum2: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                    let mut iPos: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                    rc = (*pModule).xNext.expect("non-null function pointer")(
                        pTC,
                        &raw mut zToken,
                        &raw mut nToken,
                        &raw mut iDum1,
                        &raw mut iDum2,
                        &raw mut iPos,
                    );
                    pDef = (*pCsr).pDeferred;
                    while !pDef.is_null() && rc == SQLITE_OK {
                        let mut pPT: *mut Fts3PhraseToken = (*pDef).pToken;
                        if ((*pDef).iCol >= (*p).nColumn || (*pDef).iCol == i)
                            && ((*pPT).bFirst == 0 as ::core::ffi::c_int
                                || iPos == 0 as ::core::ffi::c_int)
                            && ((*pPT).n == nToken || (*pPT).isPrefix != 0 && (*pPT).n < nToken)
                            && 0 as ::core::ffi::c_int
                                == memcmp(
                                    zToken as *const ::core::ffi::c_void,
                                    (*pPT).z as *const ::core::ffi::c_void,
                                    (*pPT).n as size_t,
                                )
                        {
                            fts3PendingListAppend(
                                &raw mut (*pDef).pList,
                                iDocid,
                                i as sqlite3_int64,
                                iPos as sqlite3_int64,
                                &raw mut rc,
                            );
                        }
                        pDef = (*pDef).pNext;
                    }
                }
                if !pTC.is_null() {
                    (*pModule).xClose.expect("non-null function pointer")(pTC);
                }
                if rc == SQLITE_DONE {
                    rc = SQLITE_OK;
                }
            }
            i += 1;
        }
        pDef = (*pCsr).pDeferred;
        while !pDef.is_null() && rc == SQLITE_OK {
            if !(*pDef).pList.is_null() {
                rc = fts3PendingListAppendVarint(&raw mut (*pDef).pList, 0 as sqlite3_int64);
            }
            pDef = (*pDef).pNext;
        }
    }
    return rc;
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3Fts3DeferredTokenList(
    mut p: *mut Fts3DeferredToken,
    mut ppData: *mut *mut ::core::ffi::c_char,
    mut pnData: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut pRet: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut nSkip: ::core::ffi::c_int = 0;
    let mut dummy: sqlite3_int64 = 0;
    *ppData = ::core::ptr::null_mut::<::core::ffi::c_char>();
    *pnData = 0 as ::core::ffi::c_int;
    if (*p).pList.is_null() {
        return SQLITE_OK;
    }
    pRet = sqlite3_malloc64((*(*p).pList).nData as sqlite3_uint64) as *mut ::core::ffi::c_char;
    if pRet.is_null() {
        return SQLITE_NOMEM;
    }
    nSkip = sqlite3Fts3GetVarint((*(*p).pList).aData, &raw mut dummy);
    *pnData = (*(*p).pList).nData - nSkip;
    *ppData = pRet;
    memcpy(
        pRet as *mut ::core::ffi::c_void,
        (*(*p).pList).aData.offset(nSkip as isize) as *mut ::core::ffi::c_char
            as *const ::core::ffi::c_void,
        *pnData as size_t,
    );
    return SQLITE_OK;
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3Fts3DeferToken(
    mut pCsr: *mut Fts3Cursor,
    mut pToken: *mut Fts3PhraseToken,
    mut iCol: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut pDeferred: *mut Fts3DeferredToken = ::core::ptr::null_mut::<Fts3DeferredToken>();
    pDeferred = sqlite3_malloc64(::core::mem::size_of::<Fts3DeferredToken>() as sqlite3_uint64)
        as *mut Fts3DeferredToken;
    if pDeferred.is_null() {
        return SQLITE_NOMEM;
    }
    memset(
        pDeferred as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<Fts3DeferredToken>() as size_t,
    );
    (*pDeferred).pToken = pToken;
    (*pDeferred).pNext = (*pCsr).pDeferred;
    (*pDeferred).iCol = iCol;
    (*pCsr).pDeferred = pDeferred;
    (*pToken).pDeferred = pDeferred;
    return SQLITE_OK;
}
unsafe extern "C" fn fts3DeleteByRowid(
    mut p: *mut Fts3Table,
    mut pRowid: *mut sqlite3_value,
    mut pnChng: *mut ::core::ffi::c_int,
    mut aSzDel: *mut u32_0,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = SQLITE_OK;
    let mut bFound: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    fts3DeleteTerms(&raw mut rc, p, pRowid, aSzDel, &raw mut bFound);
    if bFound != 0 && rc == SQLITE_OK {
        let mut isEmpty: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        rc = fts3IsEmpty(p, pRowid, &raw mut isEmpty);
        if rc == SQLITE_OK {
            if isEmpty != 0 {
                rc = fts3DeleteAll(p, 1 as ::core::ffi::c_int);
                *pnChng = 0 as ::core::ffi::c_int;
                memset(
                    aSzDel as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    (::core::mem::size_of::<u32_0>() as size_t)
                        .wrapping_mul(((*p).nColumn + 1 as ::core::ffi::c_int) as size_t)
                        .wrapping_mul(2 as size_t),
                );
            } else {
                *pnChng = *pnChng - 1 as ::core::ffi::c_int;
                if (*p).zContentTbl.is_null() {
                    fts3SqlExec(&raw mut rc, p, SQL_DELETE_CONTENT, &raw mut pRowid);
                }
                if (*p).bHasDocsize != 0 {
                    fts3SqlExec(&raw mut rc, p, SQL_DELETE_DOCSIZE, &raw mut pRowid);
                }
            }
        }
    }
    return rc;
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3Fts3UpdateMethod(
    mut pVtab: *mut sqlite3_vtab,
    mut nArg: ::core::ffi::c_int,
    mut apVal: *mut *mut sqlite3_value,
    mut pRowid: *mut sqlite_int64,
) -> ::core::ffi::c_int {
    let mut p: *mut Fts3Table = pVtab as *mut Fts3Table;
    let mut rc: ::core::ffi::c_int = SQLITE_OK;
    let mut aSzIns: *mut u32_0 = ::core::ptr::null_mut::<u32_0>();
    let mut aSzDel: *mut u32_0 = ::core::ptr::null_mut::<u32_0>();
    let mut nChng: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut bInsertDone: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if nArg > 1 as ::core::ffi::c_int
        && sqlite3_value_type(*apVal.offset(0 as ::core::ffi::c_int as isize)) == SQLITE_NULL
        && sqlite3_value_type(*apVal.offset(((*p).nColumn + 2 as ::core::ffi::c_int) as isize))
            != SQLITE_NULL
    {
        rc = fts3SpecialInsert(
            p,
            *apVal.offset(((*p).nColumn + 2 as ::core::ffi::c_int) as isize),
        );
    } else if nArg > 1 as ::core::ffi::c_int
        && sqlite3_value_int(
            *apVal.offset(
                (2 as ::core::ffi::c_int + (*p).nColumn + 2 as ::core::ffi::c_int) as isize,
            ),
        ) < 0 as ::core::ffi::c_int
    {
        rc = SQLITE_CONSTRAINT;
    } else {
        aSzDel = sqlite3_malloc64(
            (::core::mem::size_of::<u32_0>() as sqlite3_uint64)
                .wrapping_mul(
                    ((*p).nColumn as sqlite3_int64 + 1 as sqlite3_int64) as sqlite3_uint64,
                )
                .wrapping_mul(2 as sqlite3_uint64),
        ) as *mut u32_0;
        if aSzDel.is_null() {
            rc = SQLITE_NOMEM;
        } else {
            aSzIns = aSzDel.offset(((*p).nColumn + 1 as ::core::ffi::c_int) as isize) as *mut u32_0;
            memset(
                aSzDel as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                (::core::mem::size_of::<u32_0>() as size_t)
                    .wrapping_mul(((*p).nColumn + 1 as ::core::ffi::c_int) as size_t)
                    .wrapping_mul(2 as size_t),
            );
            rc = fts3Writelock(p);
            if !(rc != SQLITE_OK) {
                if nArg > 1 as ::core::ffi::c_int && (*p).zContentTbl.is_null() {
                    let mut pNewRowid: *mut sqlite3_value =
                        *apVal.offset((3 as ::core::ffi::c_int + (*p).nColumn) as isize);
                    if sqlite3_value_type(pNewRowid) == SQLITE_NULL {
                        pNewRowid = *apVal.offset(1 as ::core::ffi::c_int as isize);
                    }
                    if sqlite3_value_type(pNewRowid) != SQLITE_NULL
                        && (sqlite3_value_type(*apVal.offset(0 as ::core::ffi::c_int as isize))
                            == SQLITE_NULL
                            || sqlite3_value_int64(*apVal.offset(0 as ::core::ffi::c_int as isize))
                                != sqlite3_value_int64(pNewRowid))
                    {
                        if sqlite3_vtab_on_conflict((*p).db) == SQLITE_REPLACE {
                            rc = fts3DeleteByRowid(p, pNewRowid, &raw mut nChng, aSzDel);
                        } else {
                            rc = fts3InsertData(p, apVal, pRowid as *mut sqlite3_int64);
                            bInsertDone = 1 as ::core::ffi::c_int;
                        }
                    }
                }
                if !(rc != SQLITE_OK) {
                    if sqlite3_value_type(*apVal.offset(0 as ::core::ffi::c_int as isize))
                        != SQLITE_NULL
                    {
                        rc = fts3DeleteByRowid(
                            p,
                            *apVal.offset(0 as ::core::ffi::c_int as isize),
                            &raw mut nChng,
                            aSzDel,
                        );
                    }
                    if nArg > 1 as ::core::ffi::c_int && rc == SQLITE_OK {
                        let mut iLangid: ::core::ffi::c_int = sqlite3_value_int(*apVal.offset(
                            (2 as ::core::ffi::c_int + (*p).nColumn + 2 as ::core::ffi::c_int)
                                as isize,
                        ));
                        if bInsertDone == 0 as ::core::ffi::c_int {
                            rc = fts3InsertData(p, apVal, pRowid as *mut sqlite3_int64);
                            if rc == SQLITE_CONSTRAINT && (*p).zContentTbl.is_null() {
                                rc = FTS_CORRUPT_VTAB;
                            }
                        }
                        if rc == SQLITE_OK {
                            rc =
                                fts3PendingTermsDocid(p, 0 as ::core::ffi::c_int, iLangid, *pRowid);
                        }
                        if rc == SQLITE_OK {
                            rc = fts3InsertTerms(p, iLangid, apVal, aSzIns);
                        }
                        if (*p).bHasDocsize != 0 {
                            fts3InsertDocsize(&raw mut rc, p, aSzIns);
                        }
                        nChng += 1;
                    }
                    if (*p).bFts4 != 0 {
                        fts3UpdateDocTotals(&raw mut rc, p, aSzIns, aSzDel, nChng);
                    }
                }
            }
        }
    }
    sqlite3_free(aSzDel as *mut ::core::ffi::c_void);
    sqlite3Fts3SegmentsClose(p);
    return rc;
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3Fts3Optimize(mut p: *mut Fts3Table) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = 0;
    rc = sqlite3_exec(
        (*p).db,
        b"SAVEPOINT fts3\0" as *const u8 as *const ::core::ffi::c_char,
        None,
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
    );
    if rc == SQLITE_OK {
        rc = fts3DoOptimize(p, 1 as ::core::ffi::c_int);
        if rc == SQLITE_OK || rc == SQLITE_DONE {
            let mut rc2: ::core::ffi::c_int = sqlite3_exec(
                (*p).db,
                b"RELEASE fts3\0" as *const u8 as *const ::core::ffi::c_char,
                None,
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
            );
            if rc2 != SQLITE_OK {
                rc = rc2;
            }
        } else {
            sqlite3_exec(
                (*p).db,
                b"ROLLBACK TO fts3\0" as *const u8 as *const ::core::ffi::c_char,
                None,
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
            );
            sqlite3_exec(
                (*p).db,
                b"RELEASE fts3\0" as *const u8 as *const ::core::ffi::c_char,
                None,
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
            );
        }
    }
    sqlite3Fts3SegmentsClose(p);
    return rc;
}
