use std::collections::HashMap; // HashMap library
use std::fs; // File system reading library
use std::string; // String library

/* Struct/Class LegendParser
    Responsibility: 
        Read in text file containing keys for indexing database columns containing a collection of
        information. Examples include Longitude, Latitude, Station Name. Legend should be formatted 
        as an [Entry] delimited by space(s) and a string [Description] 
        [Entry] should not contain spaces while [Description] can hold any length or character
        Example: 

        Legend
        Long Longitude (West - , degrees) good
         
    Input: 
        - path: String variable describing absolute file path to en_climate_summaries_legend.txt
    Output: 
        - map: HashMap collection mapping [Entry] to column index
*/
// Assuming this is how I declare struct/class members
struct LegendParser {
    path: String,
    map: HashMap<String, u32>,
}

// I'm not sure if this is the correct implementation to describe method functions for a given
// class/struct in Rust. I assume this acts similar to a header file in C++ 
// Reference: https://www.educative.io/edpresso/what-are-traits-in-rust

trait Methods {
    // Not sure if I need to include Constructor function new() here
    // PUBLIC CLASS METHOD FUNCTIONS:
    // Returns HashMap of results
    pub fn get_map(&self) -> HashMap<String, u32>;
    // Generates HashMap from file path to legends.txt
    pub fn generate_map(&self, path: String);
    
    // PRIVATE FUNCTIONS:
    // Helper Method to dump contents to variable in a format similar to vector<String in C++ if
    // given path to file. Also validates that file exists and returns empty string if not
    fn read_lines<P>(&self, filename: P) -> io::Result<io::Lines<io::BufReader<File>>>;
    // Helper method to split any given line from vector<string> into [Entry] and [Description]
    fn split_line(&self, line: String) -> (String, String)
}

impl Methods for LegendParser {
    // Constructor Function for LegendParser
    // Apparently violates Rust concepts for "One True Construtor method"?
    // https://doc.rust-lang.org/nomicon/constructors.html
    pub fn new() -> LegendParser {
            path: ""
            map: HashMap::new();     
        }
    }

    pub fn get_map(&self) -> HashMap<String, u32> {
        self.map
    }

    // Public method to read a given file path to legend text and return a HashMap of the  
    pub fn generate_map(&self, path: String) {
        
        // Read entry and validate that path exists
        if let Ok(lines) =  self.read_lines(path);
        // Assume this Ok() statement throws an exception here?
        // Insert code to handle if Not Ok()

        // Assume lines is in a format similar to vector<string>
        let column_index = 0;
        // Remove first line as it is the "Legend" line with no description
        lines = lines[1:]

        let map = HashMap::new();
        // Iterate through lines and inserting string into HashMap
        for (line in lines) {
            if (not line.empty()) { // Assume vector.empty() method exists here
                // Insert first substring in line with column index.
                map.insert(line.1, column_index);
                column_index += 1; // Apparently Rust does not have increment/decrement operators??
            } 
            else {
                // throw exception error here, if line is empty -> implies text file is incorrect
                // in formatting
                // Assume that EOF or last line of text file does not contain stray empty line
            }
        }

        // Assign HashMap to struct
        self.map = map;
    }

    // Copied from https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
    // Assumes that this method reads a file given a path and returns 

    fn read_lines<P>(&self, filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
        let file = File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
    }

    // Method to split a string into two by splitting a given string line into two based on first 
    // pattern of ' '. Returns a tuple of strings (substringA, substringB)
    // https://doc.rust-lang.org/std/primitive.str.html#method.splitn

    fn split_line(&self, line: String) -> (String, String) {
        // Split line into two based on space delimiter
        let mut splitter = line.splitn(2, ' ');

        // Check if splitter has size 2, return empty string tuple if invalid!
        if (splitter.size() != 2) {
            return (String::new(), String::new());
        }
        else {
            // Get first and second substring
            let first = splitter.next().unwrap();
            let second = splitter.next().unwrap();
            // Return first and second substring in the form of a tuple
            (first, second) // Return tuple of (string first, string second);
        }
    }
}