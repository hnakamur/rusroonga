pub type va_list = *mut ::libc::c_void;
pub type off_t = ::libc::c_long;

pub const GRN_OBJ_TABLE_TYPE_MASK:        ::libc::c_uint = (0x07);
pub const GRN_OBJ_TABLE_HASH_KEY:         ::libc::c_uint = (0x00);
pub const GRN_OBJ_TABLE_PAT_KEY:          ::libc::c_uint = (0x01);
pub const GRN_OBJ_TABLE_DAT_KEY:          ::libc::c_uint = (0x02);
pub const GRN_OBJ_TABLE_NO_KEY:           ::libc::c_uint = (0x03);

pub const GRN_OBJ_KEY_MASK:               ::libc::c_uint = (0x07<<3);
pub const GRN_OBJ_KEY_UINT:               ::libc::c_uint = (0x00<<3);
pub const GRN_OBJ_KEY_INT:                ::libc::c_uint = (0x01<<3);
pub const GRN_OBJ_KEY_FLOAT:              ::libc::c_uint = (0x02<<3);
pub const GRN_OBJ_KEY_GEO_POINT:          ::libc::c_uint = (0x03<<3);

pub const GRN_OBJ_KEY_WITH_SIS:           ::libc::c_uint = (0x01<<6);
pub const GRN_OBJ_KEY_NORMALIZE:          ::libc::c_uint = (0x01<<7);

pub const GRN_OBJ_COLUMN_TYPE_MASK:       ::libc::c_uint = (0x07);
pub const GRN_OBJ_COLUMN_SCALAR:          ::libc::c_uint = (0x00);
pub const GRN_OBJ_COLUMN_VECTOR:          ::libc::c_uint = (0x01);
pub const GRN_OBJ_COLUMN_INDEX:           ::libc::c_uint = (0x02);

pub const GRN_OBJ_COMPRESS_MASK:          ::libc::c_uint = (0x07<<4);
pub const GRN_OBJ_COMPRESS_NONE:          ::libc::c_uint = (0x00<<4);
pub const GRN_OBJ_COMPRESS_ZLIB:          ::libc::c_uint = (0x01<<4);
pub const GRN_OBJ_COMPRESS_LZ4:           ::libc::c_uint = (0x02<<4);

pub const GRN_OBJ_WITH_SECTION:           ::libc::c_uint = (0x01<<7);
pub const GRN_OBJ_WITH_WEIGHT:            ::libc::c_uint = (0x01<<8);
pub const GRN_OBJ_WITH_POSITION:          ::libc::c_uint = (0x01<<9);
pub const GRN_OBJ_RING_BUFFER:            ::libc::c_uint = (0x01<<10);

pub const GRN_OBJ_UNIT_MASK:              ::libc::c_uint = (0x0f<<8);
pub const GRN_OBJ_UNIT_DOCUMENT_NONE:     ::libc::c_uint = (0x00<<8);
pub const GRN_OBJ_UNIT_DOCUMENT_SECTION:  ::libc::c_uint = (0x01<<8);
pub const GRN_OBJ_UNIT_DOCUMENT_POSITION: ::libc::c_uint = (0x02<<8);
pub const GRN_OBJ_UNIT_SECTION_NONE:      ::libc::c_uint = (0x03<<8);
pub const GRN_OBJ_UNIT_SECTION_POSITION:  ::libc::c_uint = (0x04<<8);
pub const GRN_OBJ_UNIT_POSITION_NONE:     ::libc::c_uint = (0x05<<8);
pub const GRN_OBJ_UNIT_USERDEF_DOCUMENT:  ::libc::c_uint = (0x06<<8);
pub const GRN_OBJ_UNIT_USERDEF_SECTION:   ::libc::c_uint = (0x07<<8);
pub const GRN_OBJ_UNIT_USERDEF_POSITION:  ::libc::c_uint = (0x08<<8);

pub const GRN_OBJ_NO_SUBREC:              ::libc::c_uint = (0x00<<13);
pub const GRN_OBJ_WITH_SUBREC:            ::libc::c_uint = (0x01<<13);

pub const GRN_OBJ_KEY_VAR_SIZE:           ::libc::c_uint = (0x01<<14);

pub const GRN_OBJ_TEMPORARY:              ::libc::c_uint = (0x00<<15);
pub const GRN_OBJ_PERSISTENT:             ::libc::c_uint = (0x01<<15);

/* obj types */

pub const GRN_VOID:                       ::libc::c_uint = (0x00);
pub const GRN_BULK:                       ::libc::c_uint = (0x02);
pub const GRN_PTR:                        ::libc::c_uint = (0x03);
pub const GRN_UVECTOR:                    ::libc::c_uint = (0x04); /* vector of fixed size data especially grn_id */
pub const GRN_PVECTOR:                    ::libc::c_uint = (0x05); /* vector of grn_obj* */
pub const GRN_VECTOR:                     ::libc::c_uint = (0x06); /* vector of arbitrary data */
pub const GRN_MSG:                        ::libc::c_uint = (0x07);
pub const GRN_QUERY:                      ::libc::c_uint = (0x08);
pub const GRN_ACCESSOR:                   ::libc::c_uint = (0x09);
pub const GRN_SNIP:                       ::libc::c_uint = (0x0b);
pub const GRN_PATSNIP:                    ::libc::c_uint = (0x0c);
pub const GRN_STRING:                     ::libc::c_uint = (0x0d);
pub const GRN_CURSOR_TABLE_HASH_KEY:      ::libc::c_uint = (0x10);
pub const GRN_CURSOR_TABLE_PAT_KEY:       ::libc::c_uint = (0x11);
pub const GRN_CURSOR_TABLE_DAT_KEY:       ::libc::c_uint = (0x12);
pub const GRN_CURSOR_TABLE_NO_KEY:        ::libc::c_uint = (0x13);
pub const GRN_CURSOR_COLUMN_INDEX:        ::libc::c_uint = (0x18);
pub const GRN_CURSOR_COLUMN_GEO_INDEX:    ::libc::c_uint = (0x1a);
pub const GRN_TYPE:                       ::libc::c_uint = (0x20);
pub const GRN_PROC:                       ::libc::c_uint = (0x21);
pub const GRN_EXPR:                       ::libc::c_uint = (0x22);
pub const GRN_TABLE_HASH_KEY:             ::libc::c_uint = (0x30);
pub const GRN_TABLE_PAT_KEY:              ::libc::c_uint = (0x31);
pub const GRN_TABLE_DAT_KEY:              ::libc::c_uint = (0x32);
pub const GRN_TABLE_NO_KEY:               ::libc::c_uint = (0x33);
pub const GRN_DB:                         ::libc::c_uint = (0x37);
pub const GRN_COLUMN_FIX_SIZE:            ::libc::c_uint = (0x40);
pub const GRN_COLUMN_VAR_SIZE:            ::libc::c_uint = (0x41);
pub const GRN_COLUMN_INDEX:               ::libc::c_uint = (0x48);

// obj_set_value flags
pub const GRN_OBJ_SET_MASK:               ::libc::c_uint = (0x07);
pub const GRN_OBJ_SET:                    ::libc::c_uint = (0x01);
pub const GRN_OBJ_INCR:                   ::libc::c_uint = (0x02);
pub const GRN_OBJ_DECR:                   ::libc::c_uint = (0x03);
pub const GRN_OBJ_APPEND:                 ::libc::c_uint = (0x04);
pub const GRN_OBJ_PREPEND:                ::libc::c_uint = (0x05);
pub const GRN_OBJ_GET:                    ::libc::c_uint = (0x01<<4);
pub const GRN_OBJ_COMPARE:                ::libc::c_uint = (0x01<<5);
pub const GRN_OBJ_LOCK:                   ::libc::c_uint = (0x01<<6);
pub const GRN_OBJ_UNLOCK:                 ::libc::c_uint = (0x01<<7);

/* automatically generated by rust-bindgen */

pub type grn_id = ::libc::c_uint;
pub type grn_bool = ::libc::c_uchar;
pub type Enum_Unnamed1 = ::libc::c_int;
pub const GRN_SUCCESS: ::libc::c_int = 0;
pub const GRN_END_OF_DATA: ::libc::c_int = 1;
pub const GRN_UNKNOWN_ERROR: ::libc::c_int = -1;
pub const GRN_OPERATION_NOT_PERMITTED: ::libc::c_int = -2;
pub const GRN_NO_SUCH_FILE_OR_DIRECTORY: ::libc::c_int = -3;
pub const GRN_NO_SUCH_PROCESS: ::libc::c_int = -4;
pub const GRN_INTERRUPTED_FUNCTION_CALL: ::libc::c_int = -5;
pub const GRN_INPUT_OUTPUT_ERROR: ::libc::c_int = -6;
pub const GRN_NO_SUCH_DEVICE_OR_ADDRESS: ::libc::c_int = -7;
pub const GRN_ARG_LIST_TOO_LONG: ::libc::c_int = -8;
pub const GRN_EXEC_FORMAT_ERROR: ::libc::c_int = -9;
pub const GRN_BAD_FILE_DESCRIPTOR: ::libc::c_int = -10;
pub const GRN_NO_CHILD_PROCESSES: ::libc::c_int = -11;
pub const GRN_RESOURCE_TEMPORARILY_UNAVAILABLE: ::libc::c_int = -12;
pub const GRN_NOT_ENOUGH_SPACE: ::libc::c_int = -13;
pub const GRN_PERMISSION_DENIED: ::libc::c_int = -14;
pub const GRN_BAD_ADDRESS: ::libc::c_int = -15;
pub const GRN_RESOURCE_BUSY: ::libc::c_int = -16;
pub const GRN_FILE_EXISTS: ::libc::c_int = -17;
pub const GRN_IMPROPER_LINK: ::libc::c_int = -18;
pub const GRN_NO_SUCH_DEVICE: ::libc::c_int = -19;
pub const GRN_NOT_A_DIRECTORY: ::libc::c_int = -20;
pub const GRN_IS_A_DIRECTORY: ::libc::c_int = -21;
pub const GRN_INVALID_ARGUMENT: ::libc::c_int = -22;
pub const GRN_TOO_MANY_OPEN_FILES_IN_SYSTEM: ::libc::c_int = -23;
pub const GRN_TOO_MANY_OPEN_FILES: ::libc::c_int = -24;
pub const GRN_INAPPROPRIATE_I_O_CONTROL_OPERATION: ::libc::c_int = -25;
pub const GRN_FILE_TOO_LARGE: ::libc::c_int = -26;
pub const GRN_NO_SPACE_LEFT_ON_DEVICE: ::libc::c_int = -27;
pub const GRN_INVALID_SEEK: ::libc::c_int = -28;
pub const GRN_READ_ONLY_FILE_SYSTEM: ::libc::c_int = -29;
pub const GRN_TOO_MANY_LINKS: ::libc::c_int = -30;
pub const GRN_BROKEN_PIPE: ::libc::c_int = -31;
pub const GRN_DOMAIN_ERROR: ::libc::c_int = -32;
pub const GRN_RESULT_TOO_LARGE: ::libc::c_int = -33;
pub const GRN_RESOURCE_DEADLOCK_AVOIDED: ::libc::c_int = -34;
pub const GRN_NO_MEMORY_AVAILABLE: ::libc::c_int = -35;
pub const GRN_FILENAME_TOO_LONG: ::libc::c_int = -36;
pub const GRN_NO_LOCKS_AVAILABLE: ::libc::c_int = -37;
pub const GRN_FUNCTION_NOT_IMPLEMENTED: ::libc::c_int = -38;
pub const GRN_DIRECTORY_NOT_EMPTY: ::libc::c_int = -39;
pub const GRN_ILLEGAL_BYTE_SEQUENCE: ::libc::c_int = -40;
pub const GRN_SOCKET_NOT_INITIALIZED: ::libc::c_int = -41;
pub const GRN_OPERATION_WOULD_BLOCK: ::libc::c_int = -42;
pub const GRN_ADDRESS_IS_NOT_AVAILABLE: ::libc::c_int = -43;
pub const GRN_NETWORK_IS_DOWN: ::libc::c_int = -44;
pub const GRN_NO_BUFFER: ::libc::c_int = -45;
pub const GRN_SOCKET_IS_ALREADY_CONNECTED: ::libc::c_int = -46;
pub const GRN_SOCKET_IS_NOT_CONNECTED: ::libc::c_int = -47;
pub const GRN_SOCKET_IS_ALREADY_SHUTDOWNED: ::libc::c_int = -48;
pub const GRN_OPERATION_TIMEOUT: ::libc::c_int = -49;
pub const GRN_CONNECTION_REFUSED: ::libc::c_int = -50;
pub const GRN_RANGE_ERROR: ::libc::c_int = -51;
pub const GRN_TOKENIZER_ERROR: ::libc::c_int = -52;
pub const GRN_FILE_CORRUPT: ::libc::c_int = -53;
pub const GRN_INVALID_FORMAT: ::libc::c_int = -54;
pub const GRN_OBJECT_CORRUPT: ::libc::c_int = -55;
pub const GRN_TOO_MANY_SYMBOLIC_LINKS: ::libc::c_int = -56;
pub const GRN_NOT_SOCKET: ::libc::c_int = -57;
pub const GRN_OPERATION_NOT_SUPPORTED: ::libc::c_int = -58;
pub const GRN_ADDRESS_IS_IN_USE: ::libc::c_int = -59;
pub const GRN_ZLIB_ERROR: ::libc::c_int = -60;
pub const GRN_LZ4_ERROR: ::libc::c_int = -61;
pub const GRN_STACK_OVER_FLOW: ::libc::c_int = -62;
pub const GRN_SYNTAX_ERROR: ::libc::c_int = -63;
pub const GRN_RETRY_MAX: ::libc::c_int = -64;
pub const GRN_INCOMPATIBLE_FILE_FORMAT: ::libc::c_int = -65;
pub const GRN_UPDATE_NOT_ALLOWED: ::libc::c_int = -66;
pub const GRN_TOO_SMALL_OFFSET: ::libc::c_int = -67;
pub const GRN_TOO_LARGE_OFFSET: ::libc::c_int = -68;
pub const GRN_TOO_SMALL_LIMIT: ::libc::c_int = -69;
pub const GRN_CAS_ERROR: ::libc::c_int = -70;
pub const GRN_UNSUPPORTED_COMMAND_VERSION: ::libc::c_int = -71;
pub const GRN_NORMALIZER_ERROR: ::libc::c_int = -72;
pub const GRN_TOKEN_FILTER_ERROR: ::libc::c_int = -73;
pub const GRN_COMMAND_ERROR: ::libc::c_int = -74;
pub const GRN_PLUGIN_ERROR: ::libc::c_int = -75;
pub const GRN_SCORER_ERROR: ::libc::c_int = -76;
pub type grn_rc = Enum_Unnamed1;
pub type Enum_Unnamed2 = ::libc::c_uint;
pub const GRN_ENC_DEFAULT: ::libc::c_uint = 0;
pub const GRN_ENC_NONE: ::libc::c_uint = 1;
pub const GRN_ENC_EUC_JP: ::libc::c_uint = 2;
pub const GRN_ENC_UTF8: ::libc::c_uint = 3;
pub const GRN_ENC_SJIS: ::libc::c_uint = 4;
pub const GRN_ENC_LATIN1: ::libc::c_uint = 5;
pub const GRN_ENC_KOI8R: ::libc::c_uint = 6;
pub type grn_encoding = Enum_Unnamed2;
pub type Enum_Unnamed3 = ::libc::c_uint;
pub const GRN_COMMAND_VERSION_DEFAULT: ::libc::c_uint = 0;
pub const GRN_COMMAND_VERSION_1: ::libc::c_uint = 1;
pub const GRN_COMMAND_VERSION_2: ::libc::c_uint = 2;
pub type grn_command_version = Enum_Unnamed3;
pub type Enum_Unnamed4 = ::libc::c_uint;
pub const GRN_LOG_NONE: ::libc::c_uint = 0;
pub const GRN_LOG_EMERG: ::libc::c_uint = 1;
pub const GRN_LOG_ALERT: ::libc::c_uint = 2;
pub const GRN_LOG_CRIT: ::libc::c_uint = 3;
pub const GRN_LOG_ERROR: ::libc::c_uint = 4;
pub const GRN_LOG_WARNING: ::libc::c_uint = 5;
pub const GRN_LOG_NOTICE: ::libc::c_uint = 6;
pub const GRN_LOG_INFO: ::libc::c_uint = 7;
pub const GRN_LOG_DEBUG: ::libc::c_uint = 8;
pub const GRN_LOG_DUMP: ::libc::c_uint = 9;
pub type grn_log_level = Enum_Unnamed4;
pub type Enum_Unnamed5 = ::libc::c_uint;
pub const GRN_CONTENT_NONE: ::libc::c_uint = 0;
pub const GRN_CONTENT_TSV: ::libc::c_uint = 1;
pub const GRN_CONTENT_JSON: ::libc::c_uint = 2;
pub const GRN_CONTENT_XML: ::libc::c_uint = 3;
pub const GRN_CONTENT_MSGPACK: ::libc::c_uint = 4;
pub const GRN_CONTENT_GROONGA_COMMAND_LIST: ::libc::c_uint = 5;
pub type grn_content_type = Enum_Unnamed5;
pub type grn_obj = Struct__grn_obj;
pub type grn_ctx = Struct__grn_ctx;
#[repr(C)]
#[derive(Copy)]
pub struct Union_Unnamed6 {
    pub _bindgen_data_: [u64; 1usize],
}
impl Union_Unnamed6 {
    pub unsafe fn int_value(&mut self) -> *mut ::libc::c_int {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn id(&mut self) -> *mut grn_id {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn ptr(&mut self) -> *mut *mut ::libc::c_void {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
}
impl ::std::clone::Clone for Union_Unnamed6 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Union_Unnamed6 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type grn_user_data = Union_Unnamed6;
pub type grn_proc_func =
    extern "C" fn(ctx: *mut grn_ctx, nargs: ::libc::c_int,
                  args: *mut *mut grn_obj, user_data: *mut grn_user_data)
        -> *mut grn_obj;
pub enum Struct__grn_ctx_impl { }
#[repr(C)]
#[derive(Copy)]
pub struct Struct__grn_ctx {
    pub rc: grn_rc,
    pub flags: ::libc::c_int,
    pub encoding: grn_encoding,
    pub ntrace: ::libc::c_uchar,
    pub errlvl: ::libc::c_uchar,
    pub stat: ::libc::c_uchar,
    pub seqno: ::libc::c_uint,
    pub subno: ::libc::c_uint,
    pub seqno2: ::libc::c_uint,
    pub errline: ::libc::c_uint,
    pub user_data: grn_user_data,
    pub prev: *mut grn_ctx,
    pub next: *mut grn_ctx,
    pub errfile: *const ::libc::c_char,
    pub errfunc: *const ::libc::c_char,
    pub _impl: *mut Struct__grn_ctx_impl,
    pub trace: [*mut ::libc::c_void; 16usize],
    pub errbuf: [::libc::c_char; 128usize],
}
impl ::std::clone::Clone for Struct__grn_ctx {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct__grn_ctx {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub enum Struct__grn_cache { }
pub type grn_cache = Struct__grn_cache;
pub type grn_obj_flags = ::libc::c_ushort;
pub type grn_section = Struct__grn_section;
pub type grn_obj_header = Struct__grn_obj_header;
#[repr(C)]
#[derive(Copy)]
pub struct Struct__grn_section {
    pub offset: ::libc::c_uint,
    pub length: ::libc::c_uint,
    pub weight: ::libc::c_uint,
    pub domain: grn_id,
}
impl ::std::clone::Clone for Struct__grn_section {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct__grn_section {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct__grn_obj_header {
    pub _type: ::libc::c_uchar,
    pub impl_flags: ::libc::c_uchar,
    pub flags: grn_obj_flags,
    pub domain: grn_id,
}
impl ::std::clone::Clone for Struct__grn_obj_header {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct__grn_obj_header {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct__grn_obj {
    pub header: grn_obj_header,
    pub u: Union_Unnamed7,
}
impl ::std::clone::Clone for Struct__grn_obj {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct__grn_obj {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct Union_Unnamed7 {
    pub _bindgen_data_: [u64; 3usize],
}
impl Union_Unnamed7 {
    pub unsafe fn b(&mut self) -> *mut Struct_Unnamed8 {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn v(&mut self) -> *mut Struct_Unnamed9 {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
}
impl ::std::clone::Clone for Union_Unnamed7 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Union_Unnamed7 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed8 {
    pub head: *mut ::libc::c_char,
    pub curr: *mut ::libc::c_char,
    pub tail: *mut ::libc::c_char,
}
impl ::std::clone::Clone for Struct_Unnamed8 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_Unnamed8 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed9 {
    pub body: *mut grn_obj,
    pub sections: *mut grn_section,
    pub n_sections: ::libc::c_int,
}
impl ::std::clone::Clone for Struct_Unnamed9 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_Unnamed9 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type grn_db_create_optarg = Struct__grn_db_create_optarg;
#[repr(C)]
#[derive(Copy)]
pub struct Struct__grn_db_create_optarg {
    pub builtin_type_names: *mut *mut ::libc::c_char,
    pub n_builtin_type_names: ::libc::c_int,
}
impl ::std::clone::Clone for Struct__grn_db_create_optarg {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct__grn_db_create_optarg {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type Enum_Unnamed10 = ::libc::c_uint;
pub const GRN_DB_VOID: ::libc::c_uint = 0;
pub const GRN_DB_DB: ::libc::c_uint = 1;
pub const GRN_DB_OBJECT: ::libc::c_uint = 2;
pub const GRN_DB_BOOL: ::libc::c_uint = 3;
pub const GRN_DB_INT8: ::libc::c_uint = 4;
pub const GRN_DB_UINT8: ::libc::c_uint = 5;
pub const GRN_DB_INT16: ::libc::c_uint = 6;
pub const GRN_DB_UINT16: ::libc::c_uint = 7;
pub const GRN_DB_INT32: ::libc::c_uint = 8;
pub const GRN_DB_UINT32: ::libc::c_uint = 9;
pub const GRN_DB_INT64: ::libc::c_uint = 10;
pub const GRN_DB_UINT64: ::libc::c_uint = 11;
pub const GRN_DB_FLOAT: ::libc::c_uint = 12;
pub const GRN_DB_TIME: ::libc::c_uint = 13;
pub const GRN_DB_SHORT_TEXT: ::libc::c_uint = 14;
pub const GRN_DB_TEXT: ::libc::c_uint = 15;
pub const GRN_DB_LONG_TEXT: ::libc::c_uint = 16;
pub const GRN_DB_TOKYO_GEO_POINT: ::libc::c_uint = 17;
pub const GRN_DB_WGS84_GEO_POINT: ::libc::c_uint = 18;
pub type grn_builtin_type = Enum_Unnamed10;
pub type Enum_Unnamed11 = ::libc::c_uint;
pub const GRN_DB_MECAB: ::libc::c_uint = 64;
pub const GRN_DB_DELIMIT: ::libc::c_uint = 65;
pub const GRN_DB_UNIGRAM: ::libc::c_uint = 66;
pub const GRN_DB_BIGRAM: ::libc::c_uint = 67;
pub const GRN_DB_TRIGRAM: ::libc::c_uint = 68;
pub type grn_builtin_tokenizer = Enum_Unnamed11;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed12 {
    pub name: *const ::libc::c_char,
    pub name_size: ::libc::c_uint,
    pub value: grn_obj,
}
impl ::std::clone::Clone for Struct_Unnamed12 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_Unnamed12 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type grn_expr_var = Struct_Unnamed12;
pub type grn_plugin_func =
    ::std::option::Option<extern "C" fn(ctx: *mut grn_ctx) -> grn_rc>;
pub type Enum_Unnamed13 = ::libc::c_uint;
pub const GRN_PROC_INVALID: ::libc::c_uint = 0;
pub const GRN_PROC_TOKENIZER: ::libc::c_uint = 1;
pub const GRN_PROC_COMMAND: ::libc::c_uint = 2;
pub const GRN_PROC_FUNCTION: ::libc::c_uint = 3;
pub const GRN_PROC_HOOK: ::libc::c_uint = 4;
pub const GRN_PROC_NORMALIZER: ::libc::c_uint = 5;
pub const GRN_PROC_TOKEN_FILTER: ::libc::c_uint = 6;
pub const GRN_PROC_SCORER: ::libc::c_uint = 7;
pub type grn_proc_type = Enum_Unnamed13;
pub type grn_table_cursor = grn_obj;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed14 {
    pub rid: grn_id,
    pub sid: grn_id,
    pub pos: ::libc::c_uint,
    pub tf: ::libc::c_uint,
    pub weight: ::libc::c_uint,
    pub rest: ::libc::c_uint,
}
impl ::std::clone::Clone for Struct_Unnamed14 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_Unnamed14 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type grn_posting = Struct_Unnamed14;
pub type grn_table_sort_key = Struct__grn_table_sort_key;
pub type grn_table_sort_flags = ::libc::c_uchar;
#[repr(C)]
#[derive(Copy)]
pub struct Struct__grn_table_sort_key {
    pub key: *mut grn_obj,
    pub flags: grn_table_sort_flags,
    pub offset: ::libc::c_int,
}
impl ::std::clone::Clone for Struct__grn_table_sort_key {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct__grn_table_sort_key {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type grn_table_group_result = Struct__grn_table_group_result;
pub type grn_table_group_flags = ::libc::c_uint;
pub type Enum_Unnamed15 = ::libc::c_uint;
pub const GRN_OP_PUSH: ::libc::c_uint = 0;
pub const GRN_OP_POP: ::libc::c_uint = 1;
pub const GRN_OP_NOP: ::libc::c_uint = 2;
pub const GRN_OP_CALL: ::libc::c_uint = 3;
pub const GRN_OP_INTERN: ::libc::c_uint = 4;
pub const GRN_OP_GET_REF: ::libc::c_uint = 5;
pub const GRN_OP_GET_VALUE: ::libc::c_uint = 6;
pub const GRN_OP_AND: ::libc::c_uint = 7;
pub const GRN_OP_AND_NOT: ::libc::c_uint = 8;
pub const GRN_OP_OR: ::libc::c_uint = 9;
pub const GRN_OP_ASSIGN: ::libc::c_uint = 10;
pub const GRN_OP_STAR_ASSIGN: ::libc::c_uint = 11;
pub const GRN_OP_SLASH_ASSIGN: ::libc::c_uint = 12;
pub const GRN_OP_MOD_ASSIGN: ::libc::c_uint = 13;
pub const GRN_OP_PLUS_ASSIGN: ::libc::c_uint = 14;
pub const GRN_OP_MINUS_ASSIGN: ::libc::c_uint = 15;
pub const GRN_OP_SHIFTL_ASSIGN: ::libc::c_uint = 16;
pub const GRN_OP_SHIFTR_ASSIGN: ::libc::c_uint = 17;
pub const GRN_OP_SHIFTRR_ASSIGN: ::libc::c_uint = 18;
pub const GRN_OP_AND_ASSIGN: ::libc::c_uint = 19;
pub const GRN_OP_XOR_ASSIGN: ::libc::c_uint = 20;
pub const GRN_OP_OR_ASSIGN: ::libc::c_uint = 21;
pub const GRN_OP_JUMP: ::libc::c_uint = 22;
pub const GRN_OP_CJUMP: ::libc::c_uint = 23;
pub const GRN_OP_COMMA: ::libc::c_uint = 24;
pub const GRN_OP_BITWISE_OR: ::libc::c_uint = 25;
pub const GRN_OP_BITWISE_XOR: ::libc::c_uint = 26;
pub const GRN_OP_BITWISE_AND: ::libc::c_uint = 27;
pub const GRN_OP_BITWISE_NOT: ::libc::c_uint = 28;
pub const GRN_OP_EQUAL: ::libc::c_uint = 29;
pub const GRN_OP_NOT_EQUAL: ::libc::c_uint = 30;
pub const GRN_OP_LESS: ::libc::c_uint = 31;
pub const GRN_OP_GREATER: ::libc::c_uint = 32;
pub const GRN_OP_LESS_EQUAL: ::libc::c_uint = 33;
pub const GRN_OP_GREATER_EQUAL: ::libc::c_uint = 34;
pub const GRN_OP_IN: ::libc::c_uint = 35;
pub const GRN_OP_MATCH: ::libc::c_uint = 36;
pub const GRN_OP_NEAR: ::libc::c_uint = 37;
pub const GRN_OP_NEAR2: ::libc::c_uint = 38;
pub const GRN_OP_SIMILAR: ::libc::c_uint = 39;
pub const GRN_OP_TERM_EXTRACT: ::libc::c_uint = 40;
pub const GRN_OP_SHIFTL: ::libc::c_uint = 41;
pub const GRN_OP_SHIFTR: ::libc::c_uint = 42;
pub const GRN_OP_SHIFTRR: ::libc::c_uint = 43;
pub const GRN_OP_PLUS: ::libc::c_uint = 44;
pub const GRN_OP_MINUS: ::libc::c_uint = 45;
pub const GRN_OP_STAR: ::libc::c_uint = 46;
pub const GRN_OP_SLASH: ::libc::c_uint = 47;
pub const GRN_OP_MOD: ::libc::c_uint = 48;
pub const GRN_OP_DELETE: ::libc::c_uint = 49;
pub const GRN_OP_INCR: ::libc::c_uint = 50;
pub const GRN_OP_DECR: ::libc::c_uint = 51;
pub const GRN_OP_INCR_POST: ::libc::c_uint = 52;
pub const GRN_OP_DECR_POST: ::libc::c_uint = 53;
pub const GRN_OP_NOT: ::libc::c_uint = 54;
pub const GRN_OP_ADJUST: ::libc::c_uint = 55;
pub const GRN_OP_EXACT: ::libc::c_uint = 56;
pub const GRN_OP_LCP: ::libc::c_uint = 57;
pub const GRN_OP_PARTIAL: ::libc::c_uint = 58;
pub const GRN_OP_UNSPLIT: ::libc::c_uint = 59;
pub const GRN_OP_PREFIX: ::libc::c_uint = 60;
pub const GRN_OP_SUFFIX: ::libc::c_uint = 61;
pub const GRN_OP_GEO_DISTANCE1: ::libc::c_uint = 62;
pub const GRN_OP_GEO_DISTANCE2: ::libc::c_uint = 63;
pub const GRN_OP_GEO_DISTANCE3: ::libc::c_uint = 64;
pub const GRN_OP_GEO_DISTANCE4: ::libc::c_uint = 65;
pub const GRN_OP_GEO_WITHINP5: ::libc::c_uint = 66;
pub const GRN_OP_GEO_WITHINP6: ::libc::c_uint = 67;
pub const GRN_OP_GEO_WITHINP8: ::libc::c_uint = 68;
pub const GRN_OP_OBJ_SEARCH: ::libc::c_uint = 69;
pub const GRN_OP_EXPR_GET_VAR: ::libc::c_uint = 70;
pub const GRN_OP_TABLE_CREATE: ::libc::c_uint = 71;
pub const GRN_OP_TABLE_SELECT: ::libc::c_uint = 72;
pub const GRN_OP_TABLE_SORT: ::libc::c_uint = 73;
pub const GRN_OP_TABLE_GROUP: ::libc::c_uint = 74;
pub const GRN_OP_JSON_PUT: ::libc::c_uint = 75;
pub const GRN_OP_GET_MEMBER: ::libc::c_uint = 76;
pub const GRN_OP_REGEXP: ::libc::c_uint = 77;
pub type grn_operator = Enum_Unnamed15;
#[repr(C)]
#[derive(Copy)]
pub struct Struct__grn_table_group_result {
    pub table: *mut grn_obj,
    pub key_begin: ::libc::c_uchar,
    pub key_end: ::libc::c_uchar,
    pub limit: ::libc::c_int,
    pub flags: grn_table_group_flags,
    pub op: grn_operator,
    pub max_n_subrecs: ::libc::c_uint,
    pub calc_target: *mut grn_obj,
}
impl ::std::clone::Clone for Struct__grn_table_group_result {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct__grn_table_group_result {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type Enum_Unnamed16 = ::libc::c_uint;
pub const GRN_INFO_ENCODING: ::libc::c_uint = 0;
pub const GRN_INFO_SOURCE: ::libc::c_uint = 1;
pub const GRN_INFO_DEFAULT_TOKENIZER: ::libc::c_uint = 2;
pub const GRN_INFO_ELEMENT_SIZE: ::libc::c_uint = 3;
pub const GRN_INFO_CURR_MAX: ::libc::c_uint = 4;
pub const GRN_INFO_MAX_ELEMENT_SIZE: ::libc::c_uint = 5;
pub const GRN_INFO_SEG_SIZE: ::libc::c_uint = 6;
pub const GRN_INFO_CHUNK_SIZE: ::libc::c_uint = 7;
pub const GRN_INFO_MAX_SECTION: ::libc::c_uint = 8;
pub const GRN_INFO_HOOK_LOCAL_DATA: ::libc::c_uint = 9;
pub const GRN_INFO_ELEMENT_A: ::libc::c_uint = 10;
pub const GRN_INFO_ELEMENT_CHUNK: ::libc::c_uint = 11;
pub const GRN_INFO_ELEMENT_CHUNK_SIZE: ::libc::c_uint = 12;
pub const GRN_INFO_ELEMENT_BUFFER_FREE: ::libc::c_uint = 13;
pub const GRN_INFO_ELEMENT_NTERMS: ::libc::c_uint = 14;
pub const GRN_INFO_ELEMENT_NTERMS_VOID: ::libc::c_uint = 15;
pub const GRN_INFO_ELEMENT_SIZE_IN_CHUNK: ::libc::c_uint = 16;
pub const GRN_INFO_ELEMENT_POS_IN_CHUNK: ::libc::c_uint = 17;
pub const GRN_INFO_ELEMENT_SIZE_IN_BUFFER: ::libc::c_uint = 18;
pub const GRN_INFO_ELEMENT_POS_IN_BUFFER: ::libc::c_uint = 19;
pub const GRN_INFO_ELEMENT_ESTIMATE_SIZE: ::libc::c_uint = 20;
pub const GRN_INFO_NGRAM_UNIT_SIZE: ::libc::c_uint = 21;
pub const GRN_INFO_PARTIAL_MATCH_THRESHOLD: ::libc::c_uint = 22;
pub const GRN_INFO_II_SPLIT_THRESHOLD: ::libc::c_uint = 23;
pub const GRN_INFO_SUPPORT_ZLIB: ::libc::c_uint = 24;
pub const GRN_INFO_SUPPORT_LZ4: ::libc::c_uint = 25;
pub const GRN_INFO_NORMALIZER: ::libc::c_uint = 26;
pub const GRN_INFO_TOKEN_FILTERS: ::libc::c_uint = 27;
pub type grn_info_type = Enum_Unnamed16;
pub type grn_search_optarg = Struct__grn_search_optarg;
#[repr(C)]
#[derive(Copy)]
pub struct Struct__grn_search_optarg {
    pub mode: grn_operator,
    pub similarity_threshold: ::libc::c_int,
    pub max_interval: ::libc::c_int,
    pub weight_vector: *mut ::libc::c_int,
    pub vector_size: ::libc::c_int,
    pub _proc: *mut grn_obj,
    pub max_size: ::libc::c_int,
    pub scorer: *mut grn_obj,
    pub scorer_args_expr: *mut grn_obj,
    pub scorer_args_expr_offset: ::libc::c_uint,
}
impl ::std::clone::Clone for Struct__grn_search_optarg {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct__grn_search_optarg {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type grn_selector_func =
    extern "C" fn(ctx: *mut grn_ctx, table: *mut grn_obj, index: *mut grn_obj,
                  nargs: ::libc::c_int, args: *mut *mut grn_obj,
                  res: *mut grn_obj, op: grn_operator) -> grn_rc;
pub type Enum_Unnamed17 = ::libc::c_uint;
pub const GRN_HOOK_SET: ::libc::c_uint = 0;
pub const GRN_HOOK_GET: ::libc::c_uint = 1;
pub const GRN_HOOK_INSERT: ::libc::c_uint = 2;
pub const GRN_HOOK_DELETE: ::libc::c_uint = 3;
pub const GRN_HOOK_SELECT: ::libc::c_uint = 4;
pub type grn_hook_entry = Enum_Unnamed17;
#[repr(C)]
#[derive(Copy)]
pub struct Struct__grn_index_datum {
    pub index: *mut grn_obj,
    pub section: ::libc::c_uint,
}
impl ::std::clone::Clone for Struct__grn_index_datum {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct__grn_index_datum {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type grn_index_datum = Struct__grn_index_datum;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed18 {
    pub latitude: ::libc::c_int,
    pub longitude: ::libc::c_int,
}
impl ::std::clone::Clone for Struct_Unnamed18 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_Unnamed18 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type grn_geo_point = Struct_Unnamed18;
pub type grn_snip_mapping = Struct__grn_snip_mapping;
#[repr(C)]
#[derive(Copy)]
pub struct Struct__grn_snip_mapping {
    pub dummy: *mut ::libc::c_void,
}
impl ::std::clone::Clone for Struct__grn_snip_mapping {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct__grn_snip_mapping {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type grn_logger_info = Struct__grn_logger_info;
#[repr(C)]
#[derive(Copy)]
pub struct Struct__grn_logger_info {
    pub max_level: grn_log_level,
    pub flags: ::libc::c_int,
    pub func: ::std::option::Option<extern "C" fn(arg1: ::libc::c_int,
                                                  arg2: *const ::libc::c_char,
                                                  arg3: *const ::libc::c_char,
                                                  arg4: *const ::libc::c_char,
                                                  arg5: *const ::libc::c_char,
                                                  arg6: *mut ::libc::c_void)
                                        -> ()>,
    pub func_arg: *mut ::libc::c_void,
}
impl ::std::clone::Clone for Struct__grn_logger_info {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct__grn_logger_info {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type grn_logger = Struct__grn_logger;
#[repr(C)]
#[derive(Copy)]
pub struct Struct__grn_logger {
    pub max_level: grn_log_level,
    pub flags: ::libc::c_int,
    pub user_data: *mut ::libc::c_void,
    pub log: ::std::option::Option<extern "C" fn(ctx: *mut grn_ctx,
                                                 level: grn_log_level,
                                                 timestamp:
                                                     *const ::libc::c_char,
                                                 title: *const ::libc::c_char,
                                                 message:
                                                     *const ::libc::c_char,
                                                 location:
                                                     *const ::libc::c_char,
                                                 user_data:
                                                     *mut ::libc::c_void)
                                       -> ()>,
    pub reopen: ::std::option::Option<extern "C" fn(ctx: *mut grn_ctx,
                                                    user_data:
                                                        *mut ::libc::c_void)
                                          -> ()>,
    pub fin: ::std::option::Option<extern "C" fn(ctx: *mut grn_ctx,
                                                 user_data:
                                                     *mut ::libc::c_void)
                                       -> ()>,
}
impl ::std::clone::Clone for Struct__grn_logger {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct__grn_logger {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type grn_query_logger = Struct__grn_query_logger;
#[repr(C)]
#[derive(Copy)]
pub struct Struct__grn_query_logger {
    pub flags: ::libc::c_uint,
    pub user_data: *mut ::libc::c_void,
    pub log: ::std::option::Option<extern "C" fn(ctx: *mut grn_ctx,
                                                 flag: ::libc::c_uint,
                                                 timestamp:
                                                     *const ::libc::c_char,
                                                 info: *const ::libc::c_char,
                                                 message:
                                                     *const ::libc::c_char,
                                                 user_data:
                                                     *mut ::libc::c_void)
                                       -> ()>,
    pub reopen: ::std::option::Option<extern "C" fn(ctx: *mut grn_ctx,
                                                    user_data:
                                                        *mut ::libc::c_void)
                                          -> ()>,
    pub fin: ::std::option::Option<extern "C" fn(ctx: *mut grn_ctx,
                                                 user_data:
                                                     *mut ::libc::c_void)
                                       -> ()>,
}
impl ::std::clone::Clone for Struct__grn_query_logger {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct__grn_query_logger {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed19 {
    pub orig: *const ::libc::c_char,
    pub norm: *mut ::libc::c_char,
    pub checks: *mut ::libc::c_short,
    pub ctypes: *mut ::libc::c_uchar,
    pub flags: ::libc::c_int,
    pub orig_blen: ::libc::c_uint,
    pub norm_blen: ::libc::c_uint,
    pub length: ::libc::c_uint,
    pub encoding: grn_encoding,
}
impl ::std::clone::Clone for Struct_Unnamed19 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_Unnamed19 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type grn_str = Struct_Unnamed19;
pub type Enum_Unnamed20 = ::libc::c_uint;
pub const GRN_CHAR_NULL: ::libc::c_uint = 0;
pub const GRN_CHAR_ALPHA: ::libc::c_uint = 1;
pub const GRN_CHAR_DIGIT: ::libc::c_uint = 2;
pub const GRN_CHAR_SYMBOL: ::libc::c_uint = 3;
pub const GRN_CHAR_HIRAGANA: ::libc::c_uint = 4;
pub const GRN_CHAR_KATAKANA: ::libc::c_uint = 5;
pub const GRN_CHAR_KANJI: ::libc::c_uint = 6;
pub const GRN_CHAR_OTHERS: ::libc::c_uint = 7;
pub type grn_char_type = Enum_Unnamed20;
pub type grn_ctx_info = Struct__grn_ctx_info;
#[repr(C)]
#[derive(Copy)]
pub struct Struct__grn_ctx_info {
    pub fd: ::libc::c_int,
    pub com_status: ::libc::c_uint,
    pub outbuf: *mut grn_obj,
    pub stat: ::libc::c_uchar,
}
impl ::std::clone::Clone for Struct__grn_ctx_info {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct__grn_ctx_info {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub enum Struct__grn_hash { }
pub type grn_hash = Struct__grn_hash;
pub enum Struct__grn_hash_cursor { }
pub type grn_hash_cursor = Struct__grn_hash_cursor;
pub type grn_table_delete_optarg = Struct__grn_table_delete_optarg;
#[repr(C)]
#[derive(Copy)]
pub struct Struct__grn_table_delete_optarg {
    pub flags: ::libc::c_int,
    pub func: ::std::option::Option<extern "C" fn(ctx: *mut grn_ctx,
                                                  arg1: *mut grn_obj,
                                                  arg2: grn_id,
                                                  arg3: *mut ::libc::c_void)
                                        -> ::libc::c_int>,
    pub func_arg: *mut ::libc::c_void,
}
impl ::std::clone::Clone for Struct__grn_table_delete_optarg {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct__grn_table_delete_optarg {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub enum Struct__grn_array { }
pub type grn_array = Struct__grn_array;
pub enum Struct__grn_array_cursor { }
pub type grn_array_cursor = Struct__grn_array_cursor;
pub enum Struct__grn_pat { }
pub type grn_pat = Struct__grn_pat;
pub enum Struct__grn_pat_cursor { }
pub type grn_pat_cursor = Struct__grn_pat_cursor;
pub type grn_pat_scan_hit = Struct__grn_table_scan_hit;
pub type grn_dat_scan_hit = Struct__grn_table_scan_hit;
#[repr(C)]
#[derive(Copy)]
pub struct Struct__grn_table_scan_hit {
    pub id: grn_id,
    pub offset: ::libc::c_uint,
    pub length: ::libc::c_uint,
}
impl ::std::clone::Clone for Struct__grn_table_scan_hit {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct__grn_table_scan_hit {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub enum Struct__grn_dat { }
pub type grn_dat = Struct__grn_dat;
pub enum Struct__grn_dat_cursor { }
pub type grn_dat_cursor = Struct__grn_dat_cursor;
#[link(name = "groonga")]
extern "C" {
    pub fn grn_init() -> grn_rc;
    pub fn grn_fin() -> grn_rc;
    pub fn grn_ctx_init(ctx: *mut grn_ctx, flags: ::libc::c_int) -> grn_rc;
    pub fn grn_ctx_fin(ctx: *mut grn_ctx) -> grn_rc;
    pub fn grn_ctx_open(flags: ::libc::c_int) -> *mut grn_ctx;
    pub fn grn_ctx_close(ctx: *mut grn_ctx) -> grn_rc;
    pub fn grn_ctx_set_finalizer(ctx: *mut grn_ctx,
                                 func:
                                     *mut ::std::option::Option<extern "C" fn()
                                                                    ->
                                                                        *mut grn_obj>)
     -> grn_rc;
    pub fn grn_get_default_encoding() -> grn_encoding;
    pub fn grn_set_default_encoding(encoding: grn_encoding) -> grn_rc;
    pub fn grn_get_version() -> *const ::libc::c_char;
    pub fn grn_get_package() -> *const ::libc::c_char;
    pub fn grn_get_default_command_version() -> grn_command_version;
    pub fn grn_set_default_command_version(version: grn_command_version)
     -> grn_rc;
    pub fn grn_ctx_get_command_version(ctx: *mut grn_ctx)
     -> grn_command_version;
    pub fn grn_ctx_set_command_version(ctx: *mut grn_ctx,
                                       version: grn_command_version)
     -> grn_rc;
    pub fn grn_ctx_get_match_escalation_threshold(ctx: *mut grn_ctx)
     -> ::libc::c_longlong;
    pub fn grn_ctx_set_match_escalation_threshold(ctx: *mut grn_ctx,
                                                  threshold:
                                                      ::libc::c_longlong)
     -> grn_rc;
    pub fn grn_get_default_match_escalation_threshold() -> ::libc::c_longlong;
    pub fn grn_set_default_match_escalation_threshold(threshold:
                                                          ::libc::c_longlong)
     -> grn_rc;
    pub fn grn_get_lock_timeout() -> ::libc::c_int;
    pub fn grn_set_lock_timeout(timeout: ::libc::c_int) -> grn_rc;
    pub fn grn_cache_open(ctx: *mut grn_ctx) -> *mut grn_cache;
    pub fn grn_cache_close(ctx: *mut grn_ctx, cache: *mut grn_cache)
     -> grn_rc;
    pub fn grn_cache_current_set(ctx: *mut grn_ctx, cache: *mut grn_cache)
     -> grn_rc;
    pub fn grn_cache_current_get(ctx: *mut grn_ctx) -> *mut grn_cache;
    pub fn grn_cache_set_max_n_entries(ctx: *mut grn_ctx,
                                       cache: *mut grn_cache,
                                       n: ::libc::c_uint) -> grn_rc;
    pub fn grn_cache_get_max_n_entries(ctx: *mut grn_ctx,
                                       cache: *mut grn_cache)
     -> ::libc::c_uint;
    pub fn grn_encoding_to_string(encoding: grn_encoding)
     -> *const ::libc::c_char;
    pub fn grn_encoding_parse(name: *const ::libc::c_char) -> grn_encoding;
    pub fn grn_db_create(ctx: *mut grn_ctx, path: *const ::libc::c_char,
                         optarg: *mut grn_db_create_optarg) -> *mut grn_obj;
    pub fn grn_db_open(ctx: *mut grn_ctx, path: *const ::libc::c_char)
     -> *mut grn_obj;
    pub fn grn_db_touch(ctx: *mut grn_ctx, db: *mut grn_obj) -> ();
    pub fn grn_db_recover(ctx: *mut grn_ctx, db: *mut grn_obj) -> grn_rc;
    pub fn grn_ctx_use(ctx: *mut grn_ctx, db: *mut grn_obj) -> grn_rc;
    pub fn grn_ctx_db(ctx: *mut grn_ctx) -> *mut grn_obj;
    pub fn grn_ctx_get(ctx: *mut grn_ctx, name: *const ::libc::c_char,
                       name_size: ::libc::c_int) -> *mut grn_obj;
    pub fn grn_ctx_get_all_tables(ctx: *mut grn_ctx,
                                  tables_buffer: *mut grn_obj) -> grn_rc;
    pub fn grn_ctx_at(ctx: *mut grn_ctx, id: grn_id) -> *mut grn_obj;
    pub fn grn_type_create(ctx: *mut grn_ctx, name: *const ::libc::c_char,
                           name_size: ::libc::c_uint, flags: grn_obj_flags,
                           size: ::libc::c_uint) -> *mut grn_obj;
    pub fn grn_plugin_register(ctx: *mut grn_ctx, name: *const ::libc::c_char)
     -> grn_rc;
    pub fn grn_plugin_unregister(ctx: *mut grn_ctx,
                                 name: *const ::libc::c_char) -> grn_rc;
    pub fn grn_plugin_register_by_path(ctx: *mut grn_ctx,
                                       path: *const ::libc::c_char) -> grn_rc;
    pub fn grn_plugin_unregister_by_path(ctx: *mut grn_ctx,
                                         path: *const ::libc::c_char)
     -> grn_rc;
    pub fn grn_plugin_get_system_plugins_dir() -> *const ::libc::c_char;
    pub fn grn_plugin_get_suffix() -> *const ::libc::c_char;
    pub fn grn_plugin_get_ruby_suffix() -> *const ::libc::c_char;
    pub fn grn_proc_create(ctx: *mut grn_ctx, name: *const ::libc::c_char,
                           name_size: ::libc::c_int, _type: grn_proc_type,
                           init:
                               *mut ::std::option::Option<extern "C" fn()
                                                              ->
                                                                  *mut grn_obj>,
                           next:
                               *mut ::std::option::Option<extern "C" fn()
                                                              ->
                                                                  *mut grn_obj>,
                           fin:
                               *mut ::std::option::Option<extern "C" fn()
                                                              ->
                                                                  *mut grn_obj>,
                           nvars: ::libc::c_uint, vars: *mut grn_expr_var)
     -> *mut grn_obj;
    pub fn grn_proc_get_info(ctx: *mut grn_ctx, user_data: *mut grn_user_data,
                             vars: *mut *mut grn_expr_var,
                             nvars: *mut ::libc::c_uint,
                             caller: *mut *mut grn_obj) -> *mut grn_obj;
    pub fn grn_proc_get_type(ctx: *mut grn_ctx, _proc: *mut grn_obj)
     -> grn_proc_type;
    pub fn grn_table_create(ctx: *mut grn_ctx, name: *const ::libc::c_char,
                            name_size: ::libc::c_uint,
                            path: *const ::libc::c_char, flags: grn_obj_flags,
                            key_type: *mut grn_obj, value_type: *mut grn_obj)
     -> *mut grn_obj;
    pub fn grn_table_add(ctx: *mut grn_ctx, table: *mut grn_obj,
                         key: *const ::libc::c_void, key_size: ::libc::c_uint,
                         added: *mut ::libc::c_int) -> grn_id;
    pub fn grn_table_get(ctx: *mut grn_ctx, table: *mut grn_obj,
                         key: *const ::libc::c_void, key_size: ::libc::c_uint)
     -> grn_id;
    pub fn grn_table_at(ctx: *mut grn_ctx, table: *mut grn_obj, id: grn_id)
     -> grn_id;
    pub fn grn_table_lcp_search(ctx: *mut grn_ctx, table: *mut grn_obj,
                                key: *const ::libc::c_void,
                                key_size: ::libc::c_uint) -> grn_id;
    pub fn grn_table_get_key(ctx: *mut grn_ctx, table: *mut grn_obj,
                             id: grn_id, keybuf: *mut ::libc::c_void,
                             buf_size: ::libc::c_int) -> ::libc::c_int;
    pub fn grn_table_delete(ctx: *mut grn_ctx, table: *mut grn_obj,
                            key: *const ::libc::c_void,
                            key_size: ::libc::c_uint) -> grn_rc;
    pub fn grn_table_delete_by_id(ctx: *mut grn_ctx, table: *mut grn_obj,
                                  id: grn_id) -> grn_rc;
    pub fn grn_table_update_by_id(ctx: *mut grn_ctx, table: *mut grn_obj,
                                  id: grn_id, dest_key: *const ::libc::c_void,
                                  dest_key_size: ::libc::c_uint) -> grn_rc;
    pub fn grn_table_update(ctx: *mut grn_ctx, table: *mut grn_obj,
                            src_key: *const ::libc::c_void,
                            src_key_size: ::libc::c_uint,
                            dest_key: *const ::libc::c_void,
                            dest_key_size: ::libc::c_uint) -> grn_rc;
    pub fn grn_table_truncate(ctx: *mut grn_ctx, table: *mut grn_obj)
     -> grn_rc;
    pub fn grn_table_cursor_open(ctx: *mut grn_ctx, table: *mut grn_obj,
                                 min: *const ::libc::c_void,
                                 min_size: ::libc::c_uint,
                                 max: *const ::libc::c_void,
                                 max_size: ::libc::c_uint,
                                 offset: ::libc::c_int, limit: ::libc::c_int,
                                 flags: ::libc::c_int)
     -> *mut grn_table_cursor;
    pub fn grn_table_cursor_close(ctx: *mut grn_ctx,
                                  tc: *mut grn_table_cursor) -> grn_rc;
    pub fn grn_table_cursor_next(ctx: *mut grn_ctx, tc: *mut grn_table_cursor)
     -> grn_id;
    pub fn grn_table_cursor_get_key(ctx: *mut grn_ctx,
                                    tc: *mut grn_table_cursor,
                                    key: *mut *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn grn_table_cursor_get_value(ctx: *mut grn_ctx,
                                      tc: *mut grn_table_cursor,
                                      value: *mut *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn grn_table_cursor_set_value(ctx: *mut grn_ctx,
                                      tc: *mut grn_table_cursor,
                                      value: *const ::libc::c_void,
                                      flags: ::libc::c_int) -> grn_rc;
    pub fn grn_table_cursor_delete(ctx: *mut grn_ctx,
                                   tc: *mut grn_table_cursor) -> grn_rc;
    pub fn grn_table_cursor_table(ctx: *mut grn_ctx,
                                  tc: *mut grn_table_cursor) -> *mut grn_obj;
    pub fn grn_index_cursor_open(ctx: *mut grn_ctx, tc: *mut grn_table_cursor,
                                 index: *mut grn_obj, rid_min: grn_id,
                                 rid_max: grn_id, flags: ::libc::c_int)
     -> *mut grn_obj;
    pub fn grn_index_cursor_next(ctx: *mut grn_ctx, ic: *mut grn_obj,
                                 tid: *mut grn_id) -> *mut grn_posting;
    pub fn grn_table_sort(ctx: *mut grn_ctx, table: *mut grn_obj,
                          offset: ::libc::c_int, limit: ::libc::c_int,
                          result: *mut grn_obj, keys: *mut grn_table_sort_key,
                          n_keys: ::libc::c_int) -> ::libc::c_int;
    pub fn grn_operator_to_string(op: grn_operator) -> *const ::libc::c_char;
    pub fn grn_operator_exec_equal(ctx: *mut grn_ctx, x: *mut grn_obj,
                                   y: *mut grn_obj) -> grn_bool;
    pub fn grn_operator_exec_not_equal(ctx: *mut grn_ctx, x: *mut grn_obj,
                                       y: *mut grn_obj) -> grn_bool;
    pub fn grn_operator_exec_less(ctx: *mut grn_ctx, x: *mut grn_obj,
                                  y: *mut grn_obj) -> grn_bool;
    pub fn grn_operator_exec_greater(ctx: *mut grn_ctx, x: *mut grn_obj,
                                     y: *mut grn_obj) -> grn_bool;
    pub fn grn_operator_exec_less_equal(ctx: *mut grn_ctx, x: *mut grn_obj,
                                        y: *mut grn_obj) -> grn_bool;
    pub fn grn_operator_exec_greater_equal(ctx: *mut grn_ctx, x: *mut grn_obj,
                                           y: *mut grn_obj) -> grn_bool;
    pub fn grn_operator_exec_match(ctx: *mut grn_ctx, target: *mut grn_obj,
                                   sub_text: *mut grn_obj) -> grn_bool;
    pub fn grn_operator_exec_prefix(ctx: *mut grn_ctx, target: *mut grn_obj,
                                    prefix: *mut grn_obj) -> grn_bool;
    pub fn grn_operator_exec_regexp(ctx: *mut grn_ctx, target: *mut grn_obj,
                                    pattern: *mut grn_obj) -> grn_bool;
    pub fn grn_table_group(ctx: *mut grn_ctx, table: *mut grn_obj,
                           keys: *mut grn_table_sort_key,
                           n_keys: ::libc::c_int,
                           results: *mut grn_table_group_result,
                           n_results: ::libc::c_int) -> grn_rc;
    pub fn grn_table_setoperation(ctx: *mut grn_ctx, table1: *mut grn_obj,
                                  table2: *mut grn_obj, res: *mut grn_obj,
                                  op: grn_operator) -> grn_rc;
    pub fn grn_table_difference(ctx: *mut grn_ctx, table1: *mut grn_obj,
                                table2: *mut grn_obj, res1: *mut grn_obj,
                                res2: *mut grn_obj) -> grn_rc;
    pub fn grn_table_columns(ctx: *mut grn_ctx, table: *mut grn_obj,
                             name: *const ::libc::c_char,
                             name_size: ::libc::c_uint, res: *mut grn_obj)
     -> ::libc::c_int;
    pub fn grn_obj_column(ctx: *mut grn_ctx, table: *mut grn_obj,
                          name: *const ::libc::c_char,
                          name_size: ::libc::c_uint) -> *mut grn_obj;
    pub fn grn_table_size(ctx: *mut grn_ctx, table: *mut grn_obj)
     -> ::libc::c_uint;
    pub fn grn_column_create(ctx: *mut grn_ctx, table: *mut grn_obj,
                             name: *const ::libc::c_char,
                             name_size: ::libc::c_uint,
                             path: *const ::libc::c_char,
                             flags: grn_obj_flags, _type: *mut grn_obj)
     -> *mut grn_obj;
    pub fn grn_column_index_update(ctx: *mut grn_ctx, column: *mut grn_obj,
                                   id: grn_id, section: ::libc::c_uint,
                                   oldvalue: *mut grn_obj,
                                   newvalue: *mut grn_obj) -> grn_rc;
    pub fn grn_column_table(ctx: *mut grn_ctx, column: *mut grn_obj)
     -> *mut grn_obj;
    pub fn grn_column_truncate(ctx: *mut grn_ctx, column: *mut grn_obj)
     -> grn_rc;
    pub fn grn_obj_get_info(ctx: *mut grn_ctx, obj: *mut grn_obj,
                            _type: grn_info_type, valuebuf: *mut grn_obj)
     -> *mut grn_obj;
    pub fn grn_obj_set_info(ctx: *mut grn_ctx, obj: *mut grn_obj,
                            _type: grn_info_type, value: *mut grn_obj)
     -> grn_rc;
    pub fn grn_obj_get_element_info(ctx: *mut grn_ctx, obj: *mut grn_obj,
                                    id: grn_id, _type: grn_info_type,
                                    value: *mut grn_obj) -> *mut grn_obj;
    pub fn grn_obj_set_element_info(ctx: *mut grn_ctx, obj: *mut grn_obj,
                                    id: grn_id, _type: grn_info_type,
                                    value: *mut grn_obj) -> grn_rc;
    pub fn grn_obj_get_value(ctx: *mut grn_ctx, obj: *mut grn_obj, id: grn_id,
                             value: *mut grn_obj) -> *mut grn_obj;
    pub fn grn_obj_get_values(ctx: *mut grn_ctx, obj: *mut grn_obj,
                              offset: grn_id,
                              values: *mut *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn grn_obj_set_value(ctx: *mut grn_ctx, obj: *mut grn_obj, id: grn_id,
                             value: *mut grn_obj, flags: ::libc::c_int)
     -> grn_rc;
    pub fn grn_obj_remove(ctx: *mut grn_ctx, obj: *mut grn_obj) -> grn_rc;
    pub fn grn_obj_rename(ctx: *mut grn_ctx, obj: *mut grn_obj,
                          name: *const ::libc::c_char,
                          name_size: ::libc::c_uint) -> grn_rc;
    pub fn grn_table_rename(ctx: *mut grn_ctx, table: *mut grn_obj,
                            name: *const ::libc::c_char,
                            name_size: ::libc::c_uint) -> grn_rc;
    pub fn grn_column_rename(ctx: *mut grn_ctx, column: *mut grn_obj,
                             name: *const ::libc::c_char,
                             name_size: ::libc::c_uint) -> grn_rc;
    pub fn grn_obj_close(ctx: *mut grn_ctx, obj: *mut grn_obj) -> grn_rc;
    pub fn grn_obj_reinit(ctx: *mut grn_ctx, obj: *mut grn_obj,
                          domain: grn_id, flags: ::libc::c_uchar) -> grn_rc;
    pub fn grn_obj_unlink(ctx: *mut grn_ctx, obj: *mut grn_obj) -> ();
    pub fn grn_obj_user_data(ctx: *mut grn_ctx, obj: *mut grn_obj)
     -> *mut grn_user_data;
    pub fn grn_obj_set_finalizer(ctx: *mut grn_ctx, obj: *mut grn_obj,
                                 func:
                                     *mut ::std::option::Option<extern "C" fn()
                                                                    ->
                                                                        *mut grn_obj>)
     -> grn_rc;
    pub fn grn_obj_path(ctx: *mut grn_ctx, obj: *mut grn_obj)
     -> *const ::libc::c_char;
    pub fn grn_obj_name(ctx: *mut grn_ctx, obj: *mut grn_obj,
                        namebuf: *mut ::libc::c_char, buf_size: ::libc::c_int)
     -> ::libc::c_int;
    pub fn grn_column_name(ctx: *mut grn_ctx, obj: *mut grn_obj,
                           namebuf: *mut ::libc::c_char,
                           buf_size: ::libc::c_int) -> ::libc::c_int;
    pub fn grn_obj_get_range(ctx: *mut grn_ctx, obj: *mut grn_obj) -> grn_id;
    pub fn grn_obj_expire(ctx: *mut grn_ctx, obj: *mut grn_obj,
                          threshold: ::libc::c_int) -> ::libc::c_int;
    pub fn grn_obj_check(ctx: *mut grn_ctx, obj: *mut grn_obj)
     -> ::libc::c_int;
    pub fn grn_obj_lock(ctx: *mut grn_ctx, obj: *mut grn_obj, id: grn_id,
                        timeout: ::libc::c_int) -> grn_rc;
    pub fn grn_obj_unlock(ctx: *mut grn_ctx, obj: *mut grn_obj, id: grn_id)
     -> grn_rc;
    pub fn grn_obj_clear_lock(ctx: *mut grn_ctx, obj: *mut grn_obj) -> grn_rc;
    pub fn grn_obj_is_locked(ctx: *mut grn_ctx, obj: *mut grn_obj)
     -> ::libc::c_uint;
    pub fn grn_obj_defrag(ctx: *mut grn_ctx, obj: *mut grn_obj,
                          threshold: ::libc::c_int) -> ::libc::c_int;
    pub fn grn_obj_db(ctx: *mut grn_ctx, obj: *mut grn_obj) -> *mut grn_obj;
    pub fn grn_obj_id(ctx: *mut grn_ctx, obj: *mut grn_obj) -> grn_id;
    pub fn grn_obj_search(ctx: *mut grn_ctx, obj: *mut grn_obj,
                          query: *mut grn_obj, res: *mut grn_obj,
                          op: grn_operator, optarg: *mut grn_search_optarg)
     -> grn_rc;
    pub fn grn_proc_set_selector(ctx: *mut grn_ctx, _proc: *mut grn_obj,
                                 selector: grn_selector_func) -> grn_rc;
    pub fn grn_vector_size(ctx: *mut grn_ctx, vector: *mut grn_obj)
     -> ::libc::c_uint;
    pub fn grn_vector_add_element(ctx: *mut grn_ctx, vector: *mut grn_obj,
                                  str: *const ::libc::c_char,
                                  str_len: ::libc::c_uint,
                                  weight: ::libc::c_uint, domain: grn_id)
     -> grn_rc;
    pub fn grn_vector_get_element(ctx: *mut grn_ctx, vector: *mut grn_obj,
                                  offset: ::libc::c_uint,
                                  str: *mut *const ::libc::c_char,
                                  weight: *mut ::libc::c_uint,
                                  domain: *mut grn_id) -> ::libc::c_uint;
    pub fn grn_uvector_size(ctx: *mut grn_ctx, uvector: *mut grn_obj)
     -> ::libc::c_uint;
    pub fn grn_uvector_element_size(ctx: *mut grn_ctx, uvector: *mut grn_obj)
     -> ::libc::c_uint;
    pub fn grn_uvector_add_element(ctx: *mut grn_ctx, vector: *mut grn_obj,
                                   id: grn_id, weight: ::libc::c_uint)
     -> grn_rc;
    pub fn grn_uvector_get_element(ctx: *mut grn_ctx, uvector: *mut grn_obj,
                                   offset: ::libc::c_uint,
                                   weight: *mut ::libc::c_uint) -> grn_id;
    pub fn grn_proc_call_next(ctx: *mut grn_ctx, exec_info: *mut grn_obj,
                              _in: *mut grn_obj, out: *mut grn_obj)
     -> ::libc::c_int;
    pub fn grn_proc_get_ctx_local_data(ctx: *mut grn_ctx,
                                       exec_info: *mut grn_obj)
     -> *mut ::libc::c_void;
    pub fn grn_proc_get_hook_local_data(ctx: *mut grn_ctx,
                                        exec_info: *mut grn_obj)
     -> *mut ::libc::c_void;
    pub fn grn_obj_add_hook(ctx: *mut grn_ctx, obj: *mut grn_obj,
                            entry: grn_hook_entry, offset: ::libc::c_int,
                            _proc: *mut grn_obj, data: *mut grn_obj)
     -> grn_rc;
    pub fn grn_obj_get_nhooks(ctx: *mut grn_ctx, obj: *mut grn_obj,
                              entry: grn_hook_entry) -> ::libc::c_int;
    pub fn grn_obj_get_hook(ctx: *mut grn_ctx, obj: *mut grn_obj,
                            entry: grn_hook_entry, offset: ::libc::c_int,
                            data: *mut grn_obj) -> *mut grn_obj;
    pub fn grn_obj_delete_hook(ctx: *mut grn_ctx, obj: *mut grn_obj,
                               entry: grn_hook_entry, offset: ::libc::c_int)
     -> grn_rc;
    pub fn grn_obj_open(ctx: *mut grn_ctx, _type: ::libc::c_uchar,
                        flags: grn_obj_flags, domain: grn_id) -> *mut grn_obj;
    pub fn grn_column_index(ctx: *mut grn_ctx, column: *mut grn_obj,
                            op: grn_operator, indexbuf: *mut *mut grn_obj,
                            buf_size: ::libc::c_int,
                            section: *mut ::libc::c_int) -> ::libc::c_int;
    pub fn grn_column_find_index_data(ctx: *mut grn_ctx, column: *mut grn_obj,
                                      op: grn_operator,
                                      index_data: *mut grn_index_datum,
                                      n_index_data: ::libc::c_uint)
     -> ::libc::c_uint;
    pub fn grn_obj_delete_by_id(ctx: *mut grn_ctx, db: *mut grn_obj,
                                id: grn_id, removep: grn_bool) -> grn_rc;
    pub fn grn_obj_path_by_id(ctx: *mut grn_ctx, db: *mut grn_obj, id: grn_id,
                              buffer: *mut ::libc::c_char) -> grn_rc;
    pub fn grn_geo_select_in_rectangle(ctx: *mut grn_ctx, index: *mut grn_obj,
                                       top_left_point: *mut grn_obj,
                                       bottom_right_point: *mut grn_obj,
                                       res: *mut grn_obj, op: grn_operator)
     -> grn_rc;
    pub fn grn_geo_estimate_size_in_rectangle(ctx: *mut grn_ctx,
                                              index: *mut grn_obj,
                                              top_left_point: *mut grn_obj,
                                              bottom_right_point:
                                                  *mut grn_obj)
     -> ::libc::c_uint;
    pub fn grn_geo_estimate_in_rectangle(ctx: *mut grn_ctx,
                                         index: *mut grn_obj,
                                         top_left_point: *mut grn_obj,
                                         bottom_right_point: *mut grn_obj)
     -> ::libc::c_int;
    pub fn grn_geo_cursor_open_in_rectangle(ctx: *mut grn_ctx,
                                            index: *mut grn_obj,
                                            top_left_point: *mut grn_obj,
                                            bottom_right_point: *mut grn_obj,
                                            offset: ::libc::c_int,
                                            limit: ::libc::c_int)
     -> *mut grn_obj;
    pub fn grn_geo_cursor_next(ctx: *mut grn_ctx, cursor: *mut grn_obj)
     -> *mut grn_posting;
    pub fn grn_snip_open(ctx: *mut grn_ctx, flags: ::libc::c_int,
                         width: ::libc::c_uint, max_results: ::libc::c_uint,
                         defaultopentag: *const ::libc::c_char,
                         defaultopentag_len: ::libc::c_uint,
                         defaultclosetag: *const ::libc::c_char,
                         defaultclosetag_len: ::libc::c_uint,
                         mapping: *mut grn_snip_mapping) -> *mut grn_obj;
    pub fn grn_snip_add_cond(ctx: *mut grn_ctx, snip: *mut grn_obj,
                             keyword: *const ::libc::c_char,
                             keyword_len: ::libc::c_uint,
                             opentag: *const ::libc::c_char,
                             opentag_len: ::libc::c_uint,
                             closetag: *const ::libc::c_char,
                             closetag_len: ::libc::c_uint) -> grn_rc;
    pub fn grn_snip_set_normalizer(ctx: *mut grn_ctx, snip: *mut grn_obj,
                                   normalizer: *mut grn_obj) -> grn_rc;
    pub fn grn_snip_get_normalizer(ctx: *mut grn_ctx, snip: *mut grn_obj)
     -> *mut grn_obj;
    pub fn grn_snip_exec(ctx: *mut grn_ctx, snip: *mut grn_obj,
                         string: *const ::libc::c_char,
                         string_len: ::libc::c_uint,
                         nresults: *mut ::libc::c_uint,
                         max_tagged_len: *mut ::libc::c_uint) -> grn_rc;
    pub fn grn_snip_get_result(ctx: *mut grn_ctx, snip: *mut grn_obj,
                               index: ::libc::c_uint,
                               result: *mut ::libc::c_char,
                               result_len: *mut ::libc::c_uint) -> grn_rc;
    pub fn grn_logger_info_set(ctx: *mut grn_ctx,
                               info: *const grn_logger_info) -> grn_rc;
    pub fn grn_logger_set(ctx: *mut grn_ctx, logger: *const grn_logger)
     -> grn_rc;
    pub fn grn_logger_set_max_level(ctx: *mut grn_ctx,
                                    max_level: grn_log_level) -> ();
    pub fn grn_logger_get_max_level(ctx: *mut grn_ctx) -> grn_log_level;
    pub fn grn_logger_put(ctx: *mut grn_ctx, level: grn_log_level,
                          file: *const ::libc::c_char, line: ::libc::c_int,
                          func: *const ::libc::c_char,
                          fmt: *const ::libc::c_char, ...) -> ();
    pub fn grn_logger_reopen(ctx: *mut grn_ctx) -> ();
    pub fn grn_logger_pass(ctx: *mut grn_ctx, level: grn_log_level)
     -> grn_bool;
    pub fn grn_default_logger_set_max_level(level: grn_log_level) -> ();
    pub fn grn_default_logger_get_max_level() -> grn_log_level;
    pub fn grn_default_logger_set_path(path: *const ::libc::c_char) -> ();
    pub fn grn_default_logger_get_path() -> *const ::libc::c_char;
    pub fn grn_default_logger_set_rotate_threshold_size(threshold: off_t)
     -> ();
    pub fn grn_default_logger_get_rotate_threshold_size() -> off_t;
    pub fn grn_query_logger_set(ctx: *mut grn_ctx,
                                logger: *const grn_query_logger) -> grn_rc;
    pub fn grn_query_logger_put(ctx: *mut grn_ctx, flag: ::libc::c_uint,
                                mark: *const ::libc::c_char,
                                format: *const ::libc::c_char, ...) -> ();
    pub fn grn_query_logger_reopen(ctx: *mut grn_ctx) -> ();
    pub fn grn_query_logger_pass(ctx: *mut grn_ctx, flag: ::libc::c_uint)
     -> grn_bool;
    pub fn grn_default_query_logger_set_flags(flags: ::libc::c_uint) -> ();
    pub fn grn_default_query_logger_get_flags() -> ::libc::c_uint;
    pub fn grn_default_query_logger_set_path(path: *const ::libc::c_char)
     -> ();
    pub fn grn_default_query_logger_get_path() -> *const ::libc::c_char;
    pub fn grn_default_query_logger_set_rotate_threshold_size(threshold:
                                                                  off_t)
     -> ();
    pub fn grn_default_query_logger_get_rotate_threshold_size() -> off_t;
    pub fn grn_bulk_reinit(ctx: *mut grn_ctx, bulk: *mut grn_obj,
                           size: ::libc::c_uint) -> grn_rc;
    pub fn grn_bulk_resize(ctx: *mut grn_ctx, bulk: *mut grn_obj,
                           newsize: ::libc::c_uint) -> grn_rc;
    pub fn grn_bulk_write(ctx: *mut grn_ctx, bulk: *mut grn_obj,
                          str: *const ::libc::c_char, len: ::libc::c_uint)
     -> grn_rc;
    pub fn grn_bulk_write_from(ctx: *mut grn_ctx, bulk: *mut grn_obj,
                               str: *const ::libc::c_char,
                               from: ::libc::c_uint, len: ::libc::c_uint)
     -> grn_rc;
    pub fn grn_bulk_reserve(ctx: *mut grn_ctx, bulk: *mut grn_obj,
                            len: ::libc::c_uint) -> grn_rc;
    pub fn grn_bulk_space(ctx: *mut grn_ctx, bulk: *mut grn_obj,
                          len: ::libc::c_uint) -> grn_rc;
    pub fn grn_bulk_truncate(ctx: *mut grn_ctx, bulk: *mut grn_obj,
                             len: ::libc::c_uint) -> grn_rc;
    pub fn grn_bulk_fin(ctx: *mut grn_ctx, bulk: *mut grn_obj) -> grn_rc;
    pub fn grn_text_itoa(ctx: *mut grn_ctx, bulk: *mut grn_obj,
                         i: ::libc::c_int) -> grn_rc;
    pub fn grn_text_itoa_padded(ctx: *mut grn_ctx, bulk: *mut grn_obj,
                                i: ::libc::c_int, ch: ::libc::c_char,
                                len: ::libc::c_uint) -> grn_rc;
    pub fn grn_text_lltoa(ctx: *mut grn_ctx, bulk: *mut grn_obj,
                          i: ::libc::c_longlong) -> grn_rc;
    pub fn grn_text_ftoa(ctx: *mut grn_ctx, bulk: *mut grn_obj,
                         d: ::libc::c_double) -> grn_rc;
    pub fn grn_text_itoh(ctx: *mut grn_ctx, bulk: *mut grn_obj,
                         i: ::libc::c_int, len: ::libc::c_uint) -> grn_rc;
    pub fn grn_text_itob(ctx: *mut grn_ctx, bulk: *mut grn_obj, id: grn_id)
     -> grn_rc;
    pub fn grn_text_lltob32h(ctx: *mut grn_ctx, bulk: *mut grn_obj,
                             i: ::libc::c_longlong) -> grn_rc;
    pub fn grn_text_benc(ctx: *mut grn_ctx, bulk: *mut grn_obj,
                         v: ::libc::c_uint) -> grn_rc;
    pub fn grn_text_esc(ctx: *mut grn_ctx, bulk: *mut grn_obj,
                        s: *const ::libc::c_char, len: ::libc::c_uint)
     -> grn_rc;
    pub fn grn_text_urlenc(ctx: *mut grn_ctx, buf: *mut grn_obj,
                           str: *const ::libc::c_char, len: ::libc::c_uint)
     -> grn_rc;
    pub fn grn_text_urldec(ctx: *mut grn_ctx, buf: *mut grn_obj,
                           s: *const ::libc::c_char, e: *const ::libc::c_char,
                           d: ::libc::c_char) -> *const ::libc::c_char;
    pub fn grn_text_escape_xml(ctx: *mut grn_ctx, buf: *mut grn_obj,
                               s: *const ::libc::c_char, len: ::libc::c_uint)
     -> grn_rc;
    pub fn grn_text_time2rfc1123(ctx: *mut grn_ctx, bulk: *mut grn_obj,
                                 sec: ::libc::c_int) -> grn_rc;
    pub fn grn_text_printf(ctx: *mut grn_ctx, bulk: *mut grn_obj,
                           format: *const ::libc::c_char, ...) -> grn_rc;
    pub fn grn_text_vprintf(ctx: *mut grn_ctx, bulk: *mut grn_obj,
                            format: *const ::libc::c_char, args: va_list)
     -> grn_rc;
    pub fn grn_ctx_recv_handler_set(arg1: *mut grn_ctx,
                                    func:
                                        ::std::option::Option<extern "C" fn(arg1:
                                                                                *mut grn_ctx,
                                                                            arg2:
                                                                                ::libc::c_int,
                                                                            arg3:
                                                                                *mut ::libc::c_void)
                                                                  -> ()>,
                                    func_arg: *mut ::libc::c_void) -> ();
    pub fn grn_time_now(ctx: *mut grn_ctx, obj: *mut grn_obj) -> ();
    pub fn grn_str_open(ctx: *mut grn_ctx, str: *const ::libc::c_char,
                        str_len: ::libc::c_uint, flags: ::libc::c_int)
     -> *mut grn_str;
    pub fn grn_str_close(ctx: *mut grn_ctx, nstr: *mut grn_str) -> grn_rc;
    pub fn grn_string_open(ctx: *mut grn_ctx, string: *const ::libc::c_char,
                           length_in_bytes: ::libc::c_uint,
                           normalizer: *mut grn_obj, flags: ::libc::c_int)
     -> *mut grn_obj;
    pub fn grn_string_get_original(ctx: *mut grn_ctx, string: *mut grn_obj,
                                   original: *mut *const ::libc::c_char,
                                   length_in_bytes: *mut ::libc::c_uint)
     -> grn_rc;
    pub fn grn_string_get_flags(ctx: *mut grn_ctx, string: *mut grn_obj)
     -> ::libc::c_int;
    pub fn grn_string_get_normalized(ctx: *mut grn_ctx, string: *mut grn_obj,
                                     normalized: *mut *const ::libc::c_char,
                                     length_in_bytes: *mut ::libc::c_uint,
                                     n_characters: *mut ::libc::c_uint)
     -> grn_rc;
    pub fn grn_string_set_normalized(ctx: *mut grn_ctx, string: *mut grn_obj,
                                     normalized: *mut ::libc::c_char,
                                     length_in_bytes: ::libc::c_uint,
                                     n_characters: ::libc::c_uint) -> grn_rc;
    pub fn grn_string_get_checks(ctx: *mut grn_ctx, string: *mut grn_obj)
     -> *const ::libc::c_short;
    pub fn grn_string_set_checks(ctx: *mut grn_ctx, string: *mut grn_obj,
                                 checks: *mut ::libc::c_short) -> grn_rc;
    pub fn grn_string_get_types(ctx: *mut grn_ctx, string: *mut grn_obj)
     -> *const ::libc::c_uchar;
    pub fn grn_string_set_types(ctx: *mut grn_ctx, string: *mut grn_obj,
                                types: *mut ::libc::c_uchar) -> grn_rc;
    pub fn grn_string_get_encoding(ctx: *mut grn_ctx, string: *mut grn_obj)
     -> grn_encoding;
    pub fn grn_charlen(ctx: *mut grn_ctx, str: *const ::libc::c_char,
                       end: *const ::libc::c_char) -> ::libc::c_int;
    pub fn grn_ctx_push(ctx: *mut grn_ctx, obj: *mut grn_obj) -> grn_rc;
    pub fn grn_ctx_pop(ctx: *mut grn_ctx) -> *mut grn_obj;
    pub fn grn_table_select(ctx: *mut grn_ctx, table: *mut grn_obj,
                            expr: *mut grn_obj, res: *mut grn_obj,
                            op: grn_operator) -> *mut grn_obj;
    pub fn grn_obj_columns(ctx: *mut grn_ctx, table: *mut grn_obj,
                           str: *const ::libc::c_char,
                           str_size: ::libc::c_uint, res: *mut grn_obj)
     -> ::libc::c_int;
    pub fn grn_table_sort_key_from_str(ctx: *mut grn_ctx,
                                       str: *const ::libc::c_char,
                                       str_size: ::libc::c_uint,
                                       table: *mut grn_obj,
                                       nkeys: *mut ::libc::c_uint)
     -> *mut grn_table_sort_key;
    pub fn grn_table_sort_key_close(ctx: *mut grn_ctx,
                                    keys: *mut grn_table_sort_key,
                                    nkeys: ::libc::c_uint) -> grn_rc;
    pub fn grn_table_is_grouped(ctx: *mut grn_ctx, table: *mut grn_obj)
     -> grn_bool;
    pub fn grn_table_max_n_subrecs(ctx: *mut grn_ctx, table: *mut grn_obj)
     -> ::libc::c_uint;
    pub fn grn_table_create_for_group(ctx: *mut grn_ctx,
                                      name: *const ::libc::c_char,
                                      name_size: ::libc::c_uint,
                                      path: *const ::libc::c_char,
                                      group_key: *mut grn_obj,
                                      value_type: *mut grn_obj,
                                      max_n_subrecs: ::libc::c_uint)
     -> *mut grn_obj;
    pub fn grn_table_get_subrecs(ctx: *mut grn_ctx, table: *mut grn_obj,
                                 id: grn_id, subrecbuf: *mut grn_id,
                                 scorebuf: *mut ::libc::c_int,
                                 buf_size: ::libc::c_int) -> ::libc::c_uint;
    pub fn grn_table_tokenize(ctx: *mut grn_ctx, table: *mut grn_obj,
                              str: *const ::libc::c_char,
                              str_len: ::libc::c_uint, buf: *mut grn_obj,
                              addp: grn_bool) -> *mut grn_obj;
    pub fn grn_load(ctx: *mut grn_ctx, input_type: grn_content_type,
                    table: *const ::libc::c_char, table_len: ::libc::c_uint,
                    columns: *const ::libc::c_char,
                    columns_len: ::libc::c_uint,
                    values: *const ::libc::c_char, values_len: ::libc::c_uint,
                    ifexists: *const ::libc::c_char,
                    ifexists_len: ::libc::c_uint, each: *const ::libc::c_char,
                    each_len: ::libc::c_uint) -> grn_rc;
    pub fn grn_ctx_connect(ctx: *mut grn_ctx, host: *const ::libc::c_char,
                           port: ::libc::c_int, flags: ::libc::c_int)
     -> grn_rc;
    pub fn grn_ctx_send(ctx: *mut grn_ctx, str: *const ::libc::c_char,
                        str_len: ::libc::c_uint, flags: ::libc::c_int)
     -> ::libc::c_uint;
    pub fn grn_ctx_recv(ctx: *mut grn_ctx, str: *mut *mut ::libc::c_char,
                        str_len: *mut ::libc::c_uint,
                        flags: *mut ::libc::c_int) -> ::libc::c_uint;
    pub fn grn_ctx_info_get(ctx: *mut grn_ctx, info: *mut grn_ctx_info)
     -> grn_rc;
    pub fn grn_set_segv_handler() -> grn_rc;
    pub fn grn_set_int_handler() -> grn_rc;
    pub fn grn_set_term_handler() -> grn_rc;
    pub fn grn_hash_create(ctx: *mut grn_ctx, path: *const ::libc::c_char,
                           key_size: ::libc::c_uint,
                           value_size: ::libc::c_uint, flags: ::libc::c_uint)
     -> *mut grn_hash;
    pub fn grn_hash_open(ctx: *mut grn_ctx, path: *const ::libc::c_char)
     -> *mut grn_hash;
    pub fn grn_hash_close(ctx: *mut grn_ctx, hash: *mut grn_hash) -> grn_rc;
    pub fn grn_hash_add(ctx: *mut grn_ctx, hash: *mut grn_hash,
                        key: *const ::libc::c_void, key_size: ::libc::c_uint,
                        value: *mut *mut ::libc::c_void,
                        added: *mut ::libc::c_int) -> grn_id;
    pub fn grn_hash_get(ctx: *mut grn_ctx, hash: *mut grn_hash,
                        key: *const ::libc::c_void, key_size: ::libc::c_uint,
                        value: *mut *mut ::libc::c_void) -> grn_id;
    pub fn grn_hash_get_key(ctx: *mut grn_ctx, hash: *mut grn_hash,
                            id: grn_id, keybuf: *mut ::libc::c_void,
                            bufsize: ::libc::c_int) -> ::libc::c_int;
    pub fn grn_hash_get_key2(ctx: *mut grn_ctx, hash: *mut grn_hash,
                             id: grn_id, bulk: *mut grn_obj) -> ::libc::c_int;
    pub fn grn_hash_get_value(ctx: *mut grn_ctx, hash: *mut grn_hash,
                              id: grn_id, valuebuf: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn grn_hash_set_value(ctx: *mut grn_ctx, hash: *mut grn_hash,
                              id: grn_id, value: *const ::libc::c_void,
                              flags: ::libc::c_int) -> grn_rc;
    pub fn grn_hash_delete_by_id(ctx: *mut grn_ctx, hash: *mut grn_hash,
                                 id: grn_id,
                                 optarg: *mut grn_table_delete_optarg)
     -> grn_rc;
    pub fn grn_hash_delete(ctx: *mut grn_ctx, hash: *mut grn_hash,
                           key: *const ::libc::c_void,
                           key_size: ::libc::c_uint,
                           optarg: *mut grn_table_delete_optarg) -> grn_rc;
    pub fn grn_hash_cursor_open(ctx: *mut grn_ctx, hash: *mut grn_hash,
                                min: *const ::libc::c_void,
                                min_size: ::libc::c_uint,
                                max: *const ::libc::c_void,
                                max_size: ::libc::c_uint,
                                offset: ::libc::c_int, limit: ::libc::c_int,
                                flags: ::libc::c_int) -> *mut grn_hash_cursor;
    pub fn grn_hash_cursor_next(ctx: *mut grn_ctx, c: *mut grn_hash_cursor)
     -> grn_id;
    pub fn grn_hash_cursor_close(ctx: *mut grn_ctx, c: *mut grn_hash_cursor)
     -> ();
    pub fn grn_hash_cursor_get_key(ctx: *mut grn_ctx, c: *mut grn_hash_cursor,
                                   key: *mut *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn grn_hash_cursor_get_value(ctx: *mut grn_ctx,
                                     c: *mut grn_hash_cursor,
                                     value: *mut *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn grn_hash_cursor_set_value(ctx: *mut grn_ctx,
                                     c: *mut grn_hash_cursor,
                                     value: *const ::libc::c_void,
                                     flags: ::libc::c_int) -> grn_rc;
    pub fn grn_hash_cursor_get_key_value(ctx: *mut grn_ctx,
                                         c: *mut grn_hash_cursor,
                                         key: *mut *mut ::libc::c_void,
                                         key_size: *mut ::libc::c_uint,
                                         value: *mut *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn grn_hash_cursor_delete(ctx: *mut grn_ctx, c: *mut grn_hash_cursor,
                                  optarg: *mut grn_table_delete_optarg)
     -> grn_rc;
    pub fn grn_array_create(ctx: *mut grn_ctx, path: *const ::libc::c_char,
                            value_size: ::libc::c_uint, flags: ::libc::c_uint)
     -> *mut grn_array;
    pub fn grn_array_open(ctx: *mut grn_ctx, path: *const ::libc::c_char)
     -> *mut grn_array;
    pub fn grn_array_close(ctx: *mut grn_ctx, array: *mut grn_array)
     -> grn_rc;
    pub fn grn_array_add(ctx: *mut grn_ctx, array: *mut grn_array,
                         value: *mut *mut ::libc::c_void) -> grn_id;
    pub fn grn_array_push(ctx: *mut grn_ctx, array: *mut grn_array,
                          func:
                              ::std::option::Option<extern "C" fn(ctx:
                                                                      *mut grn_ctx,
                                                                  array:
                                                                      *mut grn_array,
                                                                  id: grn_id,
                                                                  func_arg:
                                                                      *mut ::libc::c_void)
                                                        -> ()>,
                          func_arg: *mut ::libc::c_void) -> grn_id;
    pub fn grn_array_pull(ctx: *mut grn_ctx, array: *mut grn_array,
                          blockp: grn_bool,
                          func:
                              ::std::option::Option<extern "C" fn(ctx:
                                                                      *mut grn_ctx,
                                                                  array:
                                                                      *mut grn_array,
                                                                  id: grn_id,
                                                                  func_arg:
                                                                      *mut ::libc::c_void)
                                                        -> ()>,
                          func_arg: *mut ::libc::c_void) -> grn_id;
    pub fn grn_array_unblock(ctx: *mut grn_ctx, array: *mut grn_array) -> ();
    pub fn grn_array_get_value(ctx: *mut grn_ctx, array: *mut grn_array,
                               id: grn_id, valuebuf: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn grn_array_set_value(ctx: *mut grn_ctx, array: *mut grn_array,
                               id: grn_id, value: *const ::libc::c_void,
                               flags: ::libc::c_int) -> grn_rc;
    pub fn grn_array_cursor_open(ctx: *mut grn_ctx, array: *mut grn_array,
                                 min: grn_id, max: grn_id,
                                 offset: ::libc::c_int, limit: ::libc::c_int,
                                 flags: ::libc::c_int)
     -> *mut grn_array_cursor;
    pub fn grn_array_cursor_next(ctx: *mut grn_ctx,
                                 cursor: *mut grn_array_cursor) -> grn_id;
    pub fn grn_array_cursor_get_value(ctx: *mut grn_ctx,
                                      cursor: *mut grn_array_cursor,
                                      value: *mut *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn grn_array_cursor_set_value(ctx: *mut grn_ctx,
                                      cursor: *mut grn_array_cursor,
                                      value: *const ::libc::c_void,
                                      flags: ::libc::c_int) -> grn_rc;
    pub fn grn_array_cursor_delete(ctx: *mut grn_ctx,
                                   cursor: *mut grn_array_cursor,
                                   optarg: *mut grn_table_delete_optarg)
     -> grn_rc;
    pub fn grn_array_cursor_close(ctx: *mut grn_ctx,
                                  cursor: *mut grn_array_cursor) -> ();
    pub fn grn_array_delete_by_id(ctx: *mut grn_ctx, array: *mut grn_array,
                                  id: grn_id,
                                  optarg: *mut grn_table_delete_optarg)
     -> grn_rc;
    pub fn grn_array_next(ctx: *mut grn_ctx, array: *mut grn_array,
                          id: grn_id) -> grn_id;
    pub fn _grn_array_get_value(ctx: *mut grn_ctx, array: *mut grn_array,
                                id: grn_id) -> *mut ::libc::c_void;
    pub fn grn_pat_create(ctx: *mut grn_ctx, path: *const ::libc::c_char,
                          key_size: ::libc::c_uint,
                          value_size: ::libc::c_uint, flags: ::libc::c_uint)
     -> *mut grn_pat;
    pub fn grn_pat_open(ctx: *mut grn_ctx, path: *const ::libc::c_char)
     -> *mut grn_pat;
    pub fn grn_pat_close(ctx: *mut grn_ctx, pat: *mut grn_pat) -> grn_rc;
    pub fn grn_pat_remove(ctx: *mut grn_ctx, path: *const ::libc::c_char)
     -> grn_rc;
    pub fn grn_pat_get(ctx: *mut grn_ctx, pat: *mut grn_pat,
                       key: *const ::libc::c_void, key_size: ::libc::c_uint,
                       value: *mut *mut ::libc::c_void) -> grn_id;
    pub fn grn_pat_add(ctx: *mut grn_ctx, pat: *mut grn_pat,
                       key: *const ::libc::c_void, key_size: ::libc::c_uint,
                       value: *mut *mut ::libc::c_void,
                       added: *mut ::libc::c_int) -> grn_id;
    pub fn grn_pat_get_key(ctx: *mut grn_ctx, pat: *mut grn_pat, id: grn_id,
                           keybuf: *mut ::libc::c_void,
                           bufsize: ::libc::c_int) -> ::libc::c_int;
    pub fn grn_pat_get_key2(ctx: *mut grn_ctx, pat: *mut grn_pat, id: grn_id,
                            bulk: *mut grn_obj) -> ::libc::c_int;
    pub fn grn_pat_get_value(ctx: *mut grn_ctx, pat: *mut grn_pat, id: grn_id,
                             valuebuf: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn grn_pat_set_value(ctx: *mut grn_ctx, pat: *mut grn_pat, id: grn_id,
                             value: *const ::libc::c_void,
                             flags: ::libc::c_int) -> grn_rc;
    pub fn grn_pat_delete_by_id(ctx: *mut grn_ctx, pat: *mut grn_pat,
                                id: grn_id,
                                optarg: *mut grn_table_delete_optarg)
     -> grn_rc;
    pub fn grn_pat_delete(ctx: *mut grn_ctx, pat: *mut grn_pat,
                          key: *const ::libc::c_void,
                          key_size: ::libc::c_uint,
                          optarg: *mut grn_table_delete_optarg) -> grn_rc;
    pub fn grn_pat_delete_with_sis(ctx: *mut grn_ctx, pat: *mut grn_pat,
                                   id: grn_id,
                                   optarg: *mut grn_table_delete_optarg)
     -> ::libc::c_int;
    pub fn grn_pat_scan(ctx: *mut grn_ctx, pat: *mut grn_pat,
                        str: *const ::libc::c_char, str_len: ::libc::c_uint,
                        sh: *mut grn_pat_scan_hit, sh_size: ::libc::c_uint,
                        rest: *mut *const ::libc::c_char) -> ::libc::c_int;
    pub fn grn_pat_prefix_search(ctx: *mut grn_ctx, pat: *mut grn_pat,
                                 key: *const ::libc::c_void,
                                 key_size: ::libc::c_uint, h: *mut grn_hash)
     -> grn_rc;
    pub fn grn_pat_suffix_search(ctx: *mut grn_ctx, pat: *mut grn_pat,
                                 key: *const ::libc::c_void,
                                 key_size: ::libc::c_uint, h: *mut grn_hash)
     -> grn_rc;
    pub fn grn_pat_lcp_search(ctx: *mut grn_ctx, pat: *mut grn_pat,
                              key: *const ::libc::c_void,
                              key_size: ::libc::c_uint) -> grn_id;
    pub fn grn_pat_size(ctx: *mut grn_ctx, pat: *mut grn_pat)
     -> ::libc::c_uint;
    pub fn grn_pat_cursor_open(ctx: *mut grn_ctx, pat: *mut grn_pat,
                               min: *const ::libc::c_void,
                               min_size: ::libc::c_uint,
                               max: *const ::libc::c_void,
                               max_size: ::libc::c_uint,
                               offset: ::libc::c_int, limit: ::libc::c_int,
                               flags: ::libc::c_int) -> *mut grn_pat_cursor;
    pub fn grn_pat_cursor_next(ctx: *mut grn_ctx, c: *mut grn_pat_cursor)
     -> grn_id;
    pub fn grn_pat_cursor_close(ctx: *mut grn_ctx, c: *mut grn_pat_cursor)
     -> ();
    pub fn grn_pat_cursor_get_key(ctx: *mut grn_ctx, c: *mut grn_pat_cursor,
                                  key: *mut *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn grn_pat_cursor_get_value(ctx: *mut grn_ctx, c: *mut grn_pat_cursor,
                                    value: *mut *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn grn_pat_cursor_get_key_value(ctx: *mut grn_ctx,
                                        c: *mut grn_pat_cursor,
                                        key: *mut *mut ::libc::c_void,
                                        key_size: *mut ::libc::c_uint,
                                        value: *mut *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn grn_pat_cursor_set_value(ctx: *mut grn_ctx, c: *mut grn_pat_cursor,
                                    value: *const ::libc::c_void,
                                    flags: ::libc::c_int) -> grn_rc;
    pub fn grn_pat_cursor_delete(ctx: *mut grn_ctx, c: *mut grn_pat_cursor,
                                 optarg: *mut grn_table_delete_optarg)
     -> grn_rc;
    pub fn grn_dat_scan(ctx: *mut grn_ctx, dat: *mut grn_dat,
                        str: *const ::libc::c_char, str_size: ::libc::c_uint,
                        scan_hits: *mut grn_dat_scan_hit,
                        max_num_scan_hits: ::libc::c_uint,
                        str_rest: *mut *const ::libc::c_char)
     -> ::libc::c_int;
    pub fn grn_dat_lcp_search(ctx: *mut grn_ctx, dat: *mut grn_dat,
                              key: *const ::libc::c_void,
                              key_size: ::libc::c_uint) -> grn_id;
    pub fn grn_dat_create(ctx: *mut grn_ctx, path: *const ::libc::c_char,
                          key_size: ::libc::c_uint,
                          value_size: ::libc::c_uint, flags: ::libc::c_uint)
     -> *mut grn_dat;
    pub fn grn_dat_open(ctx: *mut grn_ctx, path: *const ::libc::c_char)
     -> *mut grn_dat;
    pub fn grn_dat_close(ctx: *mut grn_ctx, dat: *mut grn_dat) -> grn_rc;
    pub fn grn_dat_remove(ctx: *mut grn_ctx, path: *const ::libc::c_char)
     -> grn_rc;
    pub fn grn_dat_get(ctx: *mut grn_ctx, dat: *mut grn_dat,
                       key: *const ::libc::c_void, key_size: ::libc::c_uint,
                       value: *mut *mut ::libc::c_void) -> grn_id;
    pub fn grn_dat_add(ctx: *mut grn_ctx, dat: *mut grn_dat,
                       key: *const ::libc::c_void, key_size: ::libc::c_uint,
                       value: *mut *mut ::libc::c_void,
                       added: *mut ::libc::c_int) -> grn_id;
    pub fn grn_dat_get_key(ctx: *mut grn_ctx, dat: *mut grn_dat, id: grn_id,
                           keybuf: *mut ::libc::c_void,
                           bufsize: ::libc::c_int) -> ::libc::c_int;
    pub fn grn_dat_get_key2(ctx: *mut grn_ctx, dat: *mut grn_dat, id: grn_id,
                            bulk: *mut grn_obj) -> ::libc::c_int;
    pub fn grn_dat_delete_by_id(ctx: *mut grn_ctx, dat: *mut grn_dat,
                                id: grn_id,
                                optarg: *mut grn_table_delete_optarg)
     -> grn_rc;
    pub fn grn_dat_delete(ctx: *mut grn_ctx, dat: *mut grn_dat,
                          key: *const ::libc::c_void,
                          key_size: ::libc::c_uint,
                          optarg: *mut grn_table_delete_optarg) -> grn_rc;
    pub fn grn_dat_update_by_id(ctx: *mut grn_ctx, dat: *mut grn_dat,
                                src_key_id: grn_id,
                                dest_key: *const ::libc::c_void,
                                dest_key_size: ::libc::c_uint) -> grn_rc;
    pub fn grn_dat_update(ctx: *mut grn_ctx, dat: *mut grn_dat,
                          src_key: *const ::libc::c_void,
                          src_key_size: ::libc::c_uint,
                          dest_key: *const ::libc::c_void,
                          dest_key_size: ::libc::c_uint) -> grn_rc;
    pub fn grn_dat_size(ctx: *mut grn_ctx, dat: *mut grn_dat)
     -> ::libc::c_uint;
    pub fn grn_dat_cursor_open(ctx: *mut grn_ctx, dat: *mut grn_dat,
                               min: *const ::libc::c_void,
                               min_size: ::libc::c_uint,
                               max: *const ::libc::c_void,
                               max_size: ::libc::c_uint,
                               offset: ::libc::c_int, limit: ::libc::c_int,
                               flags: ::libc::c_int) -> *mut grn_dat_cursor;
    pub fn grn_dat_cursor_next(ctx: *mut grn_ctx, c: *mut grn_dat_cursor)
     -> grn_id;
    pub fn grn_dat_cursor_close(ctx: *mut grn_ctx, c: *mut grn_dat_cursor)
     -> ();
    pub fn grn_dat_cursor_get_key(ctx: *mut grn_ctx, c: *mut grn_dat_cursor,
                                  key: *mut *const ::libc::c_void)
     -> ::libc::c_int;
    pub fn grn_dat_cursor_delete(ctx: *mut grn_ctx, c: *mut grn_dat_cursor,
                                 optarg: *mut grn_table_delete_optarg)
     -> grn_rc;
}
