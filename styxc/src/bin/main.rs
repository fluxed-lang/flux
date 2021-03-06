use core::mem;
use std::env;
use std::fs;
use styxc::jit;

fn main() -> Result<(), ()> {
    let args: Vec<String> = env::args().collect();
    let filename = args.get(1).expect("target file not found");
    
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    // Create the JIT instance, which manages all generated functions and data.
    let mut jit = jit::JIT::default();

    run_file(&mut jit, contents).expect("success");

    Ok(())
}

fn run_file(jit: &mut jit::JIT, code: String) -> Result<isize, String> {
    unsafe { run_code(jit, code.as_str(), (1, 0)) }
}



/// Executes the given code using the cranelift JIT compiler.
///
/// Feeds the given input into the JIT compiled function and returns the resulting output.
///
/// # Safety
///
/// This function is unsafe since it relies on the caller to provide it with the correct
/// input and output types. Using incorrect types at this point may corrupt the program's state.
unsafe fn run_code<I, O>(jit: &mut jit::JIT, code: &str, input: I) -> Result<O, String> {
    // Pass the string to the JIT, and it returns a raw pointer to machine code.
    let code_ptr = jit.compile(code)?;
    // Cast the raw pointer to a typed function pointer. This is unsafe, because
    // this is the critical point where you have to trust that the generated code
    // is safe to be called.
    let code_fn = mem::transmute::<_, fn(I) -> O>(code_ptr);
    // And now we can call it!
    Ok(code_fn(input))
}
