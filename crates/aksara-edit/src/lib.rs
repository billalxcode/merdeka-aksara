//! Editing engine: selection, commands, transactions, undo/redo.
//!
//! Phase 3 over `aksara-model` (std only, no UI framework). Offsets are char
//! indices relative to the concatenated `Paragraph::text()` of one
//! paragraph. This phase covers single-run text paragraphs only;
//! multi-run style splits and tables/images are out of scope.

pub mod command;
pub mod selection;
pub mod transaction;
pub mod undo;

pub use command::EditCommand;
pub use selection::{DocPosition, Selection};
pub use transaction::{Transaction, dispatch};
pub use undo::UndoRedoManager;
