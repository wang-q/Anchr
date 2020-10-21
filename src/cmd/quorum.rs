use clap::*;
use tera::{Tera, Context};
use std::collections::HashMap;
use itertools::Itertools;

// Create clap subcommand arguments
pub fn make_subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("quorum")
        .about("Run quorum to discard bad reads")
        .after_help(
            r#"
<PE file1> [PE file2] [SE file]

Fastq files can be gzipped
"#,
        )
        .arg(
            Arg::with_name("infiles")
                .help("Sets the input file to use")
                .required(true)
                .min_values(1)
                .index(1),
        )
        .arg(
            Arg::with_name("jf")
                .help("Jellyfish hash size")
                .long("coverage")
                .takes_value(true)
                .default_value("500000000")
                .empty_values(false),
        )
        .arg(
            Arg::with_name("estsize")
                .help("Estimated genome size")
                .long("estsize")
                .takes_value(true)
                .default_value("auto")
                .empty_values(false),
        )
        .arg(
            Arg::with_name("prefix")
                .help("Prefix of .cor.fa.gz")
                .long("prefix")
                .takes_value(true)
                .default_value("pe")
                .empty_values(false),
        )
        .arg(
            Arg::with_name("parallel")
                .help("Number of threads")
                .long("parallel")
                .short("p")
                .takes_value(true)
                .default_value("8")
                .empty_values(false),
        )
        .arg(
            Arg::with_name("outfile")
                .short("o")
                .long("outfile")
                .takes_value(true)
                .default_value("quorum.sh")
                .empty_values(false)
                .help("Output filename. [stdout] for screen"),
        )
}

// command implementation
pub fn execute(args: &ArgMatches) -> std::result::Result<(), std::io::Error> {
    let mut writer = intspan::writer(args.value_of("outfile").unwrap());

    // context from args
    let mut opt = HashMap::new();
    opt.insert("jf", args.value_of("jf").unwrap());
    opt.insert("estsize", args.value_of("estsize").unwrap());
    opt.insert("prefix", args.value_of("prefix").unwrap());
    opt.insert("parallel", args.value_of("parallel").unwrap());

    let infiles = args.values_of("infiles").unwrap().collect_vec();

    let mut context = Context::new();
    context.insert("opt", &opt);
    context.insert("args", &infiles);

    // eprintln!("{:#?}", context);

    // many templates
    let mut tera = Tera::default();
    tera.add_raw_templates(vec![
        ("header", include_str!("../../templates/header.tera.sh")),
        ("quorum", include_str!("../../templates/quorum.tera.sh")),
    ]).unwrap();

    // eprintln!("{:#?}", tera);

    let rendered = tera.render("quorum", &context).unwrap();

    writer.write_all(rendered.as_ref())?;

    Ok(())
}
