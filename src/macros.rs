#[macro_export]
macro_rules! rules {
    () => (
        $crate::Rules::new()
    );
    ($($rule:expr),+ $(,)?) => {
        {
            let mut r = $crate::Rules::new();
            $(r = r.add($rule);)+
            r
        }
    };
}