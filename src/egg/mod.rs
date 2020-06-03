pub mod block;
pub mod comment;
pub mod encryption;
pub mod file;
pub mod header;

use block::*;
use comment::*;
use encryption::*;
use file::*;
use header::*;

pub struct EggArchive {
    header: EggHeader,
    optional_header: Vec<EggOptionalHeader>,
    files: EggFile,
    comments: Vec<EggCommentHeader>,
}