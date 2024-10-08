use mimalloc::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

pub mod napi;
pub mod sources;
mod threading;

#[cfg(test)]
pub mod test;
