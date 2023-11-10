//sextern crate anyhow;
use anyhow::*;

pub fn hash(value: &str, len: u32) -> u32 {
    if !value.is_empty() {
        let mut key: u32 = 2166136261;
        let mut i: usize = 0;
        while i < len.try_into().unwrap() {
            key ^= (value.chars().collect::<Vec<char>>()[i] as u8) as u32;
            key = key.overflowing_mul(16777619).0;
            i += 1;
        }
        key
    } else {
        panic!(
        "Was empty!"
        )
    }
}

pub fn hash_string(key: String, size: usize) -> u32 {
    let mut hash: u32 = 2166136261;
    let keychunks = key.as_bytes().chunks(size).map(std::str::from_utf8)
    .collect::<Result<Vec<&str>, _>>()
    .unwrap();
    let mut res: u32;
    for item in keychunks {
        let x = item.chars().collect::<Vec<char>>();
        let mut sum: u32 = 0;
        for j in &x {
            sum = sum + (format!("{:b}", *j as u32)).trim().parse::<u32>().unwrap();
        }
        let fsum = format!("{sum:b}");
        let slen = fsum.len();
        let (top, bottom) = fsum.split_at(slen / 2);
        let topu = top.to_string().trim().parse::<u32>().unwrap();
        let bottomu = bottom.to_string().trim().parse::<u32>().unwrap();
        res = topu ^ bottomu;
        for i in x.iter().take(key.len()) {
            let y = i.to_owned();
            res ^= 16777619;
            res = res.reverse_bits();
            hash = hash.reverse_bits() % (res + 1);
            hash ^= (y as u8) as u32;
            hash = hash.overflowing_mul(16777619).0;
            for chr in hash.to_string().into_bytes() {
                hash ^= format!("{:b}", chr).trim().parse::<u32>().unwrap();
            }
            let fhash = format!("{hash:0<8}");
            hash = fhash.trim().parse::<u32>().unwrap();

            
        }
    }
    //println!("{res:x}");
    hash
}

pub fn hash_string_old(key: String) -> u32{
    let mut hash: u32 = 2166136261;
    let x = key.chars().collect::<Vec<char>>();
    for i in x.iter().take(key.len()) {
        let y = i.to_owned();
        hash ^= (y as u8) as u32;
        hash = hash.overflowing_mul(16777619).0;
    }
    hash
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
/*
let handle1 = std::thread::spawn(move || {
            let mut pre_vals: Vec<u32> = vec![];
            let last_val: u32 = 0;
    
            for i in -10000..0 {
                let z = hash_string(i.to_string(), 16);
                assert_ne!(z, last_val);
                println!("{z:x}");
                for x in &pre_vals {
                    assert_ne!(&z, x, "{z} was equal to {x}!");
                }
                pre_vals.push(z);
            }
        });
*/
        

        /*
        let handle2 = std::thread::spawn(move || {
            let mut pre_vals: Vec<u32> = vec![];
            let last_val: u32 = 0;
    
            for i in 0..10000 {
                let z = hash_string(i.to_string(), 16);
                assert_ne!(z, last_val);
                //println!("{i} - {:x}", z);
                println!("{z:x}");
                let mut count = 0;
                for x in &pre_vals {
                    assert_ne!(&z, x, "{count} was equal to {x}!");
                    count += 1;
                }
                pre_vals.push(z);
            }
        });
         */
        

        let handle3 = std::thread::spawn(move || {
            let mut pre_vals: Vec<String> = vec![];
            let last_val: u32 = 0;
    
            for i in 10000..100000 {
                let z = hash_string(i.to_string(), 16);
                assert_ne!(z, last_val);
                println!("{z:x}");
                for x in &pre_vals {
                    anyhow::ensure!(&z.to_string(), x, "{} was equal to {x}!", z.to_string());
                }
                pre_vals.push(z.to_string());
            }
        });

        
        //handle1.join().unwrap();
        //handle2.join().unwrap();
        handle3.join().unwrap();
    }

    #[test]
    fn other() {
        let handle1 = std::thread::spawn(move || {
            let mut pre_vals: Vec<u32> = vec![];
            let last_val: u32 = 0;
    
            for i in -10000..0 {
                let z = hash_string_old(i.to_string());
                assert_ne!(z, last_val);
                println!("{i} - {:x}", z);
                for x in &pre_vals {
                    assert_ne!(&z, x, "{z} was equal to {x}!");
                }
                pre_vals.push(z);
            }
        });

        let handle2 = std::thread::spawn(move || {
            let mut pre_vals: Vec<u32> = vec![];
            let last_val: u32 = 0;
    
            for i in 0..10000 {
                let z = hash_string_old(i.to_string());
                assert_ne!(z, last_val);
                println!("{i} - {:x}", z);
                for x in &pre_vals {
                    assert_ne!(&z, x, "{z} was equal to {x}!");
                }
                pre_vals.push(z);
            }
        });

        let handle3 = std::thread::spawn(move || {
            let mut pre_vals: Vec<u32> = vec![];
            let last_val: u32 = 0;
    
            for i in 10000..100000 {
                let z = hash_string_old(i.to_string());
                assert_ne!(z, last_val);
                println!("{i} - {:x}", z);
                for x in &pre_vals {
                    assert_ne!(&z, x, "{z} was equal to {x}!");
                }
                pre_vals.push(z);
            }
        });
        handle1.join().unwrap();
        handle2.join().unwrap();
        handle3.join().unwrap();
    }

}
