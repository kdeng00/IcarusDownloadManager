use std::env;

mod Managers;

fn print_help() {
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Something went wrong");
        std::process::exit(-1);
    }


    // let mut actmgr = actionmanager::ActionManager{params: args };
//    let chosenAction = actmgr.retrieve_icarus_action();
//
//    chosenAction.print_action_and_flags();
//
//    let commitmgr: CommitManager = CommitManager(chosenAction);
//    commitmgr.commit_action();
}
