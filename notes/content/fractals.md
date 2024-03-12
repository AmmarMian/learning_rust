---
layout: page
title: Fractals
menu:
  main:
    weight: 3
bibFile: content/bibliography.json
mermaid: true
toc: true
---

Small project to implement Fractals.

<!--more-->

## Mandelbrot Set

Let's start with something I already know to do well in python. 

{{<highlight-block Définition>}}
Mandelbrot set is defined on the complex plane for numbers $c=a+ib$, where $a\in[-2, 1]$ and $b\in[-1,1]$[^1] as the set of complex numbers $c$ for which the sequence defined by:

$$
  \left\\{\begin{array}{l}
    z_0 = 0 \\\
    z_{n+1} = z_n^2 + c
    \end{array}\right. ,
$$

remains bounded. 

[^1]: It can be defined on the whole plane for visualizations, this is the best choice of scope.

{{</highlight-block>}}

To derive a visualization in practice, we can define a grid of complex numbers, and iterate the sequence for each point. Given a parameter $N_{\mathrm{max}}$ of iterations we decide that if the absolute value of the sequence exceeds 2, we stop and color the point according to the number of iterations.[^2]

Thus a pseudo-algorithm for this can be:

{{<pseudocode>}}
\begin{algorithm}
\begin{algorithmic}
\FUNCTION{Mandelbrot}{$N_{\mathrm{max}}$, $n_{\mathrm{rows}}$, $n_{\mathrm{cols}}$}
  \STATE Instantiate an array of size $n_{\mathrm{rows}}\times n_{\mathrm{cols}}$
  \FOR{$k\in\{0,\dots, n_{\mathrm{rows}}-1\}$}
  \STATE $b=-1 + k\frac{2}{n_{\mathrm{rows}}-1}$
    \FOR{$l\in\{0, \dots,n_{\mathrm{cols}}-1\}$}
      \STATE $A=-2 + l\frac{3}{n_{\mathrm{cols}}-1}$
      \STATE $c=a+ib$, $z=0$, $n=0$
      \WHILE{$|z|<2$ and $n < N_{\mathrm{max}}$}
        \STATE $z=z^2+c$
        \STATE $n=n+1$
      \ENDWHILE
      \IF{$n < N_{\mathrm{max}}$}
        \STATE $\mathrm{array}[k,l] = n$
      \ELSE
        \STATE $\mathrm{array}[k,l] = 0$
      \ENDIF
    \ENDFOR
  \ENDFOR
  \RETURN $\mathrm{array}$
\ENDFUNCTION
\end{algorithmic}
\end{algorithm}
{{</pseudocode>}}



[^2]: That is because, if for any $n$,  $|z_n|>2$, it will diverge to infinity. See [wikipedia page](https://en.wikipedia.org/wiki/Mandelbrot_set).


### Complex numbers in rust

One need to add `num` dependency in cargo.toml:
```toml
[dependencies]
num = "0.4.0"
```
Or more simply in the project folder: `cargo add num`.

Then in using it, after building dependencies with `cargo build`:
```rust
use core::f64;
use num::Complex

fn function(c: Complex<f64) {
...
}
```


### Intanciating the grid and the result

Since we don't know at runtime the size of the grid, we need to use a `Vec` of `Vec` of `u32` to store the result. 
```rust
let mut mandelbrot = vec![vec![0u32; n_colums as usize]; n_rows as usize];
```

{{<info>}}
Better do `as usize` than doing something like `to_usize().unwrap()` so that it is less things to write.
{{</info>}}

### Saving as an image

I used crate image to save the result as a png file [^3].

[^3]: Thanks ChatGPT

We have to scale the image between 0 and 255, and then save it. 
```rust
fn write_scaled_2d_array_to_grayscale_image(data: Vec<Vec<u32>>, path: &str) -> Result<(), image::ImageError> {
    let width = data[0].len() as u32;
    let height = data.len() as u32;

    // First, find min and max values in the 2D array for scaling
    let mut min = u32::MAX;
    let mut max = u32::MIN;
    for row in &data {
        for &val in row {
            min = min.min(val);
            max = max.max(val);
        }
    }

    let mut imgbuf: ImageBuffer<Luma<u8>, Vec<u8>> = ImageBuffer::new(width, height);

    for (y, row) in data.iter().enumerate() {
        for (x, &val) in row.iter().enumerate() {
            let scaled_val = if max == min {
                // Avoid division by zero if all values are the same
                0
            } else {
                // Scale the value
                (((val - min) as f64) / ((max - min) as f64) * 255.0).round() as u8
            };
            imgbuf.put_pixel(x as u32, y as u32, Luma([scaled_val]));
        }
    }

    imgbuf.save(path)
}
```

{{<info>}}
A more efficient approach (in terms of number of written lines) can be seen [here](https://github.com/ProgrammingRust/mandelbrot/blob/2ba847967eebf95800716fe7c03d780b5130a8f8/src/main.rs#L139) using `Image::png::PNGEncoder` and `encode` method.
{{</info>}}

### Progress bar

I used crate zzz to display a progress bar. 
```rust
let pb = ProgressBar::new(n_rows as u64);
for ..
pb.add(1);
```

### Comand line arguments

I used crate clap to parse command line arguments. 
```rust
let matches = Command::new("mandelbrot")
    .author("Ammar Mian")
    .about("Compute Mandelbrot visualization")
    .arg(
        Arg::new("n_rows")
            .long("n_rows")
            .value_name("N_ROWS")
            .help("Number of rows in the grid"))
    .arg(
        Arg::new("n_columns")
            .long("n_columns")
            .value_name("N_COLUMNS")
            .help("Number of columns in the grid"))
    .arg(
        Arg::new("n_max")
            .long("n_max")
            .value_name("N_MAX")
            .help("Maximum number of iterations")
    ).get_matches();

let n_rows_str = matches.get_one::<String>("n_rows");
let n_columns_str = matches.get_one::<String>("n_columns");
let n_max_str = matches.get_one::<String>("n_max");
```

{{<warning>}}
I use version '4.5.2'. The way it works changed from previous examples so I had to adapt from examples.
{{</warning>}}

### Parallelization

There are several ways to parallelize the algortihm. I went for a unefficient one that split the images into rows bands and spawn a thread for each. The library I used is `croossbeam` that allows to spawn threads that can borrow data from main thread. 

I had lot of trouble understanding errors on moved values in the scope of the crossbeam and I had to rely on some solution provided [here](https://github.com/ProgrammingRust/mandelbrot/blob/bands/src/main.rs). The difference is that they havea data structure of `[vec<u8>]` while I have a `Vec<Vec<u32>>`. So the main idea is that we create a `vec` that preallocate the result and then we borrow the data to the threads. 
```rust
// Compute bands corner values
let (upper_left_vec, lower_right_vec) = mandelbrot::compute_bands_corners(n_threads);

// Preallocate results and create iterator
let mut mandelbrot = vec![vec![0u32; n_columns as usize]; n_rows as usize];
let bands: Vec<&mut [Vec<u32>]> = mandelbrot.chunks_mut(n_rows_band as usize).collect();

// Dunno why this doesn't work. TODO: figure the rules of rust.
// let bands_indexes = mandelbrot::compute_bands_indexes(n_threads, n_rows);
// let bands: Vec<&mut [Vec<u32>]> =
//     (0..n_threads)
//     .into_iter()
//     .map(|i| {
//         let start = bands_indexes[i as usize] as usize;
//         let end = bands_indexes[(i+1) as usize] as usize;
//         &mut mandelbrot[start..end]
//     }).collect();

// Threads over bands
let now = Instant::now();
thread::scope(|s| {
    let mut handles = Vec::new();

    for (i, band) in bands.into_iter().enumerate() {
        let upper_left = upper_left_vec[i as usize];
        let lower_right = lower_right_vec[i as usize];
        let handle = s.spawn(move |_| {
            println!("Thread {} started", i);
            mandelbrot::render_on_grid(
                upper_left,
                lower_right,
                n_rows_band,
                n_columns,
                n_max,
                band,
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
```

{{<info >}}
Had to remove progress bar because not sure what to show with several threads. Also sometimes given a number of threads an a set of number of rows/number of columns, the splitting isn't done well. I will not correct that since it was more of a learning exercise.
{{</info>}}

{{<error >}}
There is some commented code that I do not understand why the rust rules doesn't allow to do. I have to check that later.

<details>
<summary>Chat-GPT answer</summary>
the issue you're encountering with mutable references is related to Rust's ownership and borrowing rules, but not directly to the absence of Arc. The problem lies in how Rust enforces mutability and borrowing rules, especially in concurrent contexts.

The specific error you're facing likely arises from trying to create multiple mutable references to parts of the same Vec concurrently, which Rust's borrow checker disallows to prevent data races. Rust ensures safe concurrency by enforcing at compile time that either multiple immutable references (`&T`) or exactly one mutable reference (`&mut T`) can exist for any piece of data at any point in time.

In your code snippet, you're attempting to create multiple mutable slices of mandelbrot concurrently using `&mut mandelbrot[start..end]`. This violates Rust's borrowing rules because it attempts to create multiple mutable borrows from the same original data (mandelbrot), which is not allowed.

Using Arc wouldn't directly solve this problem because `Arc` only allows multiple threads to own a reference to some data; it does not permit mutability. To safely mutate data across multiple threads, you would typically combine Arc with a thread-safe synchronization primitive like `Mutex` or `RwLock`. However, your current approach doesn't involve multiple threads directly; the issue is with how you're trying to borrow parts of the `Vec` mutably at the same time.

To fix the issue within a single-threaded context or even before spawning threads, you need to ensure that you do not violate the borrowing rules. Unfortunately, Rust does not allow splitting a vector into multiple mutable slices due to its safety guarantees. Each slice borrow checks at compile time, and you cannot have overlapping mutable borrows.

If your goal is to process parts of the vector in parallel, consider the following approaches:

* Clone the Data: If cloning the data is not too expensive and the data can be independently processed, you can clone the slices and then work on each independently. This approach is simple but increases memory usage.
* Use a `Vec<Mutex<T>>` or `Vec<RwLock<T>>`: If you need to mutate elements in place and can afford the locking overhead, wrap each element or groups of elements in a `Mutex` or `RwLock`. This allows for safe, concurrent mutation but with runtime overhead.
* Parallel Libraries: For more complex data parallelism with less boilerplate, consider using a parallel computing library like rayon, which abstracts many of these details and provides a more ergonomic interface for parallel processing.
In concurrent scenarios, carefully structuring data access and ensuring thread-safe operations is key to both performance and correctness.
</details>
{{</error>}}


I noticed a speedup of around 4.8times with 12 threads on my Mid-2019 macbook pro (467s vs 96s). But I'd have to measure the times more accurately.

Result looks like:

<img src="../mandelbrot.png" alt="Mandelbrot set" width="100%"/>


## Other fractals

I tried to implement two other fractals: the burning ship and the Julia set:

{{<highlight-block "Burning ship">}}
The burning ship fractal is defined on the complex plane for numbers $c=a+ib$, where $a\in[-2.5, 1]$ and $b\in[-1,1]$ as the set of complex numbers $c$ for which the sequence defined by:
$$
\left\\{\begin{array}{l}
z_0 = 0 \\\
z_{n+1} = (|Re(z_n)|+i|Im(z_n)|)^2 + c
\end{array}\right. ,
$$
is bounded. 
{{</highlight-block>}}

{{<highlight-block Julia>}}
Given a constant complex value $c$, the Julia set is defined as the set of complex numbers $z$ for which the sequence defined by:
$$
\left\\{\begin{array}{l}
z_0 = z \\\
z_{n+1} = z_n^2 + c
\end{array}\right. ,
$$
is bounded. 
**Here, z is the complex number for which we want to compute the sequence. c is a constant**
{{</highlight-block>}}

The pseudo-algorithm are the same wiht minor changes so we omit them.

In order to implement other fractals on complex plane, I wrote the function `render_on_grid` differently to accept a function refecrence:
```rust
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
```

and created modules for each fractal implementeing the uppe rleft corner, lower right corner and a function to compute the suite.

{{<error>}}
I was not able to pass the `compute_suite` function that had been created before the thread spaw so I had to do something like:
```rust
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
```

There is probably other methods implementing traits or `Arc Mutex` stuff but I do not have enough knowledge as it is now.
{{</error>}}

I get the following result for the burning ship fractal (by inverting colors):
<img src="../burningship.png" alt="Burning ship" width="100%"/>

and for Julia ($c=-0.8+0.156i$) set:
<img src="../julia.png" alt="Julia set" width="100%"/>
