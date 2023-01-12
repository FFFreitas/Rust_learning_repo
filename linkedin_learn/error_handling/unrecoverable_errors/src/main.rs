fn main() {
    //panic!("Houston we have a problem")
    let countdown = [5, 4, 3, 2, 1];
    for count in countdown.iter() {
        println! {"T-minus {}", count};
        let x = 1 / 1 - count;
    }
}
