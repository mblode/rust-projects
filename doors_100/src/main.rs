fn main() {
    let mut doors: [bool; 100] = [false; 100];
    let mut counter: i32 = 0;

    for i in 1..=100 {
        counter = 0;

        for j in doors.iter_mut() {
            counter += 1;

            if i == counter {
                *j = !*j;
                counter = 0;
            }
        }
    }

    for x in doors.iter() {
        print!("{} ", x);
    }

    println!("\n");
}
