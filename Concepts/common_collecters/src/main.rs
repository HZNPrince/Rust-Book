use std::collections::HashMap;

fn main(){
    let mut hashy = HashMap::new();
    hashy.insert("Prince".to_string(), 19);
    hashy.insert(String::from("Dhrumil"), 20);

    let name = String::from("Prince");
    println!("{}",hashy.get(&name).copied().unwrap_or(0));

    for (key,value) in hashy.iter(){
        println!("{key}: {value}");
    }

    hashy.entry(String::from("Garvit")).or_insert(50);
    hashy.entry("Prince".to_string()).or_insert(20);

    for (key,value) in hashy.iter(){
        println!("{key}: {value}");
    }


    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{map:?}");


    let mut h: HashMap<char, Vec<usize>> = HashMap::new();
  for (index, letter) in "hello!".chars().enumerate() {
    h.entry(letter).or_insert(Vec::new()).push(index);
  }
  let mut sum = 0;
  for index in h.get(&'l').unwrap() {
    sum += *index;
  }
  println!("{}", sum);

}

/// Removes all the zeros in-place from a vector of integers.
fn remove_zeros(v: &mut Vec<i32>) {
    for (i, t) in v.iter().enumerate().rev() {
        if *t == 0 {
            v.remove(i);
            v.shrink_to_fit();
        
        }
    }

    for i in 0..5/2{}

    fn reverse(v: &mut Vec<String>) {
        let n = v.len();
        let mut v2 = v.clone();
        for i in 0 .. n / 2 {
            std::mem::swap(&mut v[i], &mut v2[n - i - 1]);
        }
    }
}