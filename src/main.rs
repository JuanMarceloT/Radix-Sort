fn main() {
    let nums: Vec<String> = vec!["aaab".to_string(), "baba".to_string(), "aaba".to_string(), "abab".to_string()];
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
        if end == temp.len() || temp[start].as_bytes()[index] != temp[end].as_bytes()[index] {
            let sublist = sublist(&temp, start, end).to_vec();
            println!("sublist => {:?}", sublist);
            println!("{:?}", final_response);
            final_response.extend(radix(sublist, index + 1));
            start = end;
        }
    }
    final_response
}

fn counting_sort(words: Vec<String>, sort_index: usize) -> Vec<String> {
    let greater = 26;

    let char_to_sort: Vec<u8> = words.iter().map(|s| s.as_bytes()[sort_index] - b'a').collect();

    let mut vec = vec![0; (greater + 1) as usize];

    for &c in &char_to_sort {
        vec[c as usize] += 1;
    }

    for i in 1..vec.len() {
        vec[i] += vec[i - 1];
    }

    let mut sorted_words : Vec<String> = vec![String::from(" "); words.len()];

    for word in words.iter().rev() {
        let index = vec[word.as_bytes()[sort_index] as usize - b'a' as usize];
        sorted_words[index - 1] =  word.clone();
        vec[word.as_bytes()[sort_index] as usize - b'a' as usize] -= 1;
    }

    //println!("sorted lol => {:?}", sorted_words);
    sorted_words
}
