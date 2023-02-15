use std::fs;
use std::io;

use std::env;
use std::path::Path;

// use serde_json::json;
use shared::drivers::config::DialogJson;
use shared::drivers::config::Config;

use zeiss;
use werth;

use toml;


fn read_args(config: &mut Config){
    let args: Vec<String> = env::args().collect();
    let mut args_iter = args.iter();

    while let Some(item) = args_iter.next(){
        match item.as_str() {
            "--partID" => {config.part_id = Some(args_iter.next().unwrap_or(&"".to_string()).to_string())}
            _ => {}
        }
    }
}

fn main() {
    // config.toml file options:
    // [machine]
    // type="zeiss"                 # zeiss, werth
    // dmesn="501717"               # machine serial number
    // dmeswv="6.8.2606"            # software version
    // controllertyp="C32BIT"       # controllertype for zeiss
    // dmeswi="Calypso"             # software name
    // dmeid="CONT_G2"              # machine type

    // [configuation]
    // machine_result_path=".\\test_data\\results"  
    // cmm_result_path=".\\test_data"
    // xml_result_path="O:\\Measurement\\results"
    // database = { uri="hello", target="123" }


    let config_file = fs::read_to_string(r"config.toml").expect("error config.toml not found");
    let mut config: Config = toml::from_str(config_file.as_str()).expect("error loading config.toml file");

    read_args(&mut config);
    
    let dialog_path = Path::new(config.configuation.cmm_result_path.as_str())
        .join(config.part_id.as_ref().unwrap_or(&"".to_string()))
        .join("dialog.json");
    
    
    if dialog_path.exists(){
        config.dialog_data = DialogJson::from_file(dialog_path.to_str().unwrap());
        
        // if Setup is selected in the start Dialog we don't want an output.
        // if Incomming Measurement is selected we want to import the scan data
        match config.dialog_data.as_ref().unwrap().Dialog.operation.as_str() {
            "Setup" => { return },                  
            "Incomming Measurement" => {            
                config.dialog_data.as_mut().unwrap().Setup.importScan = true;
            } 
            _ => {}
        } 

    } else {
        println!("no dialog.json file {}\npress ENTER to continue", dialog_path.to_str().unwrap());
        let mut buf = String::new();
        let _res = io::stdin().read_line(&mut buf); // to be able to read the message, waiting here for operator to confirm with Enter
        
        return
    }

    match config.machine.machine_type.to_ascii_lowercase().as_str() {
        "zeiss" => {zeiss::convert(&mut config)},
        "werth" => {werth::convert(&mut config)},

        _=> {println!("invalid machine type")}
    }

    let current_counter = match config.dialog_data.as_ref() {
        Some(d) => {d.Dialog.counter.unwrap_or(0u32)},
        None => {0u32},
    };

    match config.dialog_data.as_mut() {
        Some(d) => {d.Dialog.counter = Some(current_counter + 1) },
        None => {},
    };

    println!("{}", serde_json::to_string(&config).unwrap());

    let _res = config.dialog_data.expect("dialog.json not specified").to_file(dialog_path.to_str().unwrap());

}

