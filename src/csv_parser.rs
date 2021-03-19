use std::collections::HashMap; // HashMap library
use std::fs; // File system reading library
use std::string; // String library
use csv::Error;
extern crate csv;


// Reference https://rust-lang-nursery.github.io/rust-cookbook/encoding/csv.html
// Refernece https://docs.rs/csv/1.0.0/csv/tutorial/index.html

/* Struct/Class CsvParser
    Responsibility: 
        Read in csv file containing data for regions in various Canadian Provinces & Territories
         
    Input: 
        - path: String variable describing absolute file path to en_climate_summaries_legend.txt
    Output: 
        - data:  csv::StringRecord
*/
// Assuming this is how I declare struct/class members
struct CsvParser {
    data: csv::StringRecord
}

// I'm not sure if this is the correct implementation to describe method functions for a given
// class/struct in Rust. I assume this acts similar to a header file in C++ 
// Reference: https://www.educative.io/edpresso/what-are-traits-in-rust

trait Methods {
    // PUBLIC CLASS METHOD FUNCTIONS:
    // Returns HashMap
    pub fn get_data(&self) -> csv::StringRecord;
    // Generates HashMap from file path to legends.txt
    pub fn read_csv(&self, path: String);

    // PRIVATE FUNCTIONS:
    
}

impl Methods for CsvParser {
    // Constructor Function for CsvParser
    // Apparently violates Rust concepts for "One True Construtor method"?
    // https://doc.rust-lang.org/nomicon/constructors.html
    pub fn new() -> CsvParser {
            path: ""
        }
    }
    // Return CSV reader object for processing
    pub fn get_data(&self) -> HashMap<String, u32> {
        self.data
    }

    // Public method to read csv a given file path and store csv reader object  
    pub fn read_csv(&self, path: String) {
        // Read CSV file
        // According to documentation csv Reader assumes by default there is a header
        let mut rdr = csv::Reader::from_reader(io::stdin());
        self.data = rdr;
    }

}