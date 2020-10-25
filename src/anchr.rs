extern crate clap;
use clap::*;

mod cmd;

fn main() -> std::io::Result<()> {
    let app = App::new("anchr")
        .version(crate_version!())
        .author(crate_authors!())
        .about("anchr - Assembler of N-free CHRomosomes")
        .setting(AppSettings::ArgRequiredElseHelp)
        .subcommand(cmd::dep::make_subcommand())
        .subcommand(cmd::ena::make_subcommand())
        .subcommand(cmd::kunitigs::make_subcommand())
        .subcommand(cmd::merge::make_subcommand())
        .subcommand(cmd::quorum::make_subcommand())
        .subcommand(cmd::template::make_subcommand())
        .subcommand(cmd::trim::make_subcommand());

    // Check which subcomamnd the user ran...
    match app.get_matches().subcommand() {
        ("dep", Some(sub_matches)) => cmd::dep::execute(sub_matches),
        ("ena", Some(sub_matches)) => cmd::ena::execute(sub_matches),
        ("kunitigs", Some(sub_matches)) => cmd::kunitigs::execute(sub_matches),
        ("merge", Some(sub_matches)) => cmd::merge::execute(sub_matches),
        ("quorum", Some(sub_matches)) => cmd::quorum::execute(sub_matches),
        ("template", Some(sub_matches)) => cmd::template::execute(sub_matches),
        ("trim", Some(sub_matches)) => cmd::trim::execute(sub_matches),
        (_, _) => unreachable!(),
    }?;

    Ok(())
}
