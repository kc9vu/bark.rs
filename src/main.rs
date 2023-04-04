use bark_cli::app;

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let opt = app::options()?;
    let url = app::build_url(&opt)?;
    let resp = app::push(&url)?;
    println!("{}", &resp.message);
    Ok(())
}
