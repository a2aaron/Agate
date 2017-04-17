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


struct State<'a> {
    registers: Vec<Register>,
    stackFrames: Vec<StackFrame<'a>>,
    program_counter: usize,
}

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

fn step<'a>(mut state: State, instr: Instruction) {
    use self::Instruction::*;
    match instr {
        MathOp {op, left, right, out} => {
            use self::MathOp::*;
            match op {
                Add => state.registers[out] = state.registers[left] + state.registers[right],
                Sub => state.registers[out] = state.registers[left] - state.registers[right],
                Mul => state.registers[out] = state.registers[left] * state.registers[right],
                Div => state.registers[out] = state.registers[left] / state.registers[right],
                Mod => state.registers[out] = state.registers[left] % state.registers[right],
                AND => state.registers[out] = state.registers[left] & state.registers[right],
                OR => state.registers[out] = state.registers[left] | state.registers[right],
                XOR => state.registers[out] = state.registers[left] ^ state.registers[right],
            }
        },
        Store(index, value) => {
            state.registers[index] = value
        },
        Move {from, to} => {
            state.registers[to] = state.registers[from]
        },
        Function => unimplemented!(),
        Return => unimplemented!(),
        Closure => unimplemented!(),
        Jump(distance) => {
            state.program_counter += distance
        },
        JumpIf {op, left, right, distance} => {
            use self::CompareOp::*;
            let should_jump: bool;
            match op {
                Equals => should_jump = left == right,
                LessThan => should_jump = left < right,
                GreaterThan => should_jump = left > right,
                LessOrEqualTo => should_jump = left <= right,
                GreaterOrEqualTo => should_jump = left >= right,
            }

            if should_jump {
                state.program_counter += distance
            }
        }
    }
}

#[cfg(test)]
mod tests {
    fn it_words() {

    }
}
