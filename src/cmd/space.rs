use anyhow::Result;

use crate::api::BacklogClient;

pub fn show() -> Result<()> {
    let client = BacklogClient::from_config()?;
    let space = client.get_space()?;

    println!("Space key:  {}", space.space_key);
    println!("Name:       {}", space.name);
    println!("Language:   {}", space.lang);
    println!("Timezone:   {}", space.timezone);
    println!("Formatting: {}", space.text_formatting_rule);
    println!("Created:    {}", space.created);
    println!("Updated:    {}", space.updated);

    Ok(())
}
