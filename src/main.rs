use std::io;

fn main() {
    let me_v: f32 = get_me_v();
    let student_v: f32 = get_student_v();
    let door_dist: f32 = get_door_dist();

    println!("me_v {}, student_v {}", me_v, student_v);

    // next feature to add
    // get users input, make mutable
    let me_pt: f32 = 0.0;
    let student_pt: f32 = 100.0;

    // logical equation:
    // me_pt + (vel * num_of_steps_taken) = door_dist;

    let num_of_steps_taken_to_door: f32 = (door_dist - me_pt) / me_v;

    // logical equation (slightly modified for this situation):
    // ending_up_pt = student_pt - (vel * steps)
    let ending_up_pt: f32 = student_pt - (student_v * num_of_steps_taken_to_door);

    if ending_up_pt == door_dist {
        println!("Sorry, awkward pause, wait");
    } else {
        println!("Welcome to English Class!");
    }
}

fn get_me_v() -> f32 {
    let mut raw_me_v = String::new();

    loop {

        println!("Please type in your velocity (fph)");
        io::stdin().read_line(&mut raw_me_v).expect("Error getting me vel");

        let num_me_v: f32 = match raw_me_v.trim().parse() {
            Ok(num) => num,
            Err(_) => {raw_me_v.clear();
                continue;},
        };
        return num_me_v;
    }
}

fn get_student_v() -> f32 {
    let mut raw_student_v = String::new();

    loop {
        println!("Please type in the other student's velocity (fph)");
        io::stdin().read_line(&mut raw_student_v).expect("Error getting student vel");

        let num_student_v: f32 = match raw_student_v.trim().parse() {
            Ok(num) => num,
            Err(_) => {raw_student_v.clear();
                        continue;},

            // raw_student_v.clear() because the string buffer is not cleared
            // so on the next loop iteration, it just appends contents
            // I asked the question here -> https://www.reddit.com/r/rust/comments/11q336b/hey_rustaceans_got_a_question_ask_here_112023/

        };
        return num_student_v;
    }
}

fn get_door_dist() -> f32 {
    let mut raw_door_dist = String::new();

    loop {
        println!("Please enter how many more feet you have to travel to reach the door");
        io::stdin().read_line(&mut raw_door_dist).expect("Error reading door distance");
        
        let door_dist: f32 = match raw_door_dist.trim().parse() {
        Ok(num) => num,
        Err(_) => {raw_door_dist.clear();
            continue;},
        };
        return door_dist;
    }
}