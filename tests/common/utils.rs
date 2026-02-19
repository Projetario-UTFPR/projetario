use std::collections::HashSet;
use std::hash::Hash;

pub mod esquema_db;
pub mod geradores;
pub mod headers;
pub mod queries;

pub fn gerar_combinacoes<T: Clone + Eq + PartialEq + Hash>(
    elementos: &[T],
    min: usize,
    max: usize,
) -> impl Iterator<Item = HashSet<T>> + '_ {
    let n = elementos.len();

    assert!(min <= max);
    assert!(max <= n);

    (1usize..(1 << n))
        .filter(move |mask| {
            let bits = mask.count_ones() as usize;
            bits >= min && bits <= max
        })
        .map(move |mask| {
            (0..n)
                .filter(|i| mask & (1 << i) != 0)
                .map(|i| elementos[i].clone())
                .collect::<HashSet<_>>()
        })
}
