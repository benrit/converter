

pub mod zeiss_test{
    use std::path::PathBuf;

    use serde_json::json;

    use crate::read_chr_file;
    use shared::drivers::elements::{ChrItem, HType};
    
    
    #[test]
    pub fn convert_test(){

        let chr_testFile = PathBuf::from("../test_data/results/test_chr.txt");
        let res = read_chr_file(&chr_testFile);
        
    }
    
}


