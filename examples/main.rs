use affinity::*;
use std::error::Error;

#[cfg(target_os = "windows")] 
fn bind_process() -> Result<(), Box<dyn Error>> {
    // Sets the whole proccess affinity
    println!("Binding process to cores : [0]");
    println!("(This should overwrite threads affinities previously set)");
    set_process_affinity(&[0])?;
    println!("\tCurrent thread affinity : {:?}", get_thread_affinity()?);
    println!("\tCurrent process affinity : {:?}", get_process_affinity()?);
    println!("\tTotal cores : {}", get_core_num());
    Ok(())
}

pub fn main() -> Result<(), Box<dyn Error>> {

    println!("Total cores : {}", get_core_num());
    
    let cores = (0..get_core_num()).step_by(2).collect::<Vec<usize>>();
    println!("Binding thread to cores : {:?}", &cores);
    set_thread_affinity(&cores)?;
    println!("\tCurrent thread affinity : {:?}", get_thread_affinity()?);
    println!("\tTotal cores : {}", get_core_num());

    #[cfg(target_os = "windows")]
    bind_process()?;

    Ok(())
}