use std::ops::{Deref, DerefMut};

use average::{Estimate, Mean};
use rust_code_analysis::{cognitive, cyclomatic, exit, halstead, loc, mi, nargs, nom, CodeMetrics};
use serde::Serialize;

/// Serializable mean
#[derive(Default)]
struct SerMean(Mean);

impl Deref for SerMean {
    type Target = Mean;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for SerMean {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Serialize for SerMean {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.mean().serialize(serializer)
    }
}

#[derive(Default, Serialize)]
pub struct Cognitive {
    average: SerMean,
    max: SerMean,
    min: SerMean,
    sum: SerMean,
}

impl Cognitive {
    fn observe(&mut self, stats: &cognitive::Stats) {
        let avg = stats.cognitive_average();
        if !avg.is_finite() {
            return;
        }
        self.average.add(avg);
        self.max.add(stats.cognitive_max());
        self.min.add(stats.cognitive_min());
        self.sum.add(stats.cognitive_sum());
    }
}

#[derive(Default, Serialize)]
pub struct Cyclomatic {
    average: SerMean,
    max: SerMean,
    min: SerMean,
    sum: SerMean,
}

impl Cyclomatic {
    fn observe(&mut self, stats: &cyclomatic::Stats) {
        self.average.add(stats.cyclomatic_average());
        self.max.add(stats.cyclomatic_max());
        self.min.add(stats.cyclomatic_min());
        self.sum.add(stats.cyclomatic_sum());
    }
}

#[derive(Default, Serialize)]
#[allow(non_snake_case)]
pub struct Halstead {
    N1: SerMean,
    N2: SerMean,
    bugs: SerMean,
    difficulty: SerMean,
    effort: SerMean,
    estimated_program_length: SerMean,
    length: SerMean,
    level: SerMean,
    n1: SerMean,
    n2: SerMean,
    purity_ratio: SerMean,
    time: SerMean,
    vocabulary: SerMean,
    volume: SerMean,
}

impl Halstead {
    fn observe(&mut self, stats: &halstead::Stats) {
        // most metrics don't work with 0 u_operands or u_operators
        if stats.u_operands() == 0. || stats.u_operators() == 0. {
            return;
        }
        self.N1.add(stats.operators());
        self.N2.add(stats.operands());
        self.n1.add(stats.u_operators());
        self.n2.add(stats.u_operands());
        self.bugs.add(stats.bugs());
        self.difficulty.add(stats.difficulty());
        self.effort.add(stats.effort());
        self.estimated_program_length
            .add(stats.estimated_program_length());
        self.length.add(stats.length());
        self.level.add(stats.level());
        self.purity_ratio.add(stats.purity_ratio());
        self.time.add(stats.time());
        self.vocabulary.add(stats.vocabulary());
        self.volume.add(stats.volume());
    }
}

#[derive(Default, Serialize)]
pub struct Loc {
    blank: SerMean,
    blank_average: SerMean,
    blank_max: SerMean,
    blank_min: SerMean,
    cloc: SerMean,
    cloc_average: SerMean,
    cloc_max: SerMean,
    cloc_min: SerMean,
    lloc: SerMean,
    lloc_average: SerMean,
    lloc_max: SerMean,
    lloc_min: SerMean,
    ploc: SerMean,
    ploc_average: SerMean,
    ploc_max: SerMean,
    ploc_min: SerMean,
    sloc: SerMean,
    sloc_average: SerMean,
    sloc_max: SerMean,
    sloc_min: SerMean,
}

impl Loc {
    fn observe(&mut self, stats: &loc::Stats) {
        self.blank.add(stats.blank());
        self.blank_average.add(stats.blank_average());
        self.blank_max.add(stats.blank_max());
        self.blank_min.add(stats.blank_min());
        self.cloc.add(stats.cloc());
        self.cloc_average.add(stats.cloc_average());
        self.cloc_max.add(stats.cloc_max());
        self.cloc_min.add(stats.cloc_min());
        self.lloc.add(stats.lloc());
        self.lloc_average.add(stats.lloc_average());
        self.lloc_max.add(stats.lloc_max());
        self.lloc_min.add(stats.lloc_min());
        self.ploc.add(stats.ploc());
        self.ploc_average.add(stats.ploc_average());
        self.ploc_max.add(stats.ploc_max());
        self.ploc_min.add(stats.ploc_min());
        self.sloc.add(stats.sloc());
        self.sloc_average.add(stats.sloc_average());
        self.sloc_max.add(stats.sloc_max());
        self.sloc_min.add(stats.sloc_min());
    }
}

#[derive(Default, Serialize)]
pub struct MI {
    mi_original: SerMean,
    mi_sei: SerMean,
    mi_visual_studio: SerMean,
}

impl MI {
    fn observe(&mut self, stats: &mi::Stats) {
        let original = stats.mi_original();
        let sei = stats.mi_sei();
        let visual_studio = stats.mi_visual_studio();
        if !original.is_finite() || !sei.is_finite() || !visual_studio.is_finite() {
            return;
        }
        self.mi_original.add(original);
        self.mi_sei.add(sei);
        self.mi_visual_studio.add(visual_studio);
    }
}

#[derive(Default, Serialize)]
pub struct Nargs {
    average: SerMean,
    average_closures: SerMean,
    average_functions: SerMean,
    closures_max: SerMean,
    closures_min: SerMean,
    functions_max: SerMean,
    functions_min: SerMean,
    total: SerMean,
    total_closures: SerMean,
    total_functions: SerMean,
}

impl Nargs {
    fn observe(&mut self, stats: &nargs::Stats) {
        self.average.add(stats.nargs_average());
        self.average_closures.add(stats.closure_args_average());
        self.average_functions.add(stats.fn_args_average());
        self.closures_max.add(stats.closure_args_max());
        self.closures_min.add(stats.closure_args_min());
        self.functions_max.add(stats.fn_args_max());
        self.functions_min.add(stats.fn_args_min());
        self.total.add(stats.nargs_total());
        self.total_closures.add(stats.closure_args_sum());
        self.total_functions.add(stats.fn_args_sum());
    }
}

#[derive(Default, Serialize)]
pub struct Nexits {
    average: SerMean,
    max: SerMean,
    min: SerMean,
    sum: SerMean,
}

impl Nexits {
    fn observe(&mut self, stats: &exit::Stats) {
        let avg = stats.exit_average();
        if !avg.is_finite() {
            return;
        }
        self.average.add(avg);
        self.max.add(stats.exit_max());
        self.min.add(stats.exit_min());
        self.sum.add(stats.exit_sum());
    }
}

#[derive(Default, Serialize)]
pub struct Nom {
    average: SerMean,
    closures: SerMean,
    closures_average: SerMean,
    closures_max: SerMean,
    closures_min: SerMean,
    functions: SerMean,
    functions_average: SerMean,
    functions_max: SerMean,
    functions_min: SerMean,
    total: SerMean,
}

impl Nom {
    fn observe(&mut self, stats: &nom::Stats) {
        self.average.add(stats.average());
        self.closures.add(stats.closures_sum());
        self.closures_average.add(stats.closures_average());
        self.closures_max.add(stats.closures_max());
        self.closures_min.add(stats.closures_min());
        self.functions.add(stats.functions_sum());
        self.functions_average.add(stats.functions_average());
        self.functions_max.add(stats.functions_max());
        self.functions_min.add(stats.functions_min());
        self.total.add(stats.total());
    }
}

#[derive(Default, Serialize)]
pub struct RCAMetrics {
    pub cognitive: Cognitive,
    pub cyclomatic: Cyclomatic,
    pub halstead: Halstead,
    pub loc: Loc,
    pub mi: MI,
    pub nargs: Nargs,
    pub nexits: Nexits,
    pub nom: Nom,
}

impl RCAMetrics {
    pub fn observe(&mut self, stats: &CodeMetrics) {
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

#[cfg(test)]
mod tests {
    use std::path::Path;

    use ::rust_code_analysis::ParserTrait;
    use ::rust_code_analysis::RustParser;
    use expect_test::expect;

    use super::RCAMetrics;

    #[test]
    fn cognitive() {
        let code = include_str!("./mod.rs").to_string().as_bytes().to_vec();

        let path = Path::new("mod.rs");
        let parser = RustParser::new(code, path, None);

        let stats = ::rust_code_analysis::metrics(&parser, &path)
            .unwrap()
            .metrics;
        let mut statistics = RCAMetrics::default();
        statistics.observe(&stats);
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
            }"#]].assert_eq(&actual);
    }
}
