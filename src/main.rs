use rayon::prelude::*;

const VEC_SIZE: usize = 1_000_000_001;
const CHUNK_SIZE: usize = 128;

enum FizzBuzz {
    FIFTEEN,
    THREE,
    FIVE,
    VALUE(u32), // u32::MIN = 0, u32::MAX = 4_294_967_295
}

fn main() {
    let mut result: Vec<FizzBuzz> = Vec::with_capacity(VEC_SIZE);

    result.par_extend(
        (1..VEC_SIZE)
            .into_par_iter()
            .map(|num| FizzBuzz::VALUE(num as u32)),
    );

    result
        .par_chunks_mut(CHUNK_SIZE)
        .for_each(|chunk: &mut [FizzBuzz]| {
            let mut fifteen: u8 = 0;
            let mut three: u8 = 0;
            let mut five: u8 = 0;

            for mut chunk_item in chunk {
                fizz_buzz(&mut chunk_item, &mut fifteen, &mut three, &mut five);
            }
        });
}

fn fizz_buzz(chunk_item: &mut FizzBuzz, fifteen: &mut u8, three: &mut u8, five: &mut u8) {
    *fifteen += 1;
    *three += 1;
    *five += 1;

    if *fifteen == 15 {
        *fifteen = 0;
        *three = 0;
        *five = 0;

        *chunk_item = FizzBuzz::FIFTEEN;
    } else if *three == 3 {
        *three = 0;

        *chunk_item = FizzBuzz::THREE;
    } else if *five == 5 {
        *five = 0;
        *chunk_item = FizzBuzz::FIVE;
    }
}
