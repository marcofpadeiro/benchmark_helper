use csv::{Writer, WriterBuilder};
use serde_derive::{Deserialize, Serialize};
use std::{
    env::{self},
    error::Error,
    fs::{File, OpenOptions},
    io::{self, BufRead, BufReader, Cursor, Seek, SeekFrom},
    ops::Div,
};

const RESULTS_PATH: &str = "/home/marco/benchmark_results.csv";

#[derive(Debug, Serialize, Deserialize)]
struct Output {
    #[serde(skip_deserializing)]
    description: String,
    #[serde(skip_deserializing)]
    score: String,
    gpu_temp: String,
    gpu_core_clock: String,
    gpu_mem_clock: String,
    gpu_vram_used: String,
    gpu_power: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    let description = args.get(1).unwrap();
    let score = args.get(2).unwrap();
    let path = args.get(3).unwrap();

    let temp: Vec<Output> = read_csv(path)?;
    let output: Output = average(description, score, &temp)?;
    write_csv(&output, RESULTS_PATH)?;

    Ok(())
}

fn read_csv(path: &String) -> Result<Vec<Output>, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);
    let lines: Vec<String> = reader.lines().skip(2).filter_map(Result::ok).collect();
    let csv_data = lines.join("\n");
    let csv_reader = Cursor::new(csv_data);

    let mut reader = csv::Reader::from_reader(csv_reader);

    let mut output: Vec<Output> = vec![];
    for result in reader.deserialize() {
        output.push(result?);
    }
    Ok(output)
}

fn average(
    description: &String,
    score: &String,
    output: &Vec<Output>,
) -> Result<Output, Box<dyn Error>> {
    let mut gpu_temp: f32 = 0 as f32;
    let mut gpu_core_clock: f32 = 0 as f32;
    let mut gpu_mem_clock: f32 = 0 as f32;
    let mut gpu_vram_used: f32 = 0 as f32;
    let mut gpu_power: f32 = 0 as f32;

    output.iter().for_each(|x| {
        gpu_temp += x.gpu_temp.parse::<f32>().unwrap();
        gpu_core_clock += x.gpu_core_clock.parse::<f32>().unwrap();
        gpu_mem_clock += x.gpu_mem_clock.parse::<f32>().unwrap();
        gpu_power += x.gpu_power.parse::<f32>().unwrap();
        gpu_vram_used += x.gpu_vram_used.parse::<f32>().unwrap();
    });

    let len = output.len() as f32;
    let temp: Output = Output {
        description: description.clone(),
        score: score.clone(),
        gpu_temp: gpu_temp.div(len).to_string(),
        gpu_core_clock: gpu_core_clock.div(len).to_string(),
        gpu_mem_clock: gpu_mem_clock.div(len).to_string(),
        gpu_vram_used: gpu_vram_used.div(len).to_string(),
        gpu_power: gpu_power.div(len).to_string(),
    };

    Ok(temp)
}

fn write_csv(output: &Output, path: &str) -> Result<(), Box<dyn Error>> {
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .read(true)
        .open(path)?;

    let is_empty = {
        let mut reader = BufReader::new(&file);
        reader.seek(SeekFrom::End(0))? == 0
    };

    let mut writer = WriterBuilder::new().has_headers(is_empty).from_writer(file);

    writer.serialize(output)?;
    writer.flush()?;
    Ok(())
}
