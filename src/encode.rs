use crate::chunk::Chunk;

pub type ChunkEncoding = fn(&Chunk) -> String;

pub fn chunk_encode_mono_hex(chunk: &Chunk) -> String {
    return format!("{:x}", chunk.1);
}

pub fn chunk_encode_greyscale_hex(chunk: &Chunk) -> String {
    return format!("{}{:x}", chunk.0, chunk.1);
}

