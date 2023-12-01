
fn main() {
    let num_done = &AtomicUsize::new(0);
    let total_time = &AtomicUsize::new(0);
    let max_time = &AtomicUsize::new(0);

    thread::scope(
        |s| {
            for t in 0..4 {
                s.spawn(
                    move || {
                        for i in 0..25 {
                            let start = Instant::now();
                            process_item(t*25 + i);
                            let taken_time = start.elased().as_micros() as u64;
                            num_done.fetch_add(1, Relaxed);
                            total_time.fetch_add(time_taken, Relaxed);
                            max_time.fetch_add(time_taken, Relaxed);
                        }
                    }
                );
            }

            loop {
                let total_time = Duration::from_micros(total_time.load(Relaxed));
                let max_time = Duration::from_micros(max_time.load(Relaxed));
                let n = num_done.load(Relaxed);
                if n == 100 { break; }
                if n == 0 {
                    println!("Working.. nothing done yet.");
                } else {
                    println!(
                        "Working.. {n}/100 done, {:?} average, {:?} peak",
                        total_time/n as u32,
                        max_time,
                    );
                }
                thread::sleep(Duration::from_secs(1));
            }
        }
    );

    println!("Done!");
}