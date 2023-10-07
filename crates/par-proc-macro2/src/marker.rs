use core::marker::PhantomData;
use core::panic::{RefUnwindSafe, UnwindSafe};
use std::sync::Arc;

// Zero sized marker with the correct set of autotrait impls we want all proc
// macro types to have.
pub(crate) type Marker = PhantomData<ProcMacroAutoTraits>;

pub(crate) use self::value::*;

mod value {
    pub(crate) use core::marker::PhantomData as Marker;
}

pub(crate) struct ProcMacroAutoTraits(Arc<()>);

impl UnwindSafe for ProcMacroAutoTraits {}
impl RefUnwindSafe for ProcMacroAutoTraits {}
