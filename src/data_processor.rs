use std::collections::HashMap; // HashMap library
use std::fs; // File system reading library
use std::string; // String library

/* Struct/Class DataProcessor
    Responsibility: 
        - Parse the weather data provided
        - Calculate the mean temperature for each province.
         
    Input: 
        - legend_path: String path to en_climate_summaries_legend.txt
        - csv_path: String path to en_climate_summaries_All_02-2021.csv
    Output: 
        - output: HashMap<String, f32> for <Province, mean temperature>
*/
// Assuming this is how I declare struct/class with class variable members
struct DataProcessor {
    legend_path: String,
    csv_path: String,
    // legend_parser: LegendParser, // apparently Nested structs not supported in Rust
    // csv_parser: CsvParser, // apparently Nested structs not supported in Rust
    map: HashMap<String, f32>,
}

// I'm not sure if this is the correct implementation to describe method functions for a given
// class/struct in Rust. I assume this acts similar to a header file in C++ 
// Reference: https://www.educative.io/edpresso/what-are-traits-in-rust

trait Methods {
    // Not sure if I need to declare constructor new() here/is valid method...
    fn new(&self)
    // Calculate mean provincial temperatures,
    pub fn calc_mean_prov_temp(&self);
    // Return mean provincial temperatures 
    pub fn get_mean_prov_temp(&self)->HashMap<String,f32>; // May not be optimal, maybe vector?
    // Initialize parsers
    fn generate_parsers(&self) -> (LegendParser, CsvParser);
    // Helper function to calculate mean based off a vector/array of data
    fn calculate_mean(&self, data_slice: Vector) -> f32; 
}

impl Methods for DataProcessor {
    // Constructor Function for DataProcessor
    // Initialize values and helper objects to default/empty values
    // I believe this is not possible in Rust but don't have enough time to figure out whats
    // proper Rust convention
    pub fn new(&self, pathL: String, pathCsv: String) -> DataProcessor {
        legend_path: pathL;
        csv_path: pathCsv;
        //legend_parser: LegendParser::new(),  // apparently Nested structs not supported in Rust
        //csv_parser: CsvParser::new(),  // apparently Nested structs not supported in Rust
        result: HashMap::new();  
        generate_parsers();   
    }

    pub fn calc_mean_prov_temp(&self) {
        // Generate Legend and CSV parsers    
        let (legend_parser, csv_parser) = generate_parsers()
        let legend = legend_parser.get_map();
        let csv_reader = csv_parser.get_data();

        // Get column index for Mean Temperature
        let tm_idx = legend.get("Tm");
        // Get column index for Province
        let prov_idx = legend.get("Prov");

        // Assume that data is sorted and that all data for an individual province/territory is
        // grouped together and not spread randomly throughout the CSV file

        // Iterate through each record and store indices of which range corresponds to a Province
        // or territory

        // Want to determine range of data for each individual Province/Territory
        let mut data_ranges = Vec::<(int,int)>::new();

        // Create a HashMap of a list of Terrorities so we can filter out these results
        let territory_list = HashMap::new();
        territory_list.insert("YK");
        territory_list.insert("NT");
        territory_list.insert("NU");

        let mut curr_prov = ""; // current area we are looking at
        let mut first_row = 0; // index of first row for curr_prov
        let mut counter = 1; // which row we are looking at in the csv_reader.records()

        // Assume record is a structure that follows some variant of vector which can contain 
        // any other primitive (ie String, float, double, int, char, etc)

        // We iterate through the records, I'm not sure how to implement a for loop with a built in
        // counter like in C++ for (int i=0;i<csv_reader.records.size(),i++) { ... }
        for record in csv_reader.records() {
            // Initialize the curr_prov if it is the first one, check if it is also in the 
            // territory_list hashmap and reassign it again if the first province happens to be a 
            // territory
            if (curr_prov == "" OR territory_list.contains(record[curr_prov])) {
                curr_prov = record[prov_idx];
            }
            // If the current row to be checked is a territory skip to the next one and increment 
            // the row
            if (territory_list.contains(record[prov_idx])) {
                counter += 1;
                continue;
            }
            // If it is a new province and is not a territory, update data_ranges vector with new
            // pair <int, int> that contains first_row of the province and the current row
            if (record[prov_idx] != curr_prov && record[prov_idx] != ) {
                let index_range= (first_row, counter - 1);
                data_ranges.push(index_range);
                first_row = counter;
            }
            counter += 1;
        }

        // Process and calculate mean temperature for each range (which corresponds to a province)
        for (first_index, second_index) in data_ranges {
            mean = self.calculate_mean(csv_reader.records[first_index:second_index])
            self.result.insert(csv_reader.records[first_index], mean);
        }
        
        /*
        NOTE: for optimization purposes in cases of large data, it would be beneficial to 
        parallelize the data by executing these in threads.
        An example in very simple pseudocode would be to execute at least one thread for each 
        Province instead of performing this in synchronous order
        Example:
            for each province {
                avg_prov1 = thread(calculate_mean(range1,range2-1))
                avg_prov2 = thread(calculate_mean(range2,range3-1))
                avg_prov3 = thread(calculate_mean(range3,range4-1))
            }
            insert to result when a thread is finished/awaited/joined  
        
        If data is sufficiently large enough, we can further optimize by using MapReduce, splitting 
        up one large slice of data of a province into n threads and calculate them asynchronously.
        Example:
            for one province {
                avgA = thread(calculate_mean(rangeA1,rangeA2-1))
                avgB = thread(calculate_mean(rangeA2,rangeA3-1))
                avgC = thread(calculate_mean(rangeA3,rangeA4-1))
                avg = mean(avgA + avgB + avgC) / (num of threads);
            }
        */

    }
    fn calculate_mean(&self, data_slice: Vector) -> f32 {
        let mut size = data_slice.size();
        let mut sum = 0.0;
        for data in data_slice {
            if (data[idx].is_number()) {
                sum += data[idx];
            }
        }
        let mean = sum / size;
        mean
    }

    pub fn get_mean_prov_temp(&self) -> HashMap<String, f32> {
        self.result
    }

    fn generate_parsers(&self) -> (LegendParser, CsvParser) {
        let legend_parser = LegendParser()::new();
        legend_parser.generate_map(self.legend_path);
        let csv_parser = CsvParser()::new();
        csv_parser.read_csv(self.csv_path);

        (legend_parser, csv_parser)
    }
}