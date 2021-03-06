

#[ cfg (debug_assertions) ]
pub(crate) const DUMP_VERBOSE : bool = true;

#[ cfg (not (debug_assertions)) ]
pub(crate) const DUMP_VERBOSE : bool = false;


pub(crate) const DUMP_LOG_VERBOSE : bool = DUMP_VERBOSE || false;
pub(crate) const DUMP_LOG_LEVEL : u16 = if DUMP_LOG_VERBOSE { LOG_LEVEL_DEBUG } else { LOG_LEVEL_NOTICE };
pub(crate) const DUMP_LOG_CUT : bool = DUMP_LOG_VERBOSE || (DUMP_LOG_LEVEL <= LOG_LEVEL_DEBUG) || false;

pub(crate) const LOG_LEVEL_PANIC : u16 = u16::max_value ();
pub(crate) const LOG_LEVEL_ERROR : u16 = 60_000;
pub(crate) const LOG_LEVEL_WARNING : u16 = 50_000;
pub(crate) const LOG_LEVEL_NOTICE : u16 = 40_000;
pub(crate) const LOG_LEVEL_INFORMATION : u16 = 30_000;
pub(crate) const LOG_LEVEL_DEBUG : u16 = 20_000;
pub(crate) const LOG_LEVEL_TRACE : u16 = 10_000;
pub(crate) const LOG_LEVEL_DUMP : u16 = u16::min_value ();


pub(crate) const TEMPFILE_PREFIX : &str = ".z-exec.";
pub(crate) const TEMPFILE_SUFFIX : &str = ".tmp";
pub(crate) const TEMPFILE_TOKEN : usize = 16;


pub(crate) const RPC_BUFFER_SIZE : usize = 1024 * 1024;

