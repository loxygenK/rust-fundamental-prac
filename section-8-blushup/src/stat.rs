use std::convert::TryInto;

pub struct Statistics {
    mean: f32,
    median: f32,
    mode: f32,
}

impl Statistics {
    pub fn new(mean: f32, median: f32, mode: f32) -> Self {
        Self { mean, median, mode }
    }
}

fn get_statistics(data: &Vec<f32>) -> Statistics {
    let sorted_data = {
        let mut cloned = data.clone();
        cloned.sort_by(|l, r| l.partial_cmp(r).expect("no invalid value for comparing"));
        cloned
    };

    let mean = data.iter().sum::<f32>() / (data.len() as f32);

    let median = if data.len() % 2 == 0 {
        (sorted_data[data.len() / 2 - 1] + sorted_data[data.len() / 2]) / 2.0
    } else {
        sorted_data[data.len() / 2]
    };

    let mut previous = &sorted_data[0];
    let mut appear_count: Vec<(&f32, i32)> = vec![(&sorted_data[0], 0)];
    for d in sorted_data.iter() {
        if previous != d { appear_count.push((&d, 0)); }

        // TODO: Search - why does this works?
        let index = appear_count.len() - 1;
        appear_count[index].1 += 1;

        previous = d;
    }

    let most_used_index = appear_count
        .iter()
        .enumerate()
        .max_by_key(|i| i.1.1)
        .expect("no invalid value for comparing");

    println!("{:?}", appear_count);

    Statistics::new(mean, median, *most_used_index.1.0)
}

#[test]
fn stat_test() {
    let testcase_1 = get_statistics(&vec![1f32, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0]);
    assert_eq!(testcase_1.mean, 4f32);
    assert_eq!(testcase_1.median, 4f32);
    assert_eq!(testcase_1.mode, 7f32);

    let testcase_2 = get_statistics(&vec![1f32, 2.0, 3.0, 4.0, 5.0, 6.0]);
    assert_eq!(testcase_2.mean, 3.5f32);
    assert_eq!(testcase_2.median, 3.5f32);
    assert_eq!(testcase_2.mode, 6f32);

    let testcase_2 = get_statistics(&vec![50f32, 10.0, 23.0, 10.0, 25.0, 50f32]);
    assert_eq!(testcase_2.mean, 28f32);
    assert_eq!(testcase_2.median, 24f32);
    assert_eq!(testcase_2.mode, 50f32);
}
