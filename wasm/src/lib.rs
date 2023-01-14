mod utils;

use cairo_rs::{
    hint_processor::builtin_hint_processor::hint_utils::get_integer_from_var_name,
    hint_processor::{
        builtin_hint_processor::builtin_hint_processor_definition::{
            BuiltinHintProcessor, HintFunc,
        },
        hint_processor_definition::HintReference,
    },
    serde::deserialize_program::ApTracking,
    types::{exec_scope::ExecutionScopes, program::Program},
    vm::{
        errors::vm_errors::VirtualMachineError, runners::cairo_runner::CairoRunner,
        vm_core::VirtualMachine,
    },
};

use num_bigint::{BigInt, Sign};
use std::io::Cursor;
use wasm_bindgen::prelude::*;

use std::collections::HashMap;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(msg: &str);
}

#[cfg(feature = "console_error_panic_hook")]
#[wasm_bindgen(start)]
pub fn start() {
    crate::utils::set_panic_hook();
}

macro_rules! console_log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

fn print_a_hint(
    vm: &mut VirtualMachine,
    _exec_scopes: &mut ExecutionScopes,
    ids_data: &HashMap<String, HintReference>,
    ap_tracking: &ApTracking,
    _constants: &HashMap<String, BigInt>,
) -> Result<(), VirtualMachineError> {
    let a = get_integer_from_var_name("a", vm, ids_data, ap_tracking)?;
    println!("{}", a);
    console_log!("{}", a);
    Ok(())
}

#[wasm_bindgen(js_name = runCairoProgram)]
pub fn run_cairo_program() -> Result<(), JsError> {
    const PROGRAM_JSON: &str = include_str!("./game.json");

    let program = Program::from_reader(Cursor::new(PROGRAM_JSON), Some("main"))?;
    let mut runner = CairoRunner::new(&program, "all", false)?;
    let mut vm = VirtualMachine::new(
        BigInt::new(Sign::Plus, vec![1, 0, 0, 0, 0, 0, 17, 134217728]),
        false,
    );

    let hint = HintFunc(Box::new(print_a_hint));

    let mut hint_processor = BuiltinHintProcessor::new_empty();

    hint_processor.add_hint(String::from("print(ids.a)"), hint);

    let end = runner.initialize(&mut vm)?;

    runner.initialize_builtins(&mut vm)?;
    runner.initialize_segments(&mut vm, None);

    runner.run_until_pc(end, &mut vm, &hint_processor)?;

    let mut buffer = Cursor::new(Vec::new());
    runner.write_output(&mut vm, &mut buffer)?;
    log(String::from_utf8(buffer.into_inner())?.as_str());

    // log("Hello from Rust!");

    // cairo_run(
    //     program_json,
    //     "main",
    //     false,
    //     true,
    //     "small",
    //     false,
    //     &hint_processor,
    // )
    // .expect("Couldn't run program");

    Ok(())
}
