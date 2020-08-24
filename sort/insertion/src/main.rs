// After writing a couple of implementations, I realize that using

fn impl1(v: &Vec<i32>) -> Vec<i32> {
    let mut i = 1;
    let mut mutv = v.to_vec();
    while i < mutv.len() {
        let mut j = i;
        while j > 0 && mutv[j-1] > mutv[j] {
            let item = mutv.remove(j);
            mutv.insert(j - 1, item);
            j = j - 1;
        }
        i = i + 1;
    }
    mutv
}

fn impl2(v: &Vec<i32>) -> Vec<i32> {
    let mut i = 1;
    let mut mutv = v.to_vec();
    while i < mutv.len() {
        let x = mutv[i];
        let mut j = i - 1;
        while j as i32 >= 0 && mutv[j] > x {
            let item = mutv.remove(j);
            mutv.insert(j + 1, item);
            if j > 0 {
                j = j - 1;
            }
        }
        i = i + 1;
    }
    mutv
}

//TODO: FAILED
fn impl3(v: &mut Vec<i32>, n: usize) {
    if n > 0 {
        impl3(v, n - 1);
        let x = v[n];
        let mut j = n - 1;
        while j as i32 >= 0 && v[j] > x {
            let item = v.remove(j);
            v.insert(j + 1, item);
            if j > 0 {
                j = j - 1;
            }
        }
        v[j + 1] = x;
    }
}

fn main() {
    let v: Vec<i32> = vec![9,8,7,6,5,4,3,2,1,0];
    
    println!("Implementation 1: {:?}", impl1(&v));
    println!("Implementation 2: {:?}", impl2(&v));
    let mut mutv = v.to_vec();
    impl3(&mut mutv, v.len() - 1);
    println!("Implementation 3: {:?}", mutv);
}
