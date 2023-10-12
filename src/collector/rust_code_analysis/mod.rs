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
        if avg.is_finite() {
            self.average.add(avg);
        }
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
        // most metrics don't work with 0 uprands or operators
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
        if original.is_nan() || sei.is_nan() || visual_studio.is_nan() {
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
        self.average.add(stats.exit_average());
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
