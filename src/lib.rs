use pyo3::prelude::*;
use whitespace::{bs, aquifer, incubation};

#[pyclass]
struct Ntms {
    #[pyo3()]
    pub bs: bs::Bs,
}

#[pymethods]
impl Ntms {
    #[new]
    fn new(host: &str) -> Ntms {
        Ntms {
            bs: bs::Bs::new(host),
        }

    }

    pub fn get(&mut self, page: &str) -> String {
        self.bs.get_sync(page).unwrap()
    }

    pub fn post(&mut self, page: &str, text: &str) -> String {
        self.bs.post_sync(page, text).unwrap()
    }
}

#[pyclass]
struct Whitespace {
    #[pyo3()]
    aquifer: aquifer::Aquifer,
}

#[pymethods]
impl Whitespace {
    #[new]
    fn new(host: &str) -> Whitespace {
        Whitespace {
            aquifer: aquifer::Aquifer::new(host),
        }
    }

    pub fn get(&mut self, ns: &str, page: &str) -> String {
        self.aquifer.get_text_sync(ns, page).unwrap()
    }

    pub fn post(&mut self, ns: &str, page: &str, text: &str) -> String {
        self.aquifer.set_text_sync(ns, page, text).unwrap()
    }

    pub fn getpage(&mut self, ns: &str, page: &str) -> String {
        self.aquifer.get_actual_page(ns, page)
    }
}

#[pymodule]
pub fn libvanitas(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Ntms>();
    m.add_class::<Whitespace>();

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = 4;
        assert_eq!(result, 4);
    }
}
