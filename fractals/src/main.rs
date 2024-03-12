// Define Mandelbrot functions first

use clap::{Arg, Command};
use image::{ImageBuffer, Luma};
use std::time::Instant;
use num::Complex;
use crossbeam::thread;

// use zzz::ProgressBar;
//
//

pub fn compute_bands_corners(
        n_threads: u32,
        upper_left_init: Complex<f64>,
        lower_right_init: Complex<f64>
    ) -> (Vec<Complex<f64>>, Vec<Complex<f64>>) {
    let upper_left: Vec<Complex<f64>> = (0..n_threads)
        .map(|i| Complex {
            re: upper_left_init.re,
            im: upper_left_init.im - (i as f64) * 2.0 / (n_threads as f64),
        })
        .collect();
    let lower_right: Vec<Complex<f64>> = (0..n_threads)
        .map(|i| Complex {
            re: lower_right_init.re,
            im: upper_left_init.im - (i as f64 + 1.0) * 2.0 / (n_threads as f64),
        })
        .collect();
    (upper_left, lower_right)
}


pub fn render_on_grid(
    corner_upper_left: Complex<f64>,
    corner_lower_right: Complex<f64>,
    n_rows: u32,
    n_columns: u32,
    n_max: u32,
    result: &mut [Vec<u32>],
    compute_suite: &dyn Fn(u32, Complex<f64>) -> u32) -> () {

    let delta_im = (corner_upper_left.im - corner_lower_right.im) / (n_rows as f64 - 1.0);
    let delta_re = (corner_lower_right.re - corner_upper_left.re) / (n_columns as f64 - 1.0);
    for r in 0..n_rows {
        let b = corner_upper_left.im - (r as f64) * delta_im;
        for c in 0..n_columns {
            let a = corner_upper_left.re + (c as f64) * delta_re;
            let pixel = Complex { re: a, im: b };
            result[r as usize][c as usize] = compute_suite(n_max, pixel);
        }
    }
}


mod julia_set {
    use num::Complex;
    use core::f64;

    pub const UPPER_LEFT: Complex<f64> = Complex { re: -1.5, im: 1.0 };
    pub const LOWER_RIGHT: Complex<f64> = Complex { re: 1.5, im: -1.0 };
    pub const C: Complex<f64> = Complex { re: -0.8, im: 0.156 };
    // pub const C: Complex<f64> = Complex { re: -0.7269, im: 0.1889 };
    // pub const C: Complex<f64> = Complex { re: 0.285, im: 0.0 };
    pub const R: f64 = 2.0;

    pub fn compute_suite(iterations_max: u32, z: Complex<f64>) -> u32 {
        let mut n: u32 = 0;
        let mut z = z;
        while (z.norm_sqr() <= R*R) && (n < iterations_max) {
            z = z * z + C;
            n += 1;
        }
        if z.norm_sqr() > R*R  {
            n
        } else {
            0
        }
    }
}

mod burning_ship {
    use core::f64;
    use num::{pow::Pow, traits::real::Real, Complex};

    pub const UPPER_LEFT: Complex<f64> = Complex { re: -2.5, im: 1.0 };
    pub const LOWER_RIGHT: Complex<f64> = Complex { re: 1.0, im: -1.0 };

    pub fn compute_suite(iterations_max: u32, c: Complex<f64>) -> u32 {
        let mut z = Complex { re: 0.0, im: 0.0 };
        let mut n: u32 = 0;
        while (z.norm_sqr() <= 4.0) && (n < iterations_max) {
            z = Complex {
                re: z.re.abs(),
                im: z.im.abs(),
            };
            z = z.pow(2.) + c;
            n += 1;
        }
        if z.norm_sqr() > 4.0 {
            n
        } else {
            0
        }
    }
}


mod mandelbrot {
    use core::f64;
    use num::{pow::Pow, Complex};

    pub const UPPER_LEFT: Complex<f64> = Complex { re: -2.0, im: 1.0 };
    pub const LOWER_RIGHT: Complex<f64> = Complex { re: 1.0, im: -1.0 };


    pub fn compute_suite(iterations_max: u32, c: Complex<f64>) -> u32 {
        let mut z = Complex { re: 0.0, im: 0.0 };
        let mut n: u32 = 0;
        while (z.norm_sqr() <= 4.0) && (n < iterations_max) {
            z = z.pow(2.) + c;
            n += 1;
        }
        if z.norm_sqr() > 4.0 {
            n
        } else {
            0
        }
    }

    // pub fn render_on_grid(
    //     corner_upper_left: Complex<f64>,
    //     corner_lower_right: Complex<f64>,
    //     n_rows: u32,
    //     n_columns: u32,
    //     n_max: u32,
    //     result: &mut [Vec<u32>],
    // ) -> () {
    //     let delta_im = (corner_upper_left.im - corner_lower_right.im) / (n_rows as f64 - 1.0);
    //     let delta_re = (corner_lower_right.re - corner_upper_left.re) / (n_columns as f64 - 1.0);
    //     for r in 0..n_rows {
    //         let b = corner_upper_left.im - (r as f64) * delta_im;
    //         for c in 0..n_columns {
    //             let a = corner_upper_left.re + (c as f64) * delta_re;
    //             let pixel = Complex { re: a, im: b };
    //             result[r as usize][c as usize] = compute_suite(n_max, pixel);
    //         }
    //     }
    // }
}

fn write_scaled_2d_array_to_grayscale_image(
    data: &Vec<Vec<u32>>,
    path: &str,
    invert: bool
) -> Result<(), image::ImageError> {
    let width = data[0].len() as u32;
    let height = data.len() as u32;

    // First, find min and max values in the 2D array for scaling
    let mut min = u32::MAX;
    let mut max = u32::MIN;
    for row in data {
        for &val in row {
            min = min.min(val);
            max = max.max(val);
        }
    }

    let mut imgbuf: ImageBuffer<Luma<u8>, Vec<u8>> = ImageBuffer::new(width, height);

    for (y, row) in data.iter().enumerate() {
        for (x, &val) in row.iter().enumerate() {
            let mut scaled_val = if max == min {
                // Avoid division by zero if all values are the same
                0
            } else {
                // Scale the value
                (((val - min) as f64) / ((max - min) as f64) * 255.0).round() as u8
            };
            if invert {
                scaled_val = scaled_val.abs_diff(255);
            }
            imgbuf.put_pixel(x as u32, y as u32, Luma([scaled_val]));
        }
    }

    imgbuf.save(path)
}

fn main() {

    // Parse command line arguments
    let matches = Command::new("mandelbrot")
        .author("Ammar Mian")
        .about("Compute Mandelbrot visualization")
        .arg(
            Arg::new("name")
                .long("name")
                .value_name("NAME")
                .help("Fractal to compute (mandelbrot, julia or burning_ship)")
        )
        .arg(
            Arg::new("n_rows")
                .long("n_rows")
                .value_name("N_ROWS")
                .help("Number of rows in the grid"),
        )
        .arg(
            Arg::new("n_columns")
                .long("n_columns")
                .value_name("N_COLUMNS")
                .help("Number of columns in the grid"),
        )
        .arg(
            Arg::new("n_max")
                .long("n_max")
                .value_name("N_MAX")
                .help("Maximum number of iterations"),
        )
        .arg(
            Arg::new("n_threads")
                .long("n_threads")
                .value_name("N_THREADS")
                .help("Number of threads to use"),
        )
        .arg(
            Arg::new("output")
                .long("output")
                .value_name("OUTPUT")
                .help("Output file"),
        )
        .arg(
            Arg::new("invert")
                .long("invert")
                .value_name("INVERT")
                .help("Whether to invert grayscale colormap."),
        )
        .get_matches();


    let name = matches.get_one::<String>("name").map(|s| s.as_str()).unwrap_or("mandelbrot");
    let upper_left: Complex<f64>;
    let lower_right: Complex<f64>;
    match name {
        "mandelbrot" => {
            println!("Computing Mandelbrot set");
            upper_left = mandelbrot::UPPER_LEFT;
            lower_right = mandelbrot::LOWER_RIGHT;
        }
        "burning_ship" => {
            println!("Computing Burning Ship set");
            upper_left = burning_ship::UPPER_LEFT;
            lower_right = burning_ship::LOWER_RIGHT;
        },
        "julia" => {
            println!("Computing Julia set");
            upper_left = julia_set::UPPER_LEFT;
            lower_right = julia_set::LOWER_RIGHT;
        },
        _ => {
            println!("Invalid fractal name");
            return;
        }
    };

    let n_rows_str = matches.get_one::<String>("n_rows");
    let n_columns_str = matches.get_one::<String>("n_columns");
    let n_max_str = matches.get_one::<String>("n_max");
    let output = matches.get_one::<String>("output");
    let invert = matches.get_one::<String>("invert").map(|s| s.parse::<bool>().unwrap()).unwrap_or(false);
    let n_threads = matches
        .get_one::<String>("n_threads")
        .unwrap_or(&"1".to_string())
        .parse::<u32>()
        .unwrap();

    match (n_rows_str, n_columns_str, n_max_str, output) {
        (Some(n_rows_str), Some(n_columns_str), Some(n_max_str), Some(output)) => {
            let n_rows = n_rows_str.parse::<u32>().unwrap();
            let n_columns = n_columns_str.parse::<u32>().unwrap();
            let n_max = n_max_str.parse::<u32>().unwrap();
            let output = output.to_string();
            let n_rows_band = n_rows / n_threads;

            // Compute bands corner values
            let (upper_left_vec, lower_right_vec) = 
                compute_bands_corners(n_threads, upper_left, lower_right);

            // Preallocate results and create iterator
            let mut fractal = vec![vec![0u32; n_columns as usize]; n_rows as usize];
            let bands: Vec<&mut [Vec<u32>]> = fractal.chunks_mut(n_rows_band as usize).collect();

            // Dunno why this doesn't work. TODO: figure the rules of rust.
            // let bands_indexes = fractal::compute_bands_indexes(n_threads, n_rows);
            // let bands: Vec<&mut [Vec<u32>]> =
            //     (0..n_threads)
            //     .into_iter()
            //     .map(|i| {
            //         let start = bands_indexes[i as usize] as usize;
            //         let end = bands_indexes[(i+1) as usize] as usize;
            //         &mut fractal[start..end]
            //     }).collect();

            // Threads over bands
            let now = Instant::now();
            thread::scope(|s| {
                let mut handles = Vec::new();

                for (i, band) in bands.into_iter().enumerate() {
                    let upper_left = upper_left_vec[i as usize];
                    let lower_right = lower_right_vec[i as usize];
                    let handle = s.spawn(move |_| {
                        // &dyn seems to not be able to be shared between threads..
                        // TODO: figure out how to do this
                        let compute_suite = match name {
                            "mandelbrot" => mandelbrot::compute_suite,
                            "burning_ship" => burning_ship::compute_suite,
                            "julia" => julia_set::compute_suite,
                            _ => panic!("Invalid fractal name"),
                        };
                        println!("Thread {} started", i);
                        render_on_grid(
                            upper_left,
                            lower_right,
                            n_rows_band,
                            n_columns,
                            n_max,
                            band,
                            &compute_suite
                        );
                    });
                    handles.push(handle);
                }

                for (i, handle) in handles.into_iter().enumerate() {
                    handle.join().unwrap();
                    println!("Thread {} joined", i);
                }
            })
            .unwrap();

            println!("Threads joined in {} seconds", now.elapsed().as_secs());

            // Write to fractal.png
            println!("Writing Image to {}", output);
            let _ = write_scaled_2d_array_to_grayscale_image(&fractal, &output, invert);
            println!("Done");
        }
        _ => {
            println!("Please provide all arguments. Use --help for more information.");
        }
    }
}

// Tests
#[cfg(test)]
mod test_mandelbrot {
    use super::*;

    #[test]
    fn test_suite_zero_mandelbrot() {
        let c = Complex { re: 0.0, im: 0.0 };
        let n = mandelbrot::compute_suite(1000, c);
        assert_eq!(n, 0);
    }

    #[test]
    fn test_bands_single_thread() {
        let (upper_left, lower_right) = compute_bands_corners(
            1, Complex { re: -2.0, im: 1.0 }, Complex { re: 1.0, im: -1.0 });

        assert_eq!(upper_left.len(), 1);
        assert_eq!(lower_right.len(), 1);
        assert_eq!(upper_left[0], Complex { re: -2.0, im: 1.0 });
        assert_eq!(lower_right[0], Complex { re: 1.0, im: -1.0 });
    }

    #[test]
    fn test_bands_multi_thread() {
        let (upper_left, lower_right) = compute_bands_corners(
            4, Complex { re: -2.0, im: 1.0 }, Complex { re: 1.0, im: -1.0 });
                    
        assert_eq!(upper_left.len(), 4);
        assert_eq!(lower_right.len(), 4);
        assert_eq!(upper_left[0], Complex { re: -2.0, im: 1.0 });
        assert_eq!(lower_right[0], Complex { re: 1.0, im: 0.5 });
        assert_eq!(upper_left[1], Complex { re: -2.0, im: 0.5 });
        assert_eq!(lower_right[1], Complex { re: 1.0, im: 0.0 });
        assert_eq!(upper_left[2], Complex { re: -2.0, im: 0.0 });
        assert_eq!(lower_right[2], Complex { re: 1.0, im: -0.5 });
        assert_eq!(upper_left[3], Complex { re: -2.0, im: -0.5 });
        assert_eq!(lower_right[3], Complex { re: 1.0, im: -1.0 });
    }
}
