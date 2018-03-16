#[allow(unused_imports)]
use self::ir::prelude::*;
#[allow(unused_imports)]
use std;
use std::sync::Arc;
#[allow(unused_imports)]
use utils::*;

{{>store}}

{{#each choices}}
    {{>choice_def this}}
{{/each}}

{{enums}}

{{>actions this}}

/// Propagate the changes stored in `diff`.
fn propagate_changes(diff: &mut DomainDiff, ir_instance: &mut Arc<ir::Function>,
                     store: &mut DomainStore) -> Result<(), ()> {
    {{~#each choices}}
        while let Some((({{#each arguments}}{{this.[0]}}, {{/each}}), old, new)) =
                diff.pop_{{name}}_diff() {
            debug!("propagating {{name}}{:?} {:?} -> {:?}",
                   ({{>choice.arg_names this}}), old, new);
            {{name}}::on_change(old, new,
                {{~>choice.arg_names this}}ir_instance, store, diff)?;
        }
    {{~/each}}
    Ok(())
}

/// Applies a set of decisions to the domain and propagate the changes.
#[cfg(feature="gen_applicators")]
pub fn apply_decisions(actions: Vec<Action>, ir_instance: &mut Arc<ir::Function>,
                   domain: &mut DomainStore) -> Result<(), ()> {
    let mut diff = DomainDiff::default();
    for action in actions { apply_action(action, domain, &mut diff)?; }
    while !diff.is_empty() { propagate_changes(&mut diff, ir_instance, domain)?; }
    Ok(())
}

/// Update the domain after a lowering.
#[cfg(feature="gen_applicators")]
fn process_lowering(ir_instance: &mut ir::Function,
                    domain: &mut DomainStore,
                    new_objs: &NewObjs,
                    diff: &mut DomainDiff) -> Result<Vec<Action>, ()> {
    let mut actions = Vec::new();
    domain.alloc(ir_instance, &new_objs);
    actions.extend(init_domain_partial(domain, ir_instance, &new_objs, diff)?);
    Ok(actions)
}



/// Initializes the `DomainStore` with available choices for each decision.
#[allow(unused_variables, unused_mut)]
pub fn init_domain(store: &mut DomainStore,
                   ir_instance: &mut ir::Function) -> Result<Vec<Action>, ()> {
    // Run all the filters once.
    let ref mut diff = DomainDiff::default(); // Pass an empty diff to propagate and triggers.
    let mut unused_diff = DomainDiff::default();
    {{#each choices~}}
        {{#>loop_nest iteration_space~}}
            {{>run_filters this}}
        {{/loop_nest~}}
    {{/each~}}
    {{store.filter_all}}
    // Propagate the filters where necessary.
    let mut actions: Vec<Action> = Vec::new();
    {{#each triggers~}}
        let mut trigger_{{id}} = Vec::new();
        {{#>loop_nest loop_nest}}
            if check_trigger_{{id}}({{>choice.arg_names}}ir_instance, store, diff) {
                trigger_{{id}}.push(({{>choice.arg_ids}}));
            }
        {{/loop_nest}}
    {{/each}}
    {{#each choices~}}
        {{#>loop_nest iteration_space}}{{>propagate this}}{{/loop_nest~}}
    {{/each~}}
    // Propagate triggers.
    {{#each triggers}}{{>trigger_call call_id=id delayed=true}}{{/each~}}
    Ok(actions)
}

/// Initializes the part of the `DomainStore` allocated for the given objects with available
/// choices for each decision.
#[allow(unused_variables, unused_mut)]
fn init_domain_partial(store: &mut DomainStore,
                       ir_instance: &mut ir::Function,
                       new_objs: &NewObjs,
                       diff: &mut DomainDiff) -> Result<Vec<Action>, ()> {
    let mut unused_diff = DomainDiff::default();
    // Disable new increments of existing counters.
    {{#each incr_iterators~}}
        {{#>iter_new_objects iter~}}
            {{>disable_increment}}
        {{/iter_new_objects~}}
    {{/each~}}
    // Call filters.
    {{#each partial_iterators~}}
        {{#>iter_new_objects this.[0]~}}
            {{>run_filters this.[1].choice arg_names=this.[1].arg_names}}
        {{/iter_new_objects~}}
    {{/each~}}
    // Propagate decisions that are not already propagted.
    let mut actions: Vec<Action> = Vec::new();
    {{#each partial_iterators~}}
        {{#>iter_new_objects this.[0]~}}
                {{>propagate this.[1].choice arg_names=this.[1].arg_names}}
        {{/iter_new_objects~}}
    {{/each~}}
    // Take new increment on existing counters into account.
    {{#each incr_iterators~}}
        {{#>iter_new_objects iter~}}
            {{>account_new_incrs}}
        {{/iter_new_objects~}}
    {{/each~}}
    // Check if we should call the new triggers.
    {{#each triggers~}}
        let mut trigger_{{id}} = Vec::new();
        {{#each partial_iterators}}
            {{#>iter_new_objects this.[0]}}
    if check_trigger_{{../id}}(
        {{~>choice.arg_names ../../this arg_names=this.[1]~}}ir_instance, store, diff)
    {
        trigger_{{../id}}.push(({{>choice.arg_ids ../../this arg_names=this.[1]}}));
    }
            {{/iter_new_objects~}}
        {{/each~}}
    {{/each~}}
    // Propagate triggers.
    {{#each triggers}}{{>trigger_call call_id=id delayed=true}}{{/each~}}
    Ok(actions)
}

{{#each triggers~}}
    {{>trigger_check}}
{{/each~}}

// TODO(cleanup): generate (IrInstance, Domain) pair here.

pub trait Domain: Copy + Eq {
    const FAILED: Self;

    const ALL: Self;

    /// Indicates if the domain is empty.
    fn is_failed(&self) -> bool;
    /// Indicates if the domain contains a single alternative.
    fn is_constrained(&self) -> bool;
    /// Indicates if the domain contains another.
    fn contains(&self, other: Self) -> bool;
    /// Restricts the domain to the intersection with `other`.
    fn restrict(&mut self, other: Self);
    /// Inserts alternatives into the domain.
    fn insert(&mut self, other: Self);

    /// Indicates if the domain has an alternatve in common with `other`.
    fn intersects(&self, mut other: Self) -> bool where Self: Sized {
        other.restrict(*self);
        !other.is_failed()
    }

    /// Indicates if the domain is equal to another domain.
    fn is(&self, mut other: Self) -> Trivalent where Self: Sized {
        other.restrict(*self);
        if other.is_failed() {
            Trivalent::False
        } else if other == *self {
            Trivalent::True
        } else {
            Trivalent::Maybe
        }
    }

    /// Indicates if two choices will have the same value.
    fn eq(&self, other: Self) -> bool {
        self.is_constrained() && *self == other
    }

    /// Indicates if two choices cannot be equal.
    fn neq(&self, other: Self) -> bool {
        !self.intersects(other)
    }
}

/// Abstracts integer choices by a range.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Range {
    pub min: u32,
    pub max: u32,
}

#[allow(dead_code)]
impl Range {
    /// Creates a range that only contains the given value.
    pub fn new_eq(val: u32) -> Self { Range { min: val, max: val } }

    /// Returns the range of all the integers greater than 'val'
    pub fn new_gt(val: u32) -> Self {
        Range { min: val.saturating_add(1), .. Range::ALL }
    }

    /// Returns the range of all the integers lower than 'val'
    pub fn new_lt(val: u32) -> Self {
        Range { max: val.saturating_sub(1), .. Range::ALL }
    }

    /// Returns the range of all the integers greater or equal to 'val'
    pub fn new_geq(val: u32) -> Self { Range { min: val, .. Range::ALL } }

    /// Returns the range of all the integers lesser or equal to 'val'
    pub fn new_leq(val: u32) -> Self { Range { max: val, .. Range::ALL } }

    /// Returns the difference between the min and max of two ranges.
    fn get_diff_add(&self, other: Range) -> Range {
        Range { min: other.min - self.min, max: self.max - other.max }
    }

    /// Restricts the `Range` by applying the result of `get_diff`.
    fn apply_diff_add(&mut self, diff: Range) {
        self.min += diff.min;
        self.max -= diff.max;
    }

    /// Returns the difference between the min and max of two ranges.
    fn get_diff_mul(&self, other: Range) -> Range {
        Range { min: other.min / self.min, max: self.max / other.max }
    }

    /// Restricts the `Range` by applying the result of `get_diff`.
    fn apply_diff_mul(&mut self, diff: Range) {
        self.min *= diff.min;
        self.max /= diff.max;
    }

    fn add_add(&mut self, diff: Range) {
        self.min += diff.min;
        self.max += diff.max;
    }

    fn add_mul(&mut self, diff: Range) {
        self.min *= diff.min;
        self.max *= diff.max;
    }

    fn sub_add(&mut self, diff: Range) {
        self.min -= diff.min;
        self.max -= diff.max;
    }

    fn sub_mul(&mut self, diff: Range) {
        self.min /= diff.min;
        self.max /= diff.max;
    }

    pub fn lt(&self, other: Range) -> bool { self.max < other.min }

    pub fn gt(&self, other: Range) -> bool { self.min > other.max }

    pub fn leq(&self, other: Range) -> bool { self.max <= other.min }

    pub fn geq(&self, other: Range) -> bool { self.min >= other.max }

    pub fn as_fixed(&self) -> Option<u32> {
        if self.min == self.max { Some(self.min) } else { None }
    }
}

impl Domain for Range {
    const FAILED: Range = Range { min: 1, max: 0 };

    const ALL: Range = Range { min: 0, max: std::u32::MAX };

    fn is_failed(&self) -> bool { self.min > self.max }

    fn is_constrained(&self) -> bool { self.min == self.max }

    fn contains(&self, other: Range) -> bool {
        self.min <= other.min && self.max >= other.max
    }

    fn restrict(&mut self, other: Range) {
        self.min = std::cmp::max(self.min, other.min);
        self.max = std::cmp::min(self.max, other.max);
    }

    fn insert(&mut self, other: Range) {
        self.min = std::cmp::min(self.min, other.min);
        self.max = std::cmp::max(self.max, other.max);
    }
}

/// Abstracts integer choices by a range, but only store `min`.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct HalfRange { pub min: u32 }

#[allow(dead_code)]
impl HalfRange {
    /// Returns the range of all the integers greater than 'val'
    pub fn new_gt(val: u32) -> Self {
        HalfRange { min: val.saturating_add(1) }
    }

    /// Returns the range of all the integers greater or equal to 'val'
    pub fn new_geq(val: u32) -> Self { HalfRange { min: val } }

    /// Returns the difference between the min and max of two ranges.
    fn get_diff_add(&self, other: HalfRange) -> HalfRange {
        HalfRange { min: other.min - self.min }
    }

    /// Restricts the `Range` by applying the result of `get_diff`.
    fn apply_diff_add(&mut self, diff: HalfRange) { self.min += diff.min; }

    /// Returns the difference between the min and max of two ranges.
    fn get_diff_mul(&self, other: HalfRange) -> HalfRange {
        HalfRange { min: other.min / self.min }
    }

    /// Restricts the `Range` by applying the result of `get_diff`.
    fn apply_diff_mul(&mut self, diff: HalfRange) { self.min *= diff.min; }

    fn add_add(&mut self, diff: HalfRange) {
        self.min += diff.min;
    }

    fn add_mul(&mut self, diff: HalfRange) {
        self.min *= diff.min;
    }

    fn sub_add(&mut self, diff: HalfRange) {
        self.min -= diff.min;
    }

    fn sub_mul(&mut self, diff: HalfRange) {
        self.min /= diff.min;
    }

    fn gt(&self, other: Range) -> bool { self.min > other.max }

    fn geq(&self, other: Range) -> bool { self.min >= other.max }
}

impl Domain for HalfRange {
    const FAILED: HalfRange = HalfRange { min: std::u32::MAX };

    const ALL: HalfRange = HalfRange { min: 0 };

    fn is_failed(&self) -> bool { self.min == std::u32::MAX }

    fn is_constrained(&self) -> bool { false }

    fn contains(&self, other: HalfRange) -> bool {
        self.min <= other.min
    }

    fn restrict(&mut self, other: HalfRange) {
        self.min = std::cmp::max(self.min, other.min);
    }

    fn insert(&mut self, other: HalfRange) {
        self.min = std::cmp::min(self.min, other.min);
    }
}
