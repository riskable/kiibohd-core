# kiibohd-hall-effect-keyscanning

Keyscanning embedded-hal driver for [kiibohd-hall-effect](../kiibohd-hall-effect).
Can be used with single-shot, interrupt or DMA-connected ADC drivers.

## Usage

```rust
const ADC_SAMPLES: usize = 1;
const RSIZE: usize = 6; // Matrix rows
const CSIZE: usize = 12; // Matrix columns
const MSIZE: usize = RSIZE * CSIZE; // Total matrix size
type Matrix = kiibohd_hall_effect_keyscanning::Matrix<PioX<Output<PushPull>>, CSIZE, MSIZE>;

let mut matrix = Matrix::new(cols).unwrap();
matrix.next_strobe().unwrap(); // Strobe first column

// Retrieve adc sample and key index
let sample = read_adc();
let index = determine_key_index();

// Store the sample value at the specified index
// ADC_SAMPLES specifies how many samples are needed (averaged) until a processed sense value is returned
match matrix.record::<ADC_SAMPLES>(index, sample) {
		Ok(val) => {
				// If data bucket has accumulated enough samples, pass to the next stage
				if let Some(sense) = val {
						// Processed ADC data
				}
		}
		Err(e) => {
		    // Usually this is an index error
				defmt::error!("Sample record failed ({}, {}, {}):{} -> {}", i, strobe, index, sample, e);
		}

```

## Building

```bash
cargo build
```
