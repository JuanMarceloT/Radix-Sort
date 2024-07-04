use std::env;
use std::path::PathBuf;
use std::fs;
use std::fs::File;
use std::io::{Write};
use std::collections::HashMap;

fn get_words_from_file(filename : &str) -> Vec<String> {
    let full_path = get_input_path(filename);
    let contents = fs::read_to_string(full_path)
        .expect("Should have been able to read the file");

    let words: Vec<String> = contents.split_whitespace()
    .map(|s| s.to_owned())
    .collect();

    words
}

fn sort_to_file(words: Vec<String>, filename : &str) -> std::io::Result<()> {
    let path =  get_input_path(&format!("{}_sorted.txt", filename));

    let mut file = File::create(path)?;

    for word in radix_sort(words) {
        file.write_all(word.as_bytes())?;
        file.write_all(b"\n")?;
    }

    Ok(())
}

fn write_stats_to_file(words: Vec<String>, filename : &str) -> std::io::Result<()> {
    let path =  get_input_path(&format!("{}_counted.txt", filename));

    let mut file = File::create(path)?;

    let mut sum = 1;

    for i in 1..=words.len() {
        if i == words.len() || words[i] != words[i - 1] {
            let output = format!("{} {:?}\n", words[i - 1], sum);
            file.write_all(output.as_bytes())?;
            sum = 1;
        } else {
            sum += 1;
        }
    }

    Ok(())
}

fn most_frequent(words : Vec<String>, filename : &str) -> std::io::Result<()> {
    let path =  get_input_path(&format!("{}_ranked.txt", filename));

    let mut file = File::create(path)?;

    let mut word_count = HashMap::new();
    for word in words {
        *word_count.entry(word).or_insert(0) += 1;
    }
    let mut word_count_vec: Vec<(&String, &i32)> = word_count.iter().collect();

    word_count_vec.sort_by(|a, b| b.1.cmp(a.1));

    let max_sort = 2000;
    let mut max_sort_index = 0;

    for (word, count) in word_count_vec {
        if max_sort_index < max_sort {
            let output = format!("{} {:?}\n", word, count);
            file.write_all(output.as_bytes())?;
            max_sort_index += 1;
        }
    }


    Ok(())
}


fn radix_sort(words: Vec<String>) -> Vec<String> {
    let result = radix(words, 0);
    result
}

fn max_array_size(strings: &Vec<String>) -> usize {
    strings.iter().map(|s| s.len()).max().unwrap_or(0)
}

fn radix(words: Vec<String>, index: usize) -> Vec<String> {
    if words.len() <= 1 || index >= max_array_size(&words) {
        return words;
    }
    let mut final_response: Vec<String> = vec![];
    let temp = counting_sort(words.clone(), index);
    let mut start = 0;
    for end in 1..=temp.len() {
        if end == temp.len() || char_at(&temp[start], index) != char_at(&temp[end], index) {
            let sublist = temp[start..end].to_vec();
            final_response.extend(radix(sublist, index + 1));
            start = end;
        }
    }
    final_response
}


fn char_at(word: &String, index: usize) -> u8{
    if word.len() > index {
        if word.as_bytes()[index] > 90 {  
            word.as_bytes()[index] - 96
        }else {
            word.as_bytes()[index] - 64
        }
    }else{
        0 
    }
}

fn counting_sort(words: Vec<String>, sort_index: usize) -> Vec<String> {
    let greater = 27;

    let char_to_sort: Vec<u8> = words.iter().map(|s| char_at(s, sort_index)).collect();
    // println!("{:?}", char_to_sort);
    let mut vec = vec![0; (greater + 1) as usize];

    for &c in &char_to_sort {
        vec[c as usize] += 1;
    }

    for i in 1..vec.len() {
        vec[i] += vec[i - 1];
    }

    let mut sorted_words : Vec<String> = vec![String::from(" "); words.len()];

    for word in words.iter().rev() {
        let index = vec[char_at(word, sort_index) as usize];
        sorted_words[index - 1] =  word.clone();
        vec[char_at(word, sort_index) as usize] -= 1;
    }

    // println!("sorted lol => {:?}", sorted_words);
    sorted_words
}

fn get_input_path(filename : &str) -> PathBuf{
    let exe_path = env::current_exe().expect("Failed to get executable path");

    let src_path = exe_path
        .parent()
        .expect("Failed to get parent directory")
        .parent()
        .expect("Failed to get parent's parent directory")
        .parent()
        .expect("Failed to get parent's parent's parent directory");

    let full_path = src_path.join(format!("src/input/{filename}"));

    full_path
}



fn main() {
    // let nums: Vec<String> = vec!["ab".to_string(), "zaaaba".to_string(), "acba".to_string(), "abab".to_string()];
    // println!("final => {:?}", radix_sort(nums));

    process_and_write_files("war_and_peace");
    process_and_write_files("frankestein");
}


fn process_and_write_files(filename: &str) {
    let words: Vec<String> = get_words_from_file(&format!("{}.txt", filename));

    let sorted_words: Vec<String> = radix_sort(words.clone());

    match write_stats_to_file(sorted_words.clone(), filename) {
        Ok(()) => println!("Stats file written successfully"),
        Err(e) => eprintln!("Error writing file: {}", e),
    }

    match sort_to_file(sorted_words.clone(), filename) {
        Ok(()) => println!("Sort file written successfully"),
        Err(e) => eprintln!("Error writing file: {}", e),
    }

    match most_frequent(sorted_words.clone(), filename) {
        Ok(()) => println!("Most frequent file written successfully"),
        Err(e) => eprintln!("Error writing file: {}", e),
    }
}

