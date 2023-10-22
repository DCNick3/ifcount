use super::prelude::{
    util::{Monoid, Observer, Unaggregated},
    *,
};

#[derive(Default, Debug)]
struct FileStats {
    structs_count: usize,
    enums_count: usize,
    impls_count: usize,
    // duplicates one of the RCA metrics
    // /// free-standing functions
    // all_fns_count: usize,
    /// public free-standing functions
    pub_fns_count: usize,
}

impl Visit<'_> for FileStats {
    fn visit_item_struct(&mut self, _i: &'_ syn::ItemStruct) {
        self.structs_count += 1;
    }

    fn visit_item_enum(&mut self, _i: &'_ syn::ItemEnum) {
        self.enums_count += 1;
    }

    fn visit_item_impl(&mut self, _i: &'_ syn::ItemImpl) {
        self.impls_count += 1;
    }

    fn visit_item_fn(&mut self, i: &'_ syn::ItemFn) {
        // self.all_fns_count += 1;
        if matches!(i.vis, syn::Visibility::Public(_)) {
            self.pub_fns_count += 1;
        }
    }
}

#[derive(Clone, Default, Serialize)]
struct Files<Obs = Unaggregated> {
    struct_count: Obs,
    enum_count: Obs,
    impl_block_count: Obs,
    // all_fn_count: Hist,
    pub_fn_count: Obs,
}

impl<Obs: Observer> Visit<'_> for Files<Obs> {
    fn visit_file(&mut self, i: &'_ syn::File) {
        let mut file_stats = FileStats::default();
        syn::visit::visit_file(&mut file_stats, i);
        self.struct_count.observe(file_stats.structs_count);
        self.enum_count.observe(file_stats.enums_count);
        self.impl_block_count.observe(file_stats.impls_count);
        // self.all_fn_count.observe(file_stats.all_fns_count);
        self.pub_fn_count.observe(file_stats.pub_fns_count);
    }
}

impl<T: Monoid + Default> Monoid for Files<T> {
    fn init() -> Self {
        Self::default()
    }

    fn unite(self, rhs: Self) -> Self {
        Self {
            struct_count: self.struct_count.unite(rhs.struct_count),
            enum_count: self.enum_count.unite(rhs.enum_count),
            impl_block_count: self.impl_block_count.unite(rhs.impl_block_count),
            // all_fn_count: self.all_fn_count + rhs.all_fn_count,
            pub_fn_count: self.pub_fn_count.unite(rhs.pub_fn_count),
        }
    }
}

pub fn make_collector<
    Obs: Observer + Default + Serialize + Clone + Monoid + Send + Sync + 'static,
>() -> MetricCollectorBox {
    util::VisitorCollector::new(
        "per_file",
        Files::<Obs>::default(),
        |v| v,
        |v| Monoid::reduce(v.iter().cloned()),
    )
    .make_box()
}

#[cfg(test)]
mod tests {
    use super::Files;
    use crate::collector::metrics::util::check;
    use expect_test::expect;
    use syn::parse_quote;

    #[test]
    fn count_file() {
        let code = parse_quote! {
            struct A;

            #[derive(Debug)]
            struct Name {
                field: Type
            }

            #[derive(Debug)]
            enum Name {
                Variant1,
                Variant2,
            }

            impl Name {}
            impl Name {}

            pub fn pub_fn_1() {}
            pub fn pub_fn_2() {}
            pub fn pub_fn_3() {}
        };
        check::<Files>(
            code,
            expect![[r#"
            {
              "struct_count": {
                "sum": 2,
                "avg": 2.0,
                "mode": 2
              },
              "enum_count": {
                "sum": 1,
                "avg": 1.0,
                "mode": 1
              },
              "impl_block_count": {
                "sum": 2,
                "avg": 2.0,
                "mode": 2
              },
              "pub_fn_count": {
                "sum": 3,
                "avg": 3.0,
                "mode": 3
              }
            }"#]],
        );
    }
}
