pub fn activity_selection(
    mut activities: Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    activities
    .sort_by_key(|x| x.1); // Activity selection problem 

    let mut selected = vec![];

    let mut last_end_time = 0;

    for &(start, end) in &activities {
        if start >= last_end_time { // 1 > 0
            selected.push((start, end));
            last_end_time = end; // 3
        }
    }
    selected
}

pub fn exec() {
    let activities = vec![(1,3), (2,5), (4,7), (6,9)];
    let selected = activity_selection(activities);
    println!("{:?}", selected);
}
