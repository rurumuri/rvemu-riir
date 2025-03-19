mod elfdef;
mod machine;
mod mmu;

fn main() {
    if std::env::args().len() < 2 {
        println!("Usage: {} <program>", std::env::args().nth(0).unwrap());
        std::process::exit(1);
    }
    for (i, arg) in std::env::args().enumerate() {
        if i < 1 {
            continue;
        } else {
            let mut machine: machine::machine_t = machine::machine_t::new();
            machine.machine_load_program(&arg);
            println!("entry: {:#x}", machine.get_mmu_entry());
        }
    }
}
