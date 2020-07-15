// C > tools > opencv > build > x64 > vc15 > bin > opencv_world412.dll
// add opencv world dll to same destination as folder as exe
#![windows_subsystem = "windows"]
extern crate chrono;
extern crate csv;

use std::time::SystemTime;
use std::error::Error;
use std::process;
use std::fs::OpenOptions;

use opencv::prelude::*;
use opencv::{highgui, videoio, imgcodecs, types::VectorOfPoint, imgproc::{put_text, LINE_8}};
use opencv::objdetect::QRCodeDetector;
use opencv::core::{self,Point, Scalar, Size};

use chrono::{offset::Local, DateTime};


struct Data{
	store: Vec<(String, String, String)>,
}

impl Data{
	// get data from vec then write into csv
	fn get_data(&mut self) -> Result<(), Box<dyn Error>>{
		println!("getting data");
		let systime= SystemTime::now();
		let file_date: DateTime<Local> = systime.into();
		let file_date = file_date.format("%d-%m-%Y").to_string();
		let file_path = format!("{}.csv", file_date);
		// enable create new csv file if not exist, enable write to file with same name
		let file = OpenOptions::new()
				.write(true)
				.create(true)
				.append(true)
				.open(&file_path)
				.unwrap();
		
		for (name, class, time) in &self.store{
			let mut rdr = csv::ReaderBuilder::new().has_headers(false).from_path(&file_path)?;
			// for field in rdr.records().collect::<Result<Vec<csv::StringRecord>, csv::Error>>()?{
			let all_records = rdr.records().collect::<Result<Vec<csv::StringRecord>, csv::Error>>()?;
			// write data to csv when field is blank
			if all_records.get(0) == None {
				println!("writing data");
				let mut wtr = csv::Writer::from_writer(&file);
				wtr.write_record(&[name, class, time])?;
				wtr.flush()?;
			// do not write data if same name already exists
			}else if all_records.iter().any(|n|n.get(0) == Some(name)) {
				println!("already have name");
			// write data if no same name exists
			}else{
				println!("writing data");
				let mut wtr = csv::Writer::from_writer(&file);
				wtr.write_record(&[name, class, time])?;
				wtr.flush()?;
			}
		}
		Ok(())
		
	}
}

fn main() -> Result<(), Box<dyn Error>>{
	//opening webcam and a window
	let mut data = Data{store: vec![]};
	let window = "qrscanner";
	highgui::named_window(window, 1)?;
	#[cfg(feature = "opencv-4")]
	let mut cam = videoio::VideoCapture::new_default(0)?; //0 is the default camera
	#[cfg(not(feature = "opencv-4"))]
    let mut cam = videoio::VideoCapture::new(0, videoio::CAP_ANY)?;  // 0 is the default camera
    let opened = videoio::VideoCapture::is_opened(&cam)?;
    if !opened {
        panic!("Unable to open default camera!");
    }
    loop {
        let mut frame = core::Mat::default()?;
        cam.read(&mut frame)?;
        if frame.size()?.width > 0 {
			highgui::imshow(window, &mut frame)?;
			let mut qr = QRCodeDetector::default()?;
			let mut pts = VectorOfPoint::new();
			let mut straight = Mat::default()?;
			// detect and decode qrcode
			let r = qr.detect_and_decode(&mut frame, &mut pts, &mut straight)?;
			let list: Vec<String> = r.split(", ").map(|s| s.to_string()).collect();

			if list[0] == ""{
				println!("no qrcode");
			}else{
				let name = list[0].to_owned();
				println!("name:{}", name);
				let class = list[1].to_owned();
				println!("class:{}", class);
				let systime = SystemTime::now();
				let date: DateTime<Local> = systime.into();
				let date = date.format("%d/%m/%Y %T").to_string();
				// check in vec for duplicated names
				if data.store.iter().any(|n| n.0 == name){
					println!("pass");
					put_text(&mut frame, "-----",Point::new(50,50),highgui::QT_FONT_NORMAL,1.0,Scalar::new(255.,255.,255.,0.),2,LINE_8,false)?;
					highgui::imshow(window, &mut frame)?;
				}else{
					// push data in tuple to vec
					data.store.push((name, class, date));
					put_text(&mut frame, "recorded",Point::new(50,50),highgui::QT_FONT_NORMAL,1.0,Scalar::new(255.,255.,0.,0.),2,LINE_8,false)?;
					highgui::imshow(window, &mut frame)?;
					if let Err(e) = data.get_data(){
						println!("{}", e);
						process::exit(1);
					}
					
				}
			}
			
		}
		 
        let key = highgui::wait_key(10)?;
        if key > 0 && key != 255 {
			break;
        }
	}
    Ok(())

}