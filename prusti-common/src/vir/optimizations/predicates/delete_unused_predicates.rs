use std::collections::BTreeSet;
use vir::{ast::*, cfg::CfgMethod};

use vir::CfgBlock;

use crate::vir::{cfg, Successor};

fn get_used_predicates(methods: &[CfgMethod], functions: &[Function]) -> BTreeSet<String> {
    let mut collector = UsedPredicateCollector::new();
    super::walk_methods(methods, &mut collector);
    super::walk_functions(functions, &mut collector);

    // DeadBorrowToken$ is a used predicate but it does not appear in VIR becaue it is only created when viper code is created from VIR
    collector
        .used_predicates
        .insert("DeadBorrowToken$".to_string());
    collector.used_predicates
}

fn get_used_predicates_in_predicates(predicates: &[Predicate]) -> BTreeSet<String> {
    let mut collector = UsedPredicateCollector::new();

    for pred in predicates {
        match pred {
            Predicate::Struct(StructPredicate { body: Some(e), .. }) => {
                ExprWalker::walk(&mut collector, e)
            }
            Predicate::Struct(_) => { /* ignore */ }
            Predicate::Enum(p) => {
                ExprWalker::walk(&mut collector, &p.discriminant);
                ExprWalker::walk(&mut collector, &p.discriminant_bounds);

                for (e, _, sp) in &p.variants {
                    ExprWalker::walk(&mut collector, e);
                    sp.body
                        .iter()
                        .for_each(|e| ExprWalker::walk(&mut collector, e))
                }
            }
            Predicate::Bodyless(_, _) => { /* ignore */ }
        }
    }
    collector.used_predicates
}

fn remove_body_of_predicates_if_possible(
    predicates: &[Predicate],
    predicates_only_used_in_predicates: &BTreeSet<String>,
) -> Vec<Predicate> {
    let mut new_predicates = predicates.to_vec();

    new_predicates.iter_mut().for_each(|predicate| {
        let predicates_used_in_this_predicate =
            get_used_predicates_in_predicates(&[predicate.clone()]);
        if predicates_used_in_this_predicate
            .intersection(&predicates_only_used_in_predicates)
            .next()
            .is_some()
        {
            if let Predicate::Struct(sp) = predicate {
                sp.body = None;
            }
        }
    });
    new_predicates
}

pub fn delete_unused_predicates(
    methods: &[CfgMethod],
    functions: &[Function],
    predicates: &[Predicate],
) -> Vec<Predicate> {
    let mut has_changed = true;
    let mut new_predicates: Vec<Predicate> = predicates.to_vec();

    let used_preds = get_used_predicates(methods, functions);

    dbg!(&used_preds);

    while has_changed {
        has_changed = false;

        let predicates_used_in_predicates = get_used_predicates_in_predicates(&new_predicates);
        dbg!(&predicates_used_in_predicates);
        new_predicates = new_predicates
            .into_iter()
            .filter(|p| {
                let name = p.name();
                let is_used_in_predicate = predicates_used_in_predicates.contains(name);
                let is_used_in_func_or_method = used_preds.contains(name);
                let is_used = is_used_in_predicate || is_used_in_func_or_method;
                if !is_used {
                    has_changed = true;
                }

                is_used
            })
            .collect();
    }

    let predicates_used_in_predicates = get_used_predicates_in_predicates(&new_predicates);
    let only_used_in_predicates: BTreeSet<String> = predicates_used_in_predicates
        .difference(&used_preds)
        .cloned()
        .collect();
    dbg!(&only_used_in_predicates);

    // FIXME: This acctually removes bodies that are needed
    /*new_predicates =
    remove_body_of_predicates_if_possible(&new_predicates, &only_used_in_predicates);*/
    return new_predicates;
}

struct UsedPredicateCollector {
    used_predicates: BTreeSet<String>,
}

impl UsedPredicateCollector {
    fn new() -> Self {
        UsedPredicateCollector {
            used_predicates: BTreeSet::new(),
        }
    }
}

impl ExprWalker for UsedPredicateCollector {
    fn walk_predicate_access_predicate(
        &mut self,
        name: &str,
        arg: &Expr,
        _perm_amount: PermAmount,
        _pos: &Position,
    ) {
        self.used_predicates.insert(name.to_string());
        ExprWalker::walk(self, arg);
    }

    fn walk_unfolding(
        &mut self,
        name: &str,
        args: &Vec<Expr>,
        body: &Expr,
        _perm: PermAmount,
        _variant: &MaybeEnumVariantIndex,
        _pos: &Position,
    ) {
        self.used_predicates.insert(name.to_string());
        for arg in args {
            ExprWalker::walk(self, arg);
        }
        ExprWalker::walk(self, body);
    }
}

impl StmtWalker for UsedPredicateCollector {
    fn walk_expr(&mut self, expr: &Expr) {
        ExprWalker::walk(self, expr);
    }
    fn walk_fold(
        &mut self,
        predicate_name: &str,
        args: &Vec<Expr>,
        _perm: &PermAmount,
        _variant: &MaybeEnumVariantIndex,
        _pos: &Position,
    ) {
        self.used_predicates.insert(predicate_name.to_string());
    }

    fn walk_unfold(
        &mut self,
        predicate_name: &str,
        args: &Vec<Expr>,
        _perm: &PermAmount,
        _variant: &MaybeEnumVariantIndex,
    ) {
        self.used_predicates.insert(predicate_name.to_string());
    }
}
