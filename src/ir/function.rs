//! Provides a representation of functions.
use device::Device;
use ir::mem::Block;
use ir::{
    self, Dimension, InstId, Instruction, Operator, Statement, StmtId, Value, ValueDef,
    ValueId,
};
use ir::{dim, mem, AccessPattern, Operand, SparseVec, Type};
use itertools::Itertools;
use std;
use utils::*;

/// Represents an argument of a function.
#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct Parameter {
    /// The name of the `Parameter`
    pub name: String,
    /// The type of the `Parameter`.
    pub t: Type,
}

/// Holds the signature of a function.
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Signature {
    /// Mame of the function.
    pub name: String,
    /// Arguments of the function.
    pub params: Vec<Parameter>,
    /// The number of external memory blocks.
    pub mem_blocks: u32,
}

impl Signature {
    /// Creates a new signature without any parameter.
    pub fn new(name: String) -> Self {
        Signature {
            name,
            params: vec![],
            mem_blocks: 0,
        }
    }

    /// Adds a scalar parameter.
    pub fn add_scalar(&mut self, name: String, t: ir::Type) {
        self.params.push(Parameter { name, t });
    }

    /// Adds a parameter with the given name and type to the signature.
    pub fn add_array(&mut self, name: String) -> ir::MemId {
        let id = ir::MemId::External(self.mem_blocks);
        self.mem_blocks += 1;
        self.params.push(Parameter {
            name,
            t: ir::Type::PtrTo(id),
        });
        id
    }
}

/// Describes a function and the set of its possible implementation.
#[derive(Clone)]
pub struct Function<'a, L = ir::LoweringMap> {
    signature: &'a Signature,
    device: &'a Device,
    insts: SparseVec<ir::InstId, Instruction<'a, L>>,
    dims: SparseVec<ir::DimId, Dimension<'a>>,
    static_dims: Vec<ir::DimId>,
    thread_dims: VecSet<ir::DimId>,
    mem_insts: Vec<ir::InstId>,
    mem_blocks: mem::BlockMap,
    layouts_to_lower: Vec<ir::mem::InternalId>,
    induction_vars: Vec<ir::InductionVar<'a, L>>,
    logical_dims: Vec<ir::LogicalDim<'a>>,
    dim_mappings: SparseVec<ir::DimMappingId, ir::DimMapping>,
    values: SparseVec<ValueId, Value>,
}

impl<'a, L> Function<'a, L> {
    /// Creates a new function.
    pub fn new(signature: &'a Signature, device: &'a Device) -> Self {
        let mem_blocks = mem::BlockMap::new(signature.mem_blocks);
        Function {
            signature,
            device,
            insts: SparseVec::new(),
            mem_insts: vec![],
            dims: SparseVec::new(),
            static_dims: vec![],
            thread_dims: VecSet::default(),
            mem_blocks,
            layouts_to_lower: Vec::new(),
            induction_vars: Vec::new(),
            logical_dims: Vec::new(),
            dim_mappings: SparseVec::new(),
            values: SparseVec::new(),
        }
    }

    /// Returns the function signature.
    pub fn signature(&self) -> &'a Signature {
        self.signature
    }

    /// Returns the device the function is compiled for.
    pub fn device(&self) -> &'a Device {
        self.device
    }

    /// Creates a new instruction (with given ID) without adding it to
    /// the `insts` vector. Used as an internal helper for when either
    /// adding a new instruction (`add_inst`) or filling an existing
    /// hole in the instructions vector.
    fn create_inst(
        &mut self,
        id: InstId,
        op: Operator<'a, L>,
        iter_dims: HashSet<ir::DimId>,
    ) -> Result<ir::Instruction<'a, L>, ir::Error> {
        // Create and check the instruction.
        let inst = ir::Instruction::new(op, id, iter_dims, self)?;
        // Register the instruction in iteration dimensions.
        for &dim in inst.iteration_dims() {
            self.dim_mut(dim).add_iterated(id.into());
        }
        // Register the memory blocks used.
        if let Some(mem_id) = inst.operator().mem_used() {
            self.mem_insts.push(id);
            self.mem_blocks.register_use(mem_id, id);
        }
        // Update the usepoint of all values
        for ref op in inst.operator().operands() {
            if let Operand::Value(val_id, _) = op {
                self.values[*val_id].add_usepoint(id);
            }
        }
        Ok(inst)
    }

    /// Returns a Value without adding it to self.values
    fn create_value(&self, id: ValueId, def: ValueDef) -> Result<Value, ir::Error> {
        let t = match def {
            ValueDef::Inst(id) => {
                let inst = &self.insts[id];
                unwrap!(inst.t())
            }
        };
        Ok(Value::new(id, t, def))
    }

    /// Adds an induction variable.
    pub fn add_ind_var(&mut self, ind_var: ir::InductionVar<'a, L>) -> ir::IndVarId {
        let id = ir::IndVarId(self.induction_vars.len() as u32);
        self.induction_vars.push(ind_var);
        id
    }

    /// Returns the list of instructions of the function.
    pub fn insts<'b>(&'b self) -> impl Iterator<Item = &'b Instruction<'a, L>> {
        self.insts.iter()
    }

    /// Returns the list of dimensions of the function.
    pub fn dims<'b>(&'b self) -> impl Iterator<Item = &'b Dimension<'a>> + Clone {
        self.dims.iter()
    }

    /// Returns the list of logical dimensions.
    pub fn logical_dims(&self) -> impl Iterator<Item = &ir::LogicalDim<'a>> {
        self.logical_dims.iter()
    }

    /// Returns the list of stastic dimensions in the function.
    pub fn static_dims<'b>(&'b self) -> impl Iterator<Item = &'b Dimension<'a>> {
        self.static_dims.iter().map(move |&id| self.dim(id))
    }

    pub fn values(&self) -> impl Iterator<Item = &ir::Value> {
        self.values.iter()
    }

    /// Returns the list of thread dimensions.
    pub fn thread_dims(&self) -> impl Iterator<Item = &Dimension<'a>> {
        self.thread_dims.iter().map(move |&d| self.dim(d))
    }

    /// Returns an instruction given its id.
    pub fn inst(&self, id: InstId) -> &Instruction<'a, L> {
        &self.insts[id]
    }

    /// Returns a mutable reference to an instruction given its id.
    pub fn inst_mut(&mut self, id: InstId) -> &mut Instruction<'a, L> {
        &mut self.insts[id]
    }

    /// Retuns a dimension given its id.
    pub fn dim(&self, id: ir::DimId) -> &Dimension<'a> {
        &self.dims[id]
    }

    /// Returns a mutable reference to a dimension given its ID.
    fn dim_mut(&mut self, id: ir::DimId) -> &mut Dimension<'a> {
        &mut self.dims[id]
    }

    /// Retrives a logical dimension given its ID.
    pub fn logical_dim(&self, id: ir::LogicalDimId) -> &ir::LogicalDim<'a> {
        &self.logical_dims[id.0 as usize]
    }

    /// Returns a `Value` given its id.
    pub fn value(&self, id: ir::ValueId) -> &ir::Value {
        &self.values[id]
    }

    /// Adds a value to the function. Also register its definition into the relevant instruction
    pub fn add_value(&mut self, def: ir::ValueDef) -> Result<ir::ValueId, ir::Error> {
        let id = ir::ValueId(self.values.len() as u16);
        let val = self.create_value(id, def)?;
        val.def().register(val.id(), self);
        self.values.push(val);
        Ok(id)
    }

    /// Returns the list of memory blocks. The block with id `i` is in i-th position.
    pub fn mem_blocks<'b>(&'b self) -> impl Iterator<Item = &'b mem::Block> {
        self.mem_blocks.blocks()
    }

    /// Iterates over memory instructions.
    pub fn mem_insts<'b>(&'b self) -> impl Iterator<Item = &'b Instruction<'a, L>> + 'b {
        self.mem_insts.iter().map(move |&id| self.inst(id))
    }

    /// Returns the internal memory blocks.
    pub fn internal_mem_blocks<'b>(
        &'b self,
    ) -> impl Iterator<Item = &'b mem::InternalBlock> {
        self.mem_blocks.internal_blocks()
    }

    /// Returns a memory block given its id.
    pub fn mem_block(&self, id: ir::MemId) -> &mem::Block {
        self.mem_blocks.block(id)
    }

    /// Returns an internal memory block given its id.
    pub fn internal_mem_block(&self, id: mem::InternalId) -> &mem::InternalBlock {
        self.mem_blocks.internal_block(id)
    }

    /// Retrieves an induction variable given its Id.
    pub fn induction_var(&self, id: ir::IndVarId) -> &ir::InductionVar<'_, L> {
        &self.induction_vars[id.0 as usize]
    }

    /// Iterates over induction variables.
    pub fn induction_vars<'b>(
        &'b self,
    ) -> impl Iterator<Item = (ir::IndVarId, &'b ir::InductionVar<'a, L>)> {
        self.induction_vars
            .iter()
            .enumerate()
            .map(|(id, v)| (ir::IndVarId(id as u32), v))
    }

    /// Sets a dimension as an iteration dimension for an instruction. Indicates if the
    /// iteration dimension was not aleady present in the set.
    pub fn set_iteration_dim(&mut self, inst: ir::InstId, dim: ir::DimId) -> bool {
        if self.inst_mut(inst).add_iteration_dimension(dim) {
            self.dim_mut(dim).add_iterated(inst);
            true
        } else {
            false
        }
    }

    /// Adds a thread dimension. Indicates if the the dimension was not already present
    /// in the set.
    pub fn add_thread_dim(&mut self, dim: ir::DimId) -> bool {
        self.dim_mut(dim).set_thread_dim();
        self.thread_dims.insert(dim)
    }

    /// Trigger to call when two dimensions are merged.
    // TODO(cleanup): externalize in the search space the merging of dimensions in dim
    // maps.
    pub(crate) fn merge(&mut self, src: ir::DimId, dst: ir::DimId) {
        for inst in self.insts.iter_mut() {
            inst.merge_dims(src, dst);
        }
        for var in &mut self.induction_vars {
            var.merge_dims(src, dst);
        }
        self.layouts_to_lower
            .extend(self.mem_blocks.merge_dims(src, dst));
    }

    /// Lowers a layout into conventional memory accesses.
    pub(crate) fn lower_layout(
        &mut self,
        id: mem::InternalId,
        st_dims: Vec<ir::DimId>,
        ld_dims: Vec<ir::DimId>,
    ) where
        L: Clone,
    {
        let pos = unwrap!(self.layouts_to_lower.iter().position(|&x| x == id));
        self.layouts_to_lower.swap_remove(pos);
        self.mem_blocks.lower_layout(id);
        let (st_index, st_pattern) = self.gen_internal_index(id, st_dims);
        let (ld_index, ld_pattern) = self.gen_internal_index(id, ld_dims);
        for &mem_use in self.mem_blocks.internal_block(id).uses() {
            self.insts[mem_use].lower_layout(
                ld_index.clone(),
                ld_pattern.clone(),
                st_index.clone(),
                st_pattern.clone(),
            );
        }
    }

    /// Generates an operand repesenting a pointer to a cell of a memory block.
    fn gen_internal_index(
        &mut self,
        id: mem::InternalId,
        dims: Vec<ir::DimId>,
    ) -> (Operand<'a, L>, AccessPattern<'a>) {
        let ty_len = self.mem_blocks.internal_block(id).base_size();
        self.gen_index(id.into(), ty_len, Operand::Addr(id), dims)
    }

    /// Generates an access pattern and the corresponding induction variable to access a
    /// memory block.
    fn gen_index(
        &mut self,
        mem: ir::MemId,
        base_incr: u32,
        base_addr: Operand<'a, L>,
        dims: Vec<ir::DimId>,
    ) -> (Operand<'a, L>, AccessPattern<'a>) {
        let var_type = base_addr.t();
        let base_size = ir::PartialSize::new(base_incr, vec![], 1);
        let increments = dims
            .iter()
            .rev()
            .scan(base_size, |size, &dim| {
                let old_size = size.clone();
                *size *= self.dim(dim).size();
                Some((dim, old_size))
            }).collect_vec();
        let pattern = ir::AccessPattern::Tensor {
            mem_id: mem,
            dims: increments.iter().cloned().collect(),
        };
        let ind_var = unwrap!(ir::InductionVar::new(increments, base_addr));
        let ind_var = self.add_ind_var(ind_var);
        let addr = ir::Operand::InductionVar(ind_var, var_type);
        (addr, pattern)
    }

    /// Trigger to call when two dimensions are not merged.
    pub(crate) fn dim_not_merged(&mut self, lhs: ir::DimId, rhs: ir::DimId) {
        let to_lower = self.mem_blocks.not_merged(&self.dims[lhs], rhs);
        self.layouts_to_lower.extend(to_lower);
    }

    /// Returns the list of layouts to lower.
    pub(crate) fn layouts_to_lower(&self) -> &[ir::mem::InternalId] {
        &self.layouts_to_lower
    }

    /// Returns the list of dimensions mapping.
    pub fn dim_mappings(&self) -> impl Iterator<Item = &ir::DimMapping> + '_ {
        self.dim_mappings.iter()
    }

    /// Retrives a dimension mapping given its ID.
    pub fn dim_mapping(&self, id: ir::DimMappingId) -> &ir::DimMapping {
        &self.dim_mappings[id]
    }

    /// Tries to find a mapping between two dimensions.
    pub fn find_mapping(
        &self,
        lhs: ir::DimId,
        rhs: ir::DimId,
    ) -> Option<ir::DimMappingId> {
        self.dim(lhs)
            .dim_mappings()
            .iter()
            .cloned()
            .find(|&id| self.dim_mapping(id).dims().contains(&rhs))
    }

    /// Creates a mapping between two dimensions.
    fn create_mapping(
        &mut self,
        id: ir::DimMappingId,
        dims: [ir::DimId; 2],
    ) -> ir::DimMapping {
        let mapping = ir::DimMapping::new(id, dims);
        for &dim in &dims {
            self.dim_mut(dim).register_dim_mapping(&mapping);
        }
        mapping
    }
    /// Returns true if inst2 depends on inst1 (check based on value)
    pub fn is_dependency_of(&self, inst1_id: InstId, inst2_id: InstId) -> bool {
        match self.insts[inst1_id].result_value() {
            None => false,
            Some(val_id) => self.values[val_id].is_dependency_of(inst2_id),
        }
    }
}

impl<'a> Function<'a, ()> {
    /// Adds an instruction to the function.
    pub fn add_inst(
        &mut self,
        op: Operator<'a, ()>,
        iter_dims: HashSet<ir::DimId>,
    ) -> Result<InstId, ir::Error> {
        // Create dimension mappings for the operands.
        // TODO(cleanup): the operands should list `DimMapping` rather that pairs of
        // dimensions so `DimMapping` should be allocated before.
        for operand in op.operands() {
            if let Some(dim_map) = operand.mapped_dims() {
                for &(lhs, rhs) in dim_map {
                    self.map_dimensions([lhs, rhs]);
                }
            }
        }
        let id = ir::InstId(self.insts.len() as u32);
        let inst = self.create_inst(id, op, iter_dims)?;
        self.insts.push(inst);
        Ok(id)
    }

    /// Allocates a new memory block.
    pub fn add_mem_block(&mut self, size: u32) -> mem::InternalId {
        self.mem_blocks.alloc_block(size, None)
    }

    /// Create a new logical dimension composed of multiple dimensions to implement
    /// strip-mining.
    pub fn add_logical_dim(
        &mut self,
        size: ir::Size<'a>,
        tiling_factors: Vec<u32>,
        tile_sizes: &[u32],
    ) -> Result<(ir::LogicalDimId, Vec<ir::DimId>), ir::Error> {
        // TODO(strip-mining): allow all tiling factors at all levels
        let logical_id = ir::LogicalDimId(self.logical_dims.len() as u32);
        let dim_ids = (0..tile_sizes.len() + 1)
            .map(|id| ir::DimId((id + self.dims.len()) as u32))
            .collect_vec();
        // Create the objects, but don't add anythin yet so we can rollback if an error
        // occurs.
        let mut dims = Vec::new();
        let tiling_factor = tile_sizes.iter().product();
        let logical_dim = if let Some(size) = size.as_constant() {
            let tiled_size = ir::PartialSize::new(size / tiling_factor, vec![], 1);
            dims.push(Dimension::new(tiled_size, dim_ids[0])?);
            ir::LogicalDim::new_static(logical_id, dim_ids.clone(), size)
        } else {
            let mut tiled_size: ir::PartialSize = size.clone().into();
            tiled_size.mul_divisor(tiling_factor);
            dims.push(Dimension::new(tiled_size, dim_ids[0])?);
            let factors = tiling_factors;
            let static_dims = dim_ids[1..].iter().cloned().collect();
            ir::LogicalDim::new_dynamic(
                logical_id,
                dim_ids[0],
                static_dims,
                factors,
                size,
            )
        };
        for (&id, &size) in dim_ids[1..].iter().zip_eq(tile_sizes) {
            dims.push(Dimension::new(ir::PartialSize::new(size, vec![], 1), id)?);
        }
        // Register the new objects.
        for dim in &dims {
            if dim.possible_sizes().is_some() {
                self.static_dims.push(dim.id());
            }
        }
        self.dims.extend(dims);
        self.logical_dims.push(logical_dim);
        Ok((logical_id, dim_ids))
    }

    /// Specifies two dimensions must have the same size have can be used for point-to-point
    /// communication.
    fn map_dimensions(&mut self, dims: [ir::DimId; 2]) -> ir::DimMappingId {
        self.find_mapping(dims[0], dims[1]).unwrap_or_else(|| {
            let id = ir::DimMappingId(self.dim_mappings.len() as u16);
            let mapping = self.create_mapping(id, dims);
            self.dim_mappings.push(mapping);
            id
        })
    }

    pub(crate) fn freeze(self) -> Function<'a> {
        let mut counter = ir::Counter {
            next_mem: self.mem_blocks.num_internal_blocks(),
            next_inst: self.insts.len(),
            next_dim: self.dims.len(),
            next_dim_mapping: self.dim_mappings.len() as u16,
        };
        let Function {
            signature,
            device,
            insts,
            mut dims,
            static_dims,
            thread_dims,
            mem_insts,
            mut mem_blocks,
            layouts_to_lower,
            induction_vars,
            logical_dims,
            mut dim_mappings,
            values,
        } = self;

        let mut insts = SparseVec::from_vec(
            insts
                .into_iter()
                .map(|inst| inst.map(|inst| inst.freeze(&mut counter)))
                .collect(),
        );
        let induction_vars: Vec<_> = induction_vars
            .into_iter()
            .map(|induction_var| induction_var.freeze(&mut counter))
            .collect();

        let ir::Counter {
            next_mem,
            next_inst,
            next_dim,
            next_dim_mapping,
        } = counter;
        insts.expand_to(next_inst);
        dims.expand_to(next_dim);
        mem_blocks.expand_internal_blocks_to(next_mem);
        dim_mappings.expand_to(next_dim_mapping as usize);

        Function {
            signature,
            device,
            insts,
            dims,
            static_dims,
            thread_dims,
            mem_insts,
            mem_blocks,
            layouts_to_lower,
            induction_vars,
            logical_dims,
            dim_mappings,
            values,
        }
    }
}

impl<'a> Function<'a> {
    /// Returns a `Statement` given its id.
    pub fn block(&self, id: StmtId) -> &Statement<'a> {
        match id {
            StmtId::Inst(id) => &self.insts[id],
            StmtId::Dim(id) => self.dim(id),
        }
    }

    /// Lists all `Statement`s.
    pub fn blocks<'b>(&'b self) -> impl Iterator<Item = &'b Statement<'a>> {
        self.insts
            .iter()
            .map(|x| x as _)
            .chain(self.dims().map(|x| x as _))
    }

    /// Lowers a dim map into a partially defined layout.
    pub(crate) fn lower_dim_map(
        &mut self,
        dst_inst: InstId,
        dst_operand_pos: usize,
    ) -> Result<ir::LoweredDimMap, ()> {
        // TODO(search_space): allow temporary memory generation for reduce operators.
        let (src_inst, data_type, lowered) = {
            match self.insts[dst_inst].operands()[dst_operand_pos] {
                Operand::Inst(src_id, t, dim_map, ir::DimMapScope::Global(lowering)) => {
                    (*src_id, *t, lowering.lower(dim_map))
                }
                Operand::Inst(_, _, _, _) => {
                    debug!(
                        "The dimension mapping {:?}.{} cannot be lowered",
                        dst_inst, dst_operand_pos
                    );
                    return Err(());
                }
                Operand::Reduce(..) => return Err(()),
                _ => panic!(),
            }
        };
        // Activate the new dimensions
        let st_dim_map = self.activate_mapped_dims(&lowered.st_dims_mapping, true);
        let ld_dim_map = self.activate_mapped_dims(&lowered.ld_dims_mapping, false);

        // Activate the temporary memory block
        self.mem_blocks
            .set_lazy_tmp(lowered.mem, data_type, lowered.mem_dimensions());

        // Build and activate the store instruction
        let st_operand =
            Operand::new_inst(self.inst(src_inst), st_dim_map, ir::DimMapScope::Local);
        let st = unwrap!(self.create_inst(
            lowered.store,
            Operator::TmpSt(st_operand, lowered.mem.into()),
            lowered.store_dims().collect(),
        ));
        self.insts.set_lazy(lowered.store, st);

        // Build and activate the load instruction
        let ld = unwrap!(self.create_inst(
            lowered.load,
            Operator::TmpLd(data_type, lowered.mem.into()),
            lowered.load_dims().collect(),
        ));
        self.insts.set_lazy(lowered.load, ld);
        self.insts[dst_inst].lower_dim_map(dst_operand_pos, lowered.load, ld_dim_map);

        Ok(lowered)
    }

    /// Create dimensions with preallocated IDs and maps then to existing dimensions. `mapping`
    /// provides the pairs of old and new IDs along with the mapping ID. Returns a `DimMap`
    /// containing the mapped dimensions. The boolean controls in wich order mappings should be
    /// returned: from the old dimension to the new or the converse.
    fn activate_mapped_dims(
        &mut self,
        mappings: &[(ir::DimMappingId, [ir::DimId; 2])],
        old_to_new: bool,
    ) -> ir::dim::Map {
        let dims = mappings.iter().map(|&(mapping_id, dims)| {
            let [old_dim, new_dim] = dims;
            let dimension = Dimension::with_same_size(new_dim, &self.dims[old_dim]);
            if dimension.possible_sizes().is_some() {
                self.static_dims.push(new_dim);
            }
            self.dims.set_lazy(new_dim, dimension);
            // We can only create the mapping after we activate dimensions.
            let mapping = self.create_mapping(mapping_id, dims);
            self.dim_mappings.set_lazy(mapping_id, mapping);
            if old_to_new {
                (old_dim, new_dim)
            } else {
                (new_dim, old_dim)
            }
        });
        ir::dim::Map::new(dims)
    }
}

impl<'a> std::ops::Deref for Function<'a> {
    type Target = Signature;

    fn deref(&self) -> &Self::Target {
        self.signature
    }
}
