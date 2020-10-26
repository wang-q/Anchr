use clap::*;
use std::collections::HashMap;
use tera::{Context, Tera};

// Create clap subcommand arguments
pub fn make_subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("template")
        .about("Creates Bash scripts")
        .after_help(
            r#"
* Info

    * --genome
    * --se
    * --xmx
    * --parallel 8
    * --queue mpi

* Quality check

    * --fastqc
    * --kmergenie

* Trimming

    * --trim "--dedupe"
    * --qual "25 30"
    * --len "60"
    * --filter "adapter"

* Post-trimming

    * --quorum
    * --merge
    * --prefilter
    * --ecphase "1 2 3"

* Downsampling

    * --cov "40 80"
    * --splitp 20
    * --statp 2

"#,
        )
        // Info
        .arg(
            Arg::with_name("genome")
                .long("genome")
                .help("Your best guess of the haploid genome size")
                .takes_value(true)
                .default_value("1000000")
                .empty_values(false),
        )
        .arg(Arg::with_name("se").long("se").help("Single end mode"))
        .arg(
            Arg::with_name("xmx")
                .long("xmx")
                .help("Set Java memory usage")
                .takes_value(true)
                .empty_values(false),
        )
        .arg(
            Arg::with_name("parallel")
                .long("parallel")
                .short("p")
                .help("Number of threads")
                .takes_value(true)
                .default_value("8")
                .empty_values(false),
        )
        .arg(
            Arg::with_name("queue")
                .long("queue")
                .help("Queue name of the LSF cluster")
                .takes_value(true)
                .default_value("mpi")
                .empty_values(false),
        )
        // Quality check
        .arg(Arg::with_name("fastqc").long("fastqc").help("Run FastQC"))
        .arg(
            Arg::with_name("kmergenie")
                .long("kmergenie")
                .help("Run KmerGenie"),
        )
        // Trimming
        .arg(
            Arg::with_name("trim")
                .long("trim")
                .help("Opts for trim")
                .takes_value(true)
                .default_value("--dedupe")
                .empty_values(false)
                .allow_hyphen_values(true),
        )
        .arg(
            Arg::with_name("sample")
                .long("sample")
                .help("Sampling coverage")
                .takes_value(true)
                .empty_values(false),
        )
        .arg(
            Arg::with_name("qual")
                .long("qual")
                .help("Quality threshold")
                .takes_value(true)
                .default_value("25 30")
                .empty_values(false),
        )
        .arg(
            Arg::with_name("len")
                .long("len")
                .help("Filter reads less or equal to this length")
                .takes_value(true)
                .default_value("60")
                .empty_values(false),
        )
        .arg(
            Arg::with_name("filter")
                .long("filter")
                .help("Adapter, artifact, or both")
                .takes_value(true)
                .default_value("adapter")
                .empty_values(false),
        )
        // Post-trimming
        .arg(Arg::with_name("quorum").long("quorum").help("Run quorum"))
        .arg(
            Arg::with_name("merge")
                .long("merge")
                .help("Run merge reads"),
        )
        .arg(
            Arg::with_name("prefilter")
                .long("prefilter")
                .help("Prefilter=N (1 or 2) for tadpole and bbmerge")
                .takes_value(true)
                .empty_values(false),
        )
        .arg(
            Arg::with_name("ecphase")
                .long("ecphase")
                .help("Error-correct phases. Phase 2 can be skipped")
                .takes_value(true)
                .default_value("1 2 3")
                .empty_values(false),
        )
        // Downsampling
        .arg(
            Arg::with_name("cov")
                .long("cov")
                .help("Down sampling coverages")
                .takes_value(true)
                .default_value("40 80")
                .empty_values(false),
        )
        .arg(
            Arg::with_name("splitp")
                .long("splitp")
                .help("Parts of splitting")
                .takes_value(true)
                .default_value("20")
                .empty_values(false),
        )
        .arg(
            Arg::with_name("statp")
                .long("statp")
                .help("Parts of stats")
                .takes_value(true)
                .default_value("2")
                .empty_values(false),
        )
}

// command implementation
pub fn execute(args: &ArgMatches) -> std::result::Result<(), std::io::Error> {
    //----------------------------
    // context from args
    //----------------------------
    let mut opt = HashMap::new();

    opt.insert(
        "genome",
        if args.is_present("genome") {
            args.value_of("genome").unwrap()
        } else {
            "0"
        },
    );
    opt.insert("se", if args.is_present("se") { "1" } else { "0" });
    opt.insert(
        "xmx",
        if args.is_present("xmx") {
            args.value_of("xmx").unwrap()
        } else {
            "0"
        },
    );
    opt.insert("parallel", args.value_of("parallel").unwrap());
    opt.insert("queue", args.value_of("queue").unwrap());

    opt.insert("trim", args.value_of("trim").unwrap());
    opt.insert(
        "sample",
        if args.is_present("sample") {
            args.value_of("sample").unwrap()
        } else {
            "0"
        },
    );
    opt.insert("qual", args.value_of("qual").unwrap());
    opt.insert("len", args.value_of("len").unwrap());
    opt.insert("filter", args.value_of("filter").unwrap());

    opt.insert(
        "prefilter",
        if args.is_present("prefilter") {
            args.value_of("prefilter").unwrap()
        } else {
            "0"
        },
    );
    opt.insert("ecphase", args.value_of("ecphase").unwrap());

    opt.insert("cov", args.value_of("cov").unwrap());
    opt.insert("splitp", args.value_of("splitp").unwrap());
    opt.insert("statp", args.value_of("statp").unwrap());

    let mut context = Context::new();
    context.insert("opt", &opt);

    //----------------------------
    // create scripts
    //----------------------------
    if args.is_present("fastqc") {
        gen_fastqc(&context)?;
    }

    if args.is_present("kmergenie") {
        gen_kmergenie(&context)?;
    }

    gen_trim(&context)?;

    gen_stat_reads(&context)?;

    if args.is_present("quorum") {
        gen_quorum(&context)?;
    } else {
        gen_no_quorum(&context)?;
    }
    gen_down_sampling(&context)?;
    gen_unitigs(&context)?;
    gen_anchors(&context)?;
    gen_stat_anchors(&context)?;

    if !args.is_present("se") && args.is_present("merge") {
        gen_merge(&context)?;
        gen_mr_down_sampling(&context)?;
        gen_mr_unitigs(&context)?;
        gen_mr_anchors(&context)?;
        gen_stat_mr_anchors(&context)?;
    }

    gen_cleanup(&context)?;
    gen_real_clean(&context)?;
    gen_master(&context)?;

    Ok(())
}

fn gen_fastqc(context: &Context) -> std::result::Result<(), std::io::Error> {
    let outname = "2_fastqc.sh";
    eprintln!("Create {}", outname);

    let mut tera = Tera::default();
    tera.add_raw_templates(vec![
        ("header", include_str!("../../templates/header.tera.sh")),
        ("t", include_str!("../../templates/2_fastqc.tera.sh")),
    ])
    .unwrap();

    let rendered = tera.render("t", &context).unwrap();
    intspan::write_lines(outname, &vec![rendered.as_str()])?;

    Ok(())
}

fn gen_kmergenie(context: &Context) -> std::result::Result<(), std::io::Error> {
    let outname = "2_kmergenie.sh";
    eprintln!("Create {}", outname);

    let mut tera = Tera::default();
    tera.add_raw_templates(vec![
        ("header", include_str!("../../templates/header.tera.sh")),
        ("t", include_str!("../../templates/2_kmergenie.tera.sh")),
    ])
    .unwrap();

    let rendered = tera.render("t", &context).unwrap();
    intspan::write_lines(outname, &vec![rendered.as_str()])?;

    Ok(())
}

fn gen_trim(context: &Context) -> std::result::Result<(), std::io::Error> {
    let outname = "2_trim.sh";
    eprintln!("Create {}", outname);

    let mut tera = Tera::default();
    tera.add_raw_templates(vec![
        ("header", include_str!("../../templates/header.tera.sh")),
        ("t", include_str!("../../templates/2_trim.tera.sh")),
    ])
    .unwrap();

    let rendered = tera.render("t", &context).unwrap();
    intspan::write_lines(outname, &vec![rendered.as_str()])?;

    Ok(())
}

fn gen_stat_reads(context: &Context) -> std::result::Result<(), std::io::Error> {
    let outname = "9_stat_reads.sh";
    eprintln!("Create {}", outname);

    let mut tera = Tera::default();
    tera.add_raw_templates(vec![
        ("header", include_str!("../../templates/header.tera.sh")),
        ("t", include_str!("../../templates/9_stat_reads.tera.sh")),
    ])
    .unwrap();

    let rendered = tera.render("t", &context).unwrap();
    intspan::write_lines(outname, &vec![rendered.as_str()])?;

    Ok(())
}

fn gen_quorum(context: &Context) -> std::result::Result<(), std::io::Error> {
    let outname = "2_quorum.sh";
    eprintln!("Create {}", outname);

    let mut tera = Tera::default();
    tera.add_raw_templates(vec![
        ("header", include_str!("../../templates/header.tera.sh")),
        ("t", include_str!("../../templates/2_quorum.tera.sh")),
    ])
    .unwrap();

    let rendered = tera.render("t", &context).unwrap();
    intspan::write_lines(outname, &vec![rendered.as_str()])?;

    Ok(())
}

fn gen_no_quorum(context: &Context) -> std::result::Result<(), std::io::Error> {
    let outname = "2_quorum.sh";
    eprintln!("Create {}", outname);

    let mut tera = Tera::default();
    tera.add_raw_templates(vec![
        ("header", include_str!("../../templates/header.tera.sh")),
        ("t", include_str!("../../templates/2_no_quorum.tera.sh")),
    ])
    .unwrap();

    let rendered = tera.render("t", &context).unwrap();
    intspan::write_lines(outname, &vec![rendered.as_str()])?;

    Ok(())
}

fn gen_merge(context: &Context) -> std::result::Result<(), std::io::Error> {
    let outname = "2_merge.sh";
    eprintln!("Create {}", outname);

    let mut tera = Tera::default();
    tera.add_raw_templates(vec![
        ("header", include_str!("../../templates/header.tera.sh")),
        ("t", include_str!("../../templates/2_merge.tera.sh")),
    ])
    .unwrap();

    let rendered = tera.render("t", &context).unwrap();
    intspan::write_lines(outname, &vec![rendered.as_str()])?;

    Ok(())
}

fn gen_down_sampling(context: &Context) -> std::result::Result<(), std::io::Error> {
    let outname = "4_down_sampling.sh";
    eprintln!("Create {}", outname);

    let mut tera = Tera::default();
    tera.add_raw_templates(vec![
        ("header", include_str!("../../templates/header.tera.sh")),
        ("t", include_str!("../../templates/4_down_sampling.tera.sh")),
    ])
    .unwrap();

    let rendered = tera.render("t", &context).unwrap();
    intspan::write_lines(outname, &vec![rendered.as_str()])?;

    Ok(())
}

fn gen_mr_down_sampling(context: &Context) -> std::result::Result<(), std::io::Error> {
    let outname = "6_down_sampling.sh";
    eprintln!("Create {}", outname);

    let mut tera = Tera::default();
    tera.add_raw_templates(vec![
        ("header", include_str!("../../templates/header.tera.sh")),
        ("t", include_str!("../../templates/6_down_sampling.tera.sh")),
    ])
    .unwrap();

    let rendered = tera.render("t", &context).unwrap();
    intspan::write_lines(outname, &vec![rendered.as_str()])?;

    Ok(())
}

fn gen_unitigs(context: &Context) -> std::result::Result<(), std::io::Error> {
    let outname = "4_unitigs.sh";
    eprintln!("Create {}", outname);

    let mut tera = Tera::default();
    tera.add_raw_templates(vec![
        ("header", include_str!("../../templates/header.tera.sh")),
        ("t", include_str!("../../templates/4_unitigs.tera.sh")),
    ])
    .unwrap();

    let rendered = tera.render("t", &context).unwrap();
    intspan::write_lines(outname, &vec![rendered.as_str()])?;

    Ok(())
}

fn gen_mr_unitigs(context: &Context) -> std::result::Result<(), std::io::Error> {
    let outname = "6_unitigs.sh";
    eprintln!("Create {}", outname);

    let mut tera = Tera::default();
    tera.add_raw_templates(vec![
        ("header", include_str!("../../templates/header.tera.sh")),
        ("t", include_str!("../../templates/6_unitigs.tera.sh")),
    ])
    .unwrap();

    let rendered = tera.render("t", &context).unwrap();
    intspan::write_lines(outname, &vec![rendered.as_str()])?;

    Ok(())
}

fn gen_anchors(context: &Context) -> std::result::Result<(), std::io::Error> {
    let outname = "4_anchors.sh";
    eprintln!("Create {}", outname);

    let mut tera = Tera::default();
    tera.add_raw_templates(vec![
        ("header", include_str!("../../templates/header.tera.sh")),
        ("t", include_str!("../../templates/4_anchors.tera.sh")),
    ])
    .unwrap();

    let rendered = tera.render("t", &context).unwrap();
    intspan::write_lines(outname, &vec![rendered.as_str()])?;

    Ok(())
}

fn gen_mr_anchors(context: &Context) -> std::result::Result<(), std::io::Error> {
    let outname = "6_anchors.sh";
    eprintln!("Create {}", outname);

    let mut tera = Tera::default();
    tera.add_raw_templates(vec![
        ("header", include_str!("../../templates/header.tera.sh")),
        ("t", include_str!("../../templates/6_anchors.tera.sh")),
    ])
    .unwrap();

    let rendered = tera.render("t", &context).unwrap();
    intspan::write_lines(outname, &vec![rendered.as_str()])?;

    Ok(())
}

fn gen_stat_anchors(context: &Context) -> std::result::Result<(), std::io::Error> {
    let outname = "9_stat_anchors.sh";
    eprintln!("Create {}", outname);

    let mut tera = Tera::default();
    tera.add_raw_templates(vec![
        ("header", include_str!("../../templates/header.tera.sh")),
        ("t", include_str!("../../templates/9_stat_anchors.tera.sh")),
    ])
    .unwrap();

    let rendered = tera.render("t", &context).unwrap();
    intspan::write_lines(outname, &vec![rendered.as_str()])?;

    Ok(())
}

fn gen_stat_mr_anchors(context: &Context) -> std::result::Result<(), std::io::Error> {
    let outname = "9_stat_mr_anchors.sh";
    eprintln!("Create {}", outname);

    let mut tera = Tera::default();
    tera.add_raw_templates(vec![
        ("header", include_str!("../../templates/header.tera.sh")),
        ("t", include_str!("../../templates/9_stat_mr_anchors.tera.sh")),
    ])
    .unwrap();

    let rendered = tera.render("t", &context).unwrap();
    intspan::write_lines(outname, &vec![rendered.as_str()])?;

    Ok(())
}

fn gen_cleanup(context: &Context) -> std::result::Result<(), std::io::Error> {
    let outname = "0_cleanup.sh";
    eprintln!("Create {}", outname);

    let mut tera = Tera::default();
    tera.add_raw_templates(vec![
        ("header", include_str!("../../templates/header.tera.sh")),
        ("t", include_str!("../../templates/0_cleanup.tera.sh")),
    ])
    .unwrap();

    let rendered = tera.render("t", &context).unwrap();
    intspan::write_lines(outname, &vec![rendered.as_str()])?;

    Ok(())
}

fn gen_real_clean(context: &Context) -> std::result::Result<(), std::io::Error> {
    let outname = "0_real_clean.sh";
    eprintln!("Create {}", outname);

    let mut tera = Tera::default();
    tera.add_raw_templates(vec![
        ("header", include_str!("../../templates/header.tera.sh")),
        ("t", include_str!("../../templates/0_real_clean.tera.sh")),
    ])
    .unwrap();

    let rendered = tera.render("t", &context).unwrap();
    intspan::write_lines(outname, &vec![rendered.as_str()])?;

    Ok(())
}

fn gen_master(context: &Context) -> std::result::Result<(), std::io::Error> {
    let outname = "0_master.sh";
    eprintln!("Create {}", outname);

    let mut tera = Tera::default();
    tera.add_raw_templates(vec![
        ("header", include_str!("../../templates/header.tera.sh")),
        ("t", include_str!("../../templates/0_master.tera.sh")),
    ])
    .unwrap();

    let rendered = tera.render("t", &context).unwrap();
    intspan::write_lines(outname, &vec![rendered.as_str()])?;

    Ok(())
}
