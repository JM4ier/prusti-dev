use crate::encoder::{
    errors::{ErrorCtxt, SpannedEncodingError, SpannedEncodingResult},
    mir::{
        errors::ErrorInterface, places::PlacesEncoderInterface,
        pure::SpecificationEncoderInterface, spans::SpanInterface,
        specifications::SpecificationsInterface, type_layouts::MirTypeLayoutsEncoderInterface,
    },
};
use prusti_rustc_interface::{
    data_structures::graph::WithSuccessors,
    middle::mir::{BasicBlock, BasicBlockData, RetagKind, StatementKind, TerminatorKind},
};
use std::collections::HashSet;
use vir_crate::high::{self as vir_high};

impl<'p, 'v: 'p, 'tcx: 'v> super::ProcedureEncoder<'p, 'v, 'tcx> {
    fn needs_termination(&self, bb: BasicBlock) -> bool {
        let function_termination = self.encoder.terminates(self.def_id, None);
        let ghost_block = self.specification_blocks.is_ghost_block(bb);
        function_termination || ghost_block
    }
    fn find_entry_point(&self) -> BasicBlock {
        self.mir
            .basic_blocks()
            .iter_enumerated()
            .filter(|(_, bb)| {
                bb.statements
                    .iter()
                    .any(|s| matches!(s.kind, StatementKind::Retag(RetagKind::FnEntry, _)))
            })
            .map(|(bb, _)| bb)
            .next()
            .unwrap()
    }
    fn find_terminating_blocks(&self) -> HashSet<BasicBlock> {
        let mut queue = Vec::new();
        let mut terminates = HashSet::new();
        queue.push(self.find_entry_point());
        while let Some(bb) = queue.pop() {
            if terminates.contains(&bb) {
                continue;
            }

            if self.mir.predecessors()[bb]
                .iter()
                .all(|bb| terminates.contains(bb))
            // additional condition: predecessors *excluding* back edges terminate, and there is a loop variant in the loop
            {
                terminates.insert(bb);
                let continues = match self.mir.basic_blocks()[bb].terminator().kind {
                    TerminatorKind::Call { .. } => todo!("check if function is pure"),
                    TerminatorKind::InlineAsm { .. } => false,
                    _ => true,
                };
                if continues {
                    for bb in self.mir.successors(bb) {
                        queue.push(bb);
                    }
                }
            }
        }
        terminates
    }
    pub fn encode_termination(&mut self) -> SpannedEncodingResult<()> {
        let terminating = self.find_terminating_blocks();
        for (bb, _) in self.mir.basic_blocks().iter_enumerated() {
            if self.needs_termination(bb) && !terminating.contains(&bb) {
                /// if a block needs termination but isn't guaranteed to terminate,
                /// the only valid possibility is if the block is never entered.
                ///
                /// TODO: insert `assert false` at the begin of the block
                todo!()
            }
        }
        todo!()
    }
}
