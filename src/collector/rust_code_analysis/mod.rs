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
                let mut this_space = space.clone();
                this_space.spaces.clear();
                self.function.observe_metrics(&this_space.metrics);
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
                space
                    .spaces
                    .iter()
                    .for_each(|space| self.observe_spaces(space));
            }
            rust_code_analysis::SpaceKind::Impl => {
                let mut this_space = space.clone();
                this_space.spaces.clear();
                self.r#impl.observe_metrics(&this_space.metrics);
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
                "average": [
                  0.4117647058823529
                ],
                "max": [
                  2.0
                ],
                "min": [
                  0.0
                ],
                "sum": [
                  7.0
                ]
              },
              "cyclomatic": {
                "average": [
                  1.5714285714285714
                ],
                "max": [
                  10.0
                ],
                "min": [
                  1.0
                ],
                "sum": [
                  44.0
                ]
              },
              "halstead": {
                "N1": [
                  1108.0
                ],
                "N2": [
                  416.0
                ],
                "bugs": [
                  2.291143908608136
                ],
                "difficulty": [
                  55.80487804878049
                ],
                "effort": [
                  569849.8453283607
                ],
                "estimated_program_length": [
                  619.4267599887035
                ],
                "length": [
                  1524.0
                ],
                "level": [
                  0.01791958041958042
                ],
                "n1": [
                  22.0
                ],
                "n2": [
                  82.0
                ],
                "purity_ratio": [
                  0.4064480052419314
                ],
                "time": [
                  31658.324740464483
                ],
                "vocabulary": [
                  104.0
                ],
                "volume": [
                  10211.470130447024
                ]
              },
              "loc": {
                "blank": [
                  1736.0
                ],
                "blank_average": [
                  62.0
                ],
                "blank_max": [
                  1483.0
                ],
                "blank_min": [
                  0.0
                ],
                "cloc": [
                  1.0
                ],
                "cloc_average": [
                  0.03571428571428571
                ],
                "cloc_max": [
                  1.0
                ],
                "cloc_min": [
                  0.0
                ],
                "lloc": [
                  121.0
                ],
                "lloc_average": [
                  4.321428571428571
                ],
                "lloc_max": [
                  20.0
                ],
                "lloc_min": [
                  4.0
                ],
                "ploc": [
                  339.0
                ],
                "ploc_average": [
                  12.107142857142858
                ],
                "ploc_max": [
                  50.0
                ],
                "ploc_min": [
                  8.0
                ],
                "sloc": [
                  2076.0
                ],
                "sloc_average": [
                  74.14285714285714
                ],
                "sloc_max": [
                  1498.0
                ],
                "sloc_min": [
                  8.0
                ]
              },
              "mi": {
                "mi_original": [
                  -10.861399385758688
                ],
                "mi_sei": [
                  -85.19074175074962
                ],
                "mi_visual_studio": [
                  0.0
                ]
              },
              "nargs": {
                "average": [
                  1.4705882352941178
                ],
                "average_closures": [
                  1.0
                ],
                "average_functions": [
                  1.6666666666666667
                ],
                "closures_max": [
                  1.0
                ],
                "closures_min": [
                  0.0
                ],
                "functions_max": [
                  2.0
                ],
                "functions_min": [
                  0.0
                ],
                "total": [
                  25.0
                ],
                "total_closures": [
                  5.0
                ],
                "total_functions": [
                  20.0
                ]
              },
              "nexits": {
                "average": [
                  0.23529411764705882
                ],
                "max": [
                  1.0
                ],
                "min": [
                  0.0
                ],
                "sum": [
                  4.0
                ]
              },
              "nom": {
                "average": [
                  0.6071428571428571
                ],
                "closures": [
                  5.0
                ],
                "closures_average": [
                  0.17857142857142858
                ],
                "closures_max": [
                  1.0
                ],
                "closures_min": [
                  0.0
                ],
                "functions": [
                  12.0
                ],
                "functions_average": [
                  0.42857142857142855
                ],
                "functions_max": [
                  1.0
                ],
                "functions_min": [
                  0.0
                ],
                "total": [
                  17.0
                ]
              }
            }"#]]
        .assert_eq(&actual);
    }

    #[test]
    fn unaggregated_metrics() {
        let code = r#"
            struct Test {
                a: usize,
                b: i32
            }

            struct Test2(usize);

            impl Test {
                fn aboba(&self, other: Type) -> Type{
                    other
                }
                fn abiba(&mut self){
                    fn prikol(a: bool) -> usize {
                        if a {
                            4
                        } else {
                            5
                        }
                    }

                    if self.a > 3 {
                        print!()
                    } else {
                        println!()
                    }
                }
                fn new() -> Self{
                    Test { a: 6, b: 9 }
                }
            }

            impl Test2 {
                fn f1() -> Self {
                    Test2(69)
                }

                fn f2() -> Self {
                    Test2(42)
                }
            }

            impl Test3 {}
            impl Test4 {
                fn useles(){}
            }

            trait MyTrait {
                fn aboba(&mut self);
                fn default() {
                    println!("Hello")
                }
            }
            "#
        .to_string()
        .as_bytes()
        .to_vec();

        let path = Path::new("mod.rs");
        let parser = RustParser::new(code, path, None);

        let stats = ::rust_code_analysis::metrics(&parser, &path).unwrap();
        let mut statistics = RCAMetricsKinded::<Unaggregated<f64>>::default();
        statistics.observe_spaces(&stats);
        let actual = serde_json::to_string_pretty(&statistics).unwrap();
        expect![[r#"
            {
              "function": {
                "cognitive": {
                  "average": [
                    0.0,
                    2.5,
                    3.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0
                  ],
                  "max": [
                    0.0,
                    3.0,
                    3.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0
                  ],
                  "min": [
                    0.0,
                    2.0,
                    3.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0
                  ],
                  "sum": [
                    0.0,
                    5.0,
                    3.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0
                  ]
                },
                "cyclomatic": {
                  "average": [
                    1.0,
                    2.0,
                    2.0,
                    1.0,
                    1.0,
                    1.0,
                    1.0,
                    1.0
                  ],
                  "max": [
                    1.0,
                    2.0,
                    2.0,
                    1.0,
                    1.0,
                    1.0,
                    1.0,
                    1.0
                  ],
                  "min": [
                    1.0,
                    2.0,
                    2.0,
                    1.0,
                    1.0,
                    1.0,
                    1.0,
                    1.0
                  ],
                  "sum": [
                    1.0,
                    4.0,
                    2.0,
                    1.0,
                    1.0,
                    1.0,
                    1.0,
                    1.0
                  ]
                },
                "halstead": {
                  "N1": [
                    6.0,
                    23.0,
                    9.0,
                    6.0,
                    5.0,
                    5.0,
                    3.0,
                    5.0
                  ],
                  "N2": [
                    4.0,
                    11.0,
                    5.0,
                    3.0,
                    3.0,
                    3.0,
                    1.0,
                    3.0
                  ],
                  "bugs": [
                    0.008413009854528818,
                    0.0330616830540879,
                    0.010405072229378903,
                    0.005526047247960579,
                    0.00421201861424495,
                    0.00421201861424495,
                    0.0017471609294725976,
                    0.00421201861424495
                  ],
                  "difficulty": [
                    4.0,
                    6.722222222222222,
                    3.75,
                    2.5,
                    2.0,
                    2.0,
                    1.5,
                    2.0
                  ],
                  "effort": [
                    126.79700005769249,
                    987.8006767981451,
                    174.40122498158652,
                    67.5,
                    44.91767875292167,
                    44.91767875292167,
                    12.0,
                    44.91767875292167
                  ],
                  "estimated_program_length": [
                    20.264662506490403,
                    66.58307281799108,
                    23.509775004326936,
                    16.36452797660028,
                    12.754887502163468,
                    12.754887502163468,
                    4.754887502163468,
                    12.754887502163468
                  ],
                  "length": [
                    10.0,
                    34.0,
                    14.0,
                    9.0,
                    8.0,
                    8.0,
                    4.0,
                    8.0
                  ],
                  "level": [
                    0.25,
                    0.1487603305785124,
                    0.26666666666666666,
                    0.4,
                    0.5,
                    0.5,
                    0.6666666666666666,
                    0.5
                  ],
                  "n1": [
                    6.0,
                    11.0,
                    6.0,
                    5.0,
                    4.0,
                    4.0,
                    3.0,
                    4.0
                  ],
                  "n2": [
                    3.0,
                    9.0,
                    4.0,
                    3.0,
                    3.0,
                    3.0,
                    1.0,
                    3.0
                  ],
                  "purity_ratio": [
                    2.0264662506490403,
                    1.9583256711173846,
                    1.6792696431662097,
                    1.81828088628892,
                    1.5943609377704335,
                    1.5943609377704335,
                    1.188721875540867,
                    1.5943609377704335
                  ],
                  "time": [
                    7.044277780982916,
                    54.87781537767473,
                    9.688956943421474,
                    3.75,
                    2.495426597384537,
                    2.495426597384537,
                    0.6666666666666666,
                    2.495426597384537
                  ],
                  "vocabulary": [
                    9.0,
                    20.0,
                    10.0,
                    8.0,
                    7.0,
                    7.0,
                    4.0,
                    7.0
                  ],
                  "volume": [
                    31.69925001442312,
                    146.94555522617034,
                    46.50699332842307,
                    27.0,
                    22.458839376460833,
                    22.458839376460833,
                    8.0,
                    22.458839376460833
                  ]
                },
                "loc": {
                  "blank": [
                    0.0,
                    1.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0
                  ],
                  "blank_average": [
                    0.0,
                    0.5,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0
                  ],
                  "blank_max": [
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0
                  ],
                  "blank_min": [
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0
                  ],
                  "cloc": [
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0
                  ],
                  "cloc_average": [
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0
                  ],
                  "cloc_max": [
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0
                  ],
                  "cloc_min": [
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0
                  ],
                  "lloc": [
                    0.0,
                    2.0,
                    1.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0
                  ],
                  "lloc_average": [
                    0.0,
                    1.0,
                    1.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0
                  ],
                  "lloc_max": [
                    0.0,
                    1.0,
                    1.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0
                  ],
                  "lloc_min": [
                    0.0,
                    1.0,
                    1.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0
                  ],
                  "ploc": [
                    3.0,
                    14.0,
                    7.0,
                    3.0,
                    3.0,
                    3.0,
                    1.0,
                    3.0
                  ],
                  "ploc_average": [
                    3.0,
                    7.0,
                    7.0,
                    3.0,
                    3.0,
                    3.0,
                    1.0,
                    3.0
                  ],
                  "ploc_max": [
                    3.0,
                    7.0,
                    7.0,
                    3.0,
                    3.0,
                    3.0,
                    1.0,
                    3.0
                  ],
                  "ploc_min": [
                    3.0,
                    7.0,
                    7.0,
                    3.0,
                    3.0,
                    3.0,
                    1.0,
                    3.0
                  ],
                  "sloc": [
                    3.0,
                    15.0,
                    7.0,
                    3.0,
                    3.0,
                    3.0,
                    1.0,
                    3.0
                  ],
                  "sloc_average": [
                    3.0,
                    7.5,
                    7.0,
                    3.0,
                    3.0,
                    3.0,
                    1.0,
                    3.0
                  ],
                  "sloc_max": [
                    3.0,
                    7.0,
                    7.0,
                    3.0,
                    3.0,
                    3.0,
                    1.0,
                    3.0
                  ],
                  "sloc_min": [
                    3.0,
                    7.0,
                    7.0,
                    3.0,
                    3.0,
                    3.0,
                    1.0,
                    3.0
                  ]
                },
                "mi": {
                  "mi_original": [
                    134.9997572104644,
                    100.26126358521779,
                    119.05032156899054,
                    135.83412922035413,
                    136.79172270480979,
                    136.79172270480979,
                    159.95690398326485,
                    136.79172270480979
                  ],
                  "mi_sei": [
                    119.16444811614275,
                    69.35285521326,
                    96.25609627061525,
                    120.36819247706723,
                    121.74970784827903,
                    121.74970784827903,
                    155.17000000000002,
                    121.74970784827903
                  ],
                  "mi_visual_studio": [
                    78.94722643886807,
                    58.632317886092274,
                    69.62007109297693,
                    79.4351632867568,
                    79.99515947649695,
                    79.99515947649695,
                    93.54204911302038,
                    79.99515947649695
                  ]
                },
                "nargs": {
                  "average": [
                    2.0,
                    1.0,
                    1.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0
                  ],
                  "average_closures": [
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0
                  ],
                  "average_functions": [
                    2.0,
                    1.0,
                    1.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0
                  ],
                  "closures_max": [
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0
                  ],
                  "closures_min": [
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0
                  ],
                  "functions_max": [
                    2.0,
                    1.0,
                    1.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0
                  ],
                  "functions_min": [
                    2.0,
                    1.0,
                    1.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0
                  ],
                  "total": [
                    2.0,
                    2.0,
                    1.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0
                  ],
                  "total_closures": [
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0
                  ],
                  "total_functions": [
                    2.0,
                    2.0,
                    1.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0
                  ]
                },
                "nexits": {
                  "average": [
                    1.0,
                    0.5,
                    1.0,
                    1.0,
                    1.0,
                    1.0,
                    0.0,
                    0.0
                  ],
                  "max": [
                    1.0,
                    1.0,
                    1.0,
                    1.0,
                    1.0,
                    1.0,
                    0.0,
                    0.0
                  ],
                  "min": [
                    1.0,
                    0.0,
                    1.0,
                    1.0,
                    1.0,
                    1.0,
                    0.0,
                    0.0
                  ],
                  "sum": [
                    1.0,
                    1.0,
                    1.0,
                    1.0,
                    1.0,
                    1.0,
                    0.0,
                    0.0
                  ]
                },
                "nom": {
                  "average": [
                    1.0,
                    1.0,
                    1.0,
                    1.0,
                    1.0,
                    1.0,
                    1.0,
                    1.0
                  ],
                  "closures": [
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0
                  ],
                  "closures_average": [
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0
                  ],
                  "closures_max": [
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0
                  ],
                  "closures_min": [
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0
                  ],
                  "functions": [
                    1.0,
                    2.0,
                    1.0,
                    1.0,
                    1.0,
                    1.0,
                    1.0,
                    1.0
                  ],
                  "functions_average": [
                    1.0,
                    1.0,
                    1.0,
                    1.0,
                    1.0,
                    1.0,
                    1.0,
                    1.0
                  ],
                  "functions_max": [
                    1.0,
                    1.0,
                    1.0,
                    1.0,
                    1.0,
                    1.0,
                    1.0,
                    1.0
                  ],
                  "functions_min": [
                    1.0,
                    1.0,
                    1.0,
                    1.0,
                    1.0,
                    1.0,
                    1.0,
                    1.0
                  ],
                  "total": [
                    1.0,
                    2.0,
                    1.0,
                    1.0,
                    1.0,
                    1.0,
                    1.0,
                    1.0
                  ]
                }
              },
              "struct": {
                "cognitive": {
                  "average": [],
                  "max": [],
                  "min": [],
                  "sum": []
                },
                "cyclomatic": {
                  "average": [],
                  "max": [],
                  "min": [],
                  "sum": []
                },
                "halstead": {
                  "N1": [],
                  "N2": [],
                  "bugs": [],
                  "difficulty": [],
                  "effort": [],
                  "estimated_program_length": [],
                  "length": [],
                  "level": [],
                  "n1": [],
                  "n2": [],
                  "purity_ratio": [],
                  "time": [],
                  "vocabulary": [],
                  "volume": []
                },
                "loc": {
                  "blank": [],
                  "blank_average": [],
                  "blank_max": [],
                  "blank_min": [],
                  "cloc": [],
                  "cloc_average": [],
                  "cloc_max": [],
                  "cloc_min": [],
                  "lloc": [],
                  "lloc_average": [],
                  "lloc_max": [],
                  "lloc_min": [],
                  "ploc": [],
                  "ploc_average": [],
                  "ploc_max": [],
                  "ploc_min": [],
                  "sloc": [],
                  "sloc_average": [],
                  "sloc_max": [],
                  "sloc_min": []
                },
                "mi": {
                  "mi_original": [],
                  "mi_sei": [],
                  "mi_visual_studio": []
                },
                "nargs": {
                  "average": [],
                  "average_closures": [],
                  "average_functions": [],
                  "closures_max": [],
                  "closures_min": [],
                  "functions_max": [],
                  "functions_min": [],
                  "total": [],
                  "total_closures": [],
                  "total_functions": []
                },
                "nexits": {
                  "average": [],
                  "max": [],
                  "min": [],
                  "sum": []
                },
                "nom": {
                  "average": [],
                  "closures": [],
                  "closures_average": [],
                  "closures_max": [],
                  "closures_min": [],
                  "functions": [],
                  "functions_average": [],
                  "functions_max": [],
                  "functions_min": [],
                  "total": []
                }
              },
              "trait": {
                "cognitive": {
                  "average": [
                    0.0
                  ],
                  "max": [
                    0.0
                  ],
                  "min": [
                    0.0
                  ],
                  "sum": [
                    0.0
                  ]
                },
                "cyclomatic": {
                  "average": [
                    1.0
                  ],
                  "max": [
                    1.0
                  ],
                  "min": [
                    1.0
                  ],
                  "sum": [
                    2.0
                  ]
                },
                "halstead": {
                  "N1": [
                    11.0
                  ],
                  "N2": [
                    5.0
                  ],
                  "bugs": [
                    0.011428621282029667
                  ],
                  "difficulty": [
                    3.5
                  ],
                  "effort": [
                    200.75790004038475
                  ],
                  "estimated_program_length": [
                    31.26112492884004
                  ],
                  "length": [
                    16.0
                  ],
                  "level": [
                    0.2857142857142857
                  ],
                  "n1": [
                    7.0
                  ],
                  "n2": [
                    5.0
                  ],
                  "purity_ratio": [
                    1.9538203080525025
                  ],
                  "time": [
                    11.153216668910265
                  ],
                  "vocabulary": [
                    12.0
                  ],
                  "volume": [
                    57.3594000115385
                  ]
                },
                "loc": {
                  "blank": [
                    0.0
                  ],
                  "blank_average": [
                    0.0
                  ],
                  "blank_max": [
                    0.0
                  ],
                  "blank_min": [
                    0.0
                  ],
                  "cloc": [
                    0.0
                  ],
                  "cloc_average": [
                    0.0
                  ],
                  "cloc_max": [
                    0.0
                  ],
                  "cloc_min": [
                    0.0
                  ],
                  "lloc": [
                    0.0
                  ],
                  "lloc_average": [
                    0.0
                  ],
                  "lloc_max": [
                    0.0
                  ],
                  "lloc_min": [
                    0.0
                  ],
                  "ploc": [
                    6.0
                  ],
                  "ploc_average": [
                    3.0
                  ],
                  "ploc_max": [
                    3.0
                  ],
                  "ploc_min": [
                    3.0
                  ],
                  "sloc": [
                    6.0
                  ],
                  "sloc_average": [
                    3.0
                  ],
                  "sloc_max": [
                    3.0
                  ],
                  "sloc_min": [
                    3.0
                  ]
                },
                "mi": {
                  "mi_original": [
                    120.45694557033428
                  ],
                  "mi_sei": [
                    98.28542574174926
                  ],
                  "mi_visual_studio": [
                    70.44265822826566
                  ]
                },
                "nargs": {
                  "average": [
                    0.0
                  ],
                  "average_closures": [
                    0.0
                  ],
                  "average_functions": [
                    0.0
                  ],
                  "closures_max": [
                    0.0
                  ],
                  "closures_min": [
                    0.0
                  ],
                  "functions_max": [
                    0.0
                  ],
                  "functions_min": [
                    0.0
                  ],
                  "total": [
                    0.0
                  ],
                  "total_closures": [
                    0.0
                  ],
                  "total_functions": [
                    0.0
                  ]
                },
                "nexits": {
                  "average": [
                    0.0
                  ],
                  "max": [
                    0.0
                  ],
                  "min": [
                    0.0
                  ],
                  "sum": [
                    0.0
                  ]
                },
                "nom": {
                  "average": [
                    0.5
                  ],
                  "closures": [
                    0.0
                  ],
                  "closures_average": [
                    0.0
                  ],
                  "closures_max": [
                    0.0
                  ],
                  "closures_min": [
                    0.0
                  ],
                  "functions": [
                    1.0
                  ],
                  "functions_average": [
                    0.5
                  ],
                  "functions_max": [
                    1.0
                  ],
                  "functions_min": [
                    0.0
                  ],
                  "total": [
                    1.0
                  ]
                }
              },
              "impl": {
                "cognitive": {
                  "average": [
                    1.25,
                    0.0,
                    0.0
                  ],
                  "max": [
                    3.0,
                    0.0,
                    0.0
                  ],
                  "min": [
                    0.0,
                    0.0,
                    0.0
                  ],
                  "sum": [
                    5.0,
                    0.0,
                    0.0
                  ]
                },
                "cyclomatic": {
                  "average": [
                    1.4,
                    1.0,
                    1.0,
                    1.0
                  ],
                  "max": [
                    2.0,
                    1.0,
                    1.0,
                    1.0
                  ],
                  "min": [
                    1.0,
                    1.0,
                    1.0,
                    1.0
                  ],
                  "sum": [
                    7.0,
                    3.0,
                    1.0,
                    2.0
                  ]
                },
                "halstead": {
                  "N1": [
                    36.0,
                    11.0,
                    4.0
                  ],
                  "N2": [
                    18.0,
                    6.0,
                    1.0
                  ],
                  "bugs": [
                    0.05217118764307358,
                    0.008524812693098637,
                    0.002027400665191133
                  ],
                  "difficulty": [
                    7.714285714285714,
                    2.4,
                    1.5
                  ],
                  "effort": [
                    1958.0688882999177,
                    129.33294005884633,
                    15.0
                  ],
                  "estimated_program_length": [
                    96.32251891746033,
                    19.60964047443681,
                    4.754887502163468
                  ],
                  "length": [
                    54.0,
                    17.0,
                    5.0
                  ],
                  "level": [
                    0.12962962962962962,
                    0.4166666666666667,
                    0.6666666666666666
                  ],
                  "n1": [
                    12.0,
                    4.0,
                    3.0
                  ],
                  "n2": [
                    14.0,
                    5.0,
                    1.0
                  ],
                  "purity_ratio": [
                    1.7837503503233394,
                    1.1535082632021652,
                    0.9509775004326937
                  ],
                  "time": [
                    108.78160490555098,
                    7.1851633366025744,
                    0.8333333333333334
                  ],
                  "vocabulary": [
                    26.0,
                    9.0,
                    4.0
                  ],
                  "volume": [
                    253.82374477961895,
                    53.8887250245193,
                    10.0
                  ]
                },
                "loc": {
                  "blank": [
                    1.0,
                    1.0,
                    0.0,
                    0.0
                  ],
                  "blank_average": [
                    0.2,
                    0.3333333333333333,
                    0.0,
                    0.0
                  ],
                  "blank_max": [
                    1.0,
                    0.0,
                    0.0,
                    0.0
                  ],
                  "blank_min": [
                    0.0,
                    0.0,
                    0.0,
                    0.0
                  ],
                  "cloc": [
                    0.0,
                    0.0,
                    0.0,
                    0.0
                  ],
                  "cloc_average": [
                    0.0,
                    0.0,
                    0.0,
                    0.0
                  ],
                  "cloc_max": [
                    0.0,
                    0.0,
                    0.0,
                    0.0
                  ],
                  "cloc_min": [
                    0.0,
                    0.0,
                    0.0,
                    0.0
                  ],
                  "lloc": [
                    2.0,
                    0.0,
                    0.0,
                    0.0
                  ],
                  "lloc_average": [
                    0.4,
                    0.0,
                    0.0,
                    0.0
                  ],
                  "lloc_max": [
                    2.0,
                    0.0,
                    0.0,
                    0.0
                  ],
                  "lloc_min": [
                    0.0,
                    0.0,
                    0.0,
                    0.0
                  ],
                  "ploc": [
                    22.0,
                    8.0,
                    1.0,
                    3.0
                  ],
                  "ploc_average": [
                    4.4,
                    2.6666666666666665,
                    1.0,
                    1.5
                  ],
                  "ploc_max": [
                    14.0,
                    3.0,
                    1.0,
                    1.0
                  ],
                  "ploc_min": [
                    3.0,
                    3.0,
                    1.0,
                    1.0
                  ],
                  "sloc": [
                    23.0,
                    9.0,
                    1.0,
                    3.0
                  ],
                  "sloc_average": [
                    4.6,
                    3.0,
                    1.0,
                    1.5
                  ],
                  "sloc_max": [
                    15.0,
                    3.0,
                    1.0,
                    1.0
                  ],
                  "sloc_min": [
                    3.0,
                    3.0,
                    1.0,
                    1.0
                  ]
                },
                "mi": {
                  "mi_original": [
                    89.80446514057971,
                    113.98297122851771,
                    140.76903844000756
                  ],
                  "mi_sei": [
                    54.57234353181863,
                    89.04727492337257,
                    127.58958139490298
                  ],
                  "mi_visual_studio": [
                    52.5172310763624,
                    66.65670832077059,
                    82.32107511111553
                  ]
                },
                "nargs": {
                  "average": [
                    1.0,
                    0.0,
                    0.0,
                    0.0
                  ],
                  "average_closures": [
                    0.0,
                    0.0,
                    0.0,
                    0.0
                  ],
                  "average_functions": [
                    1.0,
                    0.0,
                    0.0,
                    0.0
                  ],
                  "closures_max": [
                    0.0,
                    0.0,
                    0.0,
                    0.0
                  ],
                  "closures_min": [
                    0.0,
                    0.0,
                    0.0,
                    0.0
                  ],
                  "functions_max": [
                    2.0,
                    0.0,
                    0.0,
                    0.0
                  ],
                  "functions_min": [
                    0.0,
                    0.0,
                    0.0,
                    0.0
                  ],
                  "total": [
                    4.0,
                    0.0,
                    0.0,
                    0.0
                  ],
                  "total_closures": [
                    0.0,
                    0.0,
                    0.0,
                    0.0
                  ],
                  "total_functions": [
                    4.0,
                    0.0,
                    0.0,
                    0.0
                  ]
                },
                "nexits": {
                  "average": [
                    0.75,
                    1.0,
                    0.0
                  ],
                  "max": [
                    1.0,
                    1.0,
                    0.0
                  ],
                  "min": [
                    0.0,
                    0.0,
                    0.0
                  ],
                  "sum": [
                    3.0,
                    2.0,
                    0.0
                  ]
                },
                "nom": {
                  "average": [
                    0.8,
                    0.6666666666666666,
                    0.0,
                    0.5
                  ],
                  "closures": [
                    0.0,
                    0.0,
                    0.0,
                    0.0
                  ],
                  "closures_average": [
                    0.0,
                    0.0,
                    0.0,
                    0.0
                  ],
                  "closures_max": [
                    0.0,
                    0.0,
                    0.0,
                    0.0
                  ],
                  "closures_min": [
                    0.0,
                    0.0,
                    0.0,
                    0.0
                  ],
                  "functions": [
                    4.0,
                    2.0,
                    0.0,
                    1.0
                  ],
                  "functions_average": [
                    0.8,
                    0.6666666666666666,
                    0.0,
                    0.5
                  ],
                  "functions_max": [
                    1.0,
                    1.0,
                    0.0,
                    1.0
                  ],
                  "functions_min": [
                    0.0,
                    0.0,
                    0.0,
                    0.0
                  ],
                  "total": [
                    4.0,
                    2.0,
                    0.0,
                    1.0
                  ]
                }
              }
            }"#]]
        .assert_eq(&actual);
    }
}
