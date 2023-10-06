use super::prelude::*;
use util::{Hist, Monoid};

#[derive(Default, Clone, Serialize)]
struct MacroStats {
    argument_size: Hist<16>,
    count_per_file: Hist<16>,
}

impl Monoid for MacroStats {
    fn init() -> Self {
        Self::default()
    }
    fn unite(self, rhs: Self) -> Self {
        Self {
            argument_size: self.argument_size + rhs.argument_size,
            count_per_file: self.count_per_file + rhs.count_per_file,
        }
    }
}

impl Visit<'_> for MacroStats {
    fn visit_file(&mut self, i: &'_ syn::File) {
        let start_count = self.argument_size.count();
        syn::visit::visit_file(self, i);
        let end_count = self.argument_size.count();

        self.count_per_file
            .observe((end_count - start_count) as usize);
    }

    fn visit_macro(&mut self, i: &'_ syn::Macro) {
        self.argument_size
            .observe(i.tokens.clone().into_iter().count());
    }
}

pub fn make_collector() -> MetricCollectorBox {
    util::VisitorCollector::new(
        "macro",
        MacroStats {
            argument_size: Hist::default(),
            count_per_file: Hist::default(),
        },
        |v| v,
        |v| Monoid::reduce(v.iter().cloned()),
    )
    .make_box()
}
