#[macro_export]
macro_rules! once {
    ($($arg:tt)*) => {
        {
            static ONCE: std::sync::Once = std::sync::Once::new();
            const UNIQUE_IDENTIFIER: &'static str = concat!(stringify!(__ONCE__), line!());

            ONCE.call_once(|| {
                println!("{:?}", format_args!($($arg)*));
            });

            #[allow(unused_variables)]
            let _guard = &UNIQUE_IDENTIFIER; // Ensures the const is referenced and not optimized out
        }
    };
}
