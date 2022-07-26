use prusti_interface::{self, environment::Environment};
use prusti_rustc_interface::{
    hir::def_id::DefId,
    middle::{
        mir::{self, TerminatorKind},
        ty,
    },
};
use rustc_hash::{FxHashMap, FxHashSet};
use std::cell::RefCell;

// FIXME? take substs into account

pub(crate) trait CallDependencyInterface<'tcx> {
    fn get_direct_callees(&self, proc_def_id: DefId) -> FxHashSet<DefId>;
    fn get_all_callees(&self, proc_def_id: DefId) -> FxHashSet<DefId>;
}

#[derive(Default)]
pub(crate) struct CallDependencyState {
    cache: RefCell<Cache>,
}

#[derive(Default)]
struct Cache {
    direct_callees: FxHashMap<DefId, FxHashSet<DefId>>,
    all_callees: FxHashMap<DefId, FxHashSet<DefId>>,
}

impl Cache {
    fn direct_callees(&mut self, env: &Environment, proc: DefId) -> FxHashSet<DefId> {
        if self.direct_callees.contains_key(&proc) {
            return self.direct_callees.get(&proc).cloned().unwrap();
        }
        let procedure = env.get_procedure(proc);
        let body = procedure.get_mir();
        let mut set = FxHashSet::default();
        for block in body.basic_blocks() {
            match block.terminator().kind {
                TerminatorKind::Call {
                    func: mir::Operand::Constant(box mir::Constant { literal, .. }),
                    ..
                } => {
                    if let ty::TyKind::FnDef(def_id, _) = literal.ty().kind() {
                        set.insert(*def_id);
                    } else {
                        unimplemented!();
                    }
                }
                TerminatorKind::Call { .. } => unimplemented!(),
                _ => {}
            }
        }
        self.direct_callees.insert(proc, set);
        self.direct_callees(env, proc)
    }
    fn all_callees(&mut self, env: &Environment, proc: DefId) -> FxHashSet<DefId> {
        if self.all_callees.contains_key(&proc) {
            return self.all_callees.get(&proc).cloned().unwrap();
        }

        let mut queue = Vec::new();
        let mut visited = FxHashSet::default();
        let mut result = FxHashSet::default();

        queue.push(proc);
        visited.insert(proc);
        while let Some(p) = queue.pop() {
            let callees = self.direct_callees(env, p);
            for p in callees.into_iter() {
                result.insert(p);
                if !visited.contains(&p) {
                    queue.push(p);
                    visited.insert(p);
                }
            }
        }

        self.all_callees.insert(proc, result);
        self.all_callees(env, proc)
    }
}

impl<'v, 'tcx: 'v> CallDependencyInterface<'tcx> for super::super::super::Encoder<'v, 'tcx> {
    fn get_direct_callees(&self, proc_def_id: DefId) -> FxHashSet<DefId> {
        let mut cache = self.call_dependency_state.cache.borrow_mut();
        cache.direct_callees(self.env(), proc_def_id)
    }
    fn get_all_callees(&self, proc_def_id: DefId) -> FxHashSet<DefId> {
        let mut cache = self.call_dependency_state.cache.borrow_mut();
        cache.all_callees(self.env(), proc_def_id)
    }
}
