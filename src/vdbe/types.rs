#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct __BindgenBitfieldUnit<Storage> {
    storage: Storage,
}
pub type __builtin_va_list = [__va_list_tag; 1usize];
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __va_list_tag {
    pub gp_offset: ::std::os::raw::c_uint,
    pub fp_offset: ::std::os::raw::c_uint,
    pub overflow_arg_area: *mut ::std::os::raw::c_void,
    pub reg_save_area: *mut ::std::os::raw::c_void,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of __va_list_tag"][::std::mem::size_of::<__va_list_tag>() - 24usize];
    ["Alignment of __va_list_tag"][::std::mem::align_of::<__va_list_tag>() - 8usize];
    ["Offset of field: __va_list_tag::gp_offset"]
        [::std::mem::offset_of!(__va_list_tag, gp_offset) - 0usize];
    ["Offset of field: __va_list_tag::fp_offset"]
        [::std::mem::offset_of!(__va_list_tag, fp_offset) - 4usize];
    ["Offset of field: __va_list_tag::overflow_arg_area"]
        [::std::mem::offset_of!(__va_list_tag, overflow_arg_area) - 8usize];
    ["Offset of field: __va_list_tag::reg_save_area"]
        [::std::mem::offset_of!(__va_list_tag, reg_save_area) - 16usize];
};

impl<Storage> __BindgenBitfieldUnit<Storage> {
    #[inline]
    pub const fn new(storage: Storage) -> Self {
        Self { storage }
    }
}
impl<Storage> __BindgenBitfieldUnit<Storage>
where
    Storage: AsRef<[u8]> + AsMut<[u8]>,
{
    #[inline]
    fn extract_bit(byte: u8, index: usize) -> bool {
        let bit_index = if cfg!(target_endian = "big") {
            7 - (index % 8)
        } else {
            index % 8
        };
        let mask = 1 << bit_index;
        byte & mask == mask
    }
    #[inline]
    pub fn get_bit(&self, index: usize) -> bool {
        debug_assert!(index / 8 < self.storage.as_ref().len());
        let byte_index = index / 8;
        let byte = self.storage.as_ref()[byte_index];
        Self::extract_bit(byte, index)
    }
    #[inline]
    pub unsafe fn raw_get_bit(this: *const Self, index: usize) -> bool {
        debug_assert!(index / 8 < core::mem::size_of::<Storage>());
        let byte_index = index / 8;
        let byte = unsafe {
            *(core::ptr::addr_of!((*this).storage) as *const u8).offset(byte_index as isize)
        };
        Self::extract_bit(byte, index)
    }
    #[inline]
    fn change_bit(byte: u8, index: usize, val: bool) -> u8 {
        let bit_index = if cfg!(target_endian = "big") {
            7 - (index % 8)
        } else {
            index % 8
        };
        let mask = 1 << bit_index;
        if val { byte | mask } else { byte & !mask }
    }
    #[inline]
    pub fn set_bit(&mut self, index: usize, val: bool) {
        debug_assert!(index / 8 < self.storage.as_ref().len());
        let byte_index = index / 8;
        let byte = &mut self.storage.as_mut()[byte_index];
        *byte = Self::change_bit(*byte, index, val);
    }
    #[inline]
    pub unsafe fn raw_set_bit(this: *mut Self, index: usize, val: bool) {
        debug_assert!(index / 8 < core::mem::size_of::<Storage>());
        let byte_index = index / 8;
        let byte = unsafe {
            (core::ptr::addr_of_mut!((*this).storage) as *mut u8).offset(byte_index as isize)
        };
        unsafe { *byte = Self::change_bit(*byte, index, val) };
    }
    #[inline]
    pub fn get(&self, bit_offset: usize, bit_width: u8) -> u64 {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < self.storage.as_ref().len());
        debug_assert!((bit_offset + (bit_width as usize)) / 8 <= self.storage.as_ref().len());
        let mut val = 0;
        for i in 0..(bit_width as usize) {
            if self.get_bit(i + bit_offset) {
                let index = if cfg!(target_endian = "big") {
                    bit_width as usize - 1 - i
                } else {
                    i
                };
                val |= 1 << index;
            }
        }
        val
    }
    #[inline]
    pub unsafe fn raw_get(this: *const Self, bit_offset: usize, bit_width: u8) -> u64 {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < core::mem::size_of::<Storage>());
        debug_assert!((bit_offset + (bit_width as usize)) / 8 <= core::mem::size_of::<Storage>());
        let mut val = 0;
        for i in 0..(bit_width as usize) {
            if unsafe { Self::raw_get_bit(this, i + bit_offset) } {
                let index = if cfg!(target_endian = "big") {
                    bit_width as usize - 1 - i
                } else {
                    i
                };
                val |= 1 << index;
            }
        }
        val
    }
    #[inline]
    pub fn set(&mut self, bit_offset: usize, bit_width: u8, val: u64) {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < self.storage.as_ref().len());
        debug_assert!((bit_offset + (bit_width as usize)) / 8 <= self.storage.as_ref().len());
        for i in 0..(bit_width as usize) {
            let mask = 1 << i;
            let val_bit_is_set = val & mask == mask;
            let index = if cfg!(target_endian = "big") {
                bit_width as usize - 1 - i
            } else {
                i
            };
            self.set_bit(index + bit_offset, val_bit_is_set);
        }
    }
    #[inline]
    pub unsafe fn raw_set(this: *mut Self, bit_offset: usize, bit_width: u8, val: u64) {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < core::mem::size_of::<Storage>());
        debug_assert!((bit_offset + (bit_width as usize)) / 8 <= core::mem::size_of::<Storage>());
        for i in 0..(bit_width as usize) {
            let mask = 1 << i;
            let val_bit_is_set = val & mask == mask;
            let index = if cfg!(target_endian = "big") {
                bit_width as usize - 1 - i
            } else {
                i
            };
            unsafe { Self::raw_set_bit(this, index + bit_offset, val_bit_is_set) };
        }
    }
}

#[repr(C)]
#[derive(Default)]
pub struct __IncompleteArrayField<T>(::std::marker::PhantomData<T>, [T; 0]);
impl<T> __IncompleteArrayField<T> {
    #[inline]
    pub const fn new() -> Self {
        __IncompleteArrayField(::std::marker::PhantomData, [])
    }
    #[inline]
    pub fn as_ptr(&self) -> *const T {
        self as *const _ as *const T
    }
    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut T {
        self as *mut _ as *mut T
    }
    #[inline]
    pub unsafe fn as_slice(&self, len: usize) -> &[T] {
        unsafe { ::std::slice::from_raw_parts(self.as_ptr(), len) }
    }
    #[inline]
    pub unsafe fn as_mut_slice(&mut self, len: usize) -> &mut [T] {
        unsafe { ::std::slice::from_raw_parts_mut(self.as_mut_ptr(), len) }
    }
}
impl<T> ::std::fmt::Debug for __IncompleteArrayField<T> {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.write_str("__IncompleteArrayField")
    }
}
pub const TK_SEMI: u32 = 1;
pub const TK_EXPLAIN: u32 = 2;
pub const TK_QUERY: u32 = 3;
pub const TK_PLAN: u32 = 4;
pub const TK_BEGIN: u32 = 5;
pub const TK_TRANSACTION: u32 = 6;
pub const TK_DEFERRED: u32 = 7;
pub const TK_IMMEDIATE: u32 = 8;
pub const TK_EXCLUSIVE: u32 = 9;
pub const TK_COMMIT: u32 = 10;
pub const TK_END: u32 = 11;
pub const TK_ROLLBACK: u32 = 12;
pub const TK_SAVEPOINT: u32 = 13;
pub const TK_RELEASE: u32 = 14;
pub const TK_TO: u32 = 15;
pub const TK_TABLE: u32 = 16;
pub const TK_CREATE: u32 = 17;
pub const TK_IF: u32 = 18;
pub const TK_NOT: u32 = 19;
pub const TK_EXISTS: u32 = 20;
pub const TK_TEMP: u32 = 21;
pub const TK_LP: u32 = 22;
pub const TK_RP: u32 = 23;
pub const TK_AS: u32 = 24;
pub const TK_COMMA: u32 = 25;
pub const TK_WITHOUT: u32 = 26;
pub const TK_ABORT: u32 = 27;
pub const TK_ACTION: u32 = 28;
pub const TK_AFTER: u32 = 29;
pub const TK_ANALYZE: u32 = 30;
pub const TK_ASC: u32 = 31;
pub const TK_ATTACH: u32 = 32;
pub const TK_BEFORE: u32 = 33;
pub const TK_BY: u32 = 34;
pub const TK_CASCADE: u32 = 35;
pub const TK_CAST: u32 = 36;
pub const TK_CONFLICT: u32 = 37;
pub const TK_DATABASE: u32 = 38;
pub const TK_DESC: u32 = 39;
pub const TK_DETACH: u32 = 40;
pub const TK_EACH: u32 = 41;
pub const TK_FAIL: u32 = 42;
pub const TK_OR: u32 = 43;
pub const TK_AND: u32 = 44;
pub const TK_IS: u32 = 45;
pub const TK_ISNOT: u32 = 46;
pub const TK_MATCH: u32 = 47;
pub const TK_LIKE_KW: u32 = 48;
pub const TK_BETWEEN: u32 = 49;
pub const TK_IN: u32 = 50;
pub const TK_ISNULL: u32 = 51;
pub const TK_NOTNULL: u32 = 52;
pub const TK_NE: u32 = 53;
pub const TK_EQ: u32 = 54;
pub const TK_GT: u32 = 55;
pub const TK_LE: u32 = 56;
pub const TK_LT: u32 = 57;
pub const TK_GE: u32 = 58;
pub const TK_ESCAPE: u32 = 59;
pub const TK_ID: u32 = 60;
pub const TK_COLUMNKW: u32 = 61;
pub const TK_DO: u32 = 62;
pub const TK_FOR: u32 = 63;
pub const TK_IGNORE: u32 = 64;
pub const TK_INITIALLY: u32 = 65;
pub const TK_INSTEAD: u32 = 66;
pub const TK_NO: u32 = 67;
pub const TK_KEY: u32 = 68;
pub const TK_OF: u32 = 69;
pub const TK_OFFSET: u32 = 70;
pub const TK_PRAGMA: u32 = 71;
pub const TK_RAISE: u32 = 72;
pub const TK_RECURSIVE: u32 = 73;
pub const TK_REPLACE: u32 = 74;
pub const TK_RESTRICT: u32 = 75;
pub const TK_ROW: u32 = 76;
pub const TK_ROWS: u32 = 77;
pub const TK_TRIGGER: u32 = 78;
pub const TK_VACUUM: u32 = 79;
pub const TK_VIEW: u32 = 80;
pub const TK_VIRTUAL: u32 = 81;
pub const TK_WITH: u32 = 82;
pub const TK_NULLS: u32 = 83;
pub const TK_FIRST: u32 = 84;
pub const TK_LAST: u32 = 85;
pub const TK_CURRENT: u32 = 86;
pub const TK_FOLLOWING: u32 = 87;
pub const TK_PARTITION: u32 = 88;
pub const TK_PRECEDING: u32 = 89;
pub const TK_RANGE: u32 = 90;
pub const TK_UNBOUNDED: u32 = 91;
pub const TK_EXCLUDE: u32 = 92;
pub const TK_GROUPS: u32 = 93;
pub const TK_OTHERS: u32 = 94;
pub const TK_TIES: u32 = 95;
pub const TK_GENERATED: u32 = 96;
pub const TK_ALWAYS: u32 = 97;
pub const TK_MATERIALIZED: u32 = 98;
pub const TK_REINDEX: u32 = 99;
pub const TK_RENAME: u32 = 100;
pub const TK_CTIME_KW: u32 = 101;
pub const TK_ANY: u32 = 102;
pub const TK_BITAND: u32 = 103;
pub const TK_BITOR: u32 = 104;
pub const TK_LSHIFT: u32 = 105;
pub const TK_RSHIFT: u32 = 106;
pub const TK_PLUS: u32 = 107;
pub const TK_MINUS: u32 = 108;
pub const TK_STAR: u32 = 109;
pub const TK_SLASH: u32 = 110;
pub const TK_REM: u32 = 111;
pub const TK_CONCAT: u32 = 112;
pub const TK_PTR: u32 = 113;
pub const TK_COLLATE: u32 = 114;
pub const TK_BITNOT: u32 = 115;
pub const TK_ON: u32 = 116;
pub const TK_INDEXED: u32 = 117;
pub const TK_STRING: u32 = 118;
pub const TK_JOIN_KW: u32 = 119;
pub const TK_CONSTRAINT: u32 = 120;
pub const TK_DEFAULT: u32 = 121;
pub const TK_NULL: u32 = 122;
pub const TK_PRIMARY: u32 = 123;
pub const TK_UNIQUE: u32 = 124;
pub const TK_CHECK: u32 = 125;
pub const TK_REFERENCES: u32 = 126;
pub const TK_AUTOINCR: u32 = 127;
pub const TK_INSERT: u32 = 128;
pub const TK_DELETE: u32 = 129;
pub const TK_UPDATE: u32 = 130;
pub const TK_SET: u32 = 131;
pub const TK_DEFERRABLE: u32 = 132;
pub const TK_FOREIGN: u32 = 133;
pub const TK_DROP: u32 = 134;
pub const TK_UNION: u32 = 135;
pub const TK_ALL: u32 = 136;
pub const TK_EXCEPT: u32 = 137;
pub const TK_INTERSECT: u32 = 138;
pub const TK_SELECT: u32 = 139;
pub const TK_VALUES: u32 = 140;
pub const TK_DISTINCT: u32 = 141;
pub const TK_DOT: u32 = 142;
pub const TK_FROM: u32 = 143;
pub const TK_JOIN: u32 = 144;
pub const TK_USING: u32 = 145;
pub const TK_ORDER: u32 = 146;
pub const TK_GROUP: u32 = 147;
pub const TK_HAVING: u32 = 148;
pub const TK_LIMIT: u32 = 149;
pub const TK_WHERE: u32 = 150;
pub const TK_RETURNING: u32 = 151;
pub const TK_INTO: u32 = 152;
pub const TK_NOTHING: u32 = 153;
pub const TK_FLOAT: u32 = 154;
pub const TK_BLOB: u32 = 155;
pub const TK_INTEGER: u32 = 156;
pub const TK_VARIABLE: u32 = 157;
pub const TK_CASE: u32 = 158;
pub const TK_WHEN: u32 = 159;
pub const TK_THEN: u32 = 160;
pub const TK_ELSE: u32 = 161;
pub const TK_INDEX: u32 = 162;
pub const TK_ALTER: u32 = 163;
pub const TK_ADD: u32 = 164;
pub const TK_WINDOW: u32 = 165;
pub const TK_OVER: u32 = 166;
pub const TK_FILTER: u32 = 167;
pub const TK_COLUMN: u32 = 168;
pub const TK_AGG_FUNCTION: u32 = 169;
pub const TK_AGG_COLUMN: u32 = 170;
pub const TK_TRUEFALSE: u32 = 171;
pub const TK_FUNCTION: u32 = 172;
pub const TK_UPLUS: u32 = 173;
pub const TK_UMINUS: u32 = 174;
pub const TK_TRUTH: u32 = 175;
pub const TK_REGISTER: u32 = 176;
pub const TK_VECTOR: u32 = 177;
pub const TK_SELECT_COLUMN: u32 = 178;
pub const TK_IF_NULL_ROW: u32 = 179;
pub const TK_ASTERISK: u32 = 180;
pub const TK_SPAN: u32 = 181;
pub const TK_ERROR: u32 = 182;
pub const TK_QNUMBER: u32 = 183;
pub const TK_SPACE: u32 = 184;
pub const TK_COMMENT: u32 = 185;
pub const TK_ILLEGAL: u32 = 186;
pub const P4_NOTUSED: u32 = 0;
pub const P4_TRANSIENT: u32 = 0;
pub const P4_STATIC: i32 = -1;
pub const P4_COLLSEQ: i32 = -2;
pub const P4_INT32: i32 = -3;
pub const P4_SUBPROGRAM: i32 = -4;
pub const P4_TABLE: i32 = -5;
pub const P4_FREE_IF_LE: i32 = -6;
pub const P4_DYNAMIC: i32 = -6;
pub const P4_FUNCDEF: i32 = -7;
pub const P4_KEYINFO: i32 = -8;
pub const P4_EXPR: i32 = -9;
pub const P4_MEM: i32 = -10;
pub const P4_VTAB: i32 = -11;
pub const P4_REAL: i32 = -12;
pub const P4_INT64: i32 = -13;
pub const P4_INTARRAY: i32 = -14;
pub const P4_FUNCCTX: i32 = -15;
pub const P4_TABLEREF: i32 = -16;
pub const P4_SUBRTNSIG: i32 = -17;
pub const P5_ConstraintNotNull: u32 = 1;
pub const P5_ConstraintUnique: u32 = 2;
pub const P5_ConstraintCheck: u32 = 3;
pub const P5_ConstraintFK: u32 = 4;
pub const COLNAME_NAME: u32 = 0;
pub const COLNAME_DECLTYPE: u32 = 1;
pub const COLNAME_DATABASE: u32 = 2;
pub const COLNAME_TABLE: u32 = 3;
pub const COLNAME_COLUMN: u32 = 4;
pub const COLNAME_N: u32 = 5;
pub const OP_Savepoint: u32 = 0;
pub const OP_AutoCommit: u32 = 1;
pub const OP_Transaction: u32 = 2;
pub const OP_Checkpoint: u32 = 3;
pub const OP_JournalMode: u32 = 4;
pub const OP_Vacuum: u32 = 5;
pub const OP_VFilter: u32 = 6;
pub const OP_VUpdate: u32 = 7;
pub const OP_Init: u32 = 8;
pub const OP_Goto: u32 = 9;
pub const OP_Gosub: u32 = 10;
pub const OP_InitCoroutine: u32 = 11;
pub const OP_Yield: u32 = 12;
pub const OP_MustBeInt: u32 = 13;
pub const OP_Jump: u32 = 14;
pub const OP_Once: u32 = 15;
pub const OP_If: u32 = 16;
pub const OP_IfNot: u32 = 17;
pub const OP_IsType: u32 = 18;
pub const OP_Not: u32 = 19;
pub const OP_IfNullRow: u32 = 20;
pub const OP_SeekLT: u32 = 21;
pub const OP_SeekLE: u32 = 22;
pub const OP_SeekGE: u32 = 23;
pub const OP_SeekGT: u32 = 24;
pub const OP_IfNotOpen: u32 = 25;
pub const OP_IfNoHope: u32 = 26;
pub const OP_NoConflict: u32 = 27;
pub const OP_NotFound: u32 = 28;
pub const OP_Found: u32 = 29;
pub const OP_SeekRowid: u32 = 30;
pub const OP_NotExists: u32 = 31;
pub const OP_Last: u32 = 32;
pub const OP_IfSizeBetween: u32 = 33;
pub const OP_SorterSort: u32 = 34;
pub const OP_Sort: u32 = 35;
pub const OP_Rewind: u32 = 36;
pub const OP_IfEmpty: u32 = 37;
pub const OP_SorterNext: u32 = 38;
pub const OP_Prev: u32 = 39;
pub const OP_Next: u32 = 40;
pub const OP_IdxLE: u32 = 41;
pub const OP_IdxGT: u32 = 42;
pub const OP_Or: u32 = 43;
pub const OP_And: u32 = 44;
pub const OP_IdxLT: u32 = 45;
pub const OP_IdxGE: u32 = 46;
pub const OP_RowSetRead: u32 = 47;
pub const OP_RowSetTest: u32 = 48;
pub const OP_Program: u32 = 49;
pub const OP_FkIfZero: u32 = 50;
pub const OP_IsNull: u32 = 51;
pub const OP_NotNull: u32 = 52;
pub const OP_Ne: u32 = 53;
pub const OP_Eq: u32 = 54;
pub const OP_Gt: u32 = 55;
pub const OP_Le: u32 = 56;
pub const OP_Lt: u32 = 57;
pub const OP_Ge: u32 = 58;
pub const OP_ElseEq: u32 = 59;
pub const OP_IfPos: u32 = 60;
pub const OP_IfNotZero: u32 = 61;
pub const OP_DecrJumpZero: u32 = 62;
pub const OP_IncrVacuum: u32 = 63;
pub const OP_VNext: u32 = 64;
pub const OP_Filter: u32 = 65;
pub const OP_PureFunc: u32 = 66;
pub const OP_Function: u32 = 67;
pub const OP_Return: u32 = 68;
pub const OP_EndCoroutine: u32 = 69;
pub const OP_HaltIfNull: u32 = 70;
pub const OP_Halt: u32 = 71;
pub const OP_Integer: u32 = 72;
pub const OP_Int64: u32 = 73;
pub const OP_String: u32 = 74;
pub const OP_BeginSubrtn: u32 = 75;
pub const OP_Null: u32 = 76;
pub const OP_SoftNull: u32 = 77;
pub const OP_Blob: u32 = 78;
pub const OP_Variable: u32 = 79;
pub const OP_Move: u32 = 80;
pub const OP_Copy: u32 = 81;
pub const OP_SCopy: u32 = 82;
pub const OP_IntCopy: u32 = 83;
pub const OP_FkCheck: u32 = 84;
pub const OP_ResultRow: u32 = 85;
pub const OP_CollSeq: u32 = 86;
pub const OP_AddImm: u32 = 87;
pub const OP_RealAffinity: u32 = 88;
pub const OP_Cast: u32 = 89;
pub const OP_Permutation: u32 = 90;
pub const OP_Compare: u32 = 91;
pub const OP_IsTrue: u32 = 92;
pub const OP_ZeroOrNull: u32 = 93;
pub const OP_Offset: u32 = 94;
pub const OP_Column: u32 = 95;
pub const OP_TypeCheck: u32 = 96;
pub const OP_Affinity: u32 = 97;
pub const OP_MakeRecord: u32 = 98;
pub const OP_Count: u32 = 99;
pub const OP_ReadCookie: u32 = 100;
pub const OP_SetCookie: u32 = 101;
pub const OP_ReopenIdx: u32 = 102;
pub const OP_BitAnd: u32 = 103;
pub const OP_BitOr: u32 = 104;
pub const OP_ShiftLeft: u32 = 105;
pub const OP_ShiftRight: u32 = 106;
pub const OP_Add: u32 = 107;
pub const OP_Subtract: u32 = 108;
pub const OP_Multiply: u32 = 109;
pub const OP_Divide: u32 = 110;
pub const OP_Remainder: u32 = 111;
pub const OP_Concat: u32 = 112;
pub const OP_OpenRead: u32 = 113;
pub const OP_OpenWrite: u32 = 114;
pub const OP_BitNot: u32 = 115;
pub const OP_OpenDup: u32 = 116;
pub const OP_OpenAutoindex: u32 = 117;
pub const OP_String8: u32 = 118;
pub const OP_OpenEphemeral: u32 = 119;
pub const OP_SorterOpen: u32 = 120;
pub const OP_SequenceTest: u32 = 121;
pub const OP_OpenPseudo: u32 = 122;
pub const OP_Close: u32 = 123;
pub const OP_ColumnsUsed: u32 = 124;
pub const OP_SeekScan: u32 = 125;
pub const OP_SeekHit: u32 = 126;
pub const OP_Sequence: u32 = 127;
pub const OP_NewRowid: u32 = 128;
pub const OP_Insert: u32 = 129;
pub const OP_RowCell: u32 = 130;
pub const OP_Delete: u32 = 131;
pub const OP_ResetCount: u32 = 132;
pub const OP_SorterCompare: u32 = 133;
pub const OP_SorterData: u32 = 134;
pub const OP_RowData: u32 = 135;
pub const OP_Rowid: u32 = 136;
pub const OP_NullRow: u32 = 137;
pub const OP_SeekEnd: u32 = 138;
pub const OP_IdxInsert: u32 = 139;
pub const OP_SorterInsert: u32 = 140;
pub const OP_IdxDelete: u32 = 141;
pub const OP_DeferredSeek: u32 = 142;
pub const OP_IdxRowid: u32 = 143;
pub const OP_FinishSeek: u32 = 144;
pub const OP_Destroy: u32 = 145;
pub const OP_Clear: u32 = 146;
pub const OP_ResetSorter: u32 = 147;
pub const OP_CreateBtree: u32 = 148;
pub const OP_SqlExec: u32 = 149;
pub const OP_ParseSchema: u32 = 150;
pub const OP_LoadAnalysis: u32 = 151;
pub const OP_DropTable: u32 = 152;
pub const OP_DropIndex: u32 = 153;
pub const OP_Real: u32 = 154;
pub const OP_DropTrigger: u32 = 155;
pub const OP_IntegrityCk: u32 = 156;
pub const OP_RowSetAdd: u32 = 157;
pub const OP_Param: u32 = 158;
pub const OP_FkCounter: u32 = 159;
pub const OP_MemMax: u32 = 160;
pub const OP_OffsetLimit: u32 = 161;
pub const OP_AggInverse: u32 = 162;
pub const OP_AggStep: u32 = 163;
pub const OP_AggStep1: u32 = 164;
pub const OP_AggValue: u32 = 165;
pub const OP_AggFinal: u32 = 166;
pub const OP_Expire: u32 = 167;
pub const OP_CursorLock: u32 = 168;
pub const OP_CursorUnlock: u32 = 169;
pub const OP_TableLock: u32 = 170;
pub const OP_VBegin: u32 = 171;
pub const OP_VCreate: u32 = 172;
pub const OP_VDestroy: u32 = 173;
pub const OP_VOpen: u32 = 174;
pub const OP_VCheck: u32 = 175;
pub const OP_VInitIn: u32 = 176;
pub const OP_VColumn: u32 = 177;
pub const OP_VRename: u32 = 178;
pub const OP_Pagecount: u32 = 179;
pub const OP_MaxPgcnt: u32 = 180;
pub const OP_ClrSubtype: u32 = 181;
pub const OP_GetSubtype: u32 = 182;
pub const OP_SetSubtype: u32 = 183;
pub const OP_FilterAdd: u32 = 184;
pub const OP_Trace: u32 = 185;
pub const OP_CursorHint: u32 = 186;
pub const OP_ReleaseReg: u32 = 187;
pub const OP_Noop: u32 = 188;
pub const OP_Explain: u32 = 189;
pub const OP_Abortable: u32 = 190;
pub const OPFLG_JUMP: u32 = 1;
pub const OPFLG_IN1: u32 = 2;
pub const OPFLG_IN2: u32 = 4;
pub const OPFLG_IN3: u32 = 8;
pub const OPFLG_OUT2: u32 = 16;
pub const OPFLG_OUT3: u32 = 32;
pub const OPFLG_NCYCLE: u32 = 64;
pub const OPFLG_JUMP0: u32 = 128;
pub const SQLITE_MX_JUMP_OPCODE: u32 = 65;
pub const SQLITE_PREPARE_SAVESQL: u32 = 128;
pub const SQLITE_PREPARE_MASK: u32 = 31;
pub const SQLITE_MAX_SCHEMA_RETRY: u32 = 50;
pub const VDBE_DISPLAY_P4: u32 = 1;
pub const CURTYPE_BTREE: u32 = 0;
pub const CURTYPE_SORTER: u32 = 1;
pub const CURTYPE_VTAB: u32 = 2;
pub const CURTYPE_PSEUDO: u32 = 3;
pub const CACHE_STALE: u32 = 0;
pub const SQLITE_FRAME_MAGIC: u32 = 2275391262;
pub const MEM_Undefined: u32 = 0;
pub const MEM_Null: u32 = 1;
pub const MEM_Str: u32 = 2;
pub const MEM_Int: u32 = 4;
pub const MEM_Real: u32 = 8;
pub const MEM_Blob: u32 = 16;
pub const MEM_IntReal: u32 = 32;
pub const MEM_AffMask: u32 = 63;
pub const MEM_FromBind: u32 = 64;
pub const MEM_Cleared: u32 = 256;
pub const MEM_Term: u32 = 512;
pub const MEM_Zero: u32 = 1024;
pub const MEM_Subtype: u32 = 2048;
pub const MEM_TypeMask: u32 = 3519;
pub const MEM_Dyn: u32 = 4096;
pub const MEM_Static: u32 = 8192;
pub const MEM_Ephem: u32 = 16384;
pub const MEM_Agg: u32 = 32768;
pub const VDBE_INIT_STATE: u32 = 0;
pub const VDBE_READY_STATE: u32 = 1;
pub const VDBE_RUN_STATE: u32 = 2;
pub const VDBE_HALT_STATE: u32 = 3;
pub type sqlite_int64 = ::std::os::raw::c_longlong;
pub type sqlite_uint64 = ::std::os::raw::c_ulonglong;
pub type sqlite3_int64 = sqlite_int64;
pub type sqlite3_uint64 = sqlite_uint64;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sqlite3_file {
    pub pMethods: *const sqlite3_io_methods,
}
impl Default for sqlite3_file {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct sqlite3_io_methods {
    pub iVersion: ::std::os::raw::c_int,
    pub xClose: ::std::option::Option<
        unsafe extern "C" fn(arg1: *mut sqlite3_file) -> ::std::os::raw::c_int,
    >,
    pub xRead: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut sqlite3_file,
            arg2: *mut ::std::os::raw::c_void,
            iAmt: ::std::os::raw::c_int,
            iOfst: sqlite3_int64,
        ) -> ::std::os::raw::c_int,
    >,
    pub xWrite: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut sqlite3_file,
            arg2: *const ::std::os::raw::c_void,
            iAmt: ::std::os::raw::c_int,
            iOfst: sqlite3_int64,
        ) -> ::std::os::raw::c_int,
    >,
    pub xTruncate: ::std::option::Option<
        unsafe extern "C" fn(arg1: *mut sqlite3_file, size: sqlite3_int64) -> ::std::os::raw::c_int,
    >,
    pub xSync: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut sqlite3_file,
            flags: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub xFileSize: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut sqlite3_file,
            pSize: *mut sqlite3_int64,
        ) -> ::std::os::raw::c_int,
    >,
    pub xLock: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut sqlite3_file,
            arg2: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub xUnlock: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut sqlite3_file,
            arg2: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub xCheckReservedLock: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut sqlite3_file,
            pResOut: *mut ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub xFileControl: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut sqlite3_file,
            op: ::std::os::raw::c_int,
            pArg: *mut ::std::os::raw::c_void,
        ) -> ::std::os::raw::c_int,
    >,
    pub xSectorSize: ::std::option::Option<
        unsafe extern "C" fn(arg1: *mut sqlite3_file) -> ::std::os::raw::c_int,
    >,
    pub xDeviceCharacteristics: ::std::option::Option<
        unsafe extern "C" fn(arg1: *mut sqlite3_file) -> ::std::os::raw::c_int,
    >,
    pub xShmMap: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut sqlite3_file,
            iPg: ::std::os::raw::c_int,
            pgsz: ::std::os::raw::c_int,
            arg2: ::std::os::raw::c_int,
            arg3: *mut *mut ::std::os::raw::c_void,
        ) -> ::std::os::raw::c_int,
    >,
    pub xShmLock: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut sqlite3_file,
            offset: ::std::os::raw::c_int,
            n: ::std::os::raw::c_int,
            flags: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub xShmBarrier: ::std::option::Option<unsafe extern "C" fn(arg1: *mut sqlite3_file)>,
    pub xShmUnmap: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut sqlite3_file,
            deleteFlag: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub xFetch: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut sqlite3_file,
            iOfst: sqlite3_int64,
            iAmt: ::std::os::raw::c_int,
            pp: *mut *mut ::std::os::raw::c_void,
        ) -> ::std::os::raw::c_int,
    >,
    pub xUnfetch: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut sqlite3_file,
            iOfst: sqlite3_int64,
            p: *mut ::std::os::raw::c_void,
        ) -> ::std::os::raw::c_int,
    >,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sqlite3_mutex {
    _unused: [u8; 0],
}
pub type sqlite3_filename = *const ::std::os::raw::c_char;
pub type sqlite3_syscall_ptr = ::std::option::Option<unsafe extern "C" fn()>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sqlite3_vfs {
    pub iVersion: ::std::os::raw::c_int,
    pub szOsFile: ::std::os::raw::c_int,
    pub mxPathname: ::std::os::raw::c_int,
    pub pNext: *mut sqlite3_vfs,
    pub zName: *const ::std::os::raw::c_char,
    pub pAppData: *mut ::std::os::raw::c_void,
    pub xOpen: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut sqlite3_vfs,
            zName: sqlite3_filename,
            arg2: *mut sqlite3_file,
            flags: ::std::os::raw::c_int,
            pOutFlags: *mut ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub xDelete: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut sqlite3_vfs,
            zName: *const ::std::os::raw::c_char,
            syncDir: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub xAccess: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut sqlite3_vfs,
            zName: *const ::std::os::raw::c_char,
            flags: ::std::os::raw::c_int,
            pResOut: *mut ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub xFullPathname: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut sqlite3_vfs,
            zName: *const ::std::os::raw::c_char,
            nOut: ::std::os::raw::c_int,
            zOut: *mut ::std::os::raw::c_char,
        ) -> ::std::os::raw::c_int,
    >,
    pub xDlOpen: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut sqlite3_vfs,
            zFilename: *const ::std::os::raw::c_char,
        ) -> *mut ::std::os::raw::c_void,
    >,
    pub xDlError: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut sqlite3_vfs,
            nByte: ::std::os::raw::c_int,
            zErrMsg: *mut ::std::os::raw::c_char,
        ),
    >,
    pub xDlSym: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut sqlite3_vfs,
            arg2: *mut ::std::os::raw::c_void,
            zSymbol: *const ::std::os::raw::c_char,
        ) -> ::std::option::Option<
            unsafe extern "C" fn(
                arg1: *mut sqlite3_vfs,
                arg2: *mut ::std::os::raw::c_void,
                zSymbol: *const ::std::os::raw::c_char,
            ),
        >,
    >,
    pub xDlClose: ::std::option::Option<
        unsafe extern "C" fn(arg1: *mut sqlite3_vfs, arg2: *mut ::std::os::raw::c_void),
    >,
    pub xRandomness: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut sqlite3_vfs,
            nByte: ::std::os::raw::c_int,
            zOut: *mut ::std::os::raw::c_char,
        ) -> ::std::os::raw::c_int,
    >,
    pub xSleep: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut sqlite3_vfs,
            microseconds: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub xCurrentTime: ::std::option::Option<
        unsafe extern "C" fn(arg1: *mut sqlite3_vfs, arg2: *mut f64) -> ::std::os::raw::c_int,
    >,
    pub xGetLastError: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut sqlite3_vfs,
            arg2: ::std::os::raw::c_int,
            arg3: *mut ::std::os::raw::c_char,
        ) -> ::std::os::raw::c_int,
    >,
    pub xCurrentTimeInt64: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut sqlite3_vfs,
            arg2: *mut sqlite3_int64,
        ) -> ::std::os::raw::c_int,
    >,
    pub xSetSystemCall: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut sqlite3_vfs,
            zName: *const ::std::os::raw::c_char,
            arg2: sqlite3_syscall_ptr,
        ) -> ::std::os::raw::c_int,
    >,
    pub xGetSystemCall: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut sqlite3_vfs,
            zName: *const ::std::os::raw::c_char,
        ) -> sqlite3_syscall_ptr,
    >,
    pub xNextSystemCall: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut sqlite3_vfs,
            zName: *const ::std::os::raw::c_char,
        ) -> *const ::std::os::raw::c_char,
    >,
}
impl Default for sqlite3_vfs {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct sqlite3_module {
    pub iVersion: ::std::os::raw::c_int,
    pub xCreate: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut sqlite3,
            pAux: *mut ::std::os::raw::c_void,
            argc: ::std::os::raw::c_int,
            argv: *const *const ::std::os::raw::c_char,
            ppVTab: *mut *mut sqlite3_vtab,
            arg2: *mut *mut ::std::os::raw::c_char,
        ) -> ::std::os::raw::c_int,
    >,
    pub xConnect: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut sqlite3,
            pAux: *mut ::std::os::raw::c_void,
            argc: ::std::os::raw::c_int,
            argv: *const *const ::std::os::raw::c_char,
            ppVTab: *mut *mut sqlite3_vtab,
            arg2: *mut *mut ::std::os::raw::c_char,
        ) -> ::std::os::raw::c_int,
    >,
    pub xBestIndex: ::std::option::Option<
        unsafe extern "C" fn(
            pVTab: *mut sqlite3_vtab,
            arg1: *mut sqlite3_index_info,
        ) -> ::std::os::raw::c_int,
    >,
    pub xDisconnect: ::std::option::Option<
        unsafe extern "C" fn(pVTab: *mut sqlite3_vtab) -> ::std::os::raw::c_int,
    >,
    pub xDestroy: ::std::option::Option<
        unsafe extern "C" fn(pVTab: *mut sqlite3_vtab) -> ::std::os::raw::c_int,
    >,
    pub xOpen: ::std::option::Option<
        unsafe extern "C" fn(
            pVTab: *mut sqlite3_vtab,
            ppCursor: *mut *mut sqlite3_vtab_cursor,
        ) -> ::std::os::raw::c_int,
    >,
    pub xClose: ::std::option::Option<
        unsafe extern "C" fn(arg1: *mut sqlite3_vtab_cursor) -> ::std::os::raw::c_int,
    >,
    pub xFilter: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut sqlite3_vtab_cursor,
            idxNum: ::std::os::raw::c_int,
            idxStr: *const ::std::os::raw::c_char,
            argc: ::std::os::raw::c_int,
            argv: *mut *mut sqlite3_value,
        ) -> ::std::os::raw::c_int,
    >,
    pub xNext: ::std::option::Option<
        unsafe extern "C" fn(arg1: *mut sqlite3_vtab_cursor) -> ::std::os::raw::c_int,
    >,
    pub xEof: ::std::option::Option<
        unsafe extern "C" fn(arg1: *mut sqlite3_vtab_cursor) -> ::std::os::raw::c_int,
    >,
    pub xColumn: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut sqlite3_vtab_cursor,
            arg2: *mut sqlite3_context,
            arg3: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub xRowid: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut sqlite3_vtab_cursor,
            pRowid: *mut sqlite3_int64,
        ) -> ::std::os::raw::c_int,
    >,
    pub xUpdate: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut sqlite3_vtab,
            arg2: ::std::os::raw::c_int,
            arg3: *mut *mut sqlite3_value,
            arg4: *mut sqlite3_int64,
        ) -> ::std::os::raw::c_int,
    >,
    pub xBegin: ::std::option::Option<
        unsafe extern "C" fn(pVTab: *mut sqlite3_vtab) -> ::std::os::raw::c_int,
    >,
    pub xSync: ::std::option::Option<
        unsafe extern "C" fn(pVTab: *mut sqlite3_vtab) -> ::std::os::raw::c_int,
    >,
    pub xCommit: ::std::option::Option<
        unsafe extern "C" fn(pVTab: *mut sqlite3_vtab) -> ::std::os::raw::c_int,
    >,
    pub xRollback: ::std::option::Option<
        unsafe extern "C" fn(pVTab: *mut sqlite3_vtab) -> ::std::os::raw::c_int,
    >,
    pub xFindFunction: ::std::option::Option<
        unsafe extern "C" fn(
            pVtab: *mut sqlite3_vtab,
            nArg: ::std::os::raw::c_int,
            zName: *const ::std::os::raw::c_char,
            pxFunc: *mut ::std::option::Option<
                unsafe extern "C" fn(
                    arg1: *mut sqlite3_context,
                    arg2: ::std::os::raw::c_int,
                    arg3: *mut *mut sqlite3_value,
                ),
            >,
            ppArg: *mut *mut ::std::os::raw::c_void,
        ) -> ::std::os::raw::c_int,
    >,
    pub xRename: ::std::option::Option<
        unsafe extern "C" fn(
            pVtab: *mut sqlite3_vtab,
            zNew: *const ::std::os::raw::c_char,
        ) -> ::std::os::raw::c_int,
    >,
    pub xSavepoint: ::std::option::Option<
        unsafe extern "C" fn(
            pVTab: *mut sqlite3_vtab,
            arg1: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub xRelease: ::std::option::Option<
        unsafe extern "C" fn(
            pVTab: *mut sqlite3_vtab,
            arg1: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub xRollbackTo: ::std::option::Option<
        unsafe extern "C" fn(
            pVTab: *mut sqlite3_vtab,
            arg1: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub xShadowName: ::std::option::Option<
        unsafe extern "C" fn(arg1: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int,
    >,
    pub xIntegrity: ::std::option::Option<
        unsafe extern "C" fn(
            pVTab: *mut sqlite3_vtab,
            zSchema: *const ::std::os::raw::c_char,
            zTabName: *const ::std::os::raw::c_char,
            mFlags: ::std::os::raw::c_int,
            pzErr: *mut *mut ::std::os::raw::c_char,
        ) -> ::std::os::raw::c_int,
    >,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sqlite3_index_info {
    pub nConstraint: ::std::os::raw::c_int,
    pub aConstraint: *mut sqlite3_index_info_sqlite3_index_constraint,
    pub nOrderBy: ::std::os::raw::c_int,
    pub aOrderBy: *mut sqlite3_index_info_sqlite3_index_orderby,
    pub aConstraintUsage: *mut sqlite3_index_info_sqlite3_index_constraint_usage,
    pub idxNum: ::std::os::raw::c_int,
    pub idxStr: *mut ::std::os::raw::c_char,
    pub needToFreeIdxStr: ::std::os::raw::c_int,
    pub orderByConsumed: ::std::os::raw::c_int,
    pub estimatedCost: f64,
    pub estimatedRows: sqlite3_int64,
    pub idxFlags: ::std::os::raw::c_int,
    pub colUsed: sqlite3_uint64,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct sqlite3_index_info_sqlite3_index_constraint {
    pub iColumn: ::std::os::raw::c_int,
    pub op: ::std::os::raw::c_uchar,
    pub usable: ::std::os::raw::c_uchar,
    pub iTermOffset: ::std::os::raw::c_int,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct sqlite3_index_info_sqlite3_index_orderby {
    pub iColumn: ::std::os::raw::c_int,
    pub desc: ::std::os::raw::c_uchar,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct sqlite3_index_info_sqlite3_index_constraint_usage {
    pub argvIndex: ::std::os::raw::c_int,
    pub omit: ::std::os::raw::c_uchar,
}
impl Default for sqlite3_index_info {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sqlite3_vtab {
    pub pModule: *const sqlite3_module,
    pub nRef: ::std::os::raw::c_int,
    pub zErrMsg: *mut ::std::os::raw::c_char,
}
impl Default for sqlite3_vtab {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sqlite3_vtab_cursor {
    pub pVtab: *mut sqlite3_vtab,
}
impl Default for sqlite3_vtab_cursor {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sqlite3_pcache_page {
    pub pBuf: *mut ::std::os::raw::c_void,
    pub pExtra: *mut ::std::os::raw::c_void,
}
impl Default for sqlite3_pcache_page {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Hash {
    pub htsize: ::std::os::raw::c_uint,
    pub count: ::std::os::raw::c_uint,
    pub first: *mut HashElem,
    pub ht: *mut Hash__ht,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Hash__ht {
    pub count: ::std::os::raw::c_uint,
    pub chain: *mut HashElem,
}
impl Default for Hash__ht {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl Default for Hash {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct HashElem {
    pub next: *mut HashElem,
    pub prev: *mut HashElem,
    pub data: *mut ::std::os::raw::c_void,
    pub pKey: *const ::std::os::raw::c_char,
    pub h: ::std::os::raw::c_uint,
}
impl Default for HashElem {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type i64_ = sqlite_int64;
pub type u64_ = sqlite_uint64;
pub type u32_ = u32;
pub type u16_ = u16;
pub type i16_ = i16;
pub type u8_ = u8;
pub type i8_ = i8;
pub type bft = ::std::os::raw::c_uint;
pub type LogEst = i16;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BusyHandler {
    pub xBusyHandler: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut ::std::os::raw::c_void,
            arg2: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub pBusyArg: *mut ::std::os::raw::c_void,
    pub nBusy: ::std::os::raw::c_int,
}
impl Default for BusyHandler {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Bitvec {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RenameToken {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TableLock {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VtabCtx {
    _unused: [u8; 0],
}
pub type Bitmask = u64_;
pub type VList = ::std::os::raw::c_int;
pub type Pgno = u32_;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Pager {
    _unused: [u8; 0],
}
pub type DbPage = PgHdr;
pub type Mem = sqlite3_value;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SubrtnSig {
    pub selId: ::std::os::raw::c_int,
    pub bComplete: u8_,
    pub zAff: *mut ::std::os::raw::c_char,
    pub iTable: ::std::os::raw::c_int,
    pub iAddr: ::std::os::raw::c_int,
    pub regReturn: ::std::os::raw::c_int,
}
impl Default for SubrtnSig {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VdbeOp {
    pub opcode: u8_,
    pub p4type: ::std::os::raw::c_schar,
    pub p5: u16_,
    pub p1: ::std::os::raw::c_int,
    pub p2: ::std::os::raw::c_int,
    pub p3: ::std::os::raw::c_int,
    pub p4: VdbeOp_p4union,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union VdbeOp_p4union {
    pub i: ::std::os::raw::c_int,
    pub p: *mut ::std::os::raw::c_void,
    pub z: *mut ::std::os::raw::c_char,
    pub pI64: *mut i64_,
    pub pReal: *mut f64,
    pub pFunc: *mut FuncDef,
    pub pCtx: *mut sqlite3_context,
    pub pColl: *mut CollSeq,
    pub pMem: *mut Mem,
    pub pVtab: *mut VTable,
    pub pKeyInfo: *mut KeyInfo,
    pub ai: *mut u32_,
    pub pProgram: *mut SubProgram,
    pub pTab: *mut Table,
    pub pSubrtnSig: *mut SubrtnSig,
}
impl Default for VdbeOp_p4union {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl Default for VdbeOp {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SubProgram {
    pub aOp: *mut VdbeOp,
    pub nOp: ::std::os::raw::c_int,
    pub nMem: ::std::os::raw::c_int,
    pub nCsr: ::std::os::raw::c_int,
    pub aOnce: *mut u8_,
    pub token: *mut ::std::os::raw::c_void,
    pub pNext: *mut SubProgram,
}
impl Default for SubProgram {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct VdbeOpList {
    pub opcode: u8_,
    pub p1: ::std::os::raw::c_schar,
    pub p2: ::std::os::raw::c_schar,
    pub p3: ::std::os::raw::c_schar,
}
unsafe extern "C" {
    pub fn sqlite3VdbeCreate(arg1: *mut Parse) -> *mut Vdbe;
}
unsafe extern "C" {
    pub fn sqlite3VdbeParser(arg1: *mut Vdbe) -> *mut Parse;
}
unsafe extern "C" {
    pub fn sqlite3VdbeAddOp0(arg1: *mut Vdbe, arg2: ::std::os::raw::c_int)
    -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn sqlite3VdbeAddOp1(
        arg1: *mut Vdbe,
        arg2: ::std::os::raw::c_int,
        arg3: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn sqlite3VdbeAddOp2(
        arg1: *mut Vdbe,
        arg2: ::std::os::raw::c_int,
        arg3: ::std::os::raw::c_int,
        arg4: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn sqlite3VdbeGoto(arg1: *mut Vdbe, arg2: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn sqlite3VdbeLoadString(
        arg1: *mut Vdbe,
        arg2: ::std::os::raw::c_int,
        arg3: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn sqlite3VdbeMultiLoad(
        arg1: *mut Vdbe,
        arg2: ::std::os::raw::c_int,
        arg3: *const ::std::os::raw::c_char,
        ...
    );
}
unsafe extern "C" {
    pub fn sqlite3VdbeAddOp3(
        arg1: *mut Vdbe,
        arg2: ::std::os::raw::c_int,
        arg3: ::std::os::raw::c_int,
        arg4: ::std::os::raw::c_int,
        arg5: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn sqlite3VdbeAddOp4(
        arg1: *mut Vdbe,
        arg2: ::std::os::raw::c_int,
        arg3: ::std::os::raw::c_int,
        arg4: ::std::os::raw::c_int,
        arg5: ::std::os::raw::c_int,
        zP4: *const ::std::os::raw::c_char,
        arg6: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn sqlite3VdbeAddOp4Dup8(
        arg1: *mut Vdbe,
        arg2: ::std::os::raw::c_int,
        arg3: ::std::os::raw::c_int,
        arg4: ::std::os::raw::c_int,
        arg5: ::std::os::raw::c_int,
        arg6: *const u8_,
        arg7: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn sqlite3VdbeAddOp4Int(
        arg1: *mut Vdbe,
        arg2: ::std::os::raw::c_int,
        arg3: ::std::os::raw::c_int,
        arg4: ::std::os::raw::c_int,
        arg5: ::std::os::raw::c_int,
        arg6: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn sqlite3VdbeAddFunctionCall(
        arg1: *mut Parse,
        arg2: ::std::os::raw::c_int,
        arg3: ::std::os::raw::c_int,
        arg4: ::std::os::raw::c_int,
        arg5: ::std::os::raw::c_int,
        arg6: *const FuncDef,
        arg7: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn sqlite3VdbeEndCoroutine(arg1: *mut Vdbe, arg2: ::std::os::raw::c_int);
}
unsafe extern "C" {
    pub fn sqlite3VdbeAddOpList(
        arg1: *mut Vdbe,
        nOp: ::std::os::raw::c_int,
        aOp: *const VdbeOpList,
        iLineno: ::std::os::raw::c_int,
    ) -> *mut VdbeOp;
}
unsafe extern "C" {
    pub fn sqlite3VdbeExplain(
        arg1: *mut Parse,
        arg2: u8_,
        arg3: *const ::std::os::raw::c_char,
        ...
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn sqlite3VdbeExplainPop(arg1: *mut Parse);
}
unsafe extern "C" {
    pub fn sqlite3VdbeExplainParent(arg1: *mut Parse) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn sqlite3VdbeAddParseSchemaOp(
        arg1: *mut Vdbe,
        arg2: ::std::os::raw::c_int,
        arg3: *mut ::std::os::raw::c_char,
        arg4: u16_,
    );
}
unsafe extern "C" {
    pub fn sqlite3VdbeChangeOpcode(arg1: *mut Vdbe, addr: ::std::os::raw::c_int, arg2: u8_);
}
unsafe extern "C" {
    pub fn sqlite3VdbeChangeP1(
        arg1: *mut Vdbe,
        addr: ::std::os::raw::c_int,
        P1: ::std::os::raw::c_int,
    );
}
unsafe extern "C" {
    pub fn sqlite3VdbeChangeP2(
        arg1: *mut Vdbe,
        addr: ::std::os::raw::c_int,
        P2: ::std::os::raw::c_int,
    );
}
unsafe extern "C" {
    pub fn sqlite3VdbeChangeP3(
        arg1: *mut Vdbe,
        addr: ::std::os::raw::c_int,
        P3: ::std::os::raw::c_int,
    );
}
unsafe extern "C" {
    pub fn sqlite3VdbeChangeP5(arg1: *mut Vdbe, P5: u16_);
}
unsafe extern "C" {
    pub fn sqlite3VdbeTypeofColumn(arg1: *mut Vdbe, arg2: ::std::os::raw::c_int);
}
unsafe extern "C" {
    pub fn sqlite3VdbeJumpHere(arg1: *mut Vdbe, addr: ::std::os::raw::c_int);
}
unsafe extern "C" {
    pub fn sqlite3VdbeJumpHereOrPopInst(arg1: *mut Vdbe, addr: ::std::os::raw::c_int);
}
unsafe extern "C" {
    pub fn sqlite3VdbeChangeToNoop(
        arg1: *mut Vdbe,
        addr: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn sqlite3VdbeDeletePriorOpcode(arg1: *mut Vdbe, op: u8_) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn sqlite3VdbeChangeP4(
        arg1: *mut Vdbe,
        addr: ::std::os::raw::c_int,
        zP4: *const ::std::os::raw::c_char,
        N: ::std::os::raw::c_int,
    );
}
unsafe extern "C" {
    pub fn sqlite3VdbeAppendP4(
        arg1: *mut Vdbe,
        pP4: *mut ::std::os::raw::c_void,
        p4type: ::std::os::raw::c_int,
    );
}
unsafe extern "C" {
    pub fn sqlite3VdbeSetP4KeyInfo(arg1: *mut Parse, arg2: *mut Index);
}
unsafe extern "C" {
    pub fn sqlite3VdbeUsesBtree(arg1: *mut Vdbe, arg2: ::std::os::raw::c_int);
}
unsafe extern "C" {
    pub fn sqlite3VdbeGetOp(arg1: *mut Vdbe, arg2: ::std::os::raw::c_int) -> *mut VdbeOp;
}
unsafe extern "C" {
    pub fn sqlite3VdbeGetLastOp(arg1: *mut Vdbe) -> *mut VdbeOp;
}
unsafe extern "C" {
    pub fn sqlite3VdbeMakeLabel(arg1: *mut Parse) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn sqlite3VdbeRunOnlyOnce(arg1: *mut Vdbe);
}
unsafe extern "C" {
    pub fn sqlite3VdbeReusable(arg1: *mut Vdbe);
}
unsafe extern "C" {
    pub fn sqlite3VdbeDelete(arg1: *mut Vdbe);
}
unsafe extern "C" {
    pub fn sqlite3VdbeMakeReady(arg1: *mut Vdbe, arg2: *mut Parse);
}
unsafe extern "C" {
    pub fn sqlite3VdbeFinalize(arg1: *mut Vdbe) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn sqlite3VdbeResolveLabel(arg1: *mut Vdbe, arg2: ::std::os::raw::c_int);
}
unsafe extern "C" {
    pub fn sqlite3VdbeCurrentAddr(arg1: *mut Vdbe) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn sqlite3VdbeResetStepResult(arg1: *mut Vdbe);
}
unsafe extern "C" {
    pub fn sqlite3VdbeRewind(arg1: *mut Vdbe);
}
unsafe extern "C" {
    pub fn sqlite3VdbeReset(arg1: *mut Vdbe) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn sqlite3VdbeSetNumCols(arg1: *mut Vdbe, arg2: ::std::os::raw::c_int);
}
unsafe extern "C" {
    pub fn sqlite3VdbeSetColName(
        arg1: *mut Vdbe,
        arg2: ::std::os::raw::c_int,
        arg3: ::std::os::raw::c_int,
        arg4: *const ::std::os::raw::c_char,
        arg5: ::std::option::Option<unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void)>,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn sqlite3VdbeCountChanges(arg1: *mut Vdbe);
}
unsafe extern "C" {
    pub fn sqlite3VdbeDb(arg1: *mut Vdbe) -> *mut sqlite3;
}
unsafe extern "C" {
    pub fn sqlite3VdbePrepareFlags(arg1: *mut Vdbe) -> u8_;
}
unsafe extern "C" {
    pub fn sqlite3VdbeSetSql(
        arg1: *mut Vdbe,
        z: *const ::std::os::raw::c_char,
        n: ::std::os::raw::c_int,
        arg2: u8_,
    );
}
unsafe extern "C" {
    pub fn sqlite3VdbeSwap(arg1: *mut Vdbe, arg2: *mut Vdbe);
}
unsafe extern "C" {
    pub fn sqlite3VdbeTakeOpArray(
        arg1: *mut Vdbe,
        arg2: *mut ::std::os::raw::c_int,
        arg3: *mut ::std::os::raw::c_int,
    ) -> *mut VdbeOp;
}
unsafe extern "C" {
    pub fn sqlite3VdbeGetBoundValue(
        arg1: *mut Vdbe,
        arg2: ::std::os::raw::c_int,
        arg3: u8_,
    ) -> *mut sqlite3_value;
}
unsafe extern "C" {
    pub fn sqlite3VdbeSetVarmask(arg1: *mut Vdbe, arg2: ::std::os::raw::c_int);
}
unsafe extern "C" {
    pub fn sqlite3VdbeExpandSql(
        arg1: *mut Vdbe,
        arg2: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn sqlite3MemCompare(
        arg1: *const Mem,
        arg2: *const Mem,
        arg3: *const CollSeq,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn sqlite3BlobCompare(arg1: *const Mem, arg2: *const Mem) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn sqlite3VdbeFuncName(arg1: *const sqlite3_context) -> *const ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn sqlite3VdbeRecordUnpack(
        arg1: ::std::os::raw::c_int,
        arg2: *const ::std::os::raw::c_void,
        arg3: *mut UnpackedRecord,
    );
}
unsafe extern "C" {
    pub fn sqlite3VdbeRecordCompare(
        arg1: ::std::os::raw::c_int,
        arg2: *const ::std::os::raw::c_void,
        arg3: *mut UnpackedRecord,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn sqlite3VdbeRecordCompareWithSkip(
        arg1: ::std::os::raw::c_int,
        arg2: *const ::std::os::raw::c_void,
        arg3: *mut UnpackedRecord,
        arg4: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn sqlite3VdbeAllocUnpackedRecord(arg1: *mut KeyInfo) -> *mut UnpackedRecord;
}
pub type RecordCompare = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: ::std::os::raw::c_int,
        arg2: *const ::std::os::raw::c_void,
        arg3: *mut UnpackedRecord,
    ) -> ::std::os::raw::c_int,
>;
unsafe extern "C" {
    pub fn sqlite3VdbeFindCompare(arg1: *mut UnpackedRecord) -> RecordCompare;
}
unsafe extern "C" {
    pub fn sqlite3VdbeLinkSubProgram(arg1: *mut Vdbe, arg2: *mut SubProgram);
}
unsafe extern "C" {
    pub fn sqlite3VdbeHasSubProgram(arg1: *mut Vdbe) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn sqlite3MemSetArrayInt64(
        aMem: *mut sqlite3_value,
        iIdx: ::std::os::raw::c_int,
        val: i64_,
    );
}
unsafe extern "C" {
    pub fn sqlite3NotPureFunc(arg1: *mut sqlite3_context) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn sqlite3VdbeBytecodeVtabInit(arg1: *mut sqlite3) -> ::std::os::raw::c_int;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PCache {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PgHdr {
    pub pPage: *mut sqlite3_pcache_page,
    pub pData: *mut ::std::os::raw::c_void,
    pub pExtra: *mut ::std::os::raw::c_void,
    pub pCache: *mut PCache,
    pub pDirty: *mut PgHdr,
    pub pPager: *mut Pager,
    pub pgno: Pgno,
    pub flags: u16_,
    #[doc = " Elements above, except pCache, are public.  All that follow are\n private to pcache.c and should not be accessed by other modules.\n pCache is grouped with the public elements for efficiency."]
    pub nRef: i64_,
    pub pDirtyNext: *mut PgHdr,
    pub pDirtyPrev: *mut PgHdr,
}
impl Default for PgHdr {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Db {
    pub zDbSName: *mut ::std::os::raw::c_char,
    pub pBt: *mut Btree,
    pub safety_level: u8_,
    pub bSyncSet: u8_,
    pub pSchema: *mut Schema,
}
impl Default for Db {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Schema {
    pub schema_cookie: ::std::os::raw::c_int,
    pub iGeneration: ::std::os::raw::c_int,
    pub tblHash: Hash,
    pub idxHash: Hash,
    pub trigHash: Hash,
    pub fkeyHash: Hash,
    pub pSeqTab: *mut Table,
    pub file_format: u8_,
    pub enc: u8_,
    pub schemaFlags: u16_,
    pub cache_size: ::std::os::raw::c_int,
}
impl Default for Schema {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Lookaside {
    pub bDisable: u32_,
    pub sz: u16_,
    pub szTrue: u16_,
    pub bMalloced: u8_,
    pub nSlot: u32_,
    pub anStat: [u32_; 3usize],
    pub pInit: *mut LookasideSlot,
    pub pFree: *mut LookasideSlot,
    pub pSmallInit: *mut LookasideSlot,
    pub pSmallFree: *mut LookasideSlot,
    pub pMiddle: *mut ::std::os::raw::c_void,
    pub pStart: *mut ::std::os::raw::c_void,
    pub pEnd: *mut ::std::os::raw::c_void,
    pub pTrueEnd: *mut ::std::os::raw::c_void,
}
impl Default for Lookaside {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct LookasideSlot {
    pub pNext: *mut LookasideSlot,
}
impl Default for LookasideSlot {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type sqlite3_xauth = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut ::std::os::raw::c_void,
        arg2: ::std::os::raw::c_int,
        arg3: *const ::std::os::raw::c_char,
        arg4: *const ::std::os::raw::c_char,
        arg5: *const ::std::os::raw::c_char,
        arg6: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int,
>;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sqlite3 {
    pub pVfs: *mut sqlite3_vfs,
    pub pVdbe: *mut Vdbe,
    pub pDfltColl: *mut CollSeq,
    pub mutex: *mut sqlite3_mutex,
    pub aDb: *mut Db,
    pub nDb: ::std::os::raw::c_int,
    pub mDbFlags: u32_,
    pub flags: u64_,
    pub lastRowid: i64_,
    pub szMmap: i64_,
    pub nSchemaLock: u32_,
    pub openFlags: ::std::os::raw::c_uint,
    pub errCode: ::std::os::raw::c_int,
    pub errByteOffset: ::std::os::raw::c_int,
    pub errMask: ::std::os::raw::c_int,
    pub iSysErrno: ::std::os::raw::c_int,
    pub dbOptFlags: u32_,
    pub enc: u8_,
    pub autoCommit: u8_,
    pub temp_store: u8_,
    pub mallocFailed: u8_,
    pub bBenignMalloc: u8_,
    pub dfltLockMode: u8_,
    pub nextAutovac: ::std::os::raw::c_schar,
    pub suppressErr: u8_,
    pub vtabOnConflict: u8_,
    pub isTransactionSavepoint: u8_,
    pub mTrace: u8_,
    pub noSharedCache: u8_,
    pub nSqlExec: u8_,
    pub eOpenState: u8_,
    pub nextPagesize: ::std::os::raw::c_int,
    pub nChange: i64_,
    pub nTotalChange: i64_,
    pub aLimit: [::std::os::raw::c_int; 12usize],
    pub nMaxSorterMmap: ::std::os::raw::c_int,
    pub init: sqlite3_sqlite3InitInfo,
    pub nVdbeActive: ::std::os::raw::c_int,
    pub nVdbeRead: ::std::os::raw::c_int,
    pub nVdbeWrite: ::std::os::raw::c_int,
    pub nVdbeExec: ::std::os::raw::c_int,
    pub nVDestroy: ::std::os::raw::c_int,
    pub nExtension: ::std::os::raw::c_int,
    pub aExtension: *mut *mut ::std::os::raw::c_void,
    pub trace: sqlite3__bindgen_ty_1,
    pub pTraceArg: *mut ::std::os::raw::c_void,
    pub xProfile: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut ::std::os::raw::c_void,
            arg2: *const ::std::os::raw::c_char,
            arg3: u64_,
        ),
    >,
    pub pProfileArg: *mut ::std::os::raw::c_void,
    pub pCommitArg: *mut ::std::os::raw::c_void,
    pub xCommitCallback: ::std::option::Option<
        unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void) -> ::std::os::raw::c_int,
    >,
    pub pRollbackArg: *mut ::std::os::raw::c_void,
    pub xRollbackCallback:
        ::std::option::Option<unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void)>,
    pub pUpdateArg: *mut ::std::os::raw::c_void,
    pub xUpdateCallback: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut ::std::os::raw::c_void,
            arg2: ::std::os::raw::c_int,
            arg3: *const ::std::os::raw::c_char,
            arg4: *const ::std::os::raw::c_char,
            arg5: sqlite_int64,
        ),
    >,
    pub pAutovacPagesArg: *mut ::std::os::raw::c_void,
    pub xAutovacDestr:
        ::std::option::Option<unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void)>,
    pub xAutovacPages: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut ::std::os::raw::c_void,
            arg2: *const ::std::os::raw::c_char,
            arg3: u32_,
            arg4: u32_,
            arg5: u32_,
        ) -> ::std::os::raw::c_uint,
    >,
    pub pParse: *mut Parse,
    pub pPreUpdateArg: *mut ::std::os::raw::c_void,
    pub xPreUpdateCallback: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut ::std::os::raw::c_void,
            arg2: *mut sqlite3,
            arg3: ::std::os::raw::c_int,
            arg4: *const ::std::os::raw::c_char,
            arg5: *const ::std::os::raw::c_char,
            arg6: sqlite3_int64,
            arg7: sqlite3_int64,
        ),
    >,
    pub pPreUpdate: *mut PreUpdate,
    pub xWalCallback: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut ::std::os::raw::c_void,
            arg2: *mut sqlite3,
            arg3: *const ::std::os::raw::c_char,
            arg4: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub pWalArg: *mut ::std::os::raw::c_void,
    pub xCollNeeded: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut ::std::os::raw::c_void,
            arg2: *mut sqlite3,
            eTextRep: ::std::os::raw::c_int,
            arg3: *const ::std::os::raw::c_char,
        ),
    >,
    pub xCollNeeded16: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut ::std::os::raw::c_void,
            arg2: *mut sqlite3,
            eTextRep: ::std::os::raw::c_int,
            arg3: *const ::std::os::raw::c_void,
        ),
    >,
    pub pCollNeededArg: *mut ::std::os::raw::c_void,
    pub pErr: *mut sqlite3_value,
    pub u1: sqlite3__bindgen_ty_2,
    pub lookaside: Lookaside,
    pub xAuth: sqlite3_xauth,
    pub pAuthArg: *mut ::std::os::raw::c_void,
    pub xProgress: ::std::option::Option<
        unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void) -> ::std::os::raw::c_int,
    >,
    pub pProgressArg: *mut ::std::os::raw::c_void,
    pub nProgressOps: ::std::os::raw::c_uint,
    pub nVTrans: ::std::os::raw::c_int,
    pub aModule: Hash,
    pub pVtabCtx: *mut VtabCtx,
    pub aVTrans: *mut *mut VTable,
    pub pDisconnect: *mut VTable,
    pub aFunc: Hash,
    pub aCollSeq: Hash,
    pub busyHandler: BusyHandler,
    pub aDbStatic: [Db; 2usize],
    pub pSavepoint: *mut Savepoint,
    pub nAnalysisLimit: ::std::os::raw::c_int,
    pub busyTimeout: ::std::os::raw::c_int,
    pub nSavepoint: ::std::os::raw::c_int,
    pub nStatement: ::std::os::raw::c_int,
    pub nDeferredCons: i64_,
    pub nDeferredImmCons: i64_,
    pub pnBytesFreed: *mut ::std::os::raw::c_int,
    pub pDbData: *mut DbClientData,
    pub nSpill: u64_,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sqlite3_sqlite3InitInfo {
    pub newTnum: Pgno,
    pub iDb: u8_,
    pub busy: u8_,
    pub _bitfield_align_1: [u8; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 1usize]>,
    pub azInit: *mut *const ::std::os::raw::c_char,
}
impl Default for sqlite3_sqlite3InitInfo {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl sqlite3_sqlite3InitInfo {
    #[inline]
    pub fn orphanTrigger(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(0usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_orphanTrigger(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(0usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn orphanTrigger_raw(this: *const Self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 1usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                0usize,
                1u8,
            ) as u32)
        }
    }
    #[inline]
    pub unsafe fn set_orphanTrigger_raw(this: *mut Self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 1usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                0usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn imposterTable(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(1usize, 2u8) as u32) }
    }
    #[inline]
    pub fn set_imposterTable(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(1usize, 2u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn imposterTable_raw(this: *const Self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 1usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                1usize,
                2u8,
            ) as u32)
        }
    }
    #[inline]
    pub unsafe fn set_imposterTable_raw(this: *mut Self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 1usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                1usize,
                2u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn reopenMemdb(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(3usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_reopenMemdb(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(3usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn reopenMemdb_raw(this: *const Self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 1usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                3usize,
                1u8,
            ) as u32)
        }
    }
    #[inline]
    pub unsafe fn set_reopenMemdb_raw(this: *mut Self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 1usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                3usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        orphanTrigger: ::std::os::raw::c_uint,
        imposterTable: ::std::os::raw::c_uint,
        reopenMemdb: ::std::os::raw::c_uint,
    ) -> __BindgenBitfieldUnit<[u8; 1usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 1usize]> = Default::default();
        __bindgen_bitfield_unit.set(0usize, 1u8, {
            let orphanTrigger: u32 = unsafe { ::std::mem::transmute(orphanTrigger) };
            orphanTrigger as u64
        });
        __bindgen_bitfield_unit.set(1usize, 2u8, {
            let imposterTable: u32 = unsafe { ::std::mem::transmute(imposterTable) };
            imposterTable as u64
        });
        __bindgen_bitfield_unit.set(3usize, 1u8, {
            let reopenMemdb: u32 = unsafe { ::std::mem::transmute(reopenMemdb) };
            reopenMemdb as u64
        });
        __bindgen_bitfield_unit
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union sqlite3__bindgen_ty_1 {
    pub xLegacy: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut ::std::os::raw::c_void,
            arg2: *const ::std::os::raw::c_char,
        ),
    >,
    pub xV2: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: u32_,
            arg2: *mut ::std::os::raw::c_void,
            arg3: *mut ::std::os::raw::c_void,
            arg4: *mut ::std::os::raw::c_void,
        ) -> ::std::os::raw::c_int,
    >,
}
impl Default for sqlite3__bindgen_ty_1 {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union sqlite3__bindgen_ty_2 {
    pub isInterrupted: ::std::os::raw::c_int,
    pub notUsed1: f64,
}
impl Default for sqlite3__bindgen_ty_2 {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl Default for sqlite3 {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct FuncDef {
    pub nArg: i16_,
    pub funcFlags: u32_,
    pub pUserData: *mut ::std::os::raw::c_void,
    pub pNext: *mut FuncDef,
    pub xSFunc: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut sqlite3_context,
            arg2: ::std::os::raw::c_int,
            arg3: *mut *mut sqlite3_value,
        ),
    >,
    pub xFinalize: ::std::option::Option<unsafe extern "C" fn(arg1: *mut sqlite3_context)>,
    pub xValue: ::std::option::Option<unsafe extern "C" fn(arg1: *mut sqlite3_context)>,
    pub xInverse: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut sqlite3_context,
            arg2: ::std::os::raw::c_int,
            arg3: *mut *mut sqlite3_value,
        ),
    >,
    pub zName: *const ::std::os::raw::c_char,
    pub u: FuncDef__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union FuncDef__bindgen_ty_1 {
    pub pHash: *mut FuncDef,
    pub pDestructor: *mut FuncDestructor,
}
impl Default for FuncDef__bindgen_ty_1 {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl Default for FuncDef {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FuncDestructor {
    pub nRef: ::std::os::raw::c_int,
    pub xDestroy: ::std::option::Option<unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void)>,
    pub pUserData: *mut ::std::os::raw::c_void,
}
impl Default for FuncDestructor {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Savepoint {
    pub zName: *mut ::std::os::raw::c_char,
    pub nDeferredCons: i64_,
    pub nDeferredImmCons: i64_,
    pub pNext: *mut Savepoint,
}
impl Default for Savepoint {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Module {
    pub pModule: *const sqlite3_module,
    pub zName: *const ::std::os::raw::c_char,
    pub nRefModule: ::std::os::raw::c_int,
    pub pAux: *mut ::std::os::raw::c_void,
    pub xDestroy: ::std::option::Option<unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void)>,
    pub pEpoTab: *mut Table,
}
impl Default for Module {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Column {
    pub zCnName: *mut ::std::os::raw::c_char,
    pub _bitfield_align_1: [u8; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 1usize]>,
    pub affinity: ::std::os::raw::c_char,
    pub szEst: u8_,
    pub hName: u8_,
    pub iDflt: u16_,
    pub colFlags: u16_,
}
impl Default for Column {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl Column {
    #[inline]
    pub fn notNull(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(0usize, 4u8) as u32) }
    }
    #[inline]
    pub fn set_notNull(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(0usize, 4u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn notNull_raw(this: *const Self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 1usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                0usize,
                4u8,
            ) as u32)
        }
    }
    #[inline]
    pub unsafe fn set_notNull_raw(this: *mut Self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 1usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                0usize,
                4u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn eCType(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(4usize, 4u8) as u32) }
    }
    #[inline]
    pub fn set_eCType(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(4usize, 4u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn eCType_raw(this: *const Self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 1usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                4usize,
                4u8,
            ) as u32)
        }
    }
    #[inline]
    pub unsafe fn set_eCType_raw(this: *mut Self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 1usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                4usize,
                4u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        notNull: ::std::os::raw::c_uint,
        eCType: ::std::os::raw::c_uint,
    ) -> __BindgenBitfieldUnit<[u8; 1usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 1usize]> = Default::default();
        __bindgen_bitfield_unit.set(0usize, 4u8, {
            let notNull: u32 = unsafe { ::std::mem::transmute(notNull) };
            notNull as u64
        });
        __bindgen_bitfield_unit.set(4usize, 4u8, {
            let eCType: u32 = unsafe { ::std::mem::transmute(eCType) };
            eCType as u64
        });
        __bindgen_bitfield_unit
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CollSeq {
    pub zName: *mut ::std::os::raw::c_char,
    pub enc: u8_,
    pub pUser: *mut ::std::os::raw::c_void,
    pub xCmp: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut ::std::os::raw::c_void,
            arg2: ::std::os::raw::c_int,
            arg3: *const ::std::os::raw::c_void,
            arg4: ::std::os::raw::c_int,
            arg5: *const ::std::os::raw::c_void,
        ) -> ::std::os::raw::c_int,
    >,
    pub xDel: ::std::option::Option<unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void)>,
}
impl Default for CollSeq {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VTable {
    pub db: *mut sqlite3,
    pub pMod: *mut Module,
    pub pVtab: *mut sqlite3_vtab,
    pub nRef: ::std::os::raw::c_int,
    pub bConstraint: u8_,
    pub bAllSchemas: u8_,
    pub eVtabRisk: u8_,
    pub iSavepoint: ::std::os::raw::c_int,
    pub pNext: *mut VTable,
}
impl Default for VTable {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Table {
    pub zName: *mut ::std::os::raw::c_char,
    pub aCol: *mut Column,
    pub pIndex: *mut Index,
    pub zColAff: *mut ::std::os::raw::c_char,
    pub pCheck: *mut ExprList,
    pub tnum: Pgno,
    pub nTabRef: u32_,
    pub tabFlags: u32_,
    pub iPKey: i16_,
    pub nCol: i16_,
    pub nNVCol: i16_,
    pub nRowLogEst: LogEst,
    pub szTabRow: LogEst,
    pub keyConf: u8_,
    pub eTabType: u8_,
    pub u: Table__bindgen_ty_1,
    pub pTrigger: *mut Trigger,
    pub pSchema: *mut Schema,
    pub aHx: [u8_; 16usize],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union Table__bindgen_ty_1 {
    pub tab: Table__bindgen_ty_1__bindgen_ty_1,
    pub view: Table__bindgen_ty_1__bindgen_ty_2,
    pub vtab: Table__bindgen_ty_1__bindgen_ty_3,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Table__bindgen_ty_1__bindgen_ty_1 {
    pub addColOffset: ::std::os::raw::c_int,
    pub pFKey: *mut FKey,
    pub pDfltList: *mut ExprList,
}
impl Default for Table__bindgen_ty_1__bindgen_ty_1 {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Table__bindgen_ty_1__bindgen_ty_2 {
    pub pSelect: *mut Select,
}
impl Default for Table__bindgen_ty_1__bindgen_ty_2 {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Table__bindgen_ty_1__bindgen_ty_3 {
    pub nArg: ::std::os::raw::c_int,
    pub azArg: *mut *mut ::std::os::raw::c_char,
    pub p: *mut VTable,
}
impl Default for Table__bindgen_ty_1__bindgen_ty_3 {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl Default for Table__bindgen_ty_1 {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl Default for Table {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug)]
pub struct FKey {
    pub pFrom: *mut Table,
    pub pNextFrom: *mut FKey,
    pub zTo: *mut ::std::os::raw::c_char,
    pub pNextTo: *mut FKey,
    pub pPrevTo: *mut FKey,
    pub nCol: ::std::os::raw::c_int,
    pub isDeferred: u8_,
    pub aAction: [u8_; 2usize],
    pub apTrigger: [*mut Trigger; 2usize],
    pub aCol: __IncompleteArrayField<FKey_sColMap>,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FKey_sColMap {
    pub iFrom: ::std::os::raw::c_int,
    pub zCol: *mut ::std::os::raw::c_char,
}
impl Default for FKey_sColMap {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl Default for FKey {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug)]
pub struct KeyInfo {
    pub nRef: u32_,
    pub enc: u8_,
    pub nKeyField: u16_,
    pub nAllField: u16_,
    pub db: *mut sqlite3,
    pub aSortFlags: *mut u8_,
    pub aColl: __IncompleteArrayField<*mut CollSeq>,
}
impl Default for KeyInfo {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct UnpackedRecord {
    pub pKeyInfo: *mut KeyInfo,
    pub aMem: *mut Mem,
    pub u: UnpackedRecord__bindgen_ty_1,
    pub n: ::std::os::raw::c_int,
    pub nField: u16_,
    pub default_rc: i8_,
    pub errCode: u8_,
    pub r1: i8_,
    pub r2: i8_,
    pub eqSeen: u8_,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union UnpackedRecord__bindgen_ty_1 {
    pub z: *mut ::std::os::raw::c_char,
    pub i: i64_,
}
impl Default for UnpackedRecord__bindgen_ty_1 {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl Default for UnpackedRecord {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Index {
    pub zName: *mut ::std::os::raw::c_char,
    pub aiColumn: *mut i16_,
    pub aiRowLogEst: *mut LogEst,
    pub pTable: *mut Table,
    pub zColAff: *mut ::std::os::raw::c_char,
    pub pNext: *mut Index,
    pub pSchema: *mut Schema,
    pub aSortOrder: *mut u8_,
    pub azColl: *mut *const ::std::os::raw::c_char,
    pub pPartIdxWhere: *mut Expr,
    pub aColExpr: *mut ExprList,
    pub tnum: Pgno,
    pub szIdxRow: LogEst,
    pub nKeyCol: u16_,
    pub nColumn: u16_,
    pub onError: u8_,
    pub _bitfield_align_1: [u8; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 2usize]>,
    pub colNotIdxed: Bitmask,
}
impl Default for Index {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl Index {
    #[inline]
    pub fn idxType(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(0usize, 2u8) as u32) }
    }
    #[inline]
    pub fn set_idxType(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(0usize, 2u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn idxType_raw(this: *const Self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 2usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                0usize,
                2u8,
            ) as u32)
        }
    }
    #[inline]
    pub unsafe fn set_idxType_raw(this: *mut Self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 2usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                0usize,
                2u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn bUnordered(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(2usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_bUnordered(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(2usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn bUnordered_raw(this: *const Self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 2usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                2usize,
                1u8,
            ) as u32)
        }
    }
    #[inline]
    pub unsafe fn set_bUnordered_raw(this: *mut Self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 2usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                2usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn uniqNotNull(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(3usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_uniqNotNull(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(3usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn uniqNotNull_raw(this: *const Self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 2usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                3usize,
                1u8,
            ) as u32)
        }
    }
    #[inline]
    pub unsafe fn set_uniqNotNull_raw(this: *mut Self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 2usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                3usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn isResized(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(4usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_isResized(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(4usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn isResized_raw(this: *const Self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 2usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                4usize,
                1u8,
            ) as u32)
        }
    }
    #[inline]
    pub unsafe fn set_isResized_raw(this: *mut Self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 2usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                4usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn isCovering(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(5usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_isCovering(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(5usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn isCovering_raw(this: *const Self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 2usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                5usize,
                1u8,
            ) as u32)
        }
    }
    #[inline]
    pub unsafe fn set_isCovering_raw(this: *mut Self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 2usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                5usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn noSkipScan(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(6usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_noSkipScan(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(6usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn noSkipScan_raw(this: *const Self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 2usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                6usize,
                1u8,
            ) as u32)
        }
    }
    #[inline]
    pub unsafe fn set_noSkipScan_raw(this: *mut Self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 2usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                6usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn hasStat1(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(7usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_hasStat1(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(7usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn hasStat1_raw(this: *const Self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 2usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                7usize,
                1u8,
            ) as u32)
        }
    }
    #[inline]
    pub unsafe fn set_hasStat1_raw(this: *mut Self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 2usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                7usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn bNoQuery(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(8usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_bNoQuery(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(8usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn bNoQuery_raw(this: *const Self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 2usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                8usize,
                1u8,
            ) as u32)
        }
    }
    #[inline]
    pub unsafe fn set_bNoQuery_raw(this: *mut Self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 2usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                8usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn bAscKeyBug(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(9usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_bAscKeyBug(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(9usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn bAscKeyBug_raw(this: *const Self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 2usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                9usize,
                1u8,
            ) as u32)
        }
    }
    #[inline]
    pub unsafe fn set_bAscKeyBug_raw(this: *mut Self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 2usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                9usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn bHasVCol(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(10usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_bHasVCol(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(10usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn bHasVCol_raw(this: *const Self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 2usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                10usize,
                1u8,
            ) as u32)
        }
    }
    #[inline]
    pub unsafe fn set_bHasVCol_raw(this: *mut Self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 2usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                10usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn bHasExpr(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(11usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_bHasExpr(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(11usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn bHasExpr_raw(this: *const Self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 2usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                11usize,
                1u8,
            ) as u32)
        }
    }
    #[inline]
    pub unsafe fn set_bHasExpr_raw(this: *mut Self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 2usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                11usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        idxType: ::std::os::raw::c_uint,
        bUnordered: ::std::os::raw::c_uint,
        uniqNotNull: ::std::os::raw::c_uint,
        isResized: ::std::os::raw::c_uint,
        isCovering: ::std::os::raw::c_uint,
        noSkipScan: ::std::os::raw::c_uint,
        hasStat1: ::std::os::raw::c_uint,
        bNoQuery: ::std::os::raw::c_uint,
        bAscKeyBug: ::std::os::raw::c_uint,
        bHasVCol: ::std::os::raw::c_uint,
        bHasExpr: ::std::os::raw::c_uint,
    ) -> __BindgenBitfieldUnit<[u8; 2usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 2usize]> = Default::default();
        __bindgen_bitfield_unit.set(0usize, 2u8, {
            let idxType: u32 = unsafe { ::std::mem::transmute(idxType) };
            idxType as u64
        });
        __bindgen_bitfield_unit.set(2usize, 1u8, {
            let bUnordered: u32 = unsafe { ::std::mem::transmute(bUnordered) };
            bUnordered as u64
        });
        __bindgen_bitfield_unit.set(3usize, 1u8, {
            let uniqNotNull: u32 = unsafe { ::std::mem::transmute(uniqNotNull) };
            uniqNotNull as u64
        });
        __bindgen_bitfield_unit.set(4usize, 1u8, {
            let isResized: u32 = unsafe { ::std::mem::transmute(isResized) };
            isResized as u64
        });
        __bindgen_bitfield_unit.set(5usize, 1u8, {
            let isCovering: u32 = unsafe { ::std::mem::transmute(isCovering) };
            isCovering as u64
        });
        __bindgen_bitfield_unit.set(6usize, 1u8, {
            let noSkipScan: u32 = unsafe { ::std::mem::transmute(noSkipScan) };
            noSkipScan as u64
        });
        __bindgen_bitfield_unit.set(7usize, 1u8, {
            let hasStat1: u32 = unsafe { ::std::mem::transmute(hasStat1) };
            hasStat1 as u64
        });
        __bindgen_bitfield_unit.set(8usize, 1u8, {
            let bNoQuery: u32 = unsafe { ::std::mem::transmute(bNoQuery) };
            bNoQuery as u64
        });
        __bindgen_bitfield_unit.set(9usize, 1u8, {
            let bAscKeyBug: u32 = unsafe { ::std::mem::transmute(bAscKeyBug) };
            bAscKeyBug as u64
        });
        __bindgen_bitfield_unit.set(10usize, 1u8, {
            let bHasVCol: u32 = unsafe { ::std::mem::transmute(bHasVCol) };
            bHasVCol as u64
        });
        __bindgen_bitfield_unit.set(11usize, 1u8, {
            let bHasExpr: u32 = unsafe { ::std::mem::transmute(bHasExpr) };
            bHasExpr as u64
        });
        __bindgen_bitfield_unit
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Token {
    pub z: *const ::std::os::raw::c_char,
    pub n: ::std::os::raw::c_uint,
}
impl Default for Token {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AggInfo {
    pub directMode: u8_,
    pub useSortingIdx: u8_,
    pub nSortingColumn: u32_,
    pub sortingIdx: ::std::os::raw::c_int,
    pub sortingIdxPTab: ::std::os::raw::c_int,
    pub iFirstReg: ::std::os::raw::c_int,
    pub pGroupBy: *mut ExprList,
    pub aCol: *mut AggInfo_AggInfo_col,
    pub nColumn: ::std::os::raw::c_int,
    pub nAccumulator: ::std::os::raw::c_int,
    pub aFunc: *mut AggInfo_AggInfo_func,
    pub nFunc: ::std::os::raw::c_int,
    pub selId: u32_,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AggInfo_AggInfo_col {
    pub pTab: *mut Table,
    pub pCExpr: *mut Expr,
    pub iTable: ::std::os::raw::c_int,
    pub iColumn: ::std::os::raw::c_int,
    pub iSorterColumn: ::std::os::raw::c_int,
}
impl Default for AggInfo_AggInfo_col {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AggInfo_AggInfo_func {
    pub pFExpr: *mut Expr,
    pub pFunc: *mut FuncDef,
    pub iDistinct: ::std::os::raw::c_int,
    pub iDistAddr: ::std::os::raw::c_int,
    pub iOBTab: ::std::os::raw::c_int,
    pub bOBPayload: u8_,
    pub bOBUnique: u8_,
    pub bUseSubtype: u8_,
}
impl Default for AggInfo_AggInfo_func {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl Default for AggInfo {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type ynVar = i16_;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Expr {
    pub op: u8_,
    pub affExpr: ::std::os::raw::c_char,
    pub op2: u8_,
    pub flags: u32_,
    pub u: Expr__bindgen_ty_1,
    pub pLeft: *mut Expr,
    pub pRight: *mut Expr,
    pub x: Expr__bindgen_ty_2,
    pub nHeight: ::std::os::raw::c_int,
    pub iTable: ::std::os::raw::c_int,
    pub iColumn: ynVar,
    pub iAgg: i16_,
    pub w: Expr__bindgen_ty_3,
    pub pAggInfo: *mut AggInfo,
    pub y: Expr__bindgen_ty_4,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union Expr__bindgen_ty_1 {
    pub zToken: *mut ::std::os::raw::c_char,
    pub iValue: ::std::os::raw::c_int,
}
impl Default for Expr__bindgen_ty_1 {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union Expr__bindgen_ty_2 {
    pub pList: *mut ExprList,
    pub pSelect: *mut Select,
}
impl Default for Expr__bindgen_ty_2 {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union Expr__bindgen_ty_3 {
    pub iJoin: ::std::os::raw::c_int,
    pub iOfst: ::std::os::raw::c_int,
}
impl Default for Expr__bindgen_ty_3 {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union Expr__bindgen_ty_4 {
    pub pTab: *mut Table,
    pub pWin: *mut Window,
    pub nReg: ::std::os::raw::c_int,
    pub sub: Expr__bindgen_ty_4__bindgen_ty_1,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct Expr__bindgen_ty_4__bindgen_ty_1 {
    pub iAddr: ::std::os::raw::c_int,
    pub regReturn: ::std::os::raw::c_int,
}
impl Default for Expr__bindgen_ty_4 {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl Default for Expr {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
pub struct ExprList {
    pub nExpr: ::std::os::raw::c_int,
    pub nAlloc: ::std::os::raw::c_int,
    pub a: __IncompleteArrayField<ExprList_ExprList_item>,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ExprList_ExprList_item {
    pub pExpr: *mut Expr,
    pub zEName: *mut ::std::os::raw::c_char,
    pub fg: ExprList_ExprList_item__bindgen_ty_1,
    pub u: ExprList_ExprList_item__bindgen_ty_2,
}
#[repr(C)]
#[repr(align(4))]
#[derive(Debug, Default, Copy, Clone)]
pub struct ExprList_ExprList_item__bindgen_ty_1 {
    pub sortFlags: u8_,
    pub _bitfield_align_1: [u8; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 2usize]>,
    pub __bindgen_padding_0: u8,
}
impl ExprList_ExprList_item__bindgen_ty_1 {
    #[inline]
    pub fn eEName(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(0usize, 2u8) as u32) }
    }
    #[inline]
    pub fn set_eEName(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(0usize, 2u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn eEName_raw(this: *const Self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 2usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                0usize,
                2u8,
            ) as u32)
        }
    }
    #[inline]
    pub unsafe fn set_eEName_raw(this: *mut Self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 2usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                0usize,
                2u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn done(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(2usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_done(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(2usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn done_raw(this: *const Self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 2usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                2usize,
                1u8,
            ) as u32)
        }
    }
    #[inline]
    pub unsafe fn set_done_raw(this: *mut Self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 2usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                2usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn reusable(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(3usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_reusable(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(3usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn reusable_raw(this: *const Self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 2usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                3usize,
                1u8,
            ) as u32)
        }
    }
    #[inline]
    pub unsafe fn set_reusable_raw(this: *mut Self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 2usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                3usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn bSorterRef(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(4usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_bSorterRef(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(4usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn bSorterRef_raw(this: *const Self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 2usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                4usize,
                1u8,
            ) as u32)
        }
    }
    #[inline]
    pub unsafe fn set_bSorterRef_raw(this: *mut Self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 2usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                4usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn bNulls(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(5usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_bNulls(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(5usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn bNulls_raw(this: *const Self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 2usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                5usize,
                1u8,
            ) as u32)
        }
    }
    #[inline]
    pub unsafe fn set_bNulls_raw(this: *mut Self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 2usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                5usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn bUsed(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(6usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_bUsed(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(6usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn bUsed_raw(this: *const Self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 2usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                6usize,
                1u8,
            ) as u32)
        }
    }
    #[inline]
    pub unsafe fn set_bUsed_raw(this: *mut Self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 2usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                6usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn bUsingTerm(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(7usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_bUsingTerm(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(7usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn bUsingTerm_raw(this: *const Self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 2usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                7usize,
                1u8,
            ) as u32)
        }
    }
    #[inline]
    pub unsafe fn set_bUsingTerm_raw(this: *mut Self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 2usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                7usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn bNoExpand(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(8usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_bNoExpand(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(8usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn bNoExpand_raw(this: *const Self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 2usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                8usize,
                1u8,
            ) as u32)
        }
    }
    #[inline]
    pub unsafe fn set_bNoExpand_raw(this: *mut Self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 2usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                8usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        eEName: ::std::os::raw::c_uint,
        done: ::std::os::raw::c_uint,
        reusable: ::std::os::raw::c_uint,
        bSorterRef: ::std::os::raw::c_uint,
        bNulls: ::std::os::raw::c_uint,
        bUsed: ::std::os::raw::c_uint,
        bUsingTerm: ::std::os::raw::c_uint,
        bNoExpand: ::std::os::raw::c_uint,
    ) -> __BindgenBitfieldUnit<[u8; 2usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 2usize]> = Default::default();
        __bindgen_bitfield_unit.set(0usize, 2u8, {
            let eEName: u32 = unsafe { ::std::mem::transmute(eEName) };
            eEName as u64
        });
        __bindgen_bitfield_unit.set(2usize, 1u8, {
            let done: u32 = unsafe { ::std::mem::transmute(done) };
            done as u64
        });
        __bindgen_bitfield_unit.set(3usize, 1u8, {
            let reusable: u32 = unsafe { ::std::mem::transmute(reusable) };
            reusable as u64
        });
        __bindgen_bitfield_unit.set(4usize, 1u8, {
            let bSorterRef: u32 = unsafe { ::std::mem::transmute(bSorterRef) };
            bSorterRef as u64
        });
        __bindgen_bitfield_unit.set(5usize, 1u8, {
            let bNulls: u32 = unsafe { ::std::mem::transmute(bNulls) };
            bNulls as u64
        });
        __bindgen_bitfield_unit.set(6usize, 1u8, {
            let bUsed: u32 = unsafe { ::std::mem::transmute(bUsed) };
            bUsed as u64
        });
        __bindgen_bitfield_unit.set(7usize, 1u8, {
            let bUsingTerm: u32 = unsafe { ::std::mem::transmute(bUsingTerm) };
            bUsingTerm as u64
        });
        __bindgen_bitfield_unit.set(8usize, 1u8, {
            let bNoExpand: u32 = unsafe { ::std::mem::transmute(bNoExpand) };
            bNoExpand as u64
        });
        __bindgen_bitfield_unit
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union ExprList_ExprList_item__bindgen_ty_2 {
    pub x: ExprList_ExprList_item__bindgen_ty_2__bindgen_ty_1,
    pub iConstExprReg: ::std::os::raw::c_int,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct ExprList_ExprList_item__bindgen_ty_2__bindgen_ty_1 {
    pub iOrderByCol: u16_,
    pub iAlias: u16_,
}
impl Default for ExprList_ExprList_item__bindgen_ty_2 {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl Default for ExprList_ExprList_item {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl Default for ExprList {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug)]
pub struct IdList {
    pub nId: ::std::os::raw::c_int,
    pub a: __IncompleteArrayField<IdList_IdList_item>,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IdList_IdList_item {
    pub zName: *mut ::std::os::raw::c_char,
}
impl Default for IdList_IdList_item {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl Default for IdList {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Subquery {
    pub pSelect: *mut Select,
    pub addrFillSub: ::std::os::raw::c_int,
    pub regReturn: ::std::os::raw::c_int,
    pub regResult: ::std::os::raw::c_int,
}
impl Default for Subquery {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SrcItem {
    pub zName: *mut ::std::os::raw::c_char,
    pub zAlias: *mut ::std::os::raw::c_char,
    pub pSTab: *mut Table,
    pub fg: SrcItem__bindgen_ty_1,
    pub iCursor: ::std::os::raw::c_int,
    pub colUsed: Bitmask,
    pub u1: SrcItem__bindgen_ty_2,
    pub u2: SrcItem__bindgen_ty_3,
    pub u3: SrcItem__bindgen_ty_4,
    pub u4: SrcItem__bindgen_ty_5,
}
#[repr(C)]
#[repr(align(4))]
#[derive(Debug, Default, Copy, Clone)]
pub struct SrcItem__bindgen_ty_1 {
    pub jointype: u8_,
    pub _bitfield_align_1: [u8; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 3usize]>,
}
impl SrcItem__bindgen_ty_1 {
    #[inline]
    pub fn notIndexed(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(0usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_notIndexed(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(0usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn notIndexed_raw(this: *const Self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 3usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                0usize,
                1u8,
            ) as u32)
        }
    }
    #[inline]
    pub unsafe fn set_notIndexed_raw(this: *mut Self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 3usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                0usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn isIndexedBy(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(1usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_isIndexedBy(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(1usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn isIndexedBy_raw(this: *const Self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 3usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                1usize,
                1u8,
            ) as u32)
        }
    }
    #[inline]
    pub unsafe fn set_isIndexedBy_raw(this: *mut Self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 3usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                1usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn isSubquery(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(2usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_isSubquery(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(2usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn isSubquery_raw(this: *const Self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 3usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                2usize,
                1u8,
            ) as u32)
        }
    }
    #[inline]
    pub unsafe fn set_isSubquery_raw(this: *mut Self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 3usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                2usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn isTabFunc(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(3usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_isTabFunc(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(3usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn isTabFunc_raw(this: *const Self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 3usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                3usize,
                1u8,
            ) as u32)
        }
    }
    #[inline]
    pub unsafe fn set_isTabFunc_raw(this: *mut Self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 3usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                3usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn isCorrelated(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(4usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_isCorrelated(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(4usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn isCorrelated_raw(this: *const Self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 3usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                4usize,
                1u8,
            ) as u32)
        }
    }
    #[inline]
    pub unsafe fn set_isCorrelated_raw(this: *mut Self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 3usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                4usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn isMaterialized(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(5usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_isMaterialized(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(5usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn isMaterialized_raw(this: *const Self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 3usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                5usize,
                1u8,
            ) as u32)
        }
    }
    #[inline]
    pub unsafe fn set_isMaterialized_raw(this: *mut Self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 3usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                5usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn viaCoroutine(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(6usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_viaCoroutine(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(6usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn viaCoroutine_raw(this: *const Self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 3usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                6usize,
                1u8,
            ) as u32)
        }
    }
    #[inline]
    pub unsafe fn set_viaCoroutine_raw(this: *mut Self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 3usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                6usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn isRecursive(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(7usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_isRecursive(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(7usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn isRecursive_raw(this: *const Self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 3usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                7usize,
                1u8,
            ) as u32)
        }
    }
    #[inline]
    pub unsafe fn set_isRecursive_raw(this: *mut Self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 3usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                7usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn fromDDL(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(8usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_fromDDL(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(8usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn fromDDL_raw(this: *const Self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 3usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                8usize,
                1u8,
            ) as u32)
        }
    }
    #[inline]
    pub unsafe fn set_fromDDL_raw(this: *mut Self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 3usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                8usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn isCte(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(9usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_isCte(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(9usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn isCte_raw(this: *const Self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 3usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                9usize,
                1u8,
            ) as u32)
        }
    }
    #[inline]
    pub unsafe fn set_isCte_raw(this: *mut Self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 3usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                9usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn notCte(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(10usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_notCte(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(10usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn notCte_raw(this: *const Self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 3usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                10usize,
                1u8,
            ) as u32)
        }
    }
    #[inline]
    pub unsafe fn set_notCte_raw(this: *mut Self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 3usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                10usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn isUsing(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(11usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_isUsing(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(11usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn isUsing_raw(this: *const Self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 3usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                11usize,
                1u8,
            ) as u32)
        }
    }
    #[inline]
    pub unsafe fn set_isUsing_raw(this: *mut Self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 3usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                11usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn isOn(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(12usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_isOn(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(12usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn isOn_raw(this: *const Self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 3usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                12usize,
                1u8,
            ) as u32)
        }
    }
    #[inline]
    pub unsafe fn set_isOn_raw(this: *mut Self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 3usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                12usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn isSynthUsing(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(13usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_isSynthUsing(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(13usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn isSynthUsing_raw(this: *const Self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 3usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                13usize,
                1u8,
            ) as u32)
        }
    }
    #[inline]
    pub unsafe fn set_isSynthUsing_raw(this: *mut Self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 3usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                13usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn isNestedFrom(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(14usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_isNestedFrom(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(14usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn isNestedFrom_raw(this: *const Self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 3usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                14usize,
                1u8,
            ) as u32)
        }
    }
    #[inline]
    pub unsafe fn set_isNestedFrom_raw(this: *mut Self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 3usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                14usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn rowidUsed(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(15usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_rowidUsed(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(15usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn rowidUsed_raw(this: *const Self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 3usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                15usize,
                1u8,
            ) as u32)
        }
    }
    #[inline]
    pub unsafe fn set_rowidUsed_raw(this: *mut Self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 3usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                15usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn fixedSchema(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(16usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_fixedSchema(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(16usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn fixedSchema_raw(this: *const Self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 3usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                16usize,
                1u8,
            ) as u32)
        }
    }
    #[inline]
    pub unsafe fn set_fixedSchema_raw(this: *mut Self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 3usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                16usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn hadSchema(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(17usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_hadSchema(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(17usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn hadSchema_raw(this: *const Self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 3usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                17usize,
                1u8,
            ) as u32)
        }
    }
    #[inline]
    pub unsafe fn set_hadSchema_raw(this: *mut Self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 3usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                17usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn fromExists(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(18usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_fromExists(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(18usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn fromExists_raw(this: *const Self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 3usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                18usize,
                1u8,
            ) as u32)
        }
    }
    #[inline]
    pub unsafe fn set_fromExists_raw(this: *mut Self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 3usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                18usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        notIndexed: ::std::os::raw::c_uint,
        isIndexedBy: ::std::os::raw::c_uint,
        isSubquery: ::std::os::raw::c_uint,
        isTabFunc: ::std::os::raw::c_uint,
        isCorrelated: ::std::os::raw::c_uint,
        isMaterialized: ::std::os::raw::c_uint,
        viaCoroutine: ::std::os::raw::c_uint,
        isRecursive: ::std::os::raw::c_uint,
        fromDDL: ::std::os::raw::c_uint,
        isCte: ::std::os::raw::c_uint,
        notCte: ::std::os::raw::c_uint,
        isUsing: ::std::os::raw::c_uint,
        isOn: ::std::os::raw::c_uint,
        isSynthUsing: ::std::os::raw::c_uint,
        isNestedFrom: ::std::os::raw::c_uint,
        rowidUsed: ::std::os::raw::c_uint,
        fixedSchema: ::std::os::raw::c_uint,
        hadSchema: ::std::os::raw::c_uint,
        fromExists: ::std::os::raw::c_uint,
    ) -> __BindgenBitfieldUnit<[u8; 3usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 3usize]> = Default::default();
        __bindgen_bitfield_unit.set(0usize, 1u8, {
            let notIndexed: u32 = unsafe { ::std::mem::transmute(notIndexed) };
            notIndexed as u64
        });
        __bindgen_bitfield_unit.set(1usize, 1u8, {
            let isIndexedBy: u32 = unsafe { ::std::mem::transmute(isIndexedBy) };
            isIndexedBy as u64
        });
        __bindgen_bitfield_unit.set(2usize, 1u8, {
            let isSubquery: u32 = unsafe { ::std::mem::transmute(isSubquery) };
            isSubquery as u64
        });
        __bindgen_bitfield_unit.set(3usize, 1u8, {
            let isTabFunc: u32 = unsafe { ::std::mem::transmute(isTabFunc) };
            isTabFunc as u64
        });
        __bindgen_bitfield_unit.set(4usize, 1u8, {
            let isCorrelated: u32 = unsafe { ::std::mem::transmute(isCorrelated) };
            isCorrelated as u64
        });
        __bindgen_bitfield_unit.set(5usize, 1u8, {
            let isMaterialized: u32 = unsafe { ::std::mem::transmute(isMaterialized) };
            isMaterialized as u64
        });
        __bindgen_bitfield_unit.set(6usize, 1u8, {
            let viaCoroutine: u32 = unsafe { ::std::mem::transmute(viaCoroutine) };
            viaCoroutine as u64
        });
        __bindgen_bitfield_unit.set(7usize, 1u8, {
            let isRecursive: u32 = unsafe { ::std::mem::transmute(isRecursive) };
            isRecursive as u64
        });
        __bindgen_bitfield_unit.set(8usize, 1u8, {
            let fromDDL: u32 = unsafe { ::std::mem::transmute(fromDDL) };
            fromDDL as u64
        });
        __bindgen_bitfield_unit.set(9usize, 1u8, {
            let isCte: u32 = unsafe { ::std::mem::transmute(isCte) };
            isCte as u64
        });
        __bindgen_bitfield_unit.set(10usize, 1u8, {
            let notCte: u32 = unsafe { ::std::mem::transmute(notCte) };
            notCte as u64
        });
        __bindgen_bitfield_unit.set(11usize, 1u8, {
            let isUsing: u32 = unsafe { ::std::mem::transmute(isUsing) };
            isUsing as u64
        });
        __bindgen_bitfield_unit.set(12usize, 1u8, {
            let isOn: u32 = unsafe { ::std::mem::transmute(isOn) };
            isOn as u64
        });
        __bindgen_bitfield_unit.set(13usize, 1u8, {
            let isSynthUsing: u32 = unsafe { ::std::mem::transmute(isSynthUsing) };
            isSynthUsing as u64
        });
        __bindgen_bitfield_unit.set(14usize, 1u8, {
            let isNestedFrom: u32 = unsafe { ::std::mem::transmute(isNestedFrom) };
            isNestedFrom as u64
        });
        __bindgen_bitfield_unit.set(15usize, 1u8, {
            let rowidUsed: u32 = unsafe { ::std::mem::transmute(rowidUsed) };
            rowidUsed as u64
        });
        __bindgen_bitfield_unit.set(16usize, 1u8, {
            let fixedSchema: u32 = unsafe { ::std::mem::transmute(fixedSchema) };
            fixedSchema as u64
        });
        __bindgen_bitfield_unit.set(17usize, 1u8, {
            let hadSchema: u32 = unsafe { ::std::mem::transmute(hadSchema) };
            hadSchema as u64
        });
        __bindgen_bitfield_unit.set(18usize, 1u8, {
            let fromExists: u32 = unsafe { ::std::mem::transmute(fromExists) };
            fromExists as u64
        });
        __bindgen_bitfield_unit
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union SrcItem__bindgen_ty_2 {
    pub zIndexedBy: *mut ::std::os::raw::c_char,
    pub pFuncArg: *mut ExprList,
    pub nRow: u32_,
}
impl Default for SrcItem__bindgen_ty_2 {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union SrcItem__bindgen_ty_3 {
    pub pIBIndex: *mut Index,
    pub pCteUse: *mut CteUse,
}
impl Default for SrcItem__bindgen_ty_3 {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union SrcItem__bindgen_ty_4 {
    pub pOn: *mut Expr,
    pub pUsing: *mut IdList,
}
impl Default for SrcItem__bindgen_ty_4 {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union SrcItem__bindgen_ty_5 {
    pub pSchema: *mut Schema,
    pub zDatabase: *mut ::std::os::raw::c_char,
    pub pSubq: *mut Subquery,
}
impl Default for SrcItem__bindgen_ty_5 {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl Default for SrcItem {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
pub struct SrcList {
    pub nSrc: ::std::os::raw::c_int,
    pub nAlloc: u32_,
    pub a: __IncompleteArrayField<SrcItem>,
}
impl Default for SrcList {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Upsert {
    pub pUpsertTarget: *mut ExprList,
    pub pUpsertTargetWhere: *mut Expr,
    pub pUpsertSet: *mut ExprList,
    pub pUpsertWhere: *mut Expr,
    pub pNextUpsert: *mut Upsert,
    pub isDoUpdate: u8_,
    pub isDup: u8_,
    pub pToFree: *mut ::std::os::raw::c_void,
    pub pUpsertIdx: *mut Index,
    pub pUpsertSrc: *mut SrcList,
    pub regData: ::std::os::raw::c_int,
    pub iDataCur: ::std::os::raw::c_int,
    pub iIdxCur: ::std::os::raw::c_int,
}
impl Default for Upsert {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Select {
    pub op: u8_,
    pub nSelectRow: LogEst,
    pub selFlags: u32_,
    pub iLimit: ::std::os::raw::c_int,
    pub iOffset: ::std::os::raw::c_int,
    pub selId: u32_,
    pub addrOpenEphm: [::std::os::raw::c_int; 2usize],
    pub pEList: *mut ExprList,
    pub pSrc: *mut SrcList,
    pub pWhere: *mut Expr,
    pub pGroupBy: *mut ExprList,
    pub pHaving: *mut Expr,
    pub pOrderBy: *mut ExprList,
    pub pPrior: *mut Select,
    pub pNext: *mut Select,
    pub pLimit: *mut Expr,
    pub pWith: *mut With,
    pub pWin: *mut Window,
    pub pWinDefn: *mut Window,
}
impl Default for Select {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AutoincInfo {
    pub pNext: *mut AutoincInfo,
    pub pTab: *mut Table,
    pub iDb: ::std::os::raw::c_int,
    pub regCtr: ::std::os::raw::c_int,
}
impl Default for AutoincInfo {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TriggerPrg {
    pub pTrigger: *mut Trigger,
    pub pNext: *mut TriggerPrg,
    pub pProgram: *mut SubProgram,
    pub orconf: ::std::os::raw::c_int,
    pub aColmask: [u32_; 2usize],
}
impl Default for TriggerPrg {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type yDbMask = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IndexedExpr {
    pub pExpr: *mut Expr,
    pub iDataCur: ::std::os::raw::c_int,
    pub iIdxCur: ::std::os::raw::c_int,
    pub iIdxCol: ::std::os::raw::c_int,
    pub bMaybeNullRow: u8_,
    pub aff: u8_,
    pub pIENext: *mut IndexedExpr,
}
impl Default for IndexedExpr {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ParseCleanup {
    pub pNext: *mut ParseCleanup,
    pub pPtr: *mut ::std::os::raw::c_void,
    pub xCleanup: ::std::option::Option<
        unsafe extern "C" fn(arg1: *mut sqlite3, arg2: *mut ::std::os::raw::c_void),
    >,
}
impl Default for ParseCleanup {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Parse {
    pub db: *mut sqlite3,
    pub zErrMsg: *mut ::std::os::raw::c_char,
    pub pVdbe: *mut Vdbe,
    pub rc: ::std::os::raw::c_int,
    pub nQueryLoop: LogEst,
    pub nested: u8_,
    pub nTempReg: u8_,
    pub isMultiWrite: u8_,
    pub mayAbort: u8_,
    pub hasCompound: u8_,
    pub disableLookaside: u8_,
    pub prepFlags: u8_,
    pub withinRJSubrtn: u8_,
    pub bHasExists: u8_,
    pub mSubrtnSig: u8_,
    pub eTriggerOp: u8_,
    pub bReturning: u8_,
    pub eOrconf: u8_,
    pub disableTriggers: u8_,
    pub _bitfield_align_1: [u8; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 1usize]>,
    pub nRangeReg: ::std::os::raw::c_int,
    pub iRangeReg: ::std::os::raw::c_int,
    pub nErr: ::std::os::raw::c_int,
    pub nTab: ::std::os::raw::c_int,
    pub nMem: ::std::os::raw::c_int,
    pub szOpAlloc: ::std::os::raw::c_int,
    pub iSelfTab: ::std::os::raw::c_int,
    pub nLabel: ::std::os::raw::c_int,
    pub nLabelAlloc: ::std::os::raw::c_int,
    pub aLabel: *mut ::std::os::raw::c_int,
    pub pConstExpr: *mut ExprList,
    pub pIdxEpr: *mut IndexedExpr,
    pub pIdxPartExpr: *mut IndexedExpr,
    pub writeMask: yDbMask,
    pub cookieMask: yDbMask,
    pub nMaxArg: ::std::os::raw::c_int,
    pub nSelect: ::std::os::raw::c_int,
    pub nProgressSteps: u32_,
    pub nTableLock: ::std::os::raw::c_int,
    pub aTableLock: *mut TableLock,
    pub pAinc: *mut AutoincInfo,
    pub pToplevel: *mut Parse,
    pub pTriggerTab: *mut Table,
    pub pTriggerPrg: *mut TriggerPrg,
    pub pCleanup: *mut ParseCleanup,
    #[doc = " Fields above must be initialized to zero.  The fields that follow,\n down to the beginning of the recursive section, do not need to be\n initialized as they will be set before being used.  The boundary is\n determined by offsetof(Parse,aTempReg)."]
    pub aTempReg: [::std::os::raw::c_int; 8usize],
    pub pOuterParse: *mut Parse,
    pub sNameToken: Token,
    pub oldmask: u32_,
    pub newmask: u32_,
    pub u1: Parse__bindgen_ty_1,
    #[doc = " Above is constant between recursions.  Below is reset before and after\n each recursion.  The boundary between these two regions is determined\n using offsetof(Parse,sLastToken) so the sLastToken field must be the\n first field in the recursive region."]
    pub sLastToken: Token,
    pub nVar: ynVar,
    pub iPkSortOrder: u8_,
    pub explain: u8_,
    pub eParseMode: u8_,
    pub nVtabLock: ::std::os::raw::c_int,
    pub nHeight: ::std::os::raw::c_int,
    pub addrExplain: ::std::os::raw::c_int,
    pub pVList: *mut VList,
    pub pReprepare: *mut Vdbe,
    pub zTail: *const ::std::os::raw::c_char,
    pub pNewTable: *mut Table,
    pub pNewIndex: *mut Index,
    pub pNewTrigger: *mut Trigger,
    pub zAuthContext: *const ::std::os::raw::c_char,
    pub sArg: Token,
    pub apVtabLock: *mut *mut Table,
    pub pWith: *mut With,
    pub pRename: *mut RenameToken,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union Parse__bindgen_ty_1 {
    pub cr: Parse__bindgen_ty_1__bindgen_ty_1,
    pub d: Parse__bindgen_ty_1__bindgen_ty_2,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Parse__bindgen_ty_1__bindgen_ty_1 {
    pub addrCrTab: ::std::os::raw::c_int,
    pub regRowid: ::std::os::raw::c_int,
    pub regRoot: ::std::os::raw::c_int,
    pub constraintName: Token,
}
impl Default for Parse__bindgen_ty_1__bindgen_ty_1 {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Parse__bindgen_ty_1__bindgen_ty_2 {
    pub pReturning: *mut Returning,
}
impl Default for Parse__bindgen_ty_1__bindgen_ty_2 {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl Default for Parse__bindgen_ty_1 {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl Default for Parse {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl Parse {
    #[inline]
    pub fn colNamesSet(&self) -> bft {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(0usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_colNamesSet(&mut self, val: bft) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(0usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn colNamesSet_raw(this: *const Self) -> bft {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 1usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                0usize,
                1u8,
            ) as u32)
        }
    }
    #[inline]
    pub unsafe fn set_colNamesSet_raw(this: *mut Self, val: bft) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 1usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                0usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn bHasWith(&self) -> bft {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(1usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_bHasWith(&mut self, val: bft) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(1usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn bHasWith_raw(this: *const Self) -> bft {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 1usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                1usize,
                1u8,
            ) as u32)
        }
    }
    #[inline]
    pub unsafe fn set_bHasWith_raw(this: *mut Self, val: bft) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 1usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                1usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn okConstFactor(&self) -> bft {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(2usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_okConstFactor(&mut self, val: bft) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(2usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn okConstFactor_raw(this: *const Self) -> bft {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 1usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                2usize,
                1u8,
            ) as u32)
        }
    }
    #[inline]
    pub unsafe fn set_okConstFactor_raw(this: *mut Self, val: bft) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 1usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                2usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn checkSchema(&self) -> bft {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(3usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_checkSchema(&mut self, val: bft) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(3usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn checkSchema_raw(this: *const Self) -> bft {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 1usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                3usize,
                1u8,
            ) as u32)
        }
    }
    #[inline]
    pub unsafe fn set_checkSchema_raw(this: *mut Self, val: bft) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 1usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                3usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        colNamesSet: bft,
        bHasWith: bft,
        okConstFactor: bft,
        checkSchema: bft,
    ) -> __BindgenBitfieldUnit<[u8; 1usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 1usize]> = Default::default();
        __bindgen_bitfield_unit.set(0usize, 1u8, {
            let colNamesSet: u32 = unsafe { ::std::mem::transmute(colNamesSet) };
            colNamesSet as u64
        });
        __bindgen_bitfield_unit.set(1usize, 1u8, {
            let bHasWith: u32 = unsafe { ::std::mem::transmute(bHasWith) };
            bHasWith as u64
        });
        __bindgen_bitfield_unit.set(2usize, 1u8, {
            let okConstFactor: u32 = unsafe { ::std::mem::transmute(okConstFactor) };
            okConstFactor as u64
        });
        __bindgen_bitfield_unit.set(3usize, 1u8, {
            let checkSchema: u32 = unsafe { ::std::mem::transmute(checkSchema) };
            checkSchema as u64
        });
        __bindgen_bitfield_unit
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Trigger {
    pub zName: *mut ::std::os::raw::c_char,
    pub table: *mut ::std::os::raw::c_char,
    pub op: u8_,
    pub tr_tm: u8_,
    pub bReturning: u8_,
    pub pWhen: *mut Expr,
    pub pColumns: *mut IdList,
    pub pSchema: *mut Schema,
    pub pTabSchema: *mut Schema,
    pub step_list: *mut TriggerStep,
    pub pNext: *mut Trigger,
}
impl Default for Trigger {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TriggerStep {
    pub op: u8_,
    pub orconf: u8_,
    pub pTrig: *mut Trigger,
    pub pSelect: *mut Select,
    pub zTarget: *mut ::std::os::raw::c_char,
    pub pFrom: *mut SrcList,
    pub pWhere: *mut Expr,
    pub pExprList: *mut ExprList,
    pub pIdList: *mut IdList,
    pub pUpsert: *mut Upsert,
    pub zSpan: *mut ::std::os::raw::c_char,
    pub pNext: *mut TriggerStep,
    pub pLast: *mut TriggerStep,
}
impl Default for TriggerStep {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Returning {
    pub pParse: *mut Parse,
    pub pReturnEL: *mut ExprList,
    pub retTrig: Trigger,
    pub retTStep: TriggerStep,
    pub iRetCur: ::std::os::raw::c_int,
    pub nRetCol: ::std::os::raw::c_int,
    pub iRetReg: ::std::os::raw::c_int,
    pub zName: [::std::os::raw::c_char; 40usize],
}
impl Default for Returning {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Cte {
    pub zName: *mut ::std::os::raw::c_char,
    pub pCols: *mut ExprList,
    pub pSelect: *mut Select,
    pub zCteErr: *const ::std::os::raw::c_char,
    pub pUse: *mut CteUse,
    pub eM10d: u8_,
}
impl Default for Cte {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug)]
pub struct With {
    pub nCte: ::std::os::raw::c_int,
    pub bView: ::std::os::raw::c_int,
    pub pOuter: *mut With,
    pub a: __IncompleteArrayField<Cte>,
}
impl Default for With {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct CteUse {
    pub nUse: ::std::os::raw::c_int,
    pub addrM9e: ::std::os::raw::c_int,
    pub regRtn: ::std::os::raw::c_int,
    pub iCur: ::std::os::raw::c_int,
    pub nRowEst: LogEst,
    pub eM10d: u8_,
}
#[repr(C)]
#[derive(Debug)]
pub struct DbClientData {
    pub pNext: *mut DbClientData,
    pub pData: *mut ::std::os::raw::c_void,
    pub xDestructor: ::std::option::Option<unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void)>,
    pub zName: __IncompleteArrayField<::std::os::raw::c_char>,
}
impl Default for DbClientData {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Window {
    pub zName: *mut ::std::os::raw::c_char,
    pub zBase: *mut ::std::os::raw::c_char,
    pub pPartition: *mut ExprList,
    pub pOrderBy: *mut ExprList,
    pub eFrmType: u8_,
    pub eStart: u8_,
    pub eEnd: u8_,
    pub bImplicitFrame: u8_,
    pub eExclude: u8_,
    pub pStart: *mut Expr,
    pub pEnd: *mut Expr,
    pub ppThis: *mut *mut Window,
    pub pNextWin: *mut Window,
    pub pFilter: *mut Expr,
    pub pWFunc: *mut FuncDef,
    pub iEphCsr: ::std::os::raw::c_int,
    pub regAccum: ::std::os::raw::c_int,
    pub regResult: ::std::os::raw::c_int,
    pub csrApp: ::std::os::raw::c_int,
    pub regApp: ::std::os::raw::c_int,
    pub regPart: ::std::os::raw::c_int,
    pub pOwner: *mut Expr,
    pub nBufferCol: ::std::os::raw::c_int,
    pub iArgCol: ::std::os::raw::c_int,
    pub regOne: ::std::os::raw::c_int,
    pub regStartRowid: ::std::os::raw::c_int,
    pub regEndRowid: ::std::os::raw::c_int,
    pub bExprArgs: u8_,
}
impl Default for Window {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MemPage {
    pub isInit: u8_,
    pub intKey: u8_,
    pub intKeyLeaf: u8_,
    pub pgno: Pgno,
    pub leaf: u8_,
    pub hdrOffset: u8_,
    pub childPtrSize: u8_,
    pub max1bytePayload: u8_,
    pub nOverflow: u8_,
    pub maxLocal: u16_,
    pub minLocal: u16_,
    pub cellOffset: u16_,
    pub nFree: ::std::os::raw::c_int,
    pub nCell: u16_,
    pub maskPage: u16_,
    pub aiOvfl: [u16_; 4usize],
    pub apOvfl: [*mut u8_; 4usize],
    pub pBt: *mut BtShared,
    pub aData: *mut u8_,
    pub aDataEnd: *mut u8_,
    pub aCellIdx: *mut u8_,
    pub aDataOfst: *mut u8_,
    pub pDbPage: *mut DbPage,
    pub xCellSize:
        ::std::option::Option<unsafe extern "C" fn(arg1: *mut MemPage, arg2: *mut u8_) -> u16_>,
    pub xParseCell: ::std::option::Option<
        unsafe extern "C" fn(arg1: *mut MemPage, arg2: *mut u8_, arg3: *mut CellInfo),
    >,
}
impl Default for MemPage {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BtLock {
    pub pBtree: *mut Btree,
    pub iTable: Pgno,
    pub eLock: u8_,
    pub pNext: *mut BtLock,
}
impl Default for BtLock {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Btree {
    pub db: *mut sqlite3,
    pub pBt: *mut BtShared,
    pub inTrans: u8_,
    pub sharable: u8_,
    pub locked: u8_,
    pub hasIncrblobCur: u8_,
    pub wantToLock: ::std::os::raw::c_int,
    pub nBackup: ::std::os::raw::c_int,
    pub iBDataVersion: u32_,
    pub pNext: *mut Btree,
    pub pPrev: *mut Btree,
    pub lock: BtLock,
}
impl Default for Btree {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BtShared {
    pub pPager: *mut Pager,
    pub db: *mut sqlite3,
    pub pCursor: *mut BtCursor,
    pub pPage1: *mut MemPage,
    pub openFlags: u8_,
    pub autoVacuum: u8_,
    pub incrVacuum: u8_,
    pub bDoTruncate: u8_,
    pub inTransaction: u8_,
    pub max1bytePayload: u8_,
    pub nReserveWanted: u8_,
    pub btsFlags: u16_,
    pub maxLocal: u16_,
    pub minLocal: u16_,
    pub maxLeaf: u16_,
    pub minLeaf: u16_,
    pub pageSize: u32_,
    pub usableSize: u32_,
    pub nTransaction: ::std::os::raw::c_int,
    pub nPage: u32_,
    pub pSchema: *mut ::std::os::raw::c_void,
    pub xFreeSchema: ::std::option::Option<unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void)>,
    pub mutex: *mut sqlite3_mutex,
    pub pHasContent: *mut Bitvec,
    pub nRef: ::std::os::raw::c_int,
    pub pNext: *mut BtShared,
    pub pLock: *mut BtLock,
    pub pWriter: *mut Btree,
    pub pTmpSpace: *mut u8_,
    pub nPreformatSize: ::std::os::raw::c_int,
}
impl Default for BtShared {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CellInfo {
    pub nKey: i64_,
    pub pPayload: *mut u8_,
    pub nPayload: u32_,
    pub nLocal: u16_,
    pub nSize: u16_,
}
impl Default for CellInfo {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BtCursor {
    pub eState: u8_,
    pub curFlags: u8_,
    pub curPagerFlags: u8_,
    pub hints: u8_,
    pub skipNext: ::std::os::raw::c_int,
    pub pBtree: *mut Btree,
    pub aOverflow: *mut Pgno,
    pub pKey: *mut ::std::os::raw::c_void,
    pub pBt: *mut BtShared,
    pub pNext: *mut BtCursor,
    pub info: CellInfo,
    pub nKey: i64_,
    pub pgnoRoot: Pgno,
    pub iPage: i8_,
    pub curIntKey: u8_,
    pub ix: u16_,
    pub aiIdx: [u16_; 19usize],
    pub pKeyInfo: *mut KeyInfo,
    pub pPage: *mut MemPage,
    pub apPage: [*mut MemPage; 19usize],
}
impl Default for BtCursor {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type Op = VdbeOp;
pub type Bool = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VdbeSorter {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct VdbeCursor {
    pub eCurType: u8_,
    pub iDb: i8_,
    pub nullRow: u8_,
    pub deferredMoveto: u8_,
    pub isTable: u8_,
    pub _bitfield_align_1: [u8; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 1usize]>,
    pub seekHit: u16_,
    pub ub: VdbeCursor__bindgen_ty_1,
    pub seqCount: i64_,
    pub cacheStatus: u32_,
    pub seekResult: ::std::os::raw::c_int,
    pub pAltCursor: *mut VdbeCursor,
    pub uc: VdbeCursor__bindgen_ty_2,
    pub pKeyInfo: *mut KeyInfo,
    pub iHdrOffset: u32_,
    pub pgnoRoot: Pgno,
    pub nField: i16_,
    pub nHdrParsed: u16_,
    pub movetoTarget: i64_,
    pub aOffset: *mut u32_,
    pub aRow: *const u8_,
    pub payloadSize: u32_,
    pub szRow: u32_,
    pub pCache: *mut VdbeTxtBlbCache,
    pub aType: __IncompleteArrayField<u32_>,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union VdbeCursor__bindgen_ty_1 {
    pub pBtx: *mut Btree,
    pub aAltMap: *mut u32_,
}
impl Default for VdbeCursor__bindgen_ty_1 {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union VdbeCursor__bindgen_ty_2 {
    pub pCursor: *mut BtCursor,
    pub pVCur: *mut sqlite3_vtab_cursor,
    pub pSorter: *mut VdbeSorter,
}
impl Default for VdbeCursor__bindgen_ty_2 {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl Default for VdbeCursor {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl VdbeCursor {
    #[inline]
    pub fn isEphemeral(&self) -> Bool {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(0usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_isEphemeral(&mut self, val: Bool) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(0usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn isEphemeral_raw(this: *const Self) -> Bool {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 1usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                0usize,
                1u8,
            ) as u32)
        }
    }
    #[inline]
    pub unsafe fn set_isEphemeral_raw(this: *mut Self, val: Bool) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 1usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                0usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn useRandomRowid(&self) -> Bool {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(1usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_useRandomRowid(&mut self, val: Bool) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(1usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn useRandomRowid_raw(this: *const Self) -> Bool {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 1usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                1usize,
                1u8,
            ) as u32)
        }
    }
    #[inline]
    pub unsafe fn set_useRandomRowid_raw(this: *mut Self, val: Bool) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 1usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                1usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn isOrdered(&self) -> Bool {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(2usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_isOrdered(&mut self, val: Bool) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(2usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn isOrdered_raw(this: *const Self) -> Bool {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 1usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                2usize,
                1u8,
            ) as u32)
        }
    }
    #[inline]
    pub unsafe fn set_isOrdered_raw(this: *mut Self, val: Bool) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 1usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                2usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn noReuse(&self) -> Bool {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(3usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_noReuse(&mut self, val: Bool) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(3usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn noReuse_raw(this: *const Self) -> Bool {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 1usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                3usize,
                1u8,
            ) as u32)
        }
    }
    #[inline]
    pub unsafe fn set_noReuse_raw(this: *mut Self, val: Bool) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 1usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                3usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn colCache(&self) -> Bool {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(4usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_colCache(&mut self, val: Bool) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(4usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn colCache_raw(this: *const Self) -> Bool {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 1usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                4usize,
                1u8,
            ) as u32)
        }
    }
    #[inline]
    pub unsafe fn set_colCache_raw(this: *mut Self, val: Bool) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 1usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                4usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        isEphemeral: Bool,
        useRandomRowid: Bool,
        isOrdered: Bool,
        noReuse: Bool,
        colCache: Bool,
    ) -> __BindgenBitfieldUnit<[u8; 1usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 1usize]> = Default::default();
        __bindgen_bitfield_unit.set(0usize, 1u8, {
            let isEphemeral: u32 = unsafe { ::std::mem::transmute(isEphemeral) };
            isEphemeral as u64
        });
        __bindgen_bitfield_unit.set(1usize, 1u8, {
            let useRandomRowid: u32 = unsafe { ::std::mem::transmute(useRandomRowid) };
            useRandomRowid as u64
        });
        __bindgen_bitfield_unit.set(2usize, 1u8, {
            let isOrdered: u32 = unsafe { ::std::mem::transmute(isOrdered) };
            isOrdered as u64
        });
        __bindgen_bitfield_unit.set(3usize, 1u8, {
            let noReuse: u32 = unsafe { ::std::mem::transmute(noReuse) };
            noReuse as u64
        });
        __bindgen_bitfield_unit.set(4usize, 1u8, {
            let colCache: u32 = unsafe { ::std::mem::transmute(colCache) };
            colCache as u64
        });
        __bindgen_bitfield_unit
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VdbeTxtBlbCache {
    pub pCValue: *mut ::std::os::raw::c_char,
    pub iOffset: i64_,
    pub iCol: ::std::os::raw::c_int,
    pub cacheStatus: u32_,
    pub colCacheCtr: u32_,
}
impl Default for VdbeTxtBlbCache {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VdbeFrame {
    pub v: *mut Vdbe,
    pub pParent: *mut VdbeFrame,
    pub aOp: *mut Op,
    pub aMem: *mut Mem,
    pub apCsr: *mut *mut VdbeCursor,
    pub aOnce: *mut u8_,
    pub token: *mut ::std::os::raw::c_void,
    pub lastRowid: i64_,
    pub pAuxData: *mut AuxData,
    pub nCursor: ::std::os::raw::c_int,
    pub pc: ::std::os::raw::c_int,
    pub nOp: ::std::os::raw::c_int,
    pub nMem: ::std::os::raw::c_int,
    pub nChildMem: ::std::os::raw::c_int,
    pub nChildCsr: ::std::os::raw::c_int,
    pub nChange: i64_,
    pub nDbChange: i64_,
}
impl Default for VdbeFrame {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sqlite3_value {
    pub u: sqlite3_value_MemValue,
    pub z: *mut ::std::os::raw::c_char,
    pub n: ::std::os::raw::c_int,
    pub flags: u16_,
    pub enc: u8_,
    pub eSubtype: u8_,
    pub db: *mut sqlite3,
    pub szMalloc: ::std::os::raw::c_int,
    pub uTemp: u32_,
    pub zMalloc: *mut ::std::os::raw::c_char,
    pub xDel: ::std::option::Option<unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void)>,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union sqlite3_value_MemValue {
    pub r: f64,
    pub i: i64_,
    pub nZero: ::std::os::raw::c_int,
    pub zPType: *const ::std::os::raw::c_char,
    pub pDef: *mut FuncDef,
}
impl Default for sqlite3_value_MemValue {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl Default for sqlite3_value {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AuxData {
    pub iAuxOp: ::std::os::raw::c_int,
    pub iAuxArg: ::std::os::raw::c_int,
    pub pAux: *mut ::std::os::raw::c_void,
    pub xDeleteAux: ::std::option::Option<unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void)>,
    pub pNextAux: *mut AuxData,
}
impl Default for AuxData {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug)]
pub struct sqlite3_context {
    pub pOut: *mut Mem,
    pub pFunc: *mut FuncDef,
    pub pMem: *mut Mem,
    pub pVdbe: *mut Vdbe,
    pub iOp: ::std::os::raw::c_int,
    pub isError: ::std::os::raw::c_int,
    pub enc: u8_,
    pub skipFlag: u8_,
    pub argc: u16_,
    pub argv: __IncompleteArrayField<*mut sqlite3_value>,
}
impl Default for sqlite3_context {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ScanStatus {
    pub addrExplain: ::std::os::raw::c_int,
    pub aAddrRange: [::std::os::raw::c_int; 6usize],
    pub addrLoop: ::std::os::raw::c_int,
    pub addrVisit: ::std::os::raw::c_int,
    pub iSelectID: ::std::os::raw::c_int,
    pub nEst: LogEst,
    pub zName: *mut ::std::os::raw::c_char,
}
impl Default for ScanStatus {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DblquoteStr {
    pub pNextStr: *mut DblquoteStr,
    pub z: [::std::os::raw::c_char; 8usize],
}
impl Default for DblquoteStr {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Vdbe {
    pub db: *mut sqlite3,
    pub ppVPrev: *mut *mut Vdbe,
    pub pVNext: *mut Vdbe,
    pub pParse: *mut Parse,
    pub nVar: ynVar,
    pub nMem: ::std::os::raw::c_int,
    pub nCursor: ::std::os::raw::c_int,
    pub cacheCtr: u32_,
    pub pc: ::std::os::raw::c_int,
    pub rc: ::std::os::raw::c_int,
    pub nChange: i64_,
    pub iStatement: ::std::os::raw::c_int,
    pub iCurrentTime: i64_,
    pub nFkConstraint: i64_,
    pub nStmtDefCons: i64_,
    pub nStmtDefImmCons: i64_,
    pub aMem: *mut Mem,
    pub apArg: *mut *mut Mem,
    pub apCsr: *mut *mut VdbeCursor,
    pub aVar: *mut Mem,
    pub aOp: *mut Op,
    pub nOp: ::std::os::raw::c_int,
    pub nOpAlloc: ::std::os::raw::c_int,
    pub aColName: *mut Mem,
    pub pResultRow: *mut Mem,
    pub zErrMsg: *mut ::std::os::raw::c_char,
    pub pVList: *mut VList,
    pub startTime: i64_,
    pub nResColumn: u16_,
    pub nResAlloc: u16_,
    pub errorAction: u8_,
    pub minWriteFileFormat: u8_,
    pub prepFlags: u8_,
    pub eVdbeState: u8_,
    pub _bitfield_align_1: [u8; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 2usize]>,
    pub btreeMask: yDbMask,
    pub lockMask: yDbMask,
    pub aCounter: [u32_; 9usize],
    pub zSql: *mut ::std::os::raw::c_char,
    pub pFree: *mut ::std::os::raw::c_void,
    pub pFrame: *mut VdbeFrame,
    pub pDelFrame: *mut VdbeFrame,
    pub nFrame: ::std::os::raw::c_int,
    pub expmask: u32_,
    pub pProgram: *mut SubProgram,
    pub pAuxData: *mut AuxData,
}
impl Default for Vdbe {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl Vdbe {
    #[inline]
    pub fn expired(&self) -> bft {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(0usize, 2u8) as u32) }
    }
    #[inline]
    pub fn set_expired(&mut self, val: bft) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(0usize, 2u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn expired_raw(this: *const Self) -> bft {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 2usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                0usize,
                2u8,
            ) as u32)
        }
    }
    #[inline]
    pub unsafe fn set_expired_raw(this: *mut Self, val: bft) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 2usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                0usize,
                2u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn explain(&self) -> bft {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(2usize, 2u8) as u32) }
    }
    #[inline]
    pub fn set_explain(&mut self, val: bft) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(2usize, 2u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn explain_raw(this: *const Self) -> bft {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 2usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                2usize,
                2u8,
            ) as u32)
        }
    }
    #[inline]
    pub unsafe fn set_explain_raw(this: *mut Self, val: bft) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 2usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                2usize,
                2u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn changeCntOn(&self) -> bft {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(4usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_changeCntOn(&mut self, val: bft) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(4usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn changeCntOn_raw(this: *const Self) -> bft {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 2usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                4usize,
                1u8,
            ) as u32)
        }
    }
    #[inline]
    pub unsafe fn set_changeCntOn_raw(this: *mut Self, val: bft) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 2usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                4usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn usesStmtJournal(&self) -> bft {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(5usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_usesStmtJournal(&mut self, val: bft) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(5usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn usesStmtJournal_raw(this: *const Self) -> bft {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 2usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                5usize,
                1u8,
            ) as u32)
        }
    }
    #[inline]
    pub unsafe fn set_usesStmtJournal_raw(this: *mut Self, val: bft) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 2usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                5usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn readOnly(&self) -> bft {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(6usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_readOnly(&mut self, val: bft) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(6usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn readOnly_raw(this: *const Self) -> bft {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 2usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                6usize,
                1u8,
            ) as u32)
        }
    }
    #[inline]
    pub unsafe fn set_readOnly_raw(this: *mut Self, val: bft) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 2usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                6usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn bIsReader(&self) -> bft {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(7usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_bIsReader(&mut self, val: bft) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(7usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn bIsReader_raw(this: *const Self) -> bft {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 2usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                7usize,
                1u8,
            ) as u32)
        }
    }
    #[inline]
    pub unsafe fn set_bIsReader_raw(this: *mut Self, val: bft) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 2usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                7usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn haveEqpOps(&self) -> bft {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(8usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_haveEqpOps(&mut self, val: bft) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(8usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn haveEqpOps_raw(this: *const Self) -> bft {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 2usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                8usize,
                1u8,
            ) as u32)
        }
    }
    #[inline]
    pub unsafe fn set_haveEqpOps_raw(this: *mut Self, val: bft) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 2usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                8usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        expired: bft,
        explain: bft,
        changeCntOn: bft,
        usesStmtJournal: bft,
        readOnly: bft,
        bIsReader: bft,
        haveEqpOps: bft,
    ) -> __BindgenBitfieldUnit<[u8; 2usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 2usize]> = Default::default();
        __bindgen_bitfield_unit.set(0usize, 2u8, {
            let expired: u32 = unsafe { ::std::mem::transmute(expired) };
            expired as u64
        });
        __bindgen_bitfield_unit.set(2usize, 2u8, {
            let explain: u32 = unsafe { ::std::mem::transmute(explain) };
            explain as u64
        });
        __bindgen_bitfield_unit.set(4usize, 1u8, {
            let changeCntOn: u32 = unsafe { ::std::mem::transmute(changeCntOn) };
            changeCntOn as u64
        });
        __bindgen_bitfield_unit.set(5usize, 1u8, {
            let usesStmtJournal: u32 = unsafe { ::std::mem::transmute(usesStmtJournal) };
            usesStmtJournal as u64
        });
        __bindgen_bitfield_unit.set(6usize, 1u8, {
            let readOnly: u32 = unsafe { ::std::mem::transmute(readOnly) };
            readOnly as u64
        });
        __bindgen_bitfield_unit.set(7usize, 1u8, {
            let bIsReader: u32 = unsafe { ::std::mem::transmute(bIsReader) };
            bIsReader as u64
        });
        __bindgen_bitfield_unit.set(8usize, 1u8, {
            let haveEqpOps: u32 = unsafe { ::std::mem::transmute(haveEqpOps) };
            haveEqpOps as u64
        });
        __bindgen_bitfield_unit
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PreUpdate {
    pub v: *mut Vdbe,
    pub pCsr: *mut VdbeCursor,
    pub op: ::std::os::raw::c_int,
    pub aRecord: *mut u8_,
    pub pKeyinfo: *mut KeyInfo,
    pub pUnpacked: *mut UnpackedRecord,
    pub pNewUnpacked: *mut UnpackedRecord,
    pub iNewReg: ::std::os::raw::c_int,
    pub iBlobWrite: ::std::os::raw::c_int,
    pub iKey1: i64_,
    pub iKey2: i64_,
    pub oldipk: Mem,
    pub aNew: *mut Mem,
    pub pTab: *mut Table,
    pub pPk: *mut Index,
    pub apDflt: *mut *mut sqlite3_value,
    pub uKey: PreUpdate__bindgen_ty_1,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct PreUpdate__bindgen_ty_1 {
    pub keyinfoSpace: [u8_; 32usize],
}
impl Default for PreUpdate {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ValueList {
    pub pCsr: *mut BtCursor,
    pub pOut: *mut sqlite3_value,
}
impl Default for ValueList {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}