use std::env;

fn main() {
    let mut args = env::args()
        .skip(1)
        .collect::<Vec<String>>();

    //args.sort();
    selection_sort(&mut args);

    println!("{:?}", args);
}

fn selection_sort(items: &mut Vec<String>) {
    let length = items.len();
    for i in 0..length {
        let mut smallest_index = i;
        for j in i+1..length {
            if items[j] < items[smallest_index] {
                smallest_index = j;
            }
        }
        let temp = items[smallest_index].clone();
        items[smallest_index] = items[i].clone();
        items[i] = temp;
    }
}