use anyhow::{Ok, Result};
use clap::Parser;
use rcli::{process_csv, process_genpass, Opts, SubCommand};

fn main() -> Result<()> {
    let opts = Opts::parse();

    println!("{:?}", opts);

    match opts.cmd {
        SubCommand::Csv(opts) => {
            let format = opts.format;
            let output = match opts.output {
                Some(output) => output.clone(),
                None => format!("output.{}", &format),
            };
            process_csv(&opts.input, &output, format)?
        }
        SubCommand::GenPass(opts) => {
            let password = process_genpass(
                opts.length,
                opts.upppercase,
                opts.lowercase,
                opts.numbers,
                opts.symbol,
            )?;
            println!("{}", password);
        }
    }

    Ok(())
}
