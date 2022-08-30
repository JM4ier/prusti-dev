use crate::encoder::{
    errors::{ErrorCtxt, SpannedEncodingResult},
    mir::{
        errors::ErrorInterface, places::PlacesEncoderInterface,
        pure::SpecificationEncoderInterface, spans::SpanInterface,
        specifications::SpecificationsInterface, type_layouts::MirTypeLayoutsEncoderInterface,
    },
};
use prusti_interface::specs::typed::LoopSpecification;
use prusti_rustc_interface::middle::mir;
use vir_crate::high::{self as vir_high, BasicBlockId};

impl<'p, 'v: 'p, 'tcx: 'v> super::ProcedureEncoder<'p, 'v, 'tcx> {
    /// Encode loop invariant and loop variants
    pub(super) fn encode_loop_specs(
        &mut self,
        loop_head: mir::BasicBlock,
        invariant_block: mir::BasicBlock,
        specification_blocks: Vec<mir::BasicBlock>,
    ) -> SpannedEncodingResult<vir_high::Statement> {
        let invariant_location = mir::Location {
            block: invariant_block,
            statement_index: self.mir[invariant_block].statements.len(),
        };
        // Encode functional specification.
        let mut encoded_invariant_specs = Vec::new();
        let mut encoded_variant_specs = Vec::new();

        for block in specification_blocks {
            for statement in &self.mir[block].statements {
                if let mir::StatementKind::Assign(box (
                    _,
                    mir::Rvalue::Aggregate(
                        box mir::AggregateKind::Closure(cl_def_id, cl_substs),
                        _,
                    ),
                )) = statement.kind
                {
                    let specification = self.encoder.get_loop_specs(cl_def_id).unwrap();
                    let (spec, encoding_vec, err_ctxt) = match specification {
                        LoopSpecification::Invariant(inv) => {
                            (inv, &mut encoded_invariant_specs, ErrorCtxt::LoopInvariant)
                        }
                        LoopSpecification::Variant(var) => {
                            (var, &mut encoded_variant_specs, ErrorCtxt::LoopVariant)
                        }
                    };
                    let span = self.encoder.get_definition_span(spec.to_def_id());
                    let encoded_specification = self.encoder.set_expression_error_ctxt(
                        self.encoder.encode_loop_spec_high(
                            self.mir,
                            block,
                            self.def_id,
                            cl_substs,
                        )?,
                        span,
                        err_ctxt,
                        self.def_id,
                    );
                    encoding_vec.push(encoded_specification);
                }
            }
        }
        let encoded_back_edges: Vec<BasicBlockId> = {
            let predecessors = self.mir.predecessors();
            let dominators = self.mir.dominators();
            predecessors[loop_head]
                .iter()
                .filter(|predecessor| dominators.is_dominated_by(**predecessor, loop_head))
                .map(|back_edge| self.encode_basic_block_label(*back_edge))
                .collect()
        };
        self.init_data.seek_before(invariant_location);

        // Encode permissions.
        let initialized_places = self.initialization.get_after_statement(invariant_location);
        let allocated_locals = self.allocation.get_after_statement(invariant_location);
        let (written_places, mutably_borrowed_places, _) = self
            .procedure
            .loop_info()
            .compute_read_and_write_leaves(loop_head, self.mir, None);

        let mut maybe_modified_places = Vec::new();
        for place in written_places.into_iter().chain(mutably_borrowed_places) {
            if initialized_places.contains_prefix_of(place) {
                maybe_modified_places.push(vir_high::Predicate::owned_non_aliased_no_pos(
                    self.encoder.encode_place_high(self.mir, place, None)?,
                ));
            } else if allocated_locals.contains_prefix_of(place) {
                let mir_type = place.ty(self.mir, self.encoder.env().tcx()).ty;
                let size = self.encoder.encode_type_size_expression(mir_type)?;
                maybe_modified_places.push(vir_high::Predicate::memory_block_stack_no_pos(
                    self.encoder.encode_place_high(self.mir, place, None)?,
                    size,
                ));
            }
        }

        // Construct the variant info.
        let loop_variant = encoded_variant_specs.into_iter().next().map(|spec| {
            let var = self.fresh_ghost_variable(
                "loop_variant",
                vir_high::Type::Int(vir_high::ty::Int::Unbounded),
            );
            vir_high::ast::statement::LoopVariant { var, expr: spec }
        });

        // Construct the invariant info.
        let loop_invariant = vir_high::Statement::loop_invariant_no_pos(
            self.encode_basic_block_label(loop_head),
            encoded_back_edges,
            maybe_modified_places,
            encoded_invariant_specs,
            loop_variant,
        );
        let invariant_statement = self.set_statement_error(
            invariant_location,
            ErrorCtxt::UnexpectedStorageLive,
            loop_invariant,
        )?;

        Ok(invariant_statement)
    }
}
