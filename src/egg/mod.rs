pub mod comment;
pub mod encryption;
pub mod file;
pub mod magic;
pub mod option;

use comment::*;
use encryption::*;
use file::*;
use option::*;

pub struct EggArchive {
    header: EggHeader,
    options: Vec<EggOptionHeader>,
    files: Vec<EggFile>,
    comments: Vec<EggCommentHeader>,
}

pub struct EggHeader {
    magic: u32,
    version: u16,
    header_id: u32,
    reserved: u32,
}