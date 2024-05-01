use num_traits::{FromPrimitive, Num, ToPrimitive};

pub struct MipMap1D<T: Num + ToPrimitive + FromPrimitive> {
    data: Vec<Vec<T>>,
}

impl<T: Num + ToPrimitive + FromPrimitive + Copy> MipMap1D<T> {
    pub fn build(source: Vec<T>) -> Self {
        let mut data = vec![source.clone()];
        let mut current = source;

        while current.len() > 1 {
            let mipmap = Self::downsample(&current);
            current.clone_from(&mipmap);
            data.push(mipmap);
        }

        Self { data }
    }

    pub fn num_levels(&self) -> usize {
        self.data.len()
    }

    pub fn get_level(&self, level: usize) -> Option<&Vec<T>> {
        if level > self.num_levels() {
            return None;
        }

        Some(&self.data[level])
    }

    /// Downsamples a vector to ceil(len / 2) elements.
    /// Currently, downsampling is done by averaging the pair of elements
    fn downsample(source: &[T]) -> Vec<T> {
        source
            .chunks(2)
            .map(|pair| match pair.len() {
                1 => pair[0],
                2 => T::from_f64((pair[0] + pair[1]).to_f64().unwrap() / 2.0).unwrap(),
                _ => panic!("Unsound condition"),
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_correct_downsample_ints() {
        let data = vec![2, 4, 6, 8];
        assert_eq!(MipMap1D::downsample(&data), vec![3, 7]);
    }

    #[test]
    fn test_uneven_downsample() {
        let data = vec![2, 4, 6, 8, 9];
        assert_eq!(MipMap1D::downsample(&data), vec![3, 7, 9]);
    }

    #[test]
    fn test_uneven_mipmap() {
        let data = vec![2, 4, 6, 8, 9];
        let target = vec![vec![2, 4, 6, 8, 9], vec![3, 7, 9], vec![5, 9], vec![7]];
        let mipmap = MipMap1D::build(data);
        assert_eq!(mipmap.data, target);
    }

    #[test]
    fn test_mipmap_levels() {
        let data = vec![2, 4, 6, 8, 9];
        let target = [vec![2, 4, 6, 8, 9], vec![3, 7, 9], vec![5, 9], vec![7]];
        let mipmap = MipMap1D::build(data);

        assert_eq!(mipmap.num_levels(), target.len());
        for (level, target_item) in target.iter().enumerate() {
            let res = mipmap.get_level(level);
            assert!(res.is_some());
            let res = res.unwrap();

            assert_eq!(*res, *target_item)
        }
    }
}
