use my_module::{FilterCondition, custom_filter};

fn main() {
    let elements = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // Initialize a FilterCondition object with the desired value
    let filter_condition = FilterCondition { value: 5 };

    // Call the custom_filter function
    let filtered_result = custom_filter(&elements, &filter_condition);

    // Print the filtered result to the console
    println!("Original Collection: {:?}", elements);
    println!("Filtered Result: {:?}", filtered_result);
}

mod my_module{
  // Define a struct called FilterCondition
  pub struct FilterCondition<i32> {
    pub value: i32,
  }

  // Implement a method called is_match on the FilterCondition struct
  impl FilterCondition<i32> {
    pub fn is_match(&self, item: &i32) -> bool {
        item != &self.value
    }
  }

  // Define a function called custom_filter
  pub fn custom_filter(collection: &[i32], filter_condition: &FilterCondition<i32>) -> Vec<i32> {
    collection
        .iter()
        .filter(|&item| filter_condition.is_match(item))
        .cloned()
        .collect()
  }
}