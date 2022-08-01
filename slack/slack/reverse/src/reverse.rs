
#[allow(unused_imports)]
use wasmedge_bindgen::*;
use wasmedge_bindgen_macro::*;



#[wasmedge_bindgen]
    pub fn run(s: String) -> String {
        let mut v = vec!['$', '#'];
        s.chars().for_each(|c| {
            v.push(c);
            v.push('#');
        });
        v.push('!');
        let (mut start, mut end) = (1, 1);
        let mut arm = vec![0usize; v.len()];
        let (mut center, mut right) = (1, 1);
        for i in 1..v.len() - 1 {
            if i < right {
                arm[i] = arm[2 * center - i].min(right - i);
            }
            while v[i - arm[i] - 1] == v[i + arm[i] + 1] {
                arm[i] += 1;
            }
            if i + arm[i] > right {
                right = i + arm[i];
                center = i;
            }
            if end - start < arm[i] * 2 {
                start = i - arm[i];
                end = i + arm[i];
            }
        }
        v[start..=end].into_iter().filter(|&&c| c != '#').collect()
    }



