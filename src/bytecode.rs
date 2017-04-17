use values::Value;

// some notes:
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


/// A stack frame consists of a function (really a reference), a counter which
/// tracks where in the function the execution is, the variables the function 
/// needs (this includes any closure or local variables)
#[derive(Debug)]
struct StackFrame<'a> {
    function: &'a Function,
    function_counter: usize,
    args: Vec<Register>,
    locals: Vec<Register>,
    closures: Vec<ClosureValue>,
}

/// A function is a structure which knows about what arguements it has as well
/// as the instructions to run, but NOT the state of the function itself
#[derive(Debug)]
struct Function {
    num_args: usize,
    frame_size: usize,
    instructions: Vec<Instruction>,
}

type ClosureValue = Value;

/// A Register simply holds a number
type Register = i32;
/// Registers are named simply as "Register" and then a number
type RegisterIndex = usize;
#[derive(Debug)]
enum Instruction {
    // Preform math on two registers, storing the result into a third register
    MathOp {op: MathOp, left: RegisterIndex, right: RegisterIndex, out: RegisterIndex},
    // Store value into the specified register
    Store(RegisterIndex, i32),
    // Move a value from a register into another
    Move {from: RegisterIndex, to: RegisterIndex},
    Function, /* @TODO figure out function symbols and other stuff */
    Return, /* @TODO either a number or a pointer to a stack frame goes here */
    Closure, /* @TODO i have no idea what this would look like */
    Jump(usize), // Jump (relative)
    // Compare two registers, if the result is true then jump
    JumpIf {op: CompareOp, left: RegisterIndex, right: RegisterIndex, distance: usize},
}

#[derive(Debug)]
enum CompareOp {
    Equals,
    LessThan,
    GreaterThan,
    LessOrEqualTo,
    GreaterOrEqualTo,
}

#[derive(Debug)]
enum MathOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    AND,
    OR,
    XOR,
}

#[cfg(test)]
mod tests {
    fn it_words() {

    }
}
