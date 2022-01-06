use super::*;
use ckb_testtool::ckb_types::{bytes::Bytes, core::TransactionBuilder, packed::*, prelude::*};
use ckb_testtool::context::Context;
use std::fs::File;
use std::io::Read;

const MAX_CYCLES: u64 = 10_000_000;

#[test]
fn it_works() {
    // deploy contract
    let mut context = Context::default();
    let contract_bin = Loader::default().load_binary("ckb-std-tests");
    let contract_out_point = context.deploy_cell(contract_bin);

    // deploy shared library
    let shared_lib_bin = {
        let mut buf = Vec::new();
        File::open("shared-lib/shared-lib.so")
            .unwrap()
            .read_to_end(&mut buf)
            .expect("read code");
        Bytes::from(buf)
    };
    let shared_lib_out_point = context.deploy_cell(shared_lib_bin);
    let shared_lib_dep = CellDep::new_builder()
        .out_point(shared_lib_out_point)
        .build();

    // prepare scripts
    let lock_script = context
        .build_script(&contract_out_point, Default::default())
        .expect("script");
    let lock_script_dep = CellDep::new_builder().out_point(contract_out_point).build();

    // prepare cells
    let input_out_point = context.create_cell(
        CellOutput::new_builder()
            .capacity(1000u64.pack())
            .lock(lock_script.clone())
            .build(),
        Bytes::new(),
    );
    let input = CellInput::new_builder()
        .previous_output(input_out_point)
        .build();
    let outputs = vec![
        CellOutput::new_builder()
            .capacity(500u64.pack())
            .lock(lock_script.clone())
            .build(),
        CellOutput::new_builder()
            .capacity(500u64.pack())
            .lock(lock_script)
            .build(),
    ];

    let mut outputs_data: Vec<Bytes> = Vec::new();
    outputs_data.push(vec![42u8; 1000].into());
    outputs_data.push(Bytes::new());

    // build transaction
    let tx = TransactionBuilder::default()
        .input(input)
        .outputs(outputs)
        .outputs_data(outputs_data.pack())
        .cell_dep(lock_script_dep)
        .cell_dep(shared_lib_dep)
        .build();
    let tx = context.complete_tx(tx);

    // run
    let cycles = context
        .verify_tx(&tx, MAX_CYCLES)
        .expect("pass verification");
    println!("consumed cycles {}", cycles);
}

#[test]
fn test_exec_by_index() {
    let mut context = Context::default();
    let caller_bin = Loader::default().load_binary("exec-caller");
    let caller_out_point = context.deploy_cell(caller_bin);
    let callee_bin = Loader::default().load_binary("exec-callee");
    let callee_out_point = context.deploy_cell(callee_bin);

    let caller_lock_script_dep = CellDep::new_builder()
        .out_point(caller_out_point.clone())
        .build();
    let callee_lock_script_dep = CellDep::new_builder().out_point(callee_out_point).build();

    let caller_lock_script = context
        .build_script(&caller_out_point, Bytes::new())
        .unwrap();

    let input_out_point = context.create_cell(
        CellOutput::new_builder()
            .capacity(1000u64.pack())
            .lock(caller_lock_script.clone())
            .build(),
        Bytes::new(),
    );
    let input = CellInput::new_builder()
        .previous_output(input_out_point)
        .build();
    let outputs = vec![CellOutput::new_builder()
        .capacity(500u64.pack())
        .lock(ScriptBuilder::default().build())
        .build()];
    let outputs_data = vec![Bytes::new()];

    // build transaction
    let tx = TransactionBuilder::default()
        .input(input)
        .outputs(outputs)
        .outputs_data(outputs_data.pack())
        .cell_dep(callee_lock_script_dep)
        .cell_dep(caller_lock_script_dep)
        .build();
    let tx = context.complete_tx(tx);
    // run
    let cycles = context
        .verify_tx(&tx, MAX_CYCLES)
        .expect("pass verification");
    println!("consumed cycles {}", cycles);
}

#[test]
fn test_exec_by_code_hash() {
    let mut context = Context::default();
    let caller_bin = Loader::default().load_binary("exec-caller-by-code-hash");
    let caller_out_point = context.deploy_cell(caller_bin);
    let callee_bin = Loader::default().load_binary("exec-callee");
    let callee_out_point = context.deploy_cell(callee_bin);

    let caller_lock_script_dep = CellDep::new_builder()
        .out_point(caller_out_point.clone())
        .build();
    let callee_lock_script_dep = CellDep::new_builder().out_point(callee_out_point).build();

    let caller_lock_script = context
        .build_script(&caller_out_point, Bytes::new())
        .unwrap();

    let input_out_point = context.create_cell(
        CellOutput::new_builder()
            .capacity(1000u64.pack())
            .lock(caller_lock_script.clone())
            .build(),
        Bytes::new(),
    );
    let input = CellInput::new_builder()
        .previous_output(input_out_point)
        .build();
    let outputs = vec![CellOutput::new_builder()
        .capacity(500u64.pack())
        .lock(ScriptBuilder::default().build())
        .build()];
    let outputs_data = vec![Bytes::new()];

    // build transaction
    let tx = TransactionBuilder::default()
        .input(input)
        .outputs(outputs)
        .outputs_data(outputs_data.pack())
        .cell_dep(callee_lock_script_dep)
        .cell_dep(caller_lock_script_dep)
        .build();
    let tx = context.complete_tx(tx);
    // run
    let cycles = context
        .verify_tx(&tx, MAX_CYCLES)
        .expect("pass verification");
    println!("consumed cycles {}", cycles);
}
