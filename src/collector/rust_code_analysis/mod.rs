use rust_code_analysis::{
    cognitive, cyclomatic, exit, halstead, loc, mi, nargs, nom, CodeMetrics, FuncSpace,
};
use serde::Serialize;

use super::metrics::util::{Observer, Unaggregated};

#[derive(Default, Serialize)]
pub struct Cognitive<Obs> {
    average: Obs,
    max: Obs,
    min: Obs,
    sum: Obs,
}

impl<Obs: Observer<f64>> Cognitive<Obs> {
    fn observe(&mut self, stats: &cognitive::Stats) {
        let avg = stats.cognitive_average();
        if !avg.is_finite() {
            return;
        }
        self.average.observe(avg);
        self.max.observe(stats.cognitive_max());
        self.min.observe(stats.cognitive_min());
        self.sum.observe(stats.cognitive_sum());
    }
}

#[derive(Default, Serialize)]
pub struct Cyclomatic<Obs> {
    average: Obs,
    max: Obs,
    min: Obs,
    sum: Obs,
}

impl<Obs: Observer<f64>> Cyclomatic<Obs> {
    fn observe(&mut self, stats: &cyclomatic::Stats) {
        self.average.observe(stats.cyclomatic_average());
        self.max.observe(stats.cyclomatic_max());
        self.min.observe(stats.cyclomatic_min());
        self.sum.observe(stats.cyclomatic_sum());
    }
}

#[derive(Default, Serialize)]
#[allow(non_snake_case)]
pub struct Halstead<Obs> {
    N1: Obs,
    N2: Obs,
    bugs: Obs,
    difficulty: Obs,
    effort: Obs,
    estimated_program_length: Obs,
    length: Obs,
    level: Obs,
    n1: Obs,
    n2: Obs,
    purity_ratio: Obs,
    time: Obs,
    vocabulary: Obs,
    volume: Obs,
}

impl<Obs: Observer<f64>> Halstead<Obs> {
    fn observe(&mut self, stats: &halstead::Stats) {
        // most metrics don't work with 0 u_operands or u_operators
        if stats.u_operands() == 0. || stats.u_operators() == 0. {
            return;
        }
        self.N1.observe(stats.operators());
        self.N2.observe(stats.operands());
        self.n1.observe(stats.u_operators());
        self.n2.observe(stats.u_operands());
        self.bugs.observe(stats.bugs());
        self.difficulty.observe(stats.difficulty());
        self.effort.observe(stats.effort());
        self.estimated_program_length
            .observe(stats.estimated_program_length());
        self.length.observe(stats.length());
        self.level.observe(stats.level());
        self.purity_ratio.observe(stats.purity_ratio());
        self.time.observe(stats.time());
        self.vocabulary.observe(stats.vocabulary());
        self.volume.observe(stats.volume());
    }
}

#[derive(Default, Serialize)]
pub struct Loc<Obs> {
    blank: Obs,
    blank_average: Obs,
    blank_max: Obs,
    blank_min: Obs,
    cloc: Obs,
    cloc_average: Obs,
    cloc_max: Obs,
    cloc_min: Obs,
    lloc: Obs,
    lloc_average: Obs,
    lloc_max: Obs,
    lloc_min: Obs,
    ploc: Obs,
    ploc_average: Obs,
    ploc_max: Obs,
    ploc_min: Obs,
    sloc: Obs,
    sloc_average: Obs,
    sloc_max: Obs,
    sloc_min: Obs,
}

impl<Obs: Observer<f64>> Loc<Obs> {
    fn observe(&mut self, stats: &loc::Stats) {
        self.blank.observe(stats.blank());
        self.blank_average.observe(stats.blank_average());
        self.blank_max.observe(stats.blank_max());
        self.blank_min.observe(stats.blank_min());
        self.cloc.observe(stats.cloc());
        self.cloc_average.observe(stats.cloc_average());
        self.cloc_max.observe(stats.cloc_max());
        self.cloc_min.observe(stats.cloc_min());
        self.lloc.observe(stats.lloc());
        self.lloc_average.observe(stats.lloc_average());
        self.lloc_max.observe(stats.lloc_max());
        self.lloc_min.observe(stats.lloc_min());
        self.ploc.observe(stats.ploc());
        self.ploc_average.observe(stats.ploc_average());
        self.ploc_max.observe(stats.ploc_max());
        self.ploc_min.observe(stats.ploc_min());
        self.sloc.observe(stats.sloc());
        self.sloc_average.observe(stats.sloc_average());
        self.sloc_max.observe(stats.sloc_max());
        self.sloc_min.observe(stats.sloc_min());
    }
}

#[derive(Default, Serialize)]
pub struct MI<Obs> {
    mi_original: Obs,
    mi_sei: Obs,
    mi_visual_studio: Obs,
}

impl<Obs: Observer<f64>> MI<Obs> {
    fn observe(&mut self, stats: &mi::Stats) {
        let original = stats.mi_original();
        let sei = stats.mi_sei();
        let visual_studio = stats.mi_visual_studio();
        if !original.is_finite() || !sei.is_finite() || !visual_studio.is_finite() {
            return;
        }
        self.mi_original.observe(original);
        self.mi_sei.observe(sei);
        self.mi_visual_studio.observe(visual_studio);
    }
}

#[derive(Default, Serialize)]
pub struct Nargs<Obs> {
    average: Obs,
    average_closures: Obs,
    average_functions: Obs,
    closures_max: Obs,
    closures_min: Obs,
    functions_max: Obs,
    functions_min: Obs,
    total: Obs,
    total_closures: Obs,
    total_functions: Obs,
}

impl<Obs: Observer<f64>> Nargs<Obs> {
    fn observe(&mut self, stats: &nargs::Stats) {
        self.average.observe(stats.nargs_average());
        self.average_closures.observe(stats.closure_args_average());
        self.average_functions.observe(stats.fn_args_average());
        self.closures_max.observe(stats.closure_args_max());
        self.closures_min.observe(stats.closure_args_min());
        self.functions_max.observe(stats.fn_args_max());
        self.functions_min.observe(stats.fn_args_min());
        self.total.observe(stats.nargs_total());
        self.total_closures.observe(stats.closure_args_sum());
        self.total_functions.observe(stats.fn_args_sum());
    }
}

#[derive(Default, Serialize)]
pub struct Nexits<Obs> {
    average: Obs,
    max: Obs,
    min: Obs,
    sum: Obs,
}

impl<Obs: Observer<f64>> Nexits<Obs> {
    fn observe(&mut self, stats: &exit::Stats) {
        let avg = stats.exit_average();
        if !avg.is_finite() {
            return;
        }
        self.average.observe(avg);
        self.max.observe(stats.exit_max());
        self.min.observe(stats.exit_min());
        self.sum.observe(stats.exit_sum());
    }
}

#[derive(Default, Serialize)]
pub struct Nom<Obs = Unaggregated<f64>> {
    average: Obs,
    closures: Obs,
    closures_average: Obs,
    closures_max: Obs,
    closures_min: Obs,
    functions: Obs,
    functions_average: Obs,
    functions_max: Obs,
    functions_min: Obs,
    total: Obs,
}

impl<Obs: Observer<f64>> Nom<Obs> {
    fn observe(&mut self, stats: &nom::Stats) {
        self.average.observe(stats.average());
        self.closures.observe(stats.closures_sum());
        self.closures_average.observe(stats.closures_average());
        self.closures_max.observe(stats.closures_max());
        self.closures_min.observe(stats.closures_min());
        self.functions.observe(stats.functions_sum());
        self.functions_average.observe(stats.functions_average());
        self.functions_max.observe(stats.functions_max());
        self.functions_min.observe(stats.functions_min());
        self.total.observe(stats.total());
    }
}

#[derive(Default, Serialize)]
pub struct RCAMetrics<Obs> {
    pub cognitive: Cognitive<Obs>,
    pub cyclomatic: Cyclomatic<Obs>,
    pub halstead: Halstead<Obs>,
    pub loc: Loc<Obs>,
    pub mi: MI<Obs>,
    pub nargs: Nargs<Obs>,
    pub nexits: Nexits<Obs>,
    pub nom: Nom<Obs>,
}

impl<Obs: Observer<f64>> RCAMetrics<Obs> {
    pub fn observe_metrics(&mut self, stats: &CodeMetrics) {
        self.cognitive.observe(&stats.cognitive);
        self.cyclomatic.observe(&stats.cyclomatic);
        self.halstead.observe(&stats.halstead);
        self.loc.observe(&stats.loc);
        self.mi.observe(&stats.mi);
        self.nargs.observe(&stats.nargs);
        self.nexits.observe(&stats.nexits);
        self.nom.observe(&stats.nom);
    }
}

#[derive(Default, Serialize)]
pub struct RCAMetricsKinded<Obs> {
    function: RCAMetrics<Obs>,
    r#struct: RCAMetrics<Obs>,
    r#trait: RCAMetrics<Obs>,
    r#impl: RCAMetrics<Obs>,
}

impl<Obs: Observer<f64>> RCAMetricsKinded<Obs> {
    pub fn observe_spaces(&mut self, space: &FuncSpace) {
        match space.kind {
            rust_code_analysis::SpaceKind::Unknown => space
                .spaces
                .iter()
                .for_each(|space| self.observe_spaces(space)),
            rust_code_analysis::SpaceKind::Function => {
                self.function.observe_metrics(&space.metrics);
                space
                    .spaces
                    .iter()
                    .for_each(|space| self.observe_spaces(space));
            }
            rust_code_analysis::SpaceKind::Class => panic!("Class funcspace in rust code"),
            rust_code_analysis::SpaceKind::Struct => {
                self.r#struct.observe_metrics(&space.metrics);
                assert_eq!(
                    space.spaces.len(),
                    0,
                    "there should not be any function spaces within structs"
                );
            }
            rust_code_analysis::SpaceKind::Trait => {
                self.r#trait.observe_metrics(&space.metrics);
                assert_eq!(
                    space.spaces.len(),
                    0,
                    "there should not be any function spaces within traits"
                );
            }
            rust_code_analysis::SpaceKind::Impl => {
                self.r#impl.observe_metrics(&space.metrics);
                space
                    .spaces
                    .iter()
                    .for_each(|space| self.observe_spaces(space));
            }
            rust_code_analysis::SpaceKind::Unit => space
                .spaces
                .iter()
                .for_each(|space| self.observe_spaces(space)),
            rust_code_analysis::SpaceKind::Namespace => panic!("Namespace funcspace in rust code"),
            rust_code_analysis::SpaceKind::Interface => panic!("Interface funcspace in rust code"),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use ::rust_code_analysis::ParserTrait;
    use ::rust_code_analysis::RustParser;
    use expect_test::expect;

    use crate::collector::metrics::util::Unaggregated;

    use super::RCAMetrics;
    use super::RCAMetricsKinded;

    #[test]
    fn aggregated_metrics() {
        let code = include_str!("./mod.rs").to_string().as_bytes().to_vec();

        let path = Path::new("mod.rs");
        let parser = RustParser::new(code, path, None);

        let mut stats = ::rust_code_analysis::metrics(&parser, &path).unwrap();
        stats.spaces.clear();
        let metrics = stats.metrics;
        let mut statistics = RCAMetrics::<Unaggregated<f64>>::default();
        statistics.observe_metrics(&metrics);
        let actual = serde_json::to_string_pretty(&statistics).unwrap();
        expect![[r#"
            {
              "cognitive": {
                "average": 0.46153846153846156,
                "max": 2.0,
                "min": 0.0,
                "sum": 6.0
              },
              "cyclomatic": {
                "average": 1.3846153846153846,
                "max": 4.0,
                "min": 1.0,
                "sum": 36.0
              },
              "halstead": {
                "N1": 825.0,
                "N2": 321.0,
                "bugs": 1.7402193760474183,
                "difficulty": 51.774193548387096,
                "effort": 377214.0686662639,
                "estimated_program_length": 455.59873314173353,
                "length": 1146.0,
                "level": 0.019314641744548288,
                "n1": 20.0,
                "n2": 62.0,
                "purity_ratio": 0.3975556135617221,
                "time": 20956.33714812577,
                "vocabulary": 82.0,
                "volume": 7285.754597292324
              },
              "loc": {
                "blank": 115.0,
                "blank_average": 4.423076923076923,
                "blank_max": 88.0,
                "blank_min": 0.0,
                "cloc": 2.0,
                "cloc_average": 0.07692307692307693,
                "cloc_max": 1.0,
                "cloc_min": 0.0,
                "lloc": 98.0,
                "lloc_average": 3.769230769230769,
                "lloc_max": 20.0,
                "lloc_min": 0.0,
                "ploc": 283.0,
                "ploc_average": 10.884615384615385,
                "ploc_max": 24.0,
                "ploc_min": 5.0,
                "sloc": 400.0,
                "sloc_average": 15.384615384615385,
                "sloc_max": 101.0,
                "sloc_min": 5.0
              },
              "mi": {
                "mi_original": 19.411157599743518,
                "mi_sei": -38.56467855169339,
                "mi_visual_studio": 11.351554151896796
              },
              "nargs": {
                "average": 1.6923076923076923,
                "average_closures": 0.0,
                "average_functions": 1.6923076923076923,
                "closures_max": 0.0,
                "closures_min": 0.0,
                "functions_max": 2.0,
                "functions_min": 0.0,
                "total": 22.0,
                "total_closures": 0.0,
                "total_functions": 22.0
              },
              "nexits": {
                "average": 0.5384615384615384,
                "max": 1.0,
                "min": 0.0,
                "sum": 7.0
              },
              "nom": {
                "average": 0.5,
                "closures": 0.0,
                "closures_average": 0.0,
                "closures_max": 0.0,
                "closures_min": 0.0,
                "functions": 13.0,
                "functions_average": 0.5,
                "functions_max": 1.0,
                "functions_min": 0.0,
                "total": 13.0
              }
            }"#]]
        .assert_eq(&actual);
    }

    #[test]
    fn unaggregated_metrics() {
        let code = include_str!("./mod.rs").to_string().as_bytes().to_vec();

        let path = Path::new("mod.rs");
        let parser = RustParser::new(code, path, None);

        let stats = ::rust_code_analysis::metrics(&parser, &path).unwrap();
        let mut statistics = RCAMetricsKinded::<Unaggregated<f64>>::default();
        statistics.observe_spaces(&stats);
        let actual = serde_json::to_string_pretty(&statistics).unwrap();
        expect![[""]].assert_eq(&actual);
    }
}
