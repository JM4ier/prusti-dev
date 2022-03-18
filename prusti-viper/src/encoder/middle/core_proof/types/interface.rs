use crate::encoder::{
    errors::SpannedEncodingResult,
    high::types::HighTypeEncoderInterface,
    middle::core_proof::{
        adts::{AdtConstructor, AdtsInterface},
        lowerer::{DomainsLowererInterface, Lowerer},
        snapshots::{IntoSnapshot, SnapshotsInterface},
    },
};
use std::collections::BTreeSet;
use vir_crate::{
    common::{
        expression::{ExpressionIterator, QuantifierHelpers},
        identifier::WithIdentifier,
    },
    low::{self as vir_low, operations::ToLow},
    middle as vir_mid,
};

#[derive(Default)]
pub(in super::super) struct TypesState {
    /// The types for which the definitions were ensured.
    ensured_definitions: BTreeSet<vir_mid::Type>,
}

// struct LoweredVariant {
//     constructor: AdtConstructor,
//     /// `None` means that this contructor should not get a validity axiom.
//     /// Therfore, `None` is different from `true` that means that this type does
//     /// not have additional contrains compared to the ones implied by its
//     /// subtypes.
//     validity: Option<vir_low::Expression>,
//     /// A constructor is representational if it can be used for constructing any
//     /// valid instance of the ADT.
//     is_representational: bool,
// }

// #[derive(Default)]
// struct LoweredVariants {
//     variants: Vec<LoweredVariant>,
// }

// impl LoweredVariants {
//     fn add_constant_with_inv(
//         &mut self,
//         parameter_type: vir_low::Type,
//         invariant: vir_low::Expression,
//         is_representational: bool,
//     ) {
//         self.variants.push(LoweredVariant {
//             constructor: AdtConstructor::constant(parameter_type),
//             validity: Some(invariant),
//             is_representational,
//         });
//     }
//     fn add_struct_with_inv(
//         &mut self,
//         parameters: Vec<vir_low::VariableDecl>,
//         invariant: vir_low::Expression,
//     ) {
//         self.variants.push(LoweredVariant {
//             constructor: AdtConstructor::struct_main(parameters),
//             validity: Some(invariant),
//             is_representational: false,
//         });
//     }
//     fn add_struct_alternative_no_inv(
//         &mut self,
//         variant: impl ToString,
//         parameters: Vec<vir_low::VariableDecl>,
//     ) {
//         self.variants.push(LoweredVariant {
//             constructor: AdtConstructor::struct_alternative(variant.to_string(), parameters),
//             validity: None,
//             is_representational: false,
//         });
//     }
//     fn add_enum_variant_with_inv(
//         &mut self,
//         variant: impl ToString,
//         variant_type: vir_low::Type,
//         invariant: vir_low::Expression,
//     ) {
//         self.variants.push(LoweredVariant {
//             constructor: AdtConstructor::enum_variant(variant.to_string(), variant_type),
//             validity: Some(invariant),
//             is_representational: false,
//         });
//     }
//     fn iter_constructors(&self) -> impl Iterator<Item = &AdtConstructor> {
//         self.variants.iter().map(|variant| &variant.constructor)
//     }
//     fn iter_variants(&self) -> impl Iterator<Item = &LoweredVariant> {
//         self.variants.iter()
//     }
// }

const BOOLEAN_OPERATORS: &[vir_low::BinaryOpKind] = &[
    vir_low::BinaryOpKind::EqCmp,
    vir_low::BinaryOpKind::And,
    vir_low::BinaryOpKind::Or,
    vir_low::BinaryOpKind::Implies,
];

const COMPARISON_OPERATORS: &[vir_low::BinaryOpKind] = &[
    vir_low::BinaryOpKind::EqCmp,
    vir_low::BinaryOpKind::NeCmp,
    vir_low::BinaryOpKind::GtCmp,
    vir_low::BinaryOpKind::GeCmp,
    vir_low::BinaryOpKind::LtCmp,
    vir_low::BinaryOpKind::LeCmp,
];
const ARITHMETIC_OPERATORS: &[vir_low::BinaryOpKind] = &[
    vir_low::BinaryOpKind::Add,
    vir_low::BinaryOpKind::Sub,
    vir_low::BinaryOpKind::Mul,
    vir_low::BinaryOpKind::Div,
    vir_low::BinaryOpKind::Mod,
];
const INTEGER_SIZES: &[vir_mid::ty::Int] = &[
    vir_mid::ty::Int::I8,
    vir_mid::ty::Int::I16,
    vir_mid::ty::Int::I32,
    vir_mid::ty::Int::I64,
    vir_mid::ty::Int::I128,
    vir_mid::ty::Int::Isize,
    vir_mid::ty::Int::U8,
    vir_mid::ty::Int::U16,
    vir_mid::ty::Int::U32,
    vir_mid::ty::Int::U64,
    vir_mid::ty::Int::U128,
    vir_mid::ty::Int::Usize,
    vir_mid::ty::Int::Char,
    vir_mid::ty::Int::Unbounded,
];

trait Private {
    fn register_constant_constructor(
        &mut self,
        domain_name: &str,
        parameter_type: vir_low::Type,
    ) -> SpannedEncodingResult<()>;
    fn register_struct_constructor(
        &mut self,
        domain_name: &str,
        parameters: Vec<vir_low::VariableDecl>,
    ) -> SpannedEncodingResult<()>;
    fn register_alternative_constructor(
        &mut self,
        domain_name: &str,
        variant_name: &str,
        parameters: Vec<vir_low::VariableDecl>,
    ) -> SpannedEncodingResult<()>;
    fn encode_validity_axioms_primitive(
        &mut self,
        domain_name: &str,
        parameter_type: vir_low::Type,
        invariant: vir_low::Expression,
    ) -> SpannedEncodingResult<()>;
    fn encode_validity_axioms_struct(
        &mut self,
        domain_name: &str,
        parameters: Vec<vir_low::VariableDecl>,
        invariant: vir_low::Expression,
    ) -> SpannedEncodingResult<()>;
    fn ensure_type_definition_for_decl(
        &mut self,
        ty: &vir_mid::Type,
        type_decl: &vir_mid::TypeDecl,
    ) -> SpannedEncodingResult<()>;

    // fn compute_adt_constructors(
    //     &mut self,
    //     ty: &vir_mid::Type,
    //     type_decl: &vir_mid::TypeDecl,
    // ) -> SpannedEncodingResult<LoweredVariants>;
    // fn declare_validity_axioms<'a>(
    //     &mut self,
    //     ty: &vir_mid::Type,
    //     variants: impl Iterator<Item = &'a LoweredVariant>,
    // ) -> SpannedEncodingResult<()>;
    // fn declare_injectivity_axioms_for_representational_constructors<'a>(
    //     &mut self,
    //     ty: &vir_mid::Type,
    //     variants: impl Iterator<Item = &'a LoweredVariant>,
    // ) -> SpannedEncodingResult<()>;
    fn declare_simplification_axiom(
        &mut self,
        ty: &vir_mid::Type,
        variant: &str,
        parameters: Vec<vir_low::VariableDecl>,
        parameter_type: &vir_mid::Type,
        simplification_result: vir_low::Expression,
    ) -> SpannedEncodingResult<()>;
    fn declare_simplification_axioms(&mut self, ty: &vir_mid::Type) -> SpannedEncodingResult<()>;
}

fn valid_call(
    domain_name: &str,
    variable: &vir_low::VariableDecl,
) -> SpannedEncodingResult<vir_low::Expression> {
    Ok(vir_low::Expression::domain_function_call(
        domain_name,
        format!("valid${}", domain_name),
        vec![variable.clone().into()],
        vir_low::Type::Bool,
    ))
}

impl<'p, 'v: 'p, 'tcx: 'v> Private for Lowerer<'p, 'v, 'tcx> {
    // TODO: Move to SnapshotADTs
    fn register_constant_constructor(
        &mut self,
        domain_name: &str,
        parameter_type: vir_low::Type,
    ) -> SpannedEncodingResult<()> {
        use vir_low::macros::*;
        self.adt_register_main_constructor(
            domain_name,
            vars! { value: {parameter_type}},
            Some(valid_call),
        )
    }
    fn register_struct_constructor(
        &mut self,
        domain_name: &str,
        parameters: Vec<vir_low::VariableDecl>,
    ) -> SpannedEncodingResult<()> {
        self.adt_register_main_constructor(domain_name, parameters, Some(valid_call))
    }
    fn register_alternative_constructor(
        &mut self,
        domain_name: &str,
        variant_name: &str,
        parameters: Vec<vir_low::VariableDecl>,
    ) -> SpannedEncodingResult<()> {
        self.adt_register_variant_constructor(
            domain_name,
            variant_name,
            parameters,
            Some(valid_call),
        )
    }
    fn encode_validity_axioms_primitive(
        &mut self,
        domain_name: &str,
        parameter_type: vir_low::Type,
        invariant: vir_low::Expression,
    ) -> SpannedEncodingResult<()> {
        use vir_low::macros::*;
        let parameters = vars! { value: {parameter_type}};
        self.encode_validity_axioms_struct(domain_name, parameters, invariant)
    }
    fn encode_validity_axioms_struct(
        &mut self,
        domain_name: &str,
        parameters: Vec<vir_low::VariableDecl>,
        invariant: vir_low::Expression,
    ) -> SpannedEncodingResult<()> {
        use vir_low::macros::*;
        let mut valid_parameters = Vec::new();
        let variant_name = ""; // FIXME
        for parameter in &parameters {
            if let vir_low::Type::Domain(parameter_domain) = &parameter.ty {
                valid_parameters.push(valid_call(&parameter_domain.name, parameter)?);
            }
        }
        let constructor_call = self.adt_constructor_variant_call(
            domain_name,
            variant_name,
            parameters
                .iter()
                .map(|parameter| parameter.clone().into())
                .collect(),
        )?;
        let valid_constructor =
            self.encode_snapshot_validity_expression(domain_name, constructor_call)?;
        if parameters.is_empty() {
            let axiom = vir_low::DomainAxiomDecl {
                name: format!("{}$validity_axiom_bottom_up", domain_name),
                body: expr! { [ valid_constructor ] == [ invariant ] },
            };
            self.declare_axiom(domain_name, axiom)?;
            return Ok(());
        }
        let mut conjuncts = vec![invariant]; // FIXME: We need to replace the fields here.
        conjuncts.extend(valid_parameters.clone());
        let validity_expression = conjuncts.into_iter().conjoin();
        if parameters.iter().any(|parameter| parameter.ty.is_domain()) {
            // The top-down axiom allows proving that any of the fields is valid
            // if we know that the whole data strucure is valid. With no
            // parameters, the bottom-up and top-down axioms are equivalent.
            let mut top_down_validity_expression = validity_expression.clone();
            var_decls! { snapshot: {vir_low::Type::domain(domain_name.to_string())}};
            let valid_constructor =
                self.encode_snapshot_validity_expression(domain_name, snapshot.clone().into())?;
            let mut triggers = Vec::new();
            for parameter in &parameters {
                if let vir_low::Type::Domain(parameter_domain) = &parameter.ty {
                    let field = self.snapshot_destructor_struct_call(
                        &domain_name,
                        &parameter.name,
                        parameter.ty.clone(),
                        snapshot.clone().into(),
                    )?;
                    top_down_validity_expression = top_down_validity_expression
                        .replace_place(&parameter.clone().into(), &field);
                    let valid_field =
                        self.encode_snapshot_validity_expression(&parameter_domain.name, field)?;
                    triggers.push(vir_low::Trigger::new(vec![
                        valid_constructor.clone(),
                        valid_field.clone(),
                    ]));
                }
            }
            let axiom_top_down_body = vir_low::Expression::forall(
                vec![snapshot],
                triggers,
                expr! {
                    [ valid_constructor ] == [ top_down_validity_expression ]
                },
            );
            let axiom_top_down = vir_low::DomainAxiomDecl {
                name: format!("{}$validity_axiom_top_down", domain_name),
                body: axiom_top_down_body,
            };
            self.declare_axiom(&domain_name, axiom_top_down)?;
        }
        let axiom_bottom_up_body = {
            let mut trigger = vec![valid_constructor.clone()];
            trigger.extend(valid_parameters.clone());
            vir_low::Expression::forall(
                parameters,
                vec![vir_low::Trigger::new(trigger)],
                expr! {
                    [ valid_constructor ] == [ validity_expression ]
                },
            )
        };
        // The axiom that allows proving that the data structure is
        // valid if we know that its fields are valid.
        let axiom_bottom_up = vir_low::DomainAxiomDecl {
            name: format!("{}$validity_axiom_bottom_up", domain_name),
            body: axiom_bottom_up_body,
        };
        self.declare_axiom(domain_name, axiom_bottom_up)?;
        Ok(())
    }
    fn ensure_type_definition_for_decl(
        &mut self,
        ty: &vir_mid::Type,
        type_decl: &vir_mid::TypeDecl,
    ) -> SpannedEncodingResult<()> {
        use vir_low::macros::*;
        let domain_name = self.encode_snapshot_domain_name(ty)?;
        // let mut constructors = LoweredVariants::default();
        match type_decl {
            vir_mid::TypeDecl::Bool => {
                self.register_constant_constructor(&domain_name, vir_low::Type::Bool)?;
                let snapshot_type = ty.create_snapshot(self)?;
                let variant_name =
                    self.encode_unary_op_variant(vir_low::UnaryOpKind::Not, &vir_mid::Type::Bool)?;
                self.register_alternative_constructor(
                    &domain_name,
                    &variant_name,
                    vars! { argument: {snapshot_type.clone()} },
                )?;

                // constructors.add_constant_with_inv(vir_low::Type::Bool, true.into(), true);
                // let snapshot_type = (vir_mid::Type::Bool).create_snapshot(self)?;
                // constructors.add_struct_alternative_no_inv(
                //     &self
                //         .encode_unary_op_variant(vir_low::UnaryOpKind::Not, &vir_mid::Type::Bool)?,
                //     vars! { argument: {snapshot_type.clone()} },
                // );
                for op in BOOLEAN_OPERATORS {
                    // FIXME: Make these on demand.
                    let variant_name = self.encode_binary_op_variant(*op, &vir_mid::Type::Bool)?;
                    self.register_alternative_constructor(
                        &domain_name,
                        &variant_name,
                        vars! { left: {snapshot_type.clone()}, right: {snapshot_type.clone()} },
                    )?;
                    // constructors.add_struct_alternative_no_inv(
                    //     self.encode_binary_op_variant(*op, &vir_mid::Type::Bool)?,
                    //     vars! { left: {snapshot_type.clone()}, right: {snapshot_type.clone()} },
                    // );
                }
                for size in INTEGER_SIZES {
                    let int_ty = vir_mid::Type::Int(*size);
                    let snapshot_type = int_ty.create_snapshot(self)?;
                    for op in COMPARISON_OPERATORS {
                        // FIXME: Make these on demand.
                        let variant_name = self.encode_binary_op_variant(*op, &int_ty)?;
                        self.register_alternative_constructor(
                            &domain_name,
                            &variant_name,
                            vars! { left: {snapshot_type.clone()}, right: {snapshot_type.clone()} },
                        )?;
                        // constructors.add_struct_alternative_no_inv(
                        //     self.encode_binary_op_variant(*op, &int_ty)?,
                        //     vars! { left: {snapshot_type.clone()}, right: {snapshot_type.clone()} },
                        // );
                    }
                }

                self.encode_validity_axioms_primitive(
                    &domain_name,
                    vir_low::Type::Bool,
                    true.into(),
                )?;
            }
            vir_mid::TypeDecl::Int(decl) => {
                self.register_constant_constructor(&domain_name, vir_low::Type::Int)?;
                // constructors.add_constant_with_inv(vir_low::Type::Int, validity, true);
                let snapshot_type = ty.create_snapshot(self)?;
                let variant_name = self.encode_unary_op_variant(vir_low::UnaryOpKind::Minus, ty)?;
                self.register_alternative_constructor(
                    &domain_name,
                    &variant_name,
                    vars! { argument: {snapshot_type.clone()} },
                )?;
                // constructors.add_struct_alternative_no_inv(
                //     &self.encode_unary_op_variant(vir_low::UnaryOpKind::Minus, ty)?,
                //     vars! { argument: {snapshot_type.clone()} },
                // );
                for op in ARITHMETIC_OPERATORS {
                    // FIXME: Make these on demand.
                    let variant_name = self.encode_binary_op_variant(*op, ty)?;
                    self.register_alternative_constructor(
                        &domain_name,
                        &variant_name,
                        vars! { left: {snapshot_type.clone()}, right: {snapshot_type.clone()} },
                    )?;
                    // constructors.add_struct_alternative_no_inv(
                    //     &self.encode_binary_op_variant(*op, ty)?,
                    //     vars! { left: {snapshot_type.clone()}, right: {snapshot_type.clone()} },
                    // );
                }
                var_decls! { value: Int };
                let mut conjuncts = Vec::new();
                if let Some(lower_bound) = &decl.lower_bound {
                    conjuncts.push(expr! { [lower_bound.clone().to_low(self)? ] <= value });
                }
                if let Some(upper_bound) = &decl.upper_bound {
                    conjuncts.push(expr! { value <= [upper_bound.clone().to_low(self)? ] });
                }
                let validity = conjuncts.into_iter().conjoin();
                self.encode_validity_axioms_primitive(&domain_name, vir_low::Type::Int, validity)?;
            }
            vir_mid::TypeDecl::Tuple(decl) => {
                let mut parameters = Vec::new();
                for field in decl.iter_fields() {
                    parameters.push(vir_low::VariableDecl::new(
                        field.name.clone(),
                        field.ty.create_snapshot(self)?,
                    ));
                }
                self.register_struct_constructor(&domain_name, parameters.clone())?;
                self.encode_validity_axioms_struct(&domain_name, parameters, true.into())?;
                // constructors.add_struct_with_inv(parameters, true.into());
            }
            vir_mid::TypeDecl::Struct(decl) => {
                let mut parameters = Vec::new();
                for field in decl.iter_fields() {
                    parameters.push(vir_low::VariableDecl::new(
                        field.name.clone(),
                        field.ty.create_snapshot(self)?,
                    ));
                }
                self.register_struct_constructor(&domain_name, parameters.clone())?;
                self.encode_validity_axioms_struct(&domain_name, parameters, true.into())?;
                // constructors.add_struct_with_inv(parameters, true.into());
            }
            vir_mid::TypeDecl::Enum(decl) => {
                // for variant in &decl.variants {
                //     let variant_type = ty.clone().variant(variant.name.clone().into());
                //     let variant_type_snapshot = variant_type.create_snapshot(self)?;
                //     constructors.add_enum_variant_with_inv(
                //         &variant.name,
                //         variant_type_snapshot,
                //         true.into(),
                //     );
                //     self.ensure_type_definition(&variant_type)?;
                // }
            }
            _ => unimplemented!("type: {:?}", type_decl),
        };
        Ok(())
    }

    // fn compute_adt_constructors(
    //     &mut self,
    //     ty: &vir_mid::Type,
    //     type_decl: &vir_mid::TypeDecl,
    // ) -> SpannedEncodingResult<LoweredVariants> {
    //     use vir_low::macros::*;
    //     let mut constructors = LoweredVariants::default();
    //     match type_decl {
    //         vir_mid::TypeDecl::Bool => {
    //             constructors.add_constant_with_inv(vir_low::Type::Bool, true.into(), true);
    //             let bool_ty = (vir_mid::Type::Bool).create_snapshot(self)?;
    //             constructors.add_struct_alternative_no_inv(
    //                 &self
    //                     .encode_unary_op_variant(vir_low::UnaryOpKind::Not, &vir_mid::Type::Bool)?,
    //                 vars! { argument: {bool_ty.clone()} },
    //             );
    //             for op in BOOLEAN_OPERATORS {
    //                 // FIXME: Make these on demand.
    //                 constructors.add_struct_alternative_no_inv(
    //                     self.encode_binary_op_variant(*op, &vir_mid::Type::Bool)?,
    //                     vars! { left: {bool_ty.clone()}, right: {bool_ty.clone()} },
    //                 );
    //             }
    //             for size in INTEGER_SIZES {
    //                 let int_ty = vir_mid::Type::Int(*size);
    //                 let snapshot_type = int_ty.create_snapshot(self)?;
    //                 for op in COMPARISON_OPERATORS {
    //                     // FIXME: Make these on demand.
    //                     constructors.add_struct_alternative_no_inv(
    //                         self.encode_binary_op_variant(*op, &int_ty)?,
    //                         vars! { left: {snapshot_type.clone()}, right: {snapshot_type.clone()} },
    //                     );
    //                 }
    //             }
    //         }
    //         vir_mid::TypeDecl::Int(decl) => {
    //             var_decls! { constant: Int };
    //             let mut conjuncts = Vec::new();
    //             if let Some(lower_bound) = &decl.lower_bound {
    //                 conjuncts.push(expr! { [lower_bound.clone().to_low(self)? ] <= constant });
    //             }
    //             if let Some(upper_bound) = &decl.upper_bound {
    //                 conjuncts.push(expr! { constant <= [upper_bound.clone().to_low(self)? ] });
    //             }
    //             let validity = conjuncts.into_iter().conjoin();
    //             constructors.add_constant_with_inv(vir_low::Type::Int, validity, true);
    //             let snapshot_type = ty.create_snapshot(self)?;
    //             constructors.add_struct_alternative_no_inv(
    //                 &self.encode_unary_op_variant(vir_low::UnaryOpKind::Minus, ty)?,
    //                 vars! { argument: {snapshot_type.clone()} },
    //             );
    //             for op in ARITHMETIC_OPERATORS {
    //                 // FIXME: Make these on demand.
    //                 constructors.add_struct_alternative_no_inv(
    //                     &self.encode_binary_op_variant(*op, ty)?,
    //                     vars! { left: {snapshot_type.clone()}, right: {snapshot_type.clone()} },
    //                 );
    //             }
    //         }
    //         vir_mid::TypeDecl::Tuple(decl) => {
    //             let mut parameters = Vec::new();
    //             for field in decl.iter_fields() {
    //                 parameters.push(vir_low::VariableDecl::new(
    //                     field.name.clone(),
    //                     field.ty.create_snapshot(self)?,
    //                 ));
    //             }
    //             constructors.add_struct_with_inv(parameters, true.into());
    //         }
    //         vir_mid::TypeDecl::Struct(decl) => {
    //             let mut parameters = Vec::new();
    //             for field in decl.iter_fields() {
    //                 parameters.push(vir_low::VariableDecl::new(
    //                     field.name.clone(),
    //                     field.ty.create_snapshot(self)?,
    //                 ));
    //             }
    //             constructors.add_struct_with_inv(parameters, true.into());
    //         }
    //         vir_mid::TypeDecl::Enum(decl) => {
    //             for variant in &decl.variants {
    //                 let variant_type = ty.clone().variant(variant.name.clone().into());
    //                 let variant_type_snapshot = variant_type.create_snapshot(self)?;
    //                 constructors.add_enum_variant_with_inv(
    //                     &variant.name,
    //                     variant_type_snapshot,
    //                     true.into(),
    //                 );
    //                 self.ensure_type_definition(&variant_type)?;
    //             }
    //         }
    //         _ => unimplemented!("type: {:?}", type_decl),
    //     };
    //     Ok(constructors)
    // }
    // fn declare_validity_axioms<'a>(
    //     &mut self,
    //     ty: &vir_mid::Type,
    //     variants: impl Iterator<Item = &'a LoweredVariant>,
    // ) -> SpannedEncodingResult<()> {
    //     use vir_low::macros::*;
    //     let domain_name = self.encode_snapshot_domain_name(ty)?;
    //     for variant in variants {
    //         if let Some(validity) = &variant.validity {
    // The axiom that allows proving that the data structure is
    // valid if we know that its fields are valid.
    //             let axiom_bottom_up = {
    //                 let constructor_call =
    //                     variant.constructor.default_constructor_call(&domain_name);
    //                 let valid_constructor =
    //                     self.encode_snapshot_validity_expression(constructor_call, ty)?;
    //                 let body = if !variant.constructor.get_parameters().is_empty() {
    //                     let mut trigger = vec![valid_constructor.clone()];
    //                     let mut conjuncts = vec![
    //                         validity.clone(), // FIXME: We need to replace the fields here.
    //                     ];
    //                     for parameter in variant.constructor.get_parameters() {
    //                         if parameter.ty.is_domain() {
    //                             let mid_parameter_ty =
    //                                 self.decode_type_low_into_mid(&parameter.ty)?;
    //                             let valid_field = self.encode_snapshot_validity_expression(
    //                                 parameter.clone().into(),
    //                                 &mid_parameter_ty,
    //                             )?;
    //                             conjuncts.push(valid_field.clone());
    //                             trigger.push(valid_field);
    //                         }
    //                     }
    //                     vir_low::Expression::forall(
    //                         variant.constructor.get_parameters().to_vec(),
    //                         vec![vir_low::Trigger::new(trigger)],
    //                         expr! {
    //                             [ valid_constructor ] == [ conjuncts.into_iter().conjoin() ]
    //                         },
    //                     )
    //                 } else {
    //                     expr! {
    //                         [ valid_constructor ] == [ validity.clone() ]
    //                     }
    //                 };
    //                 vir_low::DomainAxiomDecl {
    //                     name: format!(
    //                         "{}$validity_axiom_bottom_up",
    //                         variant.constructor.constructor_name(&domain_name)
    //                     ),
    //                     body,
    //                 }
    //             };
    //             self.declare_axiom(&domain_name, axiom_bottom_up)?;
    //             if variant
    //                 .constructor
    //                 .get_parameters()
    //                 .iter()
    //                 .any(|parameter| parameter.ty.is_domain())
    //             {
    // The top-down axiom allows proving that any of the fields
    // is valid if we know that the whole data strucure is
    // valid. With no parameters, the bottom-up and top-down
    // axioms are equivalent.

    // var_decls! { snapshot: {ty.create_snapshot(self)?}};
    // let valid_constructor =
    //     self.encode_snapshot_validity_expression(snapshot.clone().into(), ty)?;
    // let mut triggers = Vec::new();
    // let mut conjuncts = vec![
    //     validity.clone(), // FIXME: We need to replace the fields here.
    // ];
    // for parameter in variant.constructor.get_parameters() {
    //     if parameter.ty.is_domain() {
    //         let field = self.snapshot_destructor_struct_call(
    //             &domain_name,
    //             &parameter.name,
    //             parameter.ty.clone(),
    //             snapshot.clone().into(),
    //         )?;
    //         let mid_parameter_ty = self.decode_type_low_into_mid(&parameter.ty)?;
    //         let valid_field =
    //             self.encode_snapshot_validity_expression(field, &mid_parameter_ty)?;
    //         triggers.push(vir_low::Trigger::new(vec![
    //             valid_constructor.clone(),
    //             valid_field.clone(),
    //         ]));
    //         conjuncts.push(valid_field.clone());
    //     }
    // }

    //                 let body = vir_low::Expression::forall(
    //                     vec![snapshot],
    //                     triggers,
    //                     expr! {
    //                         [ valid_constructor ] == [ conjuncts.into_iter().conjoin() ]
    //                     },
    //                 );
    //                 let axiom_top_down = vir_low::DomainAxiomDecl {
    //                     name: format!(
    //                         "{}$validity_axiom_top_down",
    //                         variant.constructor.constructor_name(&domain_name)
    //                     ),
    //                     body,
    //                 };
    //                 self.declare_axiom(&domain_name, axiom_top_down)?;
    //             }
    //         }
    //     }
    //     Ok(())
    // }
    // fn declare_injectivity_axioms_for_representational_constructors<'a>(
    //     &mut self,
    //     ty: &vir_mid::Type,
    //     variants: impl Iterator<Item = &'a LoweredVariant>,
    // ) -> SpannedEncodingResult<()> {
    //     use vir_low::macros::*;
    //     let domain_name = self.encode_snapshot_domain_name(ty)?;
    //     let snapshot_type = ty.create_snapshot(self)?;
    //     for variant in variants.filter(|variant| variant.is_representational) {
    //         let body = expr! {
    //             forall(
    //                 snapshot: {snapshot_type.clone()} ::
    //                 raw_code {
    //                     let valid_call = self.encode_snapshot_validity_expression_for_type(snapshot.clone().into(), ty)?;
    //                     let mut arguments = Vec::new();
    //                     for parameter in variant.constructor.get_parameters() {
    //                         // let function_name = variant.constructor.parameter_destructor_name(&domain_name, &parameter.name);
    //                         arguments.push(
    //                             variant.constructor.destructor_call(
    //                                 &domain_name,
    //                                 &parameter.name,
    //                                 parameter.ty.clone(),
    //                                 snapshot.clone().into()
    //                             )
    //                             // vir_low::Expression::domain_function_call(
    //                             //     &domain_name,
    //                             //     function_name,
    //                             //     vec![],
    //                             //     parameter.ty.clone(),
    //                             // )
    //                         );
    //                     };
    //                     // let constructor_name = variant.constructor.constructor_name(&domain_name);
    //                     let constructor_call = variant.constructor.constructor_call(
    //                         &domain_name,
    //                         arguments,
    //                     );
    //                     // vir_low::Expression::domain_function_call(
    //                     //     &domain_name,
    //                     //     constructor_name,
    //                     //     arguments,
    //                     //     snapshot_type.clone(),
    //                     // );
    //                 }
    //                 [{[valid_call.clone()]}
    //                 ]
    //                 [ valid_call ] ==> ([ constructor_call ] == snapshot)
    //             )
    //         };
    //         let axiom = vir_low::DomainAxiomDecl {
    //             name: format!(
    //                 "{}$representational_injectivity_axiom",
    //                 variant.constructor.constructor_name(&domain_name)
    //             ),
    //             body,
    //         };
    //         self.declare_axiom(&domain_name, axiom)?;
    //     }
    //     Ok(())
    // }
    fn declare_simplification_axiom(
        &mut self,
        ty: &vir_mid::Type,
        variant: &str,
        parameters: Vec<vir_low::VariableDecl>,
        parameter_type: &vir_mid::Type,
        simplification_result: vir_low::Expression,
    ) -> SpannedEncodingResult<()> {
        use vir_low::macros::*;
        let parameter_domain_name = self.encode_snapshot_domain_name(parameter_type)?;
        let domain_name = self.encode_snapshot_domain_name(ty)?;
        let snapshot_type = ty.create_snapshot(self)?;
        let mut constructor_calls = Vec::new();
        for parameter in parameters.iter() {
            constructor_calls.push(self.snapshot_constructor_constant_call(
                &parameter_domain_name,
                vec![parameter.clone().into()],
            )?);
        }
        let constructor_call_op =
            self.snapshot_constructor_constant_call(&domain_name, vec![simplification_result])?;
        let op_constructor_call = vir_low::Expression::domain_function_call(
            &domain_name,
            self.snapshot_constructor_struct_alternative_name(&domain_name, variant)?,
            constructor_calls,
            snapshot_type,
        );
        let body = vir_low::Expression::forall(
            parameters,
            vec![vir_low::Trigger::new(vec![op_constructor_call.clone()])],
            expr! { [op_constructor_call] == [constructor_call_op] },
        );
        let axiom = vir_low::DomainAxiomDecl {
            name: format!("{}$simplification_axiom", variant),
            body,
        };
        self.declare_axiom(&domain_name, axiom)?;
        Ok(())
    }
    fn declare_simplification_axioms(&mut self, ty: &vir_mid::Type) -> SpannedEncodingResult<()> {
        use vir_low::macros::*;
        match ty {
            vir_mid::Type::Bool => {
                let variant = self.encode_unary_op_variant(vir_low::UnaryOpKind::Not, ty)?;
                var_decls! { constant: Bool };
                let result = expr! { !constant };
                self.declare_simplification_axiom(ty, &variant, vec![constant], ty, result)?;
                for op in BOOLEAN_OPERATORS {
                    let variant = self.encode_binary_op_variant(*op, ty)?;
                    var_decls! { left: Bool, right: Bool };
                    let result =
                        vir_low::Expression::binary_op_no_pos(*op, expr! {left}, expr! {right});
                    self.declare_simplification_axiom(ty, &variant, vec![left, right], ty, result)?;
                }
            }
            vir_mid::Type::Int(_) => {
                let variant = self.encode_unary_op_variant(vir_low::UnaryOpKind::Minus, ty)?;
                var_decls! { constant: Int };
                let result = expr! { -constant };
                self.declare_simplification_axiom(ty, &variant, vec![constant], ty, result)?;
                for op in ARITHMETIC_OPERATORS {
                    let variant = self.encode_binary_op_variant(*op, ty)?;
                    var_decls! { left: Int, right: Int };
                    let result =
                        vir_low::Expression::binary_op_no_pos(*op, expr! {left}, expr! {right});
                    self.declare_simplification_axiom(ty, &variant, vec![left, right], ty, result)?;
                }
                // FIXME: This should be more fine grained instead of including all Bool axioms.
                self.ensure_type_definition(&vir_mid::Type::Bool)?;
                for op in COMPARISON_OPERATORS {
                    let variant = self.encode_binary_op_variant(*op, ty)?;
                    var_decls! { left: Int, right: Int };
                    let result =
                        vir_low::Expression::binary_op_no_pos(*op, expr! {left}, expr! {right});
                    self.declare_simplification_axiom(
                        &vir_mid::Type::Bool,
                        &variant,
                        vec![left, right],
                        ty,
                        result,
                    )?;
                }
            }
            _ => {
                // Other types do not need simplification axioms.
            }
        }
        Ok(())
    }
}

pub(in super::super) trait TypesInterface {
    /// Ensure that all encoders know the necessary information to encode the
    /// uses of this type that require its definition.
    fn ensure_type_definition(&mut self, ty: &vir_mid::Type) -> SpannedEncodingResult<()>;
    fn encode_unary_op_variant(
        &mut self,
        op: vir_low::UnaryOpKind,
        ty: &vir_mid::Type,
    ) -> SpannedEncodingResult<String>;
    fn encode_binary_op_variant(
        &mut self,
        op: vir_low::BinaryOpKind,
        ty: &vir_mid::Type,
    ) -> SpannedEncodingResult<String>;
    fn decode_type_low_into_mid(&self, ty: &vir_low::Type) -> SpannedEncodingResult<vir_mid::Type>;
}

impl<'p, 'v: 'p, 'tcx: 'v> TypesInterface for Lowerer<'p, 'v, 'tcx> {
    fn ensure_type_definition(&mut self, ty: &vir_mid::Type) -> SpannedEncodingResult<()> {
        if !self.types_state.ensured_definitions.contains(ty) {
            // We insert before doing the actual work to break infinite
            // recursion.
            self.types_state.ensured_definitions.insert(ty.clone());

            let type_decl = self.encoder.get_type_decl_mid(ty)?;
            self.ensure_type_definition_for_decl(ty, &type_decl)?;

            // let constructors = self.compute_adt_constructors(ty, &type_decl)?;

            // self.declare_snapshot_constructors(ty, constructors.iter_constructors())?;
            // self.declare_validity_axioms(ty, constructors.iter_variants())?;
            // self.declare_injectivity_axioms_for_representational_constructors(
            //     ty,
            //     constructors.iter_variants(),
            // )?;
            // self.declare_simplification_axioms(ty)?;
            // 1. Snapshot.
            // 2. Address.
            // 3. Place.
            // 4. ComputeAddress.
        }
        Ok(())
    }
    fn encode_unary_op_variant(
        &mut self,
        op: vir_low::UnaryOpKind,
        ty: &vir_mid::Type,
    ) -> SpannedEncodingResult<String> {
        Ok(format!("{}_{}", op, ty.get_identifier()))
    }
    fn encode_binary_op_variant(
        &mut self,
        op: vir_low::BinaryOpKind,
        ty: &vir_mid::Type,
    ) -> SpannedEncodingResult<String> {
        Ok(format!("{}_{}", op, ty.get_identifier()))
    }
    fn decode_type_low_into_mid(&self, ty: &vir_low::Type) -> SpannedEncodingResult<vir_mid::Type> {
        let decoded_type = match ty {
            vir_low::Type::Domain(domain) => {
                if let Some(decoded_type) = self.try_decoding_snapshot_type(&domain.name)? {
                    decoded_type
                } else {
                    unreachable!("Failed to decode the snapshot type: {}", ty);
                }
            }
            _ => unreachable!("Trying to decode unsupported type: {}", ty),
        };
        Ok(decoded_type)
    }
}
