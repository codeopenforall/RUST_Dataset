    let mut slot: MaybeUninit<Settings> = MaybeUninit::uninit();
    slot.as_mut_ptr().write(Settings::new(42));
