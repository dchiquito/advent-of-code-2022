use crate::advent;


pub(crate) fn solve() {
    let reader = advent::read_input(1);
    // reader.fold(Collectifier::new(), |a, b| a.fold(b.unwrap()));
    let elves = reader.fold(vec![0], |mut elves, calorie| {
        let calorie = calorie.unwrap();
        if calorie == "" {
            elves.push(0);
        } else {
            let new_calories = elves.pop().unwrap() + str::parse::<i32>(&calorie).unwrap();
            elves.push(new_calories);
        }
        elves
    });
    
    let mut max_elf = 0;
    for elf in elves.iter() {
        if *elf > max_elf {
            max_elf = *elf;
        }
    }
    println!("{}", max_elf);

    let mut second_max_elf = 0;
    for elf in elves.iter() {
        if *elf > second_max_elf && *elf < max_elf {
            second_max_elf = *elf;
        }
    }

    let mut third_max_elf = 0;
    for elf in elves.iter() {
        if *elf > third_max_elf && *elf < second_max_elf {
            third_max_elf = *elf;
        }
    }
    println!("{}", max_elf + second_max_elf + third_max_elf)
}
