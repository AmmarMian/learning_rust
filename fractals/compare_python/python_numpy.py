# Numpy python code to generate fractals using joblib over lines

import numpy as np # 
from PIL import Image # We only use PIL to save the image
from joblib import Parallel, delayed
import argparse
from tqdm import tqdm
from time import perf_counter


def compute_mandelbrot(C: np.ndarray, max_iter: int) -> np.ndarray:
    # C is the number on the grid
    Z = np.zeros(C.shape, dtype=np.complex128)
    M = np.full(C.shape, True, dtype=bool)
    R = np.zeros(C.shape, dtype=np.uint8)
    for n in range(max_iter):
        Z[M] = Z[M]**2 + C[M]
        M[np.abs(Z) > 2] = False
        R[np.logical_and(np.abs(Z)>2, R == 0)] = n
        if not np.any(M):
            break
    return R

def compute_julia(Z: np.ndarray, C: complex, max_iter: int) -> np.ndarray:
    # C is a constant for the Julia set
    M = np.full(Z.shape, True, dtype=bool)
    R = np.zeros(Z.shape, dtype=np.uint8)
    for n in range(max_iter):
        Z[M] = Z[M]**2 + C
        M[np.abs(Z) > 2] = False
        R[np.logical_and(np.abs(Z)>2, R == 0)] = n
        if not np.any(M):
            break
    return R

def compute_burning_ship(C: np.ndarray, max_iter: int) -> np.ndarray:
    # C is the number on the grid
    Z = np.zeros(C.shape, dtype=np.complex128)
    M = np.full(C.shape, True, dtype=bool)
    R = np.zeros(C.shape, dtype=np.uint8)
    for n in range(max_iter):
        Z[M] = (np.abs(Z.real) + np.abs(Z.imag)*1j)**2 + C[M]
        M[np.abs(Z) > 2] = False
        R[np.logical_and(np.abs(Z)>2, R == 0)] = n
        if not np.any(M):
            break
    return R

def render_on_grid(
        corner_upper_left: complex,
        corner_lower_right: complex,
        n_rows: int,
        n_columns: int,
        compute_suite: callable,
        max_iter: int
    ) -> np.ndarray:

    lin_im = np.linspace(corner_upper_left.imag, corner_lower_right.imag, n_rows)
    lin_re = np.linspace(corner_upper_left.real, corner_lower_right.real, n_columns)
    RE, IM = np.meshgrid(lin_re, lin_im)
    C = RE + IM*1j
    return compute_suite(C, max_iter)

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
            '--max_iter', type=int, default=100,
            help='Maximum number of iterations')
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
        compute_suite = compute_mandelbrot
    elif args.fractal == "julia":
        corner_upper_left = -1.5+1j
        corner_lower_right = 1.5-1j
        c = -0.8+0.156*1j
        compute_suite = lambda z, max_iter: compute_julia(z, c, max_iter)
    elif args.fractal == "burning_ship":
        corner_upper_left = -2.5+1j
        corner_lower_right = 1-1j
        compute_suite = compute_burning_ship
    else:
        raise ValueError("Invalid fractal")

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
                n_rows, n_columns, compute_suite, args.max_iter)
            for corner_upper_left, corner_lower_right, n_rows, n_columns in tqdm(corners))
    result = np.vstack(results)
    t1 = perf_counter()
    print(f"Elapsed time: {t1-t0:.3f} s")

    # Save the image
    print(f"Saving image to {args.output}")
    img = Image.fromarray(result, mode="L")
    img.save(args.output)



