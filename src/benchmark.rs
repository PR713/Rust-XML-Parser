use std::error::Error;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use sysinfo::System;

#[derive(Debug)]
pub struct BenchmarkResult {
    pub name: String,
    pub duration: Duration,
    pub memory_used_kb: u64,
    pub peak_memory_kb: u64
}

pub fn run_benchmark<F>(name: &str, f: F) -> Result<BenchmarkResult, Box<dyn Error>>
where
    F: FnOnce() -> Result<(), Box<dyn Error>>,
{
    let pid = sysinfo::Pid::from(std::process::id() as usize);
    let max_memory = Arc::new(Mutex::new(0u64));
    let should_stop = Arc::new(Mutex::new(false));

    // Wątek monitorujący pamięć
    let memory_thread = {
        let max_memory = Arc::clone(&max_memory);
        let should_stop = Arc::clone(&should_stop);

        thread::spawn(move || {
            let mut system = System::new();
            let mut last_print = Instant::now();

            while !*should_stop.lock().unwrap() {
                system.refresh_process(pid);
                if let Some(process) = system.process(pid) {
                    let current_mem = process.memory() / 1024;
                    let mut max_mem = max_memory.lock().unwrap();

                    if current_mem > *max_mem {
                        *max_mem = current_mem;
                    }

                    if last_print.elapsed() > Duration::from_millis(100) {
                        last_print = Instant::now();
                    }
                }
                thread::sleep(Duration::from_millis(50));
            }
        })
    };

    // Początkowy pomiar pamięci
    let initial_memory = {
        let mut system = System::new();
        system.refresh_process(pid);
        system.process(pid)
            .map(|p| p.memory() / 1024)
            .unwrap_or(0)
    };

    let start_time = Instant::now();
    f()?;
    let duration = start_time.elapsed();

    // Zatrzymaj wątek monitorujący
    *should_stop.lock().unwrap() = true;
    memory_thread.join().unwrap();

    // Końcowe wyniki
    let final_max_memory = *max_memory.lock().unwrap();
    let memory_used_kb = final_max_memory.saturating_sub(initial_memory);

    println!("\n╔═════════════════════════════════════════════════════════════════════╗");
    println!("║ {:^26} ║", name);
    println!("╠═════════════════════════════════════════════════════════════════════");
    println!("║ {:<12}: {:>10.2?}                                                   ", "Time", duration);
    println!("║ {:<12}: {:>10} KB ", "RAM Memory (diff between the start and the end)", memory_used_kb);
    println!("║                                                                     " );
    println!("║ {:<12}: {:>10} KB                                                   ", "Peak RAM", final_max_memory);
    println!("╚═════════════════════════════════════════════════════════════════════╝");


    Ok(BenchmarkResult {
        name: name.to_string(),
        duration,
        memory_used_kb,
        peak_memory_kb: final_max_memory,
    })
}