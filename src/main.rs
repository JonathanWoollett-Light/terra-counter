use indicatif::{ProgressBar, ProgressStyle};
use rand::{thread_rng, Rng};

fn main() {
    println!("Hello, world!");

    // As determined by initial conditions.
    let initial_task_difficulty = 1000;
    // As determined by the agents stats
    let agent_skill = 200;

    println!("difficulty: {initial_task_difficulty}");
    println!("skill: {agent_skill}");
    println!("ticks (weeks): {}", (initial_task_difficulty as f32 / agent_skill as f32).ceil() as u64);


    let pb = ProgressBar::new(initial_task_difficulty);
    pb.set_style(ProgressStyle::with_template("{spinner} {percent}% {human_pos}/{human_len}")
        .unwrap());

    let mut task_difficulty = initial_task_difficulty;
    let mut task_progress = 0;

    while task_progress < task_difficulty {
        pb.set_position(task_progress);
        pb.set_length(task_difficulty);

        std::thread::sleep(std::time::Duration::from_secs(1));

        task_progress += agent_skill;
        // For the sake of this experiment this fluctuates randomly.
        // In practice this would run the difficulty calculation.
        task_difficulty = reevaluate_difficulty();
    }

    pb.finish_with_message("downloaded");
}
fn reevaluate_difficulty() -> u64 {
    let mut rng = thread_rng();
    rng.gen_range(900..1100)
}
