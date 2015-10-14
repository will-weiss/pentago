extern crate itertools;
use self::itertools::Product;

pub type Coordinates = Vec<usize>;

fn nonzero_coordinates(dim: usize, length: usize) -> Vec<Coordinates> {
    Product::new(get_all_coordinates(dim - 1, length).iter(), (0..length))
        .map(|(coords, c)| {
            // There has to be a functional way to do this...
            let mut cs = coords.clone();
            cs.push(c);
            cs
        }).collect()
}

pub fn get_all_coordinates(dim: usize, length: usize) -> Vec<Coordinates> {
    if dim == 0 { vec![vec![]] }
    else { nonzero_coordinates(dim, length) }
}

pub fn coordinates_to_ix(coordinates: Coordinates, length: usize) -> usize {
    coordinates.iter().enumerate().fold(0, |ix, (dimension_ix, coordinate)| {
        let inverted_dim_ix = (coordinates.len() - dimension_ix - 1) as u32;
        let dim_multiplier = length.pow(inverted_dim_ix);
        ix + (coordinate * dim_multiplier)
    })
}
