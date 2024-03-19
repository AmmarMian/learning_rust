using Images
using Plots
using ArgParse
using LinearAlgebra: norm
using Base.Threads: @threads, nthreads

# Compute time of execution
using Dates

# Define the functions to compute the fractals
function compute_mandelbrot(C; max_iter::Int)
    Z = zeros(ComplexF64, size(C))
    M = trues(size(C))
    R = zeros(UInt8, size(C))
    for n in 1:max_iter
        Z[M] .= Z[M].^2 .+ C[M]
        M[norm.(Z) .> 2] .= false
        R[.&(norm.(Z) .> 2, R .== 0)] .= n
    end
    return R
end

function compute_julia(Z, max_iter::Int)
    C = -0.8 + 0.156im
    M = trues(size(Z))
    R = zeros(UInt8, size(Z))
    for n in 1:max_iter
        Z[M] .= Z[M].^2 .+ C
        M[norm.(Z) .> 2] .= false
        R[.&(norm.(Z) .> 2, R .== 0)] .= n
    end
    return R
end

function compute_burning_ship(C; max_iter::Int)
    Z = zeros(ComplexF64, size(C))
    M = trues(size(C))
    R = zeros(UInt8, size(C))
    for n in 1:max_iter
        Z[M] .= (abs.(real(Z[M])) .+ abs.(imag(Z[M]))*im).^2 .+ C[M]
        M[norm.(Z) .> 2] .= false
        R[.&(norm.(Z) .> 2, R .== 0)] .= n
    end
    return R
end

function render_on_grid(ul_corner::ComplexF64, lr_corner::ComplexF64, rows::Int, cols::Int, compute_fractal, max_iter::Int)
    lin_im = range(imag(ul_corner), stop=imag(lr_corner), length=rows)
    lin_re = range(real(ul_corner), stop=real(lr_corner), length=cols)
    RE, IM = meshgrid(lin_re, lin_im)
    C = RE .+ IM*im
    return compute_fractal(C, max_iter)
end

function meshgrid(x, y)
    X = repeat(reshape(x, 1, :), length(y), 1)
    Y = repeat(reshape(y, :, 1), 1, length(x))
    return X, Y
end


# Main script logic, including argument parsing and fractal generation
function main()
    s = ArgParseSettings()
    @add_arg_table! s begin
        "fractal"
            default = "julia"
            help = "Type of fractal to generate"
        "--output", "-o"
            default = "fractal.png"
            help = "Output image file"
        "--max_iter", "-i"
            default = 255
            help = "Maximum number of iterations"
            arg_type = Int
        "--n_rows", "-r"
            default = 1000
            help = "Number of rows"
            arg_type = Int
        "--n_columns", "-c"
            default = 1000
            help = "Number of columns"
            arg_type = Int
    end

    args = parse_args(s)

    # Set fractal computation function based on input
    compute_suite = nothing
    corner_upper_left = corner_lower_right = 0.0 + 0.0im
    if args["fractal"] == "mandelbrot"
        corner_upper_left = -2.0 + 1.0im
        corner_lower_right = 2.0 - 1.0im
        compute_suite = compute_mandelbrot
    elseif args["fractal"] == "julia"
        corner_upper_left = -1.5 + 1.0im
        corner_lower_right = 1.5 - 1.0im
        compute_suite = compute_julia    
    elseif args["fractal"] == "burning_ship"
        corner_upper_left = -2.5 + 1.0im
        corner_lower_right = 1.0 - 1.0im
        compute_suite = compute_burning_ship
    end

    # Generate fractal
    println("Generating fractal...")
    t1 = now()
    result = render_on_grid(corner_upper_left, corner_lower_right, args["n_rows"], args["n_columns"], compute_suite, args["max_iter"])
    t2 = now()
    println("Fractal generated")
    println("Time: ", t2 - t1)

    # heatmap
    # plotlyjs()
    heatmap(result)
    gui()
    
    

    # Save the image
    println("Saving image...")
    # # Scale between 0 and 255
    result = (result .- minimum(result)) ./ (maximum(result) - minimum(result)) .* 255
    # cast back to integer
    result = round.(UInt8, result)
    result = map(clamp01nan, result)
    save(args["output"], Gray.(result))
    println("Image saved")

end

main()

