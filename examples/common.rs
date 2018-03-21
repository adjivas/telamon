/// Function shared among examples.
use itertools::Itertools;
use telamon::device::Context;
use telamon::{explorer, ir};
use telamon::helper::SignatureBuilder;
use telamon::helper::tensor::DimSize;
use telamon::search_space::SearchSpace;
use std;

/// Generates the code for the best candidate in the search space.
pub fn gen_best<'a, T>(search_space: Vec<SearchSpace>,
                       context: &'a T,
                       out: &str) where T: Context<'a> {
    let conf = explorer::Config::read();
    let begin_time = std::time::Instant::now();
    let best = explorer::find_best(&conf, context, search_space).unwrap();
    let duration = std::time::Instant::now() - begin_time;
    warn!("best candidate found in {}s", duration.as_secs());
    let mut file = std::fs::File::create(out).unwrap();
    context.device().gen_code(&best, &mut file);
}

/// Creates a `DimSize`. If the instantiate flag is true, it uses a constant size,
/// otherwise it creates a parameter with the given name.
pub fn create_size<'a>(value: i32, name: &'a str,
                       instantiate: bool,
                       builder: &mut SignatureBuilder) -> DimSize<'a> {
    if instantiate { DimSize::Const(value as u32) } else {
        builder.param(name, value);
        DimSize::Param(name)
    }
}

/// Removes tiles of size 1.
pub fn cleanup_tiling(tiling: &[u32]) -> Vec<u32> {
    tiling.iter().cloned().filter(|&t| t > 1).collect()
}

/// Generate a name for the output file.
pub fn file_name(name: &str,
                 _: ir::Type,
                 sizes: &[i32],
                 instantiated: bool) -> String {
    const PATH: &str = "examples/out/";
    std::fs::create_dir_all(PATH).unwrap();
    let sizes = sizes.iter().format_with("", |i, f| f(&format_args!("_{}", i)));
    format!("{}{}_{}{}.c", PATH, name, instantiated, sizes)
}
