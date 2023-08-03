#![no_main]
use libfuzzer_sys::fuzz_target;
use definition::Plan;
use core::planner::create_planning;

fuzz_target!(|data: Plan| { plan(&data) });

fn plan(data: &Plan) {
    create_planning(&data);
}