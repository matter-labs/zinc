mod push;
mod pop;
mod copy_global;
mod load_push;
mod pop_store;

pub use copy_global::CopyGlobal;
pub use push::PushConst;
pub use pop::Pop;
pub use load_push::LoadPush;
pub use pop_store::PopStore;
