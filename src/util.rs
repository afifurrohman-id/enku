use std::fmt::Display;

use web_sys::js_sys::Math;

pub fn rand(count: usize) -> usize {
    Math::floor(Math::random() * count as f64) as usize
}

pub fn randomize<T, F>(data: &[T], mut f: F)
where
    T: Display + Clone,
    F: FnMut(usize, T),
{
    let mut data = Vec::from(data);
    loop {
        let count = data.len();

        if data.is_empty() {
            break;
        }

        let i = rand(count);

        let not_exist = data.get(i).is_none();
        if not_exist {
            continue;
        }

        let d = data.remove(i);

        f(count - 1, d);
    }
}

#[cfg(test)]
mod tests {
    use wasm_bindgen_test::wasm_bindgen_test;

    #[wasm_bindgen_test]
    fn rand() {
        let x = super::rand(10);
        assert!(x < 10);
        #[allow(clippy::absurd_extreme_comparisons)]
        #[allow(unused_comparisons)]
        let r = x >= 0;
        assert!(r)
    }

    #[wasm_bindgen_test]
    fn randomize() {
        let data = &[1, 2, 3, 4, 5, 6, 7, 8];
        let mut n = 0;
        super::randomize(data, |i, v| {
            #[allow(clippy::absurd_extreme_comparisons)]
            #[allow(unused_comparisons)]
            let r = i >= 0;
            assert!(r);
            assert!(i < 8);
            assert!(v >= 1);
            assert!(v <= 8);
            assert!(data.contains(&v));

            n += 1;
        });

        assert_eq!(data.len(), n);
    }
}
