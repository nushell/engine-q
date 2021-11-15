mod capitalize;
mod case;
mod collect;
mod contains;
mod downcase;
mod ends_with;
mod find_replace;
mod index_of;
mod length;
mod lpad;
mod rpad;
mod starts_with;

pub use capitalize::SubCommand as StrCapitalize;
pub use case::*;
pub use collect::*;
pub use contains::SubCommand as StrContains;
pub use downcase::SubCommand as StrDowncase;
pub use ends_with::SubCommand as StrEndswith;
pub use find_replace::SubCommand as StrFindReplace;
pub use index_of::SubCommand as StrIndexOf;
pub use length::SubCommand as StrLength;
pub use lpad::SubCommand as StrLpad;
pub use rpad::SubCommand as StrRpad;
pub use starts_with::SubCommand as StrStartsWith;
