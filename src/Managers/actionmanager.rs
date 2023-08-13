mod Models;

mod Managers {

    struct ActionManager {
        // pub action: String,
        // flags: Vec<Flags>,
        // pub params: Vec<String>,
        pub params: Vec<String>
        // pub paramcount: i32
    }


    impl ActionManager {
        fn retrieve_icarus_action() -> Models::IcarusAction {
            Models::IcarusAction{t: 4}
        }
    }
}
