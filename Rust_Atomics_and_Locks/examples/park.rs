use std::collections::VecDequeue;

fn main() {
    let queue = Mutex::new(VecDeque::new());

    thread::scope(
        |s| {
            let t = s.spawn(
                || loop {
                    let item = queue.lock().unwrap().pop_front();
                    if let Some(item) = item {
                        dbg!(item);
                    } else {
                        thread::park();
                    }
                }
            )

            for i in 0.. {
                queue.lock().unwrap().push_back(i);
                t.thread().unwrap();
                thread::sleep(Duration::from_secs(1));
            }
        }
    )
}