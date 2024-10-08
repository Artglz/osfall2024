fn is_even(n: i32) -> bool {
    n % 2 == 0
}

fn main() {
    let numbers = [12, 15, 4, 17, 21, 48, 2, 45, 9, 12];
    // make a for loop to iterate through the numbers and check if the conditions are met
    for &num in &numbers {
        if is_even(num) {
            print!("{} is Even", num);
        } else {
            print!("{} is Odd", num);
        }

        if num % 3 == 0 && num % 5 == 0 {
            println!(" - FizzBuzz");
        } else if num % 3 == 0 {
            println!(" - Fizz");
        } else if num % 5 == 0 {
            println!(" - Buzz");
        } else {
            println!();
        }
    }

    let mut sum = 0;
    let mut i = 0;
    while i < numbers.len() {
        sum += numbers[i];
        i += 1;
    }
    println!("The sum of all numbers is: {}", sum);

    let mut largest = numbers[0];
    for &num in &numbers {
        if num > largest {
            largest = num;
        }
    }
    println!("The largest number in the array is: {}", largest);
}

