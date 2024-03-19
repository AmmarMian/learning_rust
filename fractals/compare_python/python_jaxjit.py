# Code ot generate fractals using JAX JIT

import numpy
import jax.numpy as np 
from PIL import Image # We only use PIL to save the image
from joblib import Parallel, delayed
import argparse
from tqdm import tqdm
from time import perf_counter
from jax import jit

def compute_mandelbrot(C: np.ndarray) -> np.ndarray:
    z = np.zeros(C.shape, dtype=np.complex128)
    fractal = np.full(C.shape, 256, dtype=np.int8)
    for i in range(256):
        z = z*z + C
        diverged = np.abs(z) > 2
        diverging_now = diverged & (fractal == 256)
        fractal = np.where(diverging_now, i, fractal)
    fractal = np.where(fractal == 256, 0, fractal)
    return fractal


def compute_julia(D: np.ndarray) -> np.ndarray:
    # C is a constant for the Julia set
    c = -0.8+0.156*1j
    Z = D
    fractal = np.full(Z.shape, 256, dtype=np.int8)
    for i in range(256):
        Z = Z*Z + c
        diverged = np.abs(Z) > 2
        diverging_now = diverged & (fractal == 256)
        fractal = np.where(diverging_now, i, fractal)
    fractal = np.where(fractal == 256, 0, fractal)
    return fractal

def compute_burning_ship(C: np.ndarray) -> np.ndarray:
    # C is the number on the grid
    fractal = np.full(C.shape, 256, dtype=np.int8)
    z = np.zeros(C.shape, dtype=np.complex128)
    for i in range(256):
        z = (np.abs(z.real) + np.abs(z.imag)*1j)**2 + C
        diverged = np.abs(z) > 2
        diverging_now = diverged & (fractal == 256)
        fractal = np.where(diverging_now, i, fractal)
    fractal = np.where(fractal == 256, 0, fractal)
    return fractal

def render_on_grid(
        corner_upper_left: complex,
        corner_lower_right: complex,
        n_rows: int,
        n_columns: int,
        compute_suite: callable,
    ) -> np.ndarray:

    re = np.linspace(corner_upper_left.real, corner_lower_right.real, n_columns)
    im = np.linspace(corner_upper_left.imag, corner_lower_right.imag, n_rows)
    RE, IM = np.meshgrid(re, im)
    C = RE + IM*1j
    return numpy.asarray(compute_suite(C).block_until_ready())

def compute_corners_threads(
        corner_upper_left: complex,
        corner_lower_right: complex,
        n_rows: int,
        n_columns: int,
        n_threads: int) -> list:
    # We only do a split on rows
    corners = []
    n_rows_per_thread = n_rows // n_threads
    delta_y = (corner_upper_left.imag - corner_lower_right.imag) / n_threads
    for i in range(n_threads):
        start = corner_upper_left - i*delta_y*1j
        if i == n_threads-1:
            end = corner_lower_right
        else:
            end = 1j*(corner_upper_left.imag - (i+1)*delta_y) + corner_lower_right.real
        corners.append((start, end, n_rows_per_thread, n_columns))
    return corners


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description='Generate fractals')
    parser.add_argument(
            'fractal', type=str,
            help='Fractal to generate: mandelbrot, julia or burning_ship')
    parser.add_argument(
            'output', type=str,
            help='Output file')
    parser.add_argument(
            '--n_rows', type=int, default=1000,
            help='Number of rows')
    parser.add_argument(
            '--n_columns', type=int, default=1000,
            help='Number of columns')
    parser.add_argument(
            '--n_threads', type=int, default=1,
            help='Number of threads')
    args = parser.parse_args()


    # Set the fractal to generate
    if args.fractal == "mandelbrot":
        corner_upper_left = -2+1j
        corner_lower_right = 2-1j
        compute_suite = jit(compute_mandelbrot)
    elif args.fractal == "julia":
        corner_upper_left = -1.5+1j
        corner_lower_right = 1.5-1j
        compute_suite = jit(compute_julia)
    elif args.fractal == "burning_ship":
        corner_upper_left = -2.5+1j
        corner_lower_right = 1-1j
        compute_suite = jit(compute_burning_ship)
    else:
        raise ValueError("Invalid fractal")

    # Warm up the JIT
    print("Warming up the JIT")
    for _ in range(10):
        mock_C = numpy.random.rand(100, 100) + 1j*numpy.random.rand(100, 100)
        mock_C = np.array(mock_C)
        compute_suite(mock_C)

    t0 = perf_counter()
    # Compute the corners for each thread
    corners = compute_corners_threads(
            corner_upper_left, corner_lower_right,
            args.n_rows, args.n_columns, args.n_threads)

    # Generate the fractal
    print("Generating fractal")
    results = Parallel(n_jobs=args.n_threads)(
            delayed(render_on_grid)(
                corner_upper_left, corner_lower_right,
                n_rows, n_columns, compute_suite)
            for corner_upper_left, corner_lower_right, n_rows, n_columns in tqdm(corners))
    result = numpy.vstack(results)
    t1 = perf_counter()
    print(f"Elapsed time: {t1-t0:.3f} s")

    # Save the image
    print(f"Saving image to {args.output}")
    img = Image.fromarray(result, mode="L")
    img.save(args.output)



