use anyhow::Result;

pub fn plan_command(goal: Vec<String>) -> Result<()> {
    println!("Creating plan: {}", goal.join(" "));
    Ok(())
}
