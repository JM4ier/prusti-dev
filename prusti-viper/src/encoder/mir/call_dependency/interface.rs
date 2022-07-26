use prusti_interface::{self};
use prusti_rustc_interface::{
    hir::def_id::DefId,
    middle::{
        mir::{self, TerminatorKind},
        ty,
        ty::subst::SubstsRef,
    },
};
use rustc_hash::{FxHashMap, FxHashSet};
use std::cell::RefCell;

pub(crate) trait CallDependencyInterface<'tcx> {
    fn calls(&self, caller: DefId, callee: DefId, call_substs: SubstsRef<'tcx>) -> bool;
}

impl<'v, 'tcx: 'v> CallDependencyInterface<'tcx> for super::super::super::Encoder<'v, 'tcx> {
    fn calls(&self, caller: DefId, callee: DefId, call_substs: SubstsRef<'tcx>) -> bool {
        todo!()
    }
}
