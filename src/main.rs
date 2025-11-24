// mod old_main;
mod serde_version;

fn main() {
    // old_main::main();
    let res = serde_version::main();
    match res{
        Ok(()) => (),
        Err(error) => panic!("Problem opening the file: {error:?}")
    };
}
