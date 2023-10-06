use super::prelude::{
    util::{Hist, Monoid},
    *,
};

#[derive(Default, Debug)]
struct FileStats {
    structs_count: usize,
    enums_count: usize,
    impls_count: usize,
    /// free-standing functions
    all_fns_count: usize,
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
        self.all_fns_count += 1;
        if matches!(i.vis, syn::Visibility::Public(_)) {
            self.pub_fns_count += 1;
        }
    }
}

#[derive(Clone, Default, Serialize)]
struct Files {
    struct_count: Hist<64>,
    enum_count: Hist<64>,
    impl_block_count: Hist<64>,
    all_fn_count: Hist<64>,
    pub_fn_count: Hist<32>,
}

impl Visit<'_> for Files {
    fn visit_file(&mut self, i: &'_ syn::File) {
        let mut file_stats = FileStats::default();
        syn::visit::visit_file(&mut file_stats, i);
        self.struct_count.observe(file_stats.structs_count);
        self.enum_count.observe(file_stats.enums_count);
        self.impl_block_count.observe(file_stats.impls_count);
        self.all_fn_count.observe(file_stats.all_fns_count);
        self.pub_fn_count.observe(file_stats.pub_fns_count);
    }
}

impl Monoid for Files {
    fn init() -> Self {
        Self::default()
    }

    fn unite(self, rhs: Self) -> Self {
        Self {
            struct_count: self.struct_count + rhs.struct_count,
            enum_count: self.enum_count + rhs.enum_count,
            impl_block_count: self.impl_block_count + rhs.impl_block_count,
            all_fn_count: self.all_fn_count + rhs.all_fn_count,
            pub_fn_count: self.pub_fn_count + rhs.pub_fn_count,
        }
    }
}

pub fn make_collector() -> MetricCollectorBox {
    util::VisitorCollector::new(
        "per_file",
        Files::default(),
        |v| v,
        |v| Monoid::reduce(v.iter().cloned()),
    )
    .make_box()
}

#[cfg(test)]
mod tests {
    use syn::{parse_quote, visit::Visit, File};

    use super::Files;

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
        };
        let syntax_tree: File = syn::parse2(code).unwrap();
        let mut files = Files::default();
        files.visit_file(&syntax_tree);
        assert_eq!(files.struct_count.sum(), 2);
        assert_eq!(files.enum_count.sum(), 1)
    }
}
