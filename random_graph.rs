/// This is the rust code for the implementation of the QFT algorithm. 
use num_complex::Complex64;
use std::f64::consts::PI;
use std::io;

/// Build the QFT matrix of size 2^n × 2^n
fn qft_matrix(n: usize) -> Vec<Vec<Complex64>> {
    let dim = 1 << n; // 2^n
    let norm = 1.0 / (dim as f64).sqrt();
    let mut matrix = vec![vec![Complex64::new(0.0, 0.0); dim]; dim];

    for j in 0..dim {
        for k in 0..dim {
            let angle = 2.0 * PI * (j * k) as f64 / dim as f64;
            matrix[j][k] = Complex64::from_polar(norm, angle);
        }
    }

    matrix
}

/// Construct a basis state vector |x⟩ with 1 at position x and 0 elsewhere
fn basis_state(n: usize, x: usize) -> Vec<Complex64> {
    let dim = 1 << n;
    let mut state = vec![Complex64::new(0.0, 0.0); dim];
    if x < dim {
        state[x] = Complex64::new(1.0, 0.0);
    }
    state
}

/// Apply the QFT matrix to an input state vector
fn apply_qft(qft: &[Vec<Complex64>], input: &[Complex64]) -> Vec<Complex64> {
    let dim = qft.len();
    let mut output = vec![Complex64::new(0.0, 0.0); dim];

    for i in 0..dim {
        for j in 0..dim {
            output[i] += qft[i][j] * input[j];
        }
    }

    output
}

/// Print a complex vector as a quantum state
fn print_quantum_state(state: &[Complex64]) {
    for (i, amp) in state.iter().enumerate() {
        if amp.norm_sqr() > 1e-10 {
            println!("|{:b}⟩: {:.4} + {:.4}i", i, amp.re, amp.im);
        }
    }
}
fn inverse_qft_matrix(qft: &[Vec<Complex64>]) -> Vec<Vec<Complex64>> {
    let dim = qft.len();
    let mut inv = vec![vec![Complex64::new(0.0, 0.0); dim]; dim];

    for i in 0..dim {
        for j in 0..dim {
            inv[i][j] = qft[j][i].conj(); // Transpose + conjugate
        }
    }

    inv
}
use plotters::prelude::*;

pub fn plot_real_imag_parts(
    state: &[Complex64],
    filename: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new(filename, (800, 400)).into_drawing_area();
    root.fill(&WHITE)?;

    let (upper, lower) = root.split_vertically(200);

    let dim = state.len();
    let max = state
        .iter()
        .map(|c| c.re.abs().max(c.im.abs()))
        .fold(0.0, f64::max);

    // Real part plot
    let mut chart = ChartBuilder::on(&upper)
        .caption("Real Part", ("sans-serif", 16))
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0..dim, -max..max)?;

    chart.configure_mesh().draw()?;

    chart.draw_series(
        state
            .iter()
            .enumerate()
            .map(|(i, c)| Rectangle::new([(i, 0.0), (i + 1, c.re)], BLUE.filled())),
    )?;

    // Imaginary part plot
    let mut chart = ChartBuilder::on(&lower)
        .caption("Imaginary Part", ("sans-serif", 16))
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0..dim, -max..max)?;

    chart.configure_mesh().draw()?;

    chart.draw_series(
        state
            .iter()
            .enumerate()
            .map(|(i, c)| Rectangle::new([(i, 0.0), (i + 1, c.im)], RED.filled())),
    )?;

    root.present()?;
    println!("Real/Imag plot saved to {filename}");
    Ok(())
}



fn main() {
    println!("Enter number of qubits (e.g., 2):");
    let mut qubit_input = String::new();
    io::stdin().read_line(&mut qubit_input).unwrap();
    let num_qubits: usize = qubit_input.trim().parse().unwrap();

    println!("Enter input basis state index (e.g., 1 for |01⟩):");
    let mut index_input = String::new();
    io::stdin().read_line(&mut index_input).unwrap();
    let input_index: usize = index_input.trim().parse().unwrap();

    let qft = qft_matrix(num_qubits);
    let input_state = basis_state(num_qubits, input_index);
    let output_state = apply_qft(&qft, &input_state);

    println!("\nQFT Matrix:");
    for row in &qft {
        for entry in row {
            print!("{:.2}+{:.2}i\t", entry.re, entry.im);
        }
        println!();
    }
    

    println!("\nInput State |{}⟩ → Output State After QFT:", input_index);
    print_quantum_state(&output_state);

    println!("\nOutput State After QFT:");
    print_quantum_state(&output_state);

    // 🖼️ Save a bar plot of amplitudes
    let _ = plot_real_imag_parts(&output_state, "real_imag_output.png");



    println!("\nApplying inverse QFT to output state...");
    let inverse = inverse_qft_matrix(&qft);
    let recovered_state = apply_qft(&inverse, &output_state);

    println!("\nRecovered (original) state:");
    print_quantum_state(&recovered_state);
}
