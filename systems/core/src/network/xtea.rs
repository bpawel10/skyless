use std::convert::TryInto;
use std::mem::size_of;

const DELTA: u32 = 0x9E3779B9;
const ROUNDS: usize = 32;
const BLOCK_SIZE: usize = 8;

#[derive(Debug, Clone, Copy)]
pub struct Xtea {
    key: [u32; 4],
}

impl Xtea {
    pub fn new(key: [u32; 4]) -> Self {
        Self { key }
    }

    pub fn key(&self) -> [u32; 4] {
        self.key
    }

    pub fn encrypt(&self, buffer: Vec<u8>) -> Vec<u8> {
        self.process(Self::encipher, buffer)
    }

    pub fn decrypt(&self, buffer: Vec<u8>) -> Vec<u8> {
        self.process(Self::decipher, buffer)
    }

    fn process<F>(&self, f: F, mut buffer: Vec<u8>) -> Vec<u8>
    where
        F: Fn(&mut [u32], &[u32], u32, usize),
    {
        let padding = (BLOCK_SIZE - (buffer.len() % BLOCK_SIZE)) % BLOCK_SIZE;
        if padding > 0 {
            buffer.extend_from_slice(&vec![0x33; padding])
        };

        let mut vec_u32 = Vec::with_capacity(buffer.len() / size_of::<u32>());

        for chunk in buffer.chunks(size_of::<u32>()) {
            let array: [u8; 4] = chunk.try_into().unwrap();
            vec_u32.push(u32::from_le_bytes(array));
        }

        f(&mut vec_u32, &self.key, DELTA, ROUNDS);

        let mut vec_u8 = Vec::with_capacity(buffer.len());

        for block_u32 in vec_u32 {
            vec_u8.extend_from_slice(&block_u32.to_le_bytes().to_vec());
        }

        vec_u8
    }

    fn encipher(blocks: &mut [u32], key: &[u32], delta: u32, rounds: usize) {
        let n = blocks.len();
        if n < 2 {
            return;
        }

        let mut a: u32;
        let mut b: u32;
        let mut s: u32;

        for i in 0..(n / 2) {
            a = blocks[i * 2];
            b = blocks[(i * 2) + 1];
            s = 0;

            for _ in 0..rounds {
                a = a.wrapping_add(
                    (((b << 4) ^ (b >> 5)).wrapping_add(b)) ^ s.wrapping_add(key[(s & 3) as usize]),
                );
                s = s.wrapping_add(delta);
                b = b.wrapping_add(
                    (((a << 4) ^ (a >> 5)).wrapping_add(a))
                        ^ s.wrapping_add(key[((s >> 11) & 3) as usize]),
                );
            }

            blocks[i * 2] = a;
            blocks[(i * 2) + 1] = b;
        }
    }

    fn decipher(blocks: &mut [u32], key: &[u32], delta: u32, rounds: usize) {
        let n = blocks.len();
        if n < 2 {
            return;
        }

        let mut a: u32;
        let mut b: u32;
        let mut s: u32;
        let sum: u32 = delta.wrapping_mul(rounds as u32);

        for i in 0..(n / 2) {
            a = blocks[i * 2];
            b = blocks[(i * 2) + 1];
            s = sum;

            for _ in 0..rounds {
                b = b.wrapping_sub(
                    (((a << 4) ^ (a >> 5)).wrapping_add(a))
                        ^ s.wrapping_add(key[((s >> 11) & 3) as usize]),
                );
                s = s.wrapping_sub(delta);
                a = a.wrapping_sub(
                    (((b << 4) ^ (b >> 5)).wrapping_add(b)) ^ s.wrapping_add(key[(s & 3) as usize]),
                );
            }

            blocks[i * 2] = a;
            blocks[(i * 2) + 1] = b;
        }
    }
}
