use rw_memory::*;

fn main() {
    let name = "bin-test.exe";
    //let name = "factorio.exe";

    let proc_id = get_proc_id_from_string(name.to_string());
    println!("Process ID: {:X}", proc_id);

    let h_process = open_process(proc_id);
    let module_base = get_module_base_address(h_process, name.to_string(), proc_id);
    println!("Module base adress: 0x{:X}", module_base);

    // multi-popinters jumping
    //let addr = jump(
    //    h_process,
    //    module_base,
    //    &[0x01CC39F0, 0x60, 0x158, 0x60, 0x8, 0x98, 0x30, 0x30],
    //);
    let addr = jump(
        h_process,
        module_base,
        &[0x000261E0, 0x18, 0x88, 0x830, 0x0, 0x0, 0x10, 0x5C4],
    );
    println!("Addr: 0x{:X}", addr);

    // Read
    let mut v = 10;
    match read::read_process_memory(h_process, addr, &mut v) {
        Ok(it) => it,
        Err(err) => println!("{}", err),
    };

    // Write
    match write::write_process_memory(h_process, addr, v + 10) {
        Ok(it) => it,
        Err(err) => println!("{}", err),
    };
    println!("{} -> {} + 10 = {}", v, v, v + 10);
}
