use values::Value;
// start at line 1
// (some array of code for current fn)
// program counter -> count up for each line
// (make sure you jump the counter correctly for jmp)
// push a new frame (more )

// a stack frame has:
// pointer to current function
// program counter of function
// local vars of func
// closure for the func (maybe empty)

// closure needs to know what func it's supposted be in

struct StackFrame<'a> {
    function: &'a Function,
    function_counter: usize,
    args: Vec<Value>,
    locals: Vec<Value>,
    closures: Vec<ClosureValue>,
}
struct Function {
    num_args: usize,
    frame_size: usize,
    instructions: Vec<Instruction>,
}

type ClosureValue = Value;

enum Instruction {
    // @TODO: what do i put here
}

#[cfg(test)]
mod tests {
    fn it_words() {

    }
}
