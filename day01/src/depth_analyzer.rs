pub fn scan(depths: Vec<u32>) -> u32 {
    let mut last: u32 = depths[0];
    let mut result = 0;
    for i in 0..depths.len() {
        if i == 0 {
            continue;
        }
        result += if last < depths[i] { 1 } else { 0 };
        last = depths[i];
    }
    return result;
}

pub fn merge(depths: Vec<u32>) -> Vec<u32> {
    let mut result = Vec::new();
    for i in 0..(depths.len()-2) {
        result.push(depths[i] + depths[i+1] + depths[i+2]);
    }
    return result; 
}
