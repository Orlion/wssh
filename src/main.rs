mod command;
mod error;
mod jumpserver;
mod terminal;
mod wss;

#[tokio::main]
async fn main() {
    let clap_command = clap::Command::new("wssh")
        .version("0.1.0")
        .author("Orlion")
        .about("SSH over Websocket 客户端")
        .arg(
            clap::Arg::new("env")
                .long("env")
                .short('e')
                .help("环境 test/preview")
                .value_name("ENV")
                .required(true),
        );

    let mut jumpserver = jumpserver::Jumpserver::new();
    if let Err(e) = jumpserver.login() {
        eprintln!("jumpserver登录失败: {}", e);
        return;
    }

    let command = command::Command::new(clap_command, jumpserver);
    if let Err(e) = command.run().await {
        eprintln!("执行命令失败: {}", e);
    }
}
