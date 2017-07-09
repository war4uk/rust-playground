use std::collections::HashMap;

pub fn average(list: &Vec<i32>) -> f32 {
    let mut average = 0.0;
    for value in list.iter() {
        average = average + (*value as f32);
    }
    // lets believe we won't have tooooo large lists
    average / (list.len() as f32)
}

pub fn median(list: &Vec<i32>) -> f32 {
    let mut owned_list = list.to_owned();
    let length = owned_list.len();
    owned_list.sort();

    if length % (2 as usize) == 0 {
        let ind = length / 2;
        average(&vec![owned_list[ind - 1], owned_list[ind]])
    } else {
        owned_list[length / 2] as f32
    } 
}

pub fn mode(list: &Vec<i32>) -> i32 {
    let mut hash: HashMap<i32, i32> = HashMap::new();
    for value in list.iter() {
        let entry = hash.entry(*value).or_insert(0);

        *entry += 1;
    }
    let mut max_pair = (0,0);
    for (key, value) in hash.iter() {
        let (_, max_occurencies) = max_pair;
       if value > &max_occurencies {
            max_pair = (*key, *value);
        }
    }

    max_pair.0
}