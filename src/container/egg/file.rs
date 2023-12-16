use crate::egg::header::{BlockHeader, CommentHeader, EncryptionHeader256, FileHeader, FilenameHeader, PosixFileInfo, WindowsFileInfo};

#[derive(Debug, Default)]
pub struct EggFile {
    pub file_header: FileHeader,
    pub filename_header: Option<FilenameHeader>,
    pub comment_header: Option<CommentHeader>,
    pub windows_file_info: Option<WindowsFileInfo>,
    pub posix_file_info: Option<PosixFileInfo>,
    pub encryption_header: Option<EncryptionHeader256>,

    pub block_header: BlockHeader,

    pub data: Vec<u8>,
}