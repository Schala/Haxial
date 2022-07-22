type Variable = (u64, u64, Vec<u8>);

enum Node {
	/// type hash, variables
	Record(u64, Vec<Variable>),

	/// type hash, variables
	Tuple(u64, Vec<Variable>),

	/// ID hash, type hash, binary data
	Variable(u64, u64, Vec<u8>),
}
