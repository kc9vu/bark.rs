use structopt::StructOpt as _;

fn main() {
    let mut opt = bark_cli::my_types::Opt::from_args();
    opt.check().expect("出错啦!");

    let resp = bark_cli::app::new_push(&opt);
    if resp.code == 200 {
        println!("{}", &resp.message);
    } else {
        eprintln!("{}", &resp.message);
    }
}
