use structopt::StructOpt as _;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut opt = bark_cli::my_types::Opt::from_args();
    // opt.patch(app::parse_config(&opt.file)?)?;

    if let Err(error) = opt.check() {
        eprintln!("Error: {}", error);
    } else {
        let resp = opt.notify().await?;
        if resp.code == 200 {
            println!("{}", &resp.message);
        } else {
            eprintln!("{}", &resp.message);
        }
        // println!("{:#?}", &opt);
        // println!("{}", &opt.dumps());
    }
    Ok(())
}
