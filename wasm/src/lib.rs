use anyhow::Error;
use cairo_rs::{
    hint_processor::builtin_hint_processor::{
        builtin_hint_processor_definition::BuiltinHintProcessor,
        hint_utils::{get_integer_from_var_name, get_ptr_from_var_name},
    },
    types::{
        program::Program,
        relocatable::{MaybeRelocatable, Relocatable},
    },
    vm::{runners::cairo_runner::CairoRunner, vm_core::VirtualMachine},
};
use num_bigint::BigInt;
use std::path::Path;
use std::{collections::HashMap, io::Cursor};
use wasm_bindgen::prelude::*;

macro_rules! bigint {
    ($val : expr) => {
        Into::<BigInt>::into($val)
    };
}

macro_rules! mayberelocatable {
    ($val1 : expr, $val2 : expr) => {
        MaybeRelocatable::from(($val1, $val2))
    };
    ($val1 : expr) => {
        MaybeRelocatable::from((bigint!($val1)))
    };
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(msg: &str);
}

pub fn run_cairo_program(
    num_pts: u32,
    theta_0_deg: i32,
    v_0: u32,
) -> Result<VirtualMachine, Error> {
    const PROGRAM_JSON: &str = include_str!("./projectile_plot_compiled.json");

    let program = Program::from_reader(Cursor::new(PROGRAM_JSON), Some("projectile_path"))?;

    let mut cairo_runner = CairoRunner::new(&program, "all", false).unwrap();
    let mut vm = VirtualMachine::new(program.prime, true);

    let hint_processor = BuiltinHintProcessor::new_empty();

    let entrypoint = program
        .identifiers
        .get(&format!("__main__.{}", "projectile_path"))
        .unwrap()
        .pc
        .unwrap();

    cairo_runner.initialize_builtins(&mut vm).unwrap();
    cairo_runner.initialize_segments(&mut vm, None);

    cairo_runner
        .run_from_entrypoint(
            entrypoint,
            vec![
                &MaybeRelocatable::from((2, 0)), //range check builtin
                &mayberelocatable!(num_pts),
                &mayberelocatable!(theta_0_deg),
                &mayberelocatable!(v_0),
            ],
            false,
            true,
            true,
            &mut vm,
            &hint_processor,
        )
        .unwrap();
    Ok(vm)
}

#[derive(Deserialize, Serialize)]
pub struct CairoOutput {
    pub x_positions: Vec<BigInt>,
    pub y_positions: Vec<BigInt>,
}

fn print_two_array_hint(
    vm: &mut VirtualMachine,
    _exec_scopes: &mut ExecutionScopes,
    ids_data: &HashMap<String, HintReference>,
    ap_tracking: &ApTracking,
    _constants: &HashMap<String, BigInt>,
) -> Result<(), VirtualMachineError> {
    let x_len = get_integer_from_var_name("x_fp_s_len", vm, ids_data, ap_tracking)?
        .to_u32_digits()
        .1[0];
    let y_len = get_integer_from_var_name("y_fp_s_len", vm, ids_data, ap_tracking)?
        .to_u32_digits()
        .1[0];
    let x = get_ptr_from_var_name("x_fp_s", vm, ids_data, ap_tracking)?;
    let y = get_ptr_from_var_name("y_fp_s", vm, ids_data, ap_tracking)?;
    for i in 0..x_len as usize {
        let word_address = Relocatable {
            segment_index: x.segment_index,
            offset: i,
        };
        let value = vm.get_integer(&word_address)?;
        console_log!("{}", value);
    }
    for i in 0..y_len as usize {
        let word_address = Relocatable {
            segment_index: y.segment_index,
            offset: i,
        };
        let value = vm.get_integer(&word_address)?;
        console_log!("{}", value);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_function() {
        let vm = run_cairo_program(25, 60, 40).unwrap();
        let mut x_positions: Vec<BigInt> = vec![];
        let mut y_positions: Vec<BigInt> = vec![];
        if let [x_len_big, maybe_x, y_len_big, maybe_y] = &vm.get_return_values(4).unwrap()[..] {
            let x_len = x_len_big.get_int_ref().unwrap().to_u32_digits().1[0];
            let y_len = y_len_big.get_int_ref().unwrap().to_u32_digits().1[0];
            let x = maybe_x.get_relocatable().unwrap();
            let y = maybe_y.get_relocatable().unwrap();
            for i in 0..x_len {
                let word_address = Relocatable {
                    segment_index: x.segment_index,
                    offset: i as usize,
                };
                x_positions.push(vm.get_integer(&word_address).unwrap().into_owned());
            }
            for i in 0..y_len {
                let word_address = Relocatable {
                    segment_index: y.segment_index,
                    offset: i as usize,
                };
                y_positions.push(vm.get_integer(&word_address).unwrap().into_owned());
            }
        }
    }
}
