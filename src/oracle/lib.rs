// (https://docs.bandchain.org/develop/custom-scripts/oracle-script/tutorial)
use obi::{OBIDecode, OBIEncode, OBISchema};
use owasm_kit::{execute_entry_point, ext, oei, prepare_entry_point};

#[derive(OBIDecode, OBISchema)]
struct Input {
    repeat: u64,
}

#[derive(OBIEncode, OBISchema)]
struct Output {
    response: String,
}

const DATA_SOURCE_ID: i64 = 327;
const EXTERNAL_ID: i64 = 0;

#[no_mangle]
fn prepare_impl(_input: Input) {
    oei::ask_external_data(
        EXTERNAL_ID,    // The assigned external ID for this data source
        DATA_SOURCE_ID, // The data source to call by ID
        b"",            // Calldata to be sent to the data source
    )
}

#[no_mangle]
fn execute_impl(input: Input) -> Output {
    // Total Supply
    // Output {
    //     total_supply: ext::load_majority::<String>(EXTERNAL_ID).unwrap(),
    // }

    // Price
    let raw_result = ext::load_input::<String>(EXTERNAL_ID); // Raw results from the given external ID
    let result: Vec<String> = raw_result.collect();
    let majority_result: String = ext::stats::majority(result).unwrap(); // Majority result
    Output {
        response: majority_result.repeat(input.repeat as usize),
    }
}

prepare_entry_point!(prepare_impl);
execute_entry_point!(execute_impl);
