use expect_test::Expect;
use serde::Serialize;
use syn::{visit::Visit, File};

pub fn check<T>(syntax_tree: File, expect: Expect)
where
    T: for<'ast> Visit<'ast> + Default + Serialize,
{
    let mut metric = T::default();
    metric.visit_file(&syntax_tree);
    let representation = serde_json::to_string_pretty(&metric).unwrap();
    expect.assert_eq(&representation);
}
