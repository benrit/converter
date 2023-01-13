use std::fs;
use std::env;
use std::path::Path;

use shared::drivers::config::DialogJson;
use shared::drivers::config::Config;

use zeiss;
use werth;

use toml;

fn main() {

    let config_file = fs::read_to_string(r"config.toml").expect("error config.toml not found");
    let mut config: Config = toml::from_str(config_file.as_str()).expect("error loading config.toml file");

    let args: Vec<String> = env::args().collect();
    let mut args_iter = args.iter();

    while let Some(item) = args_iter.next(){
        match item.as_str() {
            "--partID" => {config.part_id = Some(args_iter.next().unwrap_or(&"".to_string()).to_string())}
            _ => {}
        }
    }

    let dialog_path = Path::new(config.configuation.cmm_result_file.as_str())
        .join(config.part_id.as_ref().unwrap())
        .join("dialog.json");


    config.dialog_data = DialogJson::from_file(dialog_path.to_str().unwrap());
    if config.dialog_data.as_ref().unwrap().Dialog.operation == "Setup" {return}


    match config.machine.machine_type.to_ascii_lowercase().as_str() {
        "zeiss" => {zeiss::convert(&mut config)},
        "werth" => {werth::convert(&mut config)},

        _=> {println!("invalid machine type")}
    }

    // println!("{:?}", config.dialog_data);
    let current_counter = match config.dialog_data.as_ref() {
        Some(d) => {d.Dialog.counter.unwrap_or(0u32)},
        None => {0u32},
    };

    match config.dialog_data.as_mut() {
        Some(d) => {d.Dialog.counter = Some(current_counter + 1) },
        None => {},
    };

    let _res = config.dialog_data.expect("dialog.json not specified").to_file(dialog_path.to_str().unwrap());

}

