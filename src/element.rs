use core::f64;
use ndarray::Array2;
use std::fmt::{Display, Error, Formatter};

const ERADIUS_TIMES_RESTMASS: f64 = 0.959976365e-9;
const C_Q: f64 = 3.83193864121903e-13;

#[derive(Debug)]
pub enum EleType {
    EleTypeMarker,
    EleTypeDrift,
    EleTypeBend,
    EleTypeQuad,
    EleTypeSext,
    EleTypeOct,
    EleTypeMult,
    EleTypeCav,
}

#[derive(Debug, Clone)]
pub struct Element {
    pub name: String,
    pub length: f64,
    pub k: [f64; 4],
    pub _frequency: f64,
    pub _voltage: f64,
    pub _harmonic: f64,
    pub _lag: f64,
    pub r_matrix: Array2<f64>,
    pub eta_prop_matrix: Array2<f64>,
}

impl Display for Element {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match element_type(self) {
            EleType::EleTypeMarker => {
                write!(f, "{name}: Marker", name = self.name)
            }
            EleType::EleTypeDrift => {
                write!(f, "{name}: Drift", name = self.name)
            }
            EleType::EleTypeBend => {
                write!(f, "{name}: Bend", name = self.name)
            }
            EleType::EleTypeQuad => {
                write!(f, "{name}: Quad", name = self.name)
            }
            EleType::EleTypeSext => {
                write!(f, "{name}: Sext", name = self.name)
            }
            EleType::EleTypeOct => {
                write!(f, "{name}: Oct", name = self.name)
            }
            EleType::EleTypeMult => {
                write!(f, "{name}: Mult", name = self.name)
            }
            EleType::EleTypeCav => {
                write!(f, "{name}: RFCav", name = self.name)
            }
        }
    }
}

impl Default for Element {
    fn default() -> Self {
        let r_matrix = Array2::<f64>::eye(6);

        Self {
            name: "".to_string(),
            length: 0.0,
            k: [0.0; 4],
            _frequency: 0.0,
            _voltage: 0.0,
            _harmonic: 0.0,
            _lag: 0.0,
            eta_prop_matrix: make_eta_prop_matrix(&r_matrix),
            r_matrix,
        }
    }
}

impl Element {
    pub fn bending_radius(&self) -> f64 {
        if self.k[0] == 0.0 {
            f64::INFINITY
        } else {
            self.length / self.k[0]
        }
    }
}

pub fn element_type(ele: &Element) -> EleType {
    if ele._voltage != 0.0 || ele._harmonic != 0.0 || ele._lag != 0.0 {
        EleType::EleTypeCav
    } else if ele.length == 0.0 {
        EleType::EleTypeMarker
    } else if ele.k[0] == 0.0 && ele.k[1] == 0.0 && ele.k[2] == 0.0 && ele.k[3] == 0.0 {
        EleType::EleTypeDrift
    } else if ele.k[0] != 0.0 && ele.k[2] == 0.0 && ele.k[3] == 0.0 {
        EleType::EleTypeBend
    } else if ele.k[0] == 0.0 && ele.k[1] != 0.0 && ele.k[2] == 0.0 && ele.k[3] == 0.0 {
        EleType::EleTypeQuad
    } else if ele.k[0] == 0.0 && ele.k[1] == 0.0 && ele.k[2] != 0.0 && ele.k[3] == 0.0 {
        EleType::EleTypeSext
    } else if ele.k[0] == 0.0 && ele.k[1] == 0.0 && ele.k[2] == 0.0 && ele.k[3] != 0.0 {
        EleType::EleTypeOct
    } else {
        EleType::EleTypeMult
    }
}

pub fn make_marker(name: String) -> Element {
    Element {
        name,
        ..Default::default()
    }
}

pub fn make_drift(name: String, length: f64) -> Element {
    let mut r_matrix = Array2::eye(6);
    r_matrix[[0, 1]] = length;
    r_matrix[[2, 3]] = length;
    Element {
        name,
        length,
        eta_prop_matrix: make_eta_prop_matrix(&r_matrix),
        r_matrix,
        ..Default::default()
    }
}

pub fn make_cavity(
    name: String,
    length: f64,
    freq: f64,
    voltage: f64,
    phase: f64,
    harmonic: f64,
) -> Element {
    let mut r_matrix = Array2::eye(6);
    r_matrix[[0, 1]] = length;
    r_matrix[[2, 3]] = length;
    Element {
        name,
        length,
        eta_prop_matrix: make_eta_prop_matrix(&r_matrix),
        r_matrix,
        _frequency: freq,
        _harmonic: harmonic,
        _voltage: voltage,
        _lag: phase,
        ..Default::default()
    }
}

pub fn make_sext(name: String, length: f64, k2: f64) -> Element {
    let mut r_matrix = Array2::eye(6);
    r_matrix[[0, 1]] = length;
    r_matrix[[2, 3]] = length;
    Element {
        name,
        length,
        eta_prop_matrix: make_eta_prop_matrix(&r_matrix),
        r_matrix,
        k: [0.0, 0.0, k2, 0.0],
        ..Default::default()
    }
}

pub fn make_oct(name: String, length: f64, k3: f64) -> Element {
    let mut r_matrix = Array2::eye(6);
    r_matrix[[0, 1]] = length;
    r_matrix[[2, 3]] = length;
    Element {
        name,
        length,
        eta_prop_matrix: make_eta_prop_matrix(&r_matrix),
        r_matrix,
        k: [0.0, 0.0, 0.0, k3],
        ..Default::default()
    }
}

pub fn make_quad(name: String, length: f64, k1: f64) -> Element {
    if length == 0.0 {
        return make_marker(name);
    }

    if k1 == 0.0 {
        return make_drift(name, length);
    }

    let omega: f64 = k1.abs().sqrt();
    let omega_l: f64 = omega * length;

    let c_omega_l: f64 = omega_l.cos();
    let s_omega_l: f64 = omega_l.sin();
    let ch_omega_l: f64 = omega_l.cosh();
    let sh_omega_l: f64 = omega_l.sinh();

    let mut r_matrix = Array2::eye(6);

    if k1 > 0.0 {
        // Focusing 2x2
        r_matrix[[0, 0]] = c_omega_l;
        r_matrix[[0, 1]] = s_omega_l / omega;
        r_matrix[[1, 0]] = s_omega_l * (-omega);
        r_matrix[[1, 1]] = c_omega_l;
        // Defocusing 2x2
        r_matrix[[2, 2]] = ch_omega_l;
        r_matrix[[2, 3]] = sh_omega_l / omega;
        r_matrix[[3, 2]] = sh_omega_l * omega;
        r_matrix[[3, 3]] = ch_omega_l;
    } else {
        // Focusing 2x2
        r_matrix[[2, 2]] = c_omega_l;
        r_matrix[[2, 3]] = s_omega_l / omega;
        r_matrix[[3, 2]] = s_omega_l * (-omega);
        r_matrix[[3, 3]] = c_omega_l;
        // Defocusing 2x2
        r_matrix[[0, 0]] = ch_omega_l;
        r_matrix[[0, 1]] = sh_omega_l / omega;
        r_matrix[[1, 0]] = sh_omega_l * omega;
        r_matrix[[1, 1]] = ch_omega_l;
    }

    Element {
        name,
        length,
        k: [0.0, k1, 0.0, 0.0],
        eta_prop_matrix: make_eta_prop_matrix(&r_matrix),
        r_matrix,
        ..Default::default()
    }
}

pub fn make_sbend(name: String, length: f64, angle: f64, k1: f64) -> Element {
    if length == 0.0 {
        return make_marker(name);
    }
    if angle == 0.0 {
        return make_quad(name, length, k1);
    }

    let h = angle / length;

    let omega_x_sqr = h.powi(2) + k1;
    let omega_x = omega_x_sqr.abs().sqrt();
    let omega_x_l = omega_x * length;

    let omega_y_sqr = k1;
    let omega_y = omega_y_sqr.abs().sqrt();
    let omega_y_l = omega_y * length;

    let mut r_matrix = Array2::eye(6);

    if omega_x_sqr == 0.0 {
        r_matrix[[0, 1]] = length;
        r_matrix[[4, 0]] = h * length;
    } else if omega_x_sqr > 0.0 {
        r_matrix[[0, 0]] = omega_x_l.cos();
        r_matrix[[0, 1]] = omega_x_l.sin() / omega_x;
        r_matrix[[1, 0]] = omega_x_l.sin() * (-omega_x);
        r_matrix[[1, 1]] = omega_x_l.cos();
        r_matrix[[0, 5]] = (h / omega_x_sqr) * (1.0 - omega_x_l.cos());
        r_matrix[[1, 5]] = (h / omega_x) * omega_x_l.sin();
        r_matrix[[4, 0]] = r_matrix[[1, 5]];
        r_matrix[[4, 1]] = r_matrix[[0, 5]];
        r_matrix[[4, 5]] = -h.powi(2) * (omega_x_l - omega_x_l.sin()) / omega_x.powi(3);
    } else {
        r_matrix[[0, 0]] = omega_x_l.cosh();
        r_matrix[[0, 1]] = omega_x_l.sinh() / omega_x;
        r_matrix[[1, 0]] = omega_x_l.sinh() * (omega_x);
        r_matrix[[1, 1]] = omega_x_l.cosh();
        r_matrix[[0, 5]] = (h / omega_x_sqr) * (1.0 - omega_x_l.cosh());
        r_matrix[[1, 5]] = (h / omega_x) * omega_x_l.sinh();
        r_matrix[[4, 0]] = r_matrix[[1, 5]];
        r_matrix[[4, 1]] = r_matrix[[0, 5]];
        r_matrix[[4, 5]] = -h.powi(2) * (omega_x_l - omega_x_l.sinh()) / omega_x.powi(3);
    }

    if omega_y_sqr == 0.0 {
        r_matrix[[0, 1]] = length;
        r_matrix[[4, 0]] = h * length;
    } else if omega_y_sqr < 0.0 {
        r_matrix[[2, 2]] = omega_y_l.cos();
        r_matrix[[2, 3]] = omega_y_l.sin() / omega_y;
        r_matrix[[3, 2]] = omega_y_l.sin() * (-omega_y);
        r_matrix[[3, 3]] = omega_y_l.cos();
    } else {
        r_matrix[[2, 2]] = omega_y_l.cosh();
        r_matrix[[2, 3]] = omega_y_l.sinh() / omega_y;
        r_matrix[[3, 2]] = omega_y_l.sinh() * (omega_y);
        r_matrix[[3, 3]] = omega_y_l.cosh();
    }

    Element {
        name,
        length,
        k: [angle, k1, 0.0, 0.0],
        eta_prop_matrix: make_eta_prop_matrix(&r_matrix),
        r_matrix,
        ..Default::default()
    }
}

fn make_eta_prop_matrix(r_matrix: &Array2<f64>) -> Array2<f64> {
    let mut retval: Array2<f64> = Array2::zeros((3, 3));

    retval[[0, 0]] = r_matrix[[0, 0]];
    retval[[0, 1]] = r_matrix[[0, 1]];
    retval[[0, 2]] = r_matrix[[0, 5]];

    retval[[1, 0]] = r_matrix[[1, 0]];
    retval[[1, 1]] = r_matrix[[1, 1]];
    retval[[1, 2]] = r_matrix[[1, 5]];

    retval[[2, 0]] = 0.0;
    retval[[2, 1]] = 0.0;
    retval[[2, 2]] = 1.0;

    retval
}

pub fn get_line_matrix(line: &[Element]) -> Array2<f64> {
    let mut retval: Array2<f64> = Array2::eye(6);

    for ele in line {
        retval = retval.dot(&ele.r_matrix);
    }

    retval
}

pub fn apply_matrix_n_times(matrix: &Array2<f64>, n: usize) -> Array2<f64> {
    let mut result = matrix.clone();

    for _ in 1..n {
        result = result.dot(matrix);
    }

    result
}

pub fn print_matrix(matrix: &Array2<f64>) {
    for row in matrix.outer_iter() {
        for (i, item) in row.iter().enumerate() {
            print!(" {}", fmt_f64(*item, 10, 6, 2));
            if i != 5 {
                print!(" ");
            }
        }
        println!();
    }
}

fn fmt_f64(num: f64, width: usize, precision: usize, exp_pad: usize) -> String {
    if num.is_nan() {
        return format!("{:>width$}", "NaN", width = width);
    }
    if num.is_infinite() {
        return format!(
            "{:>width$}",
            if num.is_sign_negative() {
                "-inf"
            } else {
                "+inf"
            },
            width = width
        );
    }
    let mut num = format!("{num:+.precision$e}");
    // Safe to `unwrap` as `num` is guaranteed to contain `'e'`
    let exp = num.split_off(num.find('e').unwrap());

    let (sign, exp) = if exp.starts_with("e-") {
        ('-', &exp[2..])
    } else {
        ('+', &exp[1..])
    };
    num.push_str(&format!("e{sign}{exp:0>exp_pad$}"));

    format!("{num:>width$}")
}

pub fn get_bending_angle(line: &[Element]) -> f64 {
    line.iter().fold(0.0, |acc, x| acc + x.k[0])
}

pub fn get_line_length(line: &[Element]) -> f64 {
    line.iter().fold(0.0, |acc, x| acc + x.length)
}

pub fn synch_rad_integral_2(line: &[Element]) -> f64 {
    line.iter()
        .fold(0.0, |acc, x| acc + x.length / x.bending_radius().powi(2))
}

pub fn synch_rad_integral_3(line: &[Element]) -> f64 {
    line.iter().fold(0.0, |acc, x| {
        acc + x.length / x.bending_radius().abs().powi(3)
    })
}

pub fn get_curly_h(ele: &Element, eta0: f64, etap0: f64, beta0: f64, alpha0: f64) -> f64 {
    let gamma0 = (1.0 / beta0) * (1.0 + alpha0 * alpha0);

    if ele.k[0] == 0.0 || ele.length == 0.0 {
        return gamma0 * eta0 * eta0 + 2.0 * alpha0 * eta0 * etap0 + beta0 * etap0 * etap0;
    }

    let k1 = ele.k[1];
    let l = ele.length;
    let angle = ele.k[0];
    let h = (angle / l).abs();
    let cube_h = h.powi(3);

    let k_sqr = h.powi(2) + k1;
    let k = k_sqr.abs().sqrt();

    let big_k = k_sqr.abs();
    let psi = k * l;

    let i_5 = if k_sqr > 0.0 {
        l * cube_h.abs()
            * (gamma0 * eta0.powi(2) + 2e0 * alpha0 * eta0 * etap0 + beta0 * etap0.powi(2))
            - 2e0 * h.powi(4) / (big_k.powf(3e0 / 2e0))
                * (big_k.sqrt() * (alpha0 * eta0 + beta0 * etap0) * (psi.cos() - 1e0)
                    + (gamma0 * eta0 + alpha0 * etap0) * (psi - psi.sin()))
            + h.powi(5).abs() / (4e0 * big_k.powf(5e0 / 2e0))
                * (2e0 * alpha0 * big_k.sqrt() * (4e0 * psi.cos() - (2e0 * psi).cos() - 3e0)
                    + beta0 * big_k * (2e0 * psi - (2e0 * psi).sin())
                    + gamma0 * (6e0 * psi - 8e0 * psi.sin() + (2e0 * psi).sin()))
    } else {
        l * h.powi(3).abs()
            * (gamma0 * eta0.powi(2) + 2e0 * alpha0 * eta0 * etap0 + beta0 * etap0.powi(2))
            + 2e0 * h.powi(4) / (big_k.powf(3e0 / 2e0))
                * (big_k.sqrt() * (alpha0 * eta0 + beta0 * etap0) * (psi.cosh() - 1e0)
                    + (gamma0 * eta0 + alpha0 * etap0) * (psi - psi.sinh()))
            + h.powi(5).abs() / (4e0 * big_k.powf(5e0 / 2e0))
                * (2e0 * alpha0 * big_k.sqrt() * (4e0 * psi.cosh() - (2e0 * psi).cosh() - 3e0)
                    - beta0 * big_k * (2e0 * psi - (2e0 * psi).sinh())
                    + gamma0 * (6e0 * psi - 8e0 * psi.sinh() + (2e0 * psi).sinh()))
    };

    i_5 / (l * cube_h.abs())
}

pub fn e_loss_per_turn(i_2: f64, gamma0: f64) -> f64 {
    ERADIUS_TIMES_RESTMASS * i_2 * gamma0.powi(4)
}

pub fn natural_emittance_x(i_2: f64, i_4: f64, i_5: f64, gamma0: f64) -> f64 {
    C_Q * gamma0.powi(2) * i_5 / (i_2 - i_4)
}

pub fn energy_spread(i_2: f64, i_3: f64, i_4: f64, gamma0: f64) -> f64 {
    (C_Q * gamma0.powi(2) * i_3 / (2.0 * i_2 + i_4)).sqrt()
}
