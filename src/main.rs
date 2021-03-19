use std::env 
use std::io; // Input/Output IO
use std::error::Error; // Think this is Error library similar to try/catch in C++

mod data_processor;
mod legend_parser;
mod csv_parser;
mod tokios

/* CORE ASSUMPTIONS
    Ignore data from Non-Provinces like Yukon, Northwest Terrotories, and Nunavut (YK, NT, NU)
    Data can be incomplete and/or missing but columns with at least 1 data point must exist
    Only need to process Mean Temperature (Tm), no modular design for other use cases such as 
    Calculating highest/lowest temperature ranges, total time with sunshine, etc
    Computing unit does not need to account for sudden failures such as node failure or power loss
    Computing unit has an unholy/infinite number of CPU working threads   
*/

fn main() -> Result<(), Error> {
    
    // Get std input for absolute/relative path to the (1) en_climate_summaries_All_02-2021 
    // and (2) en_climate_summaries_legend.txt
    let args: Vec<String> = env::args().collect();

    let legend_path = &args[1];
    let csv_path = &args[2];
    
    
    let processor = DataProcessor{legend_path, csv_path};
    if (processor.calc_mean_prov_temp()) {
        processor.get_mean_prov_temp(); // return HashMap<String, Results>

        return 0; // EXIT SUCCEEDED 
    }
    else {
        return 1; // EXIT CODE FAILURE OCCURED
    }


    
}
