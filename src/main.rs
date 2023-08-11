use csv::Reader;
use image::{ImageBuffer, Rgb};
use rand::random;
use std::collections::HashMap;
use std::env;
use std::path::Path;

// displayarray, a program to display integer data from a csv file as a png file.
// v0.1 command line (non-gui) version: displayarray <CSV filename> [image width]
//      automatically saves .png file with the first part of the .csv filename.

fn main() {
// get command line arguments
    let args: Vec<String> = env::args().collect();
 // help info 
    if args.len() < 2 || args[1] == "help" {
        println!("This program reads a CSV file and generates an image with a unique color for each unique value in the CSV data. Zero values are displayed as black.");
        println!("Usage: {} <CSV file> [image width]", args[0]);
        println!("\t<CSV file>: The path to the input CSV file. The file should have a .csv extension.");
        println!("\t[image width]: The desired width of the output image in pixels. If not provided, it defaults to the number of columns in the CSV file.");
        return;
    }
// set file name to read from the command line
    let filename = &args[1];
    if !filename.ends_with(".csv") {
        panic!("The file should be a CSV file (extension .csv).");
    }
//set width scale if provided
    let image_width: usize = if args.len() >= 3 {
        args[2].parse().unwrap_or_else(|_| {
            panic!("Failed to parse image width from command-line arguments.")
        })
    } else {
        0 // temporary value, to be replaced with the number of columns in the CSV file
    };

    let save_as_png = true;

    let mut data = vec![];
    let mut rdr = csv::ReaderBuilder::new().has_headers(false).from_path(filename).unwrap();

	for result in rdr.records() {
		let record = result.unwrap();
		let row: Vec<u8> = record.iter().map(|s| s.parse::<u8>().unwrap()).collect();
		data.push(row);
	}

    let (rows, cols) = (data.len(), data[0].len());

// If the image width was not provided in the command-line arguments,
// set it to the number of columns in the CSV file.
    let image_width = if image_width == 0 { cols } else { image_width };

    let image_height = image_width * rows / cols;

    let mut colors: HashMap<u8, Rgb<u8>> = HashMap::new();

    let img = ImageBuffer::from_fn(image_width as u32, image_height as u32, |x, y| {
        let data_x = (x as usize * cols) / image_width;
        let data_y = (y as usize * rows) / image_height;
        let val = data[data_y][data_x];
// Display 0 values as black
        if val == 0 {
            Rgb([0, 0, 0])
        } else {
            *colors.entry(val).or_insert_with(|| Rgb([random(), random(), random()]))
        }
    });
// Save the image as a png file.  use the imagemagick command 'display' to view it.
    if save_as_png {
        let output_filename = filename.replace(".csv", ".png");
        img.save(Path::new(&output_filename)).unwrap();
    }
    

//testing - uncomment to print the array
//	for (i, row) in data.iter().enumerate() {
//  	println!("Row {}: {:?}", i, row);
//	}
// End of main
}
