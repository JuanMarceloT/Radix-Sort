fn main() {
    let nums: Vec<String> = vec!["ab".to_string(), "aaaaba".to_string(), "acba".to_string(), "abab".to_string()];
    println!("final => {:?}", radix_sort(nums));
}

fn radix_sort(nums: Vec<String>) -> Vec<String> {
    let finala = radix(nums, 0);
    finala
}

fn sublist<T>(list: &[T], start: usize, end: usize) -> &[T] {
    &list[start..end]
}

fn radix(nums: Vec<String>, index: usize) -> Vec<String> {
    if nums.len() <= 1 || index >= 5 {
        return nums.to_vec();
    }
    let mut final_response: Vec<String> = vec![];
    let mut start = 0;
    let mut temp = counting_sort(nums.clone(), index);
    for end in 1..=nums.len() {
        if end == temp.len() || char_at(&temp[start], index) != char_at(&temp[end], index) {
            let sublist = sublist(&temp, start, end).to_vec();
            println!("sublist => {:?}", sublist);
            println!("{:?}", final_response);
            final_response.extend(radix(sublist, index + 1));
            start = end;
        }
    }
    final_response
}

fn char_at(word: &String, index: usize) -> u8{
    if word.len() > index {
        word.as_bytes()[index] - 96
    }else{
        0 
    }
}

fn counting_sort(words: Vec<String>, sort_index: usize) -> Vec<String> {
    let greater = 27;

    let char_to_sort: Vec<u8> = words.iter().map(|s| char_at(s, sort_index)).collect();
    println!("{:?}", char_to_sort);
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

    println!("sorted lol => {:?}", sorted_words);
    sorted_words
}

