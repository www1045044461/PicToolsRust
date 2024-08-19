use clap::Parser;
use fileinfos::param;

mod compares;
mod fileinfos;

fn main() {
    let cli = param::Parameters::parse();

    if let Some(m) = &cli.sub_commands {
        println!("指令模式:{:?}", m);
    } else {
        println!("无效的指令模式");
    }
}
