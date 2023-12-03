use std::sync::atomic::AtomicPtr;

fn get_data() -> &'static Data {
    static PTR: AtomicPtr<Data> = AtomicPtr::new(std::ptr::null_mut());

    let mut p = PTR.load(Acquire);

    if p.is_null() {
        p = Box::into_raw(Box::new(generate_data()));
        if let Err(e) = PTR.compare_exchange(
            std::ptr::null_mut(),
            p,
            Release,
            Acquire
        ) {
            drop(unsafe{Box::from_raw(p)});
            p = e;
        }
    }

    unsafe{&*p}
}