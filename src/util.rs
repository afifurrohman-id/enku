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

        if data.is_empty() || count == 0 {
            break;
        }

        let i = rand(count);

        let not_exist = data.get(i).is_none();
        if not_exist {
            continue;
        }

        let d = data.remove(i);

        f(count, d);
    }
}

#[cfg(test)]
mod tests {
    //TODO
}
