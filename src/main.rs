mod constants;
mod help;
mod managers;
mod models;
mod parsers;
mod syncers;
mod utilities;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let args_len = args.len() as i32;

    if args_len == 1 {
        help::print_help();
        utilities::checks::exit_program(-1);
    }

    println!("Argument count: {}", args_len);

    let mut act_mgr = managers::action_managers::ActionManager::default();
    act_mgr.set_params(&args);
    act_mgr.initialize();

    let chosen_act = act_mgr.retrieve_icarus_action();
    chosen_act.print_action_and_flags();

    let mut cmt_mgr = managers::commit_manager::CommitManager {
        ica_action: chosen_act,
    };

    cmt_mgr.commit_action();
}
