use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::path::{Path, PathBuf};
use super::{HeightMap, latlong_to_world};
use crate::config::{MIN_HEIGHT, MAX_HEIGHT};

const HGT_NAME_LEN: usize = "abbcddd.hgt".len();
const DEGREE_PER_SAMPLE: f32 = 1.0 / 1200.0;

#[derive(Debug)]
pub struct HGTFile {
    pub file: PathBuf,
    pub long: f32,
    pub lat: f32,
}

impl HGTFile {
    
    fn is_hgt(file: &PathBuf) -> bool {
        if !file.is_file() {
            return false;
        }
        if file.file_name().unwrap().len() != HGT_NAME_LEN {
            return false;
        } 
        match file.extension().and_then(std::ffi::OsStr::to_str) {
            Some("hgt") => true,
            _ => false,
        }
    }

    pub fn new(file: PathBuf) -> Option<Self> {
        if !Self::is_hgt(&file) { return None; }
        if let Some(s) = file.file_name().and_then(std::ffi::OsStr::to_str) {
            let name = s.as_bytes();
            // calculate lattitude value
            let mut lat = match name[0] {
                b'n' | b'N' => 1.0,
                b's' | b'S' => -1.0,
                _ => return None,
            };
            let n = s[1..3].parse::<f32>();
            if n.is_err() {
                return None;
            }
            lat *= n.unwrap();
            // calculate longitude value
            let mut long = match name[3] {
                b'e' | b'E' => 1.0,
                b'w' | b'W' => -1.0,
                _ => return None,
            };
            let n = s[4..7].parse::<f32>();
            if n.is_err() {
                return None;
            }
            long *= n.unwrap();
            Some(Self {
                file,
                long,
                lat,
            })
        } else {
            None
        }
    }
}


pub struct SRTM3 {
    pub hgt_files: Vec<HGTFile>,
    loaded: usize,
}

impl SRTM3 {
    pub fn load<P: AsRef<Path>>(from_dir: P) -> io::Result<Self> {
        let entries = from_dir.as_ref().read_dir()?;
        let mut files: Vec<HGTFile> = Vec::with_capacity(entries.count());
        let entries = from_dir.as_ref().read_dir()?;
        for entry in entries {
            if let Ok(entry) = entry {
                HGTFile::new(entry.path()).and_then(|hgt| { files.push(hgt); None::<()> });
            }
        }
        let srtm3 = Self {
            hgt_files: files,
            loaded: 0,
        };
        Ok(srtm3)
    }
}

impl Iterator for SRTM3 {
    type Item = HeightMap;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(hgt) = self.hgt_files.get(self.loaded) {
            let f = File::open(&hgt.file).expect("Error while reading hgt file: {}");
            let mut reader = BufReader::with_capacity(2 * 1201usize.pow(2), f);
            let buff = reader.fill_buf().unwrap();
            let (height_map, world_pos): (Vec<f32>, Vec<glm::Vec3>) = buff
                .chunks(2)
                .map(|b| i16::from_be_bytes([b[0], b[1]]))
                .map(|height| 
                     (((height as i32).clamp(MIN_HEIGHT, MAX_HEIGHT) + MIN_HEIGHT) as f32) / 
                     (MAX_HEIGHT as f32)
                     )
                .enumerate()
                .map(|(idx, height)| {
                    let long = hgt.long + (idx % 1201) as f32 * DEGREE_PER_SAMPLE;
                    let lat = hgt.lat + 1.0 - (idx / 1201) as f32 * DEGREE_PER_SAMPLE;
                    let world_pos = latlong_to_world(lat, long);
                    (height, world_pos)
                })
                .unzip();
            self.loaded += 1;
            Some( HeightMap {
                height: height_map,
                world_position: world_pos
            })
        } else {
            None 
        }
    }
}
