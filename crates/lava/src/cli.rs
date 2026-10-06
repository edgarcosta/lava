use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    name = "lava",
    version,
    about = "A community-maintained multi-tool for the Magma language",
    long_about = "lava is a single-binary CLI offering formatter, highlighter,\n\
                  and parsing tools for Magma. Use `lava <subcommand> --help` for\n\
                  per-subcommand options."
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Format Magma source files
    #[command(alias = "fmt")]
    Format(FormatArgs),

    /// Output syntax-highlighted source (default: ANSI terminal colours)
    #[command(alias = "hl")]
    Highlight(HighlightArgs),

    /// Parse a file and print the syntax tree (planned for v0.2)
    Parse,

    // TODO: Replace this with a short description, and make the long description appear at --help
    // TODO: make count exclude ignored tests (write X/Y tests)
    // TODO: rename --include-ignored to --show-ignored
    // TODO: include single-thread option
    // TODO: print test results as they complete
    // TODO: terminate magma processes gracefully
    #[command(
        alias = "t",
        about = "Run Magma test procedures",
	long_about = "Run Magma test procedures.\n\
		      Tests are discovered from .m files in the test/ or tests/ directory.\n\
		      Each top-level procedure is treated as a test case. To skip a test,\n\
		      add a `// ignore` comment (case-insensitive) on the line before it:\n\n\
		      // ignore\n\
		      procedure TestNotReadyYet()\n\
		      ...\n\
		      end procedure;\n\n\
		      Tests are run in parallel by default.\n\n\
		      To run tests in serial - for example for file manipulations - include \"serial\"\n\
		      in a preceding comment like so:\n\n\
		      // serial\n\
		      procedure IUseAFile()\n\
		      ...\n\
		      end procedure;\n"
    )]
    Test(TestArgs),
}

#[derive(clap::Args, Debug)]
pub struct HighlightArgs {
    /// Files to highlight. With zero paths or `-`, reads stdin.
    pub paths: Vec<PathBuf>,

    /// Path to a custom highlight query file.
    #[arg(long, value_name = "PATH")]
    pub query: Option<PathBuf>,

    /// Output HTML instead of ANSI terminal colours.
    #[arg(long)]
    pub html: bool,
}

#[derive(clap::Args, Debug)]
pub struct FormatArgs {
    /// Files to format. With zero paths or `-`, reads stdin.
    pub paths: Vec<PathBuf>,

    /// Rewrite files in place (atomic).
    #[arg(short = 'w', long)]
    pub write: bool,

    /// Exit 1 if any file would change; do not modify files.
    #[arg(short = 'c', long, conflicts_with = "write")]
    pub check: bool,

    /// Print a unified diff alongside --check (default: paths only).
    #[arg(long, requires = "check")]
    pub diff: bool,

    /// Path to a custom magma.scm query file.
    #[arg(long, value_name = "PATH")]
    pub query: Option<PathBuf>,

    /// Format files containing parse errors on a best-effort basis.
    #[arg(long)]
    pub tolerate_parse_errors: bool,

    /// Recurse into directories (respects .gitignore and .lavaignore).
    #[arg(short = 'r', long)]
    pub recursive: bool,

    /// Filename to display in diagnostics when reading from stdin.
    #[arg(long, value_name = "PATH")]
    pub stdin_filename: Option<PathBuf>,
}

#[derive(clap::Args, Debug)]
pub struct TestArgs {
    /// Files or directories containing tests. Defaults to `test/` or `tests/`.
    pub paths: Vec<PathBuf>,

    /// Path to the Magma binary. Defaults to `magma` in PATH.
    #[arg(long, value_name = "PATH")]
    pub magma: Option<PathBuf>,

    /// Show ignored tests in output (hidden by default).
    #[arg(long)]
    pub include_ignored: bool,

    /// Run tests in serial. Default 
    #[arg(short = 's', long)]
    pub serial: bool
}
