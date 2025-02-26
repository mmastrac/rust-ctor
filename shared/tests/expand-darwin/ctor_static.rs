shared::ctor_parse!(
    #[ctor]
    static STATIC_CTOR: HashMap<u32, &'static str> = unsafe {
        let m = HashMap::new();
        m
    };
);
