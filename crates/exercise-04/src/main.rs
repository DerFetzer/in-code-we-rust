use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::Path;
use std::sync::{Arc, Mutex};

const BIBLE_URL: &str = "https://www.sermon-online.com/download/german/MartinLuther-1912/Martin_Luther_Uebersetzung_1912.txt";
const CHUNK_SIZE: usize = 1000;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    todo!()
}
