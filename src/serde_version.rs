use serde::{Serialize};
use chrono::{NaiveDateTime,FixedOffset};
use std::fs;
use std::error::Error;
use csv::Writer;
use::serde_json::Value;

// We will use this struct with serde to create a row of the output CSV from row of timewarrior
// json
#[derive(Serialize, Debug)]
struct CalendarEntry {
    #[serde(rename = "Subject")]
    subject: String,
    #[serde(rename = "Start Date")]
    start_date: String,
    #[serde(rename = "Start Time")]
    start_time: String,
    #[serde(rename = "End Date")]
    end_date: String,
    #[serde(rename = "End Time")]
    end_time: String,
    #[serde(rename = "Description")]
    description: String,
    #[serde(rename = "Location")]
    location: String,
}

fn convert_timewarrior_to_calendar(input_file: &str, output_file: &str) -> Result<(), Box<dyn Error>> {
    // Read and parse JSON file
    let json_content = fs::read_to_string(input_file)?;
    let entries: Vec<Value> = serde_json::from_str(&json_content)?;

    let mut wtr = Writer::from_path(output_file)?;
    // -4 hour offset from UTC: to be added to parsed times to get EST
    let datetime = FixedOffset::west_opt(4 * 3600).unwrap();

    for entry in entries {
        // let id = entry["id"].as_u64().unwrap() as u32;
        let start = entry["start"].as_str().unwrap();
        let end = entry["end"].as_str().unwrap();
        let tags: Vec<String> = entry["tags"].as_array().unwrap()
            .iter()
            .map(|v| v.as_str().unwrap().to_string())
            .collect();
        // Create subject from tags (join with spaces)
        let subject = if tags.is_empty() {
            "Timewarrior Task".to_string()
        } else {
            tags.join(" ")
        };
        // Extract times that are in UTC and add -4 hrs to get to ET
        let dt = NaiveDateTime::parse_from_str(&start,"%Y%m%dT%H%M%SZ")?;
        let dt = dt+datetime;
        let dte = NaiveDateTime::parse_from_str(&end,"%Y%m%dT%H%M%SZ")?;
        let dte = dte+datetime;

        // Create the struct that will be serialized into the file
        let calendar_entry = CalendarEntry {
                subject,
                start_date: dt.format("%m/%d/%Y").to_string(),
                start_time: dt.format("%I:%M %p").to_string(),
                end_date: dte.format("%m/%d/%Y").to_string(),
                end_time: dte.format("%I:%M %p").to_string(),
                // description: format!("Timewarrior entry ID: {},", id), 
                description: String::new(),
                location: String::new(), // Empty location
        };
        wtr.serialize(&calendar_entry)?;
    }

    wtr.flush()?;
    println!("Successfully converted {} to {}", input_file, output_file);
    Ok(())
}

pub fn main() -> Result<(), Box<dyn Error>> {
    // This part is untouched from Claude. 
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() != 3 {
        eprintln!("Usage: {} <input.json> <output.csv>", args[0]);
        std::process::exit(1);
    }
    
    let input_file = &args[1];
    let output_file = &args[2];

    convert_timewarrior_to_calendar(input_file, output_file)?;
    
    Ok(())
}
