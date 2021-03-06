use std::os;

use cargo::core::MultiShell;
use cargo::ops::CompileOptions;
use cargo::ops;
use cargo::util::important_paths::{find_root_manifest_for_cwd};
use cargo::util::{CliResult, CliError};
use docopt;

docopt!(Options, "
Compile a local package and all of its dependencies

Usage:
    cargo build [options]

Options:
    -h, --help              Print this message
    -j N, --jobs N          The number of jobs to run in parallel
    --release               Build artifacts in release mode, with optimizations
    --target TRIPLE         Build for the target triple
    -u, --update-remotes    Deprecated option, use `cargo update` instead
    --manifest-path PATH    Path to the manifest to compile
    -v, --verbose           Use verbose output
",  flag_jobs: Option<uint>, flag_target: Option<String>,
    flag_manifest_path: Option<String>)

pub fn execute(options: Options, shell: &mut MultiShell) -> CliResult<Option<()>> {
    debug!("executing; cmd=cargo-build; args={}", os::args());
    shell.set_verbose(options.flag_verbose);

    let root = try!(find_root_manifest_for_cwd(options.flag_manifest_path));

    let env = if options.flag_release {
        "release"
    } else {
        "compile"
    };

    let mut opts = CompileOptions {
        update: options.flag_update_remotes,
        env: env,
        shell: shell,
        jobs: options.flag_jobs,
        target: options.flag_target.as_ref().map(|t| t.as_slice()),
        dev_deps: false,
    };

    ops::compile(&root, &mut opts).map(|_| None).map_err(|err| {
        CliError::from_boxed(err, 101)
    })
}
