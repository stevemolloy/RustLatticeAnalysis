use ndarray::Array2;

pub trait Element {
    fn length(&self) -> f64;
}

#[derive(Default)]
pub struct Drift {
    l: f64,
    r_mat: Array2<f64>,
}

pub struct Quad {
    l: f64,
    k1: f64,
    r_mat: Array2<f64>,
}

pub struct Sbend {
    l: f64,
    angle: f64,
    k1: f64,
    e1: f64,
    e2:f64,
    r_mat: Array2<f64>,
}

pub struct Mult {
    l: f64,
    k1l: f64,
    k2l: f64,
    k3l: f64,
    r_mat: Array2<f64>,
}

pub struct Sext {
    l: f64,
    k2: f64,
    r_mat: Array2<f64>,
}

pub struct Oct {
    l: f64,
    k3: f64,
    r_mat: Array2<f64>,
}

impl Element for Drift {
    fn length(&self) -> f64 { self.l }
}

impl Drift {
    fn make_mat(&mut self) {
        self.r_mat = eye(6);
    }

}

impl Element for Quad {
    fn length(&self) -> f64 { self.l }
}

impl Element for Sbend {
    fn length(&self) -> f64 { self.l }
}

impl Element for Sext {
    fn length(&self) -> f64 { self.l }
}

impl Element for Oct {
    fn length(&self) -> f64 { self.l }
}

impl Element for Mult {
    fn length(&self) -> f64 { self.l }
}

fn main() {
    let s1: Sext = Sext{l: 1.5, k2: 9.6};
    println!("s1.l = {l}", l=s1.l);

    // let line: Vec<Box<dyn Element>> = vec![Box::new(s1)];
}
