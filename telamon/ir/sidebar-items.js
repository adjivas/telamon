initSidebarItems({"enum":[["AccessPattern",""],["BBId","Provides a unique identifer for a basic block."],["BinOp","Represents binary arithmetic operators."],["DimMapScope","Indicates how dimensions can be mapped. The `L` type indicates how to lower mapped dimensions."],["Error","An error occuring while manipulating an ir instance."],["Operand","Represents an instruction operand."],["Operator","The operation performed by an instruction."],["Stride","A stride on a given dimensions."],["Type","Values and intructions types."],["TypeError","Errors that can be raised when creating an IR instance."]],"mod":[["dim","Defines iteration dimensions properties."],["mem","A module for handling accesses to the device memory."],["op","Defines operators."],["prelude","Defines traits to import in the environment to use the IR."]],"struct":[["Counter","A wrapper used to count extra dimensions that will be needed in the future and allocates IDs for them. This is used when freezing in order to pre-allocate IDs for the various objects (internal memory block, instructions, dimensions, etc.) required for future lowering."],["DimId","Provides a unique identifier for iteration dimensions."],["DimMap","Represents a mapping between dimenions."],["Dimension","Represents an iteration dimension."],["Function","Describes a function and the set of its possible implementation."],["IndVarId","Unique identifier for `InductionVar`"],["InductionVar","A multidimentional induction variable. No dimension should appear twice in dims."],["InstId","Uniquely identifies an instruction."],["Instruction","Represents an instruction."],["LoweredDimMap","A point-to-point communication lowered into a store and a load."],["LoweringMap",""],["NewObjs","Stores the objects created by a lowering."],["Parameter","Represents an argument of a function."],["Signature","Holds the signature of a function."],["Size","The size of an iteration dimension. The size is of the form: `(factor * dividend_0 * dividend_1 * ...)) / divisor` where the reminder of the division is null."]],"trait":[["BasicBlock","Represents a basic block in an Exhaust function."]]});