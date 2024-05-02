fn main() {
    let nums: &[String] = &["aaab".to_string(), "baba".to_string(), "aaba".to_string(), "abab".to_string()];
    radix(nums);
}

fn radix(nums: &[String]) {
    let max_len = longest_word(nums);

    counting_sort(nums, 0);
    counting_sort(nums, 1);
    counting_sort(nums, 2);
    counting_sort(nums, 3);    

    println!("{:?}", max_len);
}

fn counting_sort(words: &[String], sort_index: usize) {
    let greater = 26;

    let char_to_sort: Vec<char> = words.iter().map(|s| s.chars().nth(sort_index).unwrap()).collect();


    let mut vec = vec![0; (greater + 1) as usize];

    for c in &char_to_sort {
        vec[(*c as u8 - b'a') as usize] += 1;
    }

    for i in 1..vec.len() {
        vec[i] += vec[i - 1];
    }

    let mut sorted_words = vec![String::new(); words.len()];

    for word in words.iter().rev() {
        let index = vec[word.chars().nth(sort_index).unwrap() as usize - 'a' as usize];
        sorted_words[index - 1] = word.clone();
        vec[word.chars().nth(sort_index).unwrap() as usize - 'a' as usize] -= 1;
    }

    println!("{:?}", sorted_words);
}

fn longest_word(words: &[String]) -> usize {
    let mut bigger_len = 0;

    for word in words {
        if word.len() > bigger_len {
            bigger_len = word.len();
        }
    }
    bigger_len
}
