use crate::encoder::{errors::SpannedEncodingResult, mir::specifications::SpecificationsInterface};
use prusti_rustc_interface::{
    data_structures::graph::WithSuccessors,
    middle::{
        mir::{self, BasicBlock, TerminatorKind, START_BLOCK},
        ty,
    },
};
use std::collections::HashSet;

impl<'p, 'v: 'p, 'tcx: 'v> super::ProcedureEncoder<'p, 'v, 'tcx> {
    fn needs_termination(&self, bb: BasicBlock) -> bool {
        let function_termination = self.encoder.terminates(self.def_id, None);
        let ghost_block = self.specification_blocks.is_ghost_block(bb);
        function_termination || ghost_block
    }
    /// returns block that terminate, either by having no loops and function calls / or if termination of that is ensured by loop variants or termination measures on the call
    fn find_terminating_blocks(&self) -> SpannedEncodingResult<HashSet<BasicBlock>> {
        let mut queue = Vec::new();
        let mut terminates = HashSet::new();
        queue.push(START_BLOCK);
        while let Some(bb) = queue.pop() {
            if terminates.contains(&bb) {
                continue;
            }

            log::debug!(
                "analyzing termination of {:?}, {:?}",
                bb,
                self.mir.predecessors()[bb]
            );

            let all_pred_term = self.mir.predecessors()[bb]
                .iter()
                .all(|bb| terminates.contains(bb));

            let dom = self.mir.dominators();
            let non_dom_term = self.mir.predecessors()[bb]
                .iter()
                .all(|pred| dom.is_dominated_by(*pred, bb) || terminates.contains(pred));

            let has_loop_variant = self.mir.basic_blocks().iter_enumerated().any(|(bb1, _)| {
                let same_loop = bb == bb1 || dom.immediate_dominator(bb1) == bb;
                same_loop
                    && self
                        .specification_blocks
                        .loop_variant_blocks()
                        .contains_key(&bb1)
            });

            if all_pred_term || (non_dom_term && has_loop_variant) {
                let continues = match self.mir.basic_blocks()[bb].terminator().kind {
                    TerminatorKind::Call {
                        func: mir::Operand::Constant(box mir::Constant { literal, .. }),
                        ..
                    } => {
                        if let ty::TyKind::FnDef(def_id, call_substs) = literal.ty().kind() {
                            // TODO(jonas) disallow calls on trait functions as those act as a black box that could allow undetected (mutual) recursion
                            self.encoder.terminates(*def_id, Some(call_substs))
                        } else {
                            unimplemented!();
                        }
                    }
                    TerminatorKind::Call { .. } => unimplemented!(),
                    TerminatorKind::InlineAsm { .. } => false,
                    _ => true,
                };
                if continues {
                    terminates.insert(bb);
                    for bb in self.mir.successors(bb) {
                        queue.push(bb);
                    }
                }
            }
        }
        log::debug!("terminating blocks: {:?}", terminates);
        Ok(terminates)
    }
    pub fn encode_termination(&mut self) -> SpannedEncodingResult<HashSet<BasicBlock>> {
        let terminating = self.find_terminating_blocks()?;
        let mut needs_unreachability = HashSet::new();
        for (bb, _) in self.mir.basic_blocks().iter_enumerated() {
            if self.needs_termination(bb)
                && !terminating.contains(&bb)
                && self.reachable_blocks.contains(&bb)
            {
                // if a block needs termination but isn't guaranteed to terminate,
                // the only valid possibility is if the block is never entered.
                needs_unreachability.insert(bb);
            }
        }

        Ok(needs_unreachability)
    }
}
