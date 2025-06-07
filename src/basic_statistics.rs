use std::collections::HashMap;

fn mean(values_vector: &Vec<i64>) -> Option<i64> {
    // Create a mutable variable called sum to store the sum of the numbers in the vector
    let mut sum: i64 = 0;
    
    // Looping over the values in the vector and adding each to sum
    for value in values_vector {
        sum += value;
    }
    
    // Store the number of values in a variable called number_of_values
    let number_of_values: i64 = values_vector.len() as i64;
    
    // Using the match control flow construct to prevent division by 0 if an empty vector is passed
    match &number_of_values {
        0 => return None,
        _ => {
            let mean: i64 = &sum/&number_of_values;
            return Some(mean);
        },
    };
}

fn median(values_vector: &Vec<i64>) -> Option<i64> {
    // Create a new vector that stores a copy of the values_vector
    let mut values_vector_copy: Vec<i64> = values_vector.to_vec();
    
    // Calculate the length of the vector
    let number_of_values: usize = values_vector_copy.len();
    
    // Sort the copied vector in ascending order
    values_vector_copy.sort();
    
    // Handling the case of an empty vector using match control flow construct
    match &number_of_values {
        0 => return None,
        _ => {
            // If odd number of values
            if &number_of_values % 2 != 0 {
                let median: i64 = values_vector_copy[(number_of_values + 1)/2];
                return Some(median);
            } else {
                let median: i64 = (values_vector_copy[number_of_values/2] + values_vector_copy[&number_of_values/2 + 1]) / 2;
                return Some(median);
            }
        },
    };

}

fn mode(values_vector: &Vec<i64>) -> Option<Vec<i64>> {
    // Handle the case where an empty values_vector is passed by returning None
    if values_vector.len() == 0 {
        return None;
    } else {
        // Create a new HashMap to store the count for each number
        let mut value_frequency = HashMap::new();
        
        // Looping over value in values_vector
        for value in values_vector {
            let value_access = value_frequency.entry(value).or_insert(0);
            *value_access += 1;
        };
        
        // Identfying the frequency of the most frequent value
        // Creating a highest_frequency variable that is mutable and has an initial value of 0
        let mut highest_frequency = 0;
        
        // Looping through the values in the HashMap
        for frequency in value_frequency.values() {
            if frequency > &highest_frequency {
                highest_frequency = *frequency;
            };
        }
        
        // Creating a new Output Vector to store the modes
        let mut modes_vector = Vec::new();
        
        // Looping through key-value pairs in the HashMap and collect modes
        for (key, value) in value_frequency {
            if value == highest_frequency {
                modes_vector.push(*key);
            };
        }
        
        // Return Option containing modes
        return Some(modes_vector);
        };
}

fn main() {
    let mut test_vector = vec![0,-100,30,675,1000,-2,34,100,34,-2];
    
    let mean = match mean(&test_vector) {
        Some(mean) => mean,
        None => {
            println!("The input vector is empty");
            0
        },
    };
    
    let median = match median(&test_vector) {
        Some(median) => median,
        None => {
            println!("The input vector is empty");
            0
        },
    };
    
    let mode = match mode(&test_vector) {
        Some(mode) => mode,
        None => {
            println!("The input vector is empty");
            vec![]
        },
    };
    
    test_vector.sort();
    
    println!("The sorted vector is {:?}", test_vector);
    println!("");
    println!("Mean = {mean}");
    println!("Median = {median}");
    println!("Mode = {:?}", mode);

} 
    
