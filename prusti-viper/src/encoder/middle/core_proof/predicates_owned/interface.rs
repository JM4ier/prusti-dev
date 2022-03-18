use crate::encoder::{
    errors::SpannedEncodingResult,
    high::types::HighTypeEncoderInterface,
    middle::core_proof::{
        compute_address::ComputeAddressInterface,
        lowerer::Lowerer,
        places::PlacesInterface,
        predicates_memory_block::PredicatesMemoryBlockInterface,
        snapshots::{IntoSnapshot, SnapshotsInterface},
        type_layouts::TypeLayoutsInterface,
        types::TypesInterface,
    },
};
use rustc_hash::FxHashSet;
use std::borrow::Cow;
use vir_crate::{common::expression::ExpressionIterator, low as vir_low, middle as vir_mid};

#[derive(Default)]
pub(in super::super) struct PredicatesOwnedState {
    unfolded_owned_non_aliased_predicates: FxHashSet<vir_mid::Type>,
}

pub(in super::super) trait Private {
    fn encode_owned_non_aliased(
        &mut self,
        ty: &vir_mid::Type,
    ) -> SpannedEncodingResult<vir_low::PredicateDecl>;
    fn encode_owned_non_aliased_with_fields<'a>(
        &mut self,
        ty: &vir_mid::Type,
        snapshot: vir_low::VariableDecl,
        snapshot_type: vir_low::Type,
        validity: vir_low::Expression,
        fields: impl Iterator<Item = Cow<'a, vir_mid::FieldDecl>>,
    ) -> SpannedEncodingResult<vir_low::PredicateDecl>;
}

impl<'p, 'v: 'p, 'tcx: 'v> Private for Lowerer<'p, 'v, 'tcx> {
    #[allow(unused_parens)] // Our macros require to put parenthesis around, but currently there is no way of putting this inside the macro.
    fn encode_owned_non_aliased(
        &mut self,
        ty: &vir_mid::Type,
    ) -> SpannedEncodingResult<vir_low::PredicateDecl> {
        self.encode_compute_address(ty)?;
        use vir_low::macros::*;
        let type_decl = self.encoder.get_type_decl_mid(ty)?;
        self.encode_snapshot_to_bytes_function(ty)?;
        let snapshot_type = ty.create_snapshot(self)?;
        var_decls! {
            place: Place,
            root_address: Address,
            snapshot: {snapshot_type.clone()}
        }
        let compute_address = ty!(Address);
        let to_bytes = ty! { Bytes };
        let validity = self.encode_snapshot_validity_expression(snapshot.clone().into(), ty)?;
        let size_of = self.encode_type_size_expression(ty)?;
        let compute_address = expr! { ComputeAddress::compute_address(place, root_address) };
        let bytes =
            self.encode_memory_block_bytes_expression(compute_address.clone(), size_of.clone())?;
        let predicate = match &type_decl {
            vir_mid::TypeDecl::Bool | vir_mid::TypeDecl::Int(_) | vir_mid::TypeDecl::Float(_) => {
                predicate! {
                    OwnedNonAliased<ty>(place: Place, root_address: Address, snapshot: {snapshot_type})
                    {(
                        ([validity]) &&
                        (acc(MemoryBlock([compute_address], [size_of]))) &&
                        (([bytes]) == (Snap<ty>::to_bytes(snapshot)))
                    )}
                }
            }
            // vir_mid::TypeDecl::TypeVar(TypeVar) => {},
            vir_mid::TypeDecl::Tuple(tuple_decl) => self.encode_owned_non_aliased_with_fields(
                ty,
                snapshot,
                snapshot_type,
                validity,
                tuple_decl.iter_fields(),
            )?,
            vir_mid::TypeDecl::Struct(struct_decl) => self.encode_owned_non_aliased_with_fields(
                ty,
                snapshot,
                snapshot_type,
                validity,
                struct_decl.iter_fields(),
            )?,
            vir_mid::TypeDecl::Enum(decl) => {
                let mut variant_predicates = Vec::new();
                for variant in &decl.variants {
                    let variant_ty = ty.clone().variant(variant.name.clone().into());
                    self.encode_owned_non_aliased(&variant_ty)?;
                    let acc = expr! {
                        acc(OwnedNonAliased<(&variant_ty)>(
                            place, root_address, snapshot
                        ))
                    };
                    variant_predicates.push(acc);
                }
                predicate! {
                    OwnedNonAliased<ty>(place: Place, root_address: Address, snapshot: {snapshot_type})
                    {(
                        ([validity]) &&
                        ([variant_predicates.into_iter().conjoin()])
                    )}
                }
            },
            // vir_mid::TypeDecl::Array(Array) => {},
            // vir_mid::TypeDecl::Reference(Reference) => {},
            // vir_mid::TypeDecl::Never => {},
            // vir_mid::TypeDecl::Closure(Closure) => {},
            // vir_mid::TypeDecl::Unsupported(Unsupported) => {},
            x => unimplemented!("{}", x),
        };
        Ok(predicate)
    }
    #[allow(unused_parens)] // Our macros require to put parenthesis around, but currently there is no way of putting this inside the macro.
    fn encode_owned_non_aliased_with_fields<'a>(
        &mut self,
        ty: &vir_mid::Type,
        snapshot: vir_low::VariableDecl,
        snapshot_type: vir_low::Type,
        validity: vir_low::Expression,
        fields: impl Iterator<Item = Cow<'a, vir_mid::FieldDecl>>,
    ) -> SpannedEncodingResult<vir_low::PredicateDecl> {
        use vir_low::macros::*;
        var_decls! {
            place: Place,
            root_address: Address
        }
        let mut field_predicates = Vec::new();
        for field in fields {
            let field_place =
                self.encode_field_place(ty, &field, place.clone().into(), Default::default())?;
            let field_value = self.encode_field_snapshot(
                ty,
                &field,
                snapshot.clone().into(),
                Default::default(),
            )?;
            let acc = expr! {
                acc(OwnedNonAliased<(&field.ty)>(
                    [field_place], root_address, [field_value]
                ))
            };
            field_predicates.push(acc);
        }
        if field_predicates.is_empty() {
            // TODO: Extract this into a separate method and deduplicate with
            // primitive type encoding.

            // FIXME: We should add an extra memory block not only for ZSTs, but
            // also for types to represent padding bytes and similar stuff.
            let compute_address = ty!(Address);
            let to_bytes = ty! { Bytes };
            let compute_address = expr! { ComputeAddress::compute_address(place, root_address) };
            let size_of = self.encode_type_size_expression(ty)?;
            let bytes = self
                .encode_memory_block_bytes_expression(compute_address.clone(), size_of.clone())?;
            field_predicates.push(expr! {
                (acc(MemoryBlock([compute_address], [size_of])))
            });
            field_predicates.push(expr! {
                (([bytes]) == (Snap<ty>::to_bytes(snapshot)))
            });
        }
        let predicate_decl = predicate! {
            OwnedNonAliased<ty>(place: Place, root_address: Address, snapshot: {snapshot_type})
            {(
                ([validity]) &&
                ([field_predicates.into_iter().conjoin()])
            )}
        };
        Ok(predicate_decl)
    }
}

pub(in super::super) trait PredicatesOwnedInterface {
    /// Marks that `OwnedNonAliased<ty>` was unfolded in the program and we need
    /// to provide its body.
    fn mark_owned_non_aliased_as_unfolded(
        &mut self,
        ty: &vir_mid::Type,
    ) -> SpannedEncodingResult<()>;
    fn collect_owned_predicate_decls(
        &mut self,
    ) -> SpannedEncodingResult<Vec<vir_low::PredicateDecl>>;
}

impl<'p, 'v: 'p, 'tcx: 'v> PredicatesOwnedInterface for Lowerer<'p, 'v, 'tcx> {
    fn mark_owned_non_aliased_as_unfolded(
        &mut self,
        ty: &vir_mid::Type,
    ) -> SpannedEncodingResult<()> {
        if !self
            .predicates_owned_state
            .unfolded_owned_non_aliased_predicates
            .contains(ty)
        {
            self.ensure_type_definition(ty)?;
            self.predicates_owned_state
                .unfolded_owned_non_aliased_predicates
                .insert(ty.clone());
        }
        Ok(())
    }
    fn collect_owned_predicate_decls(
        &mut self,
    ) -> SpannedEncodingResult<Vec<vir_low::PredicateDecl>> {
        let mut predicates = Vec::new();
        for ty in std::mem::take(
            &mut self
                .predicates_owned_state
                .unfolded_owned_non_aliased_predicates,
        ) {
            predicates.push(self.encode_owned_non_aliased(&ty)?);
        }
        Ok(predicates)
    }
}
