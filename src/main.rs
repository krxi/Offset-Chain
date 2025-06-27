use std::mem::size_of;
use toy_arms::external::module::Module;
use toy_arms::external::process::Process;
use toy_arms::external::read;


use winapi::um::winnt::HANDLE;
fn read_offset_chain(handle: HANDLE, base: u32, offsets: &[u32]) -> u32 {
    //changes every round of loop
    let mut addr = base;

    //iteration
    for &offset in offsets {
        let mut temp_next: u32 = 0;
        let val = read::<u32>(
            &handle,
            addr as usize + offset as usize,
            size_of::<u32>(),
            &mut temp_next as *mut u32,
        );
        
        //error handle
        if val.is_err() {
            println!("An error occured when reading memory");
            return 0;
        };

        //changing main address with temp variable as u32
        addr = temp_next as u32;
    }

    return addr as u32;
}

fn main() {
    let game: Process;
    match Process::from_process_name("test.exe") {
        Ok(p) => game = p,
        Err(e) => {
            println!("{}", e);
            return;
        }
    }
    
    let client: Module;
    match game.get_module_info("test.exe") {
        Ok(m) => client = m,
        Err(e) => {
            println!("{}", e);
            return;
        }
    }

    let mut start_address: u32 = 0;
    let _ = read::<u32>(
        &game.handle,
        client.base_address + 0x025FAE88 as usize,
        size_of::<u32>(),
        &mut start_address as *mut u32,
    );
    
    let final_addr = read_offset_chain(game.handle, start_address, &[0x984, 0x4C, 0xACC, 0x3B0]);
    if final_addr != 0 {
        println!("Your final address is: {:x}",final_addr);
    }
}
