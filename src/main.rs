extern crate log;
extern crate simplelog;

use aliyunddns::cli::{Mode, Options};
use aliyunddns::init_log;

fn main() {
    let mut option = Options::from_args();

    // 当使用Docker方式运行时，通过环境变量获取参数
    if let Mode::Docker = option.mode {
        option = Options::from_env();
    }

    init_log();
    log::info!("{:#?}", option);
}