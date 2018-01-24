#[derive(Copy, Clone, PartialEq)]
pub struct Rgb<T = f32> {
    pub red: T,
    pub green: T,
    pub blue: T,
}
#[derive(Copy, Clone, PartialEq)]
pub struct Alpha<C, T = f32> {
    pub alpha: T,
    pub color: C,
}
pub struct Hsla<T = f32> {
    pub hue: T,
    pub saturation: T,
    pub lightness: T,
    pub alpha: T,
}
pub struct Hsv<T = f32> {
    pub hue: Hue<T>,
    pub saturation: T,
    pub value: T,
}
pub type Rgba<T> = Alpha<Rgb<T>, T>;
#[derive(Copy, Clone, From)]
pub struct Hue<T>(T);

impl<T> Hue<T> {
    pub fn to_degrees(self) -> T {
        self.0
    }
}

pub trait RgbPixel<T> {
    fn from_rgba(red: T, green: T, blue: T, alpha: T) -> Self;
    fn to_rgba(&self) -> (T, T, T, T);
}

impl RgbPixel<f32> for (f32, f32, f32, f32) {
    fn from_rgba(red: f32, green: f32, blue: f32, alpha: f32) -> (f32, f32, f32, f32) {
        (red, green, blue, alpha)
    }

    fn to_rgba(&self) -> (f32, f32, f32, f32) {
        *self
    }
}

fn hsl_to_rgb(h: f32, s: f32, l: f32) -> (f32, f32, f32) {
    let h = h / 360.0;
    if s <= 0.0 {
        (l, l, l)
    } else {
        fn hue_to_rgb(p: f32, q: f32, mut t: f32) -> f32 {
            if t < 0.0 {
                t += 1.0;
            }
            if t > 1.0 {
                t -= 1.0;
            }
            if t < 1.0 / 6.0 {
                return p + (q - p) * 6.0 * t;
            }
            if t < 1.0 / 2.0 {
                return q;
            }
            if t < 2.0 / 3.0 {
                return p + (q - p) * (2.0 / 3.0 - t) * 6.0;
            }
            return p;
        }

        let q = if l < 0.5 {
            l * (1.0 + s)
        } else {
            l + s - l * s
        };
        let p = 2.0 * l - q;
        (
            hue_to_rgb(p, q, h + 1.0 / 3.0),
            hue_to_rgb(p, q, h),
            hue_to_rgb(p, q, h - 1.0 / 3.0),
        )
    }
}

fn rgb_to_hsv(r: f32, g: f32, b: f32) -> (f32, f32, f32) {
    let (min, max) = (r.min(g).min(b), r.max(g).max(b));

    let v = max;

    let delta = max - min;
    if delta < 0.0001 || max <= 0.0 {
        return (0.0, 0.0, v);
    }

    let s = delta / max;

    let mut h = 60.0 * if r >= max {
        (g - b) / delta
    } else if g >= max {
        2.0 + (b - r) / delta
    } else {
        4.0 + (r - g) / delta
    };
    if h < 0.0 {
        h += 360.0;
    }

    (h, s, v)
}

fn hsv_to_rgb(h: f64, s: f64, v: f64) -> (f64, f64, f64) {
    if s <= 0.0 {
        (v, v, v)
    } else {
        let hh = (h % 360.0) / 60.0;
        let i = hh as usize;
        let ff = hh - hh.floor();
        let p = v * (1.0 - s);
        let q = v * (1.0 - s * ff);
        let t = v * (1.0 - s * (1.0 - ff));

        match i {
            0 => (v, t, p),
            1 => (q, v, p),
            2 => (p, v, t),
            3 => (q, p, v),
            4 => (t, p, v),
            _ => (v, p, q),
        }
    }
}

impl RgbPixel<f32> for [f32; 4] {
    fn from_rgba(red: f32, green: f32, blue: f32, alpha: f32) -> [f32; 4] {
        [red, green, blue, alpha]
    }

    fn to_rgba(&self) -> (f32, f32, f32, f32) {
        (self[0], self[1], self[2], self[3])
    }
}

impl Alpha<Rgb<f32>, f32> {
    pub fn to_pixel(&self) -> [f32; 4] {
        [
            self.color.red,
            self.color.green,
            self.color.blue,
            self.alpha,
        ]
    }

    pub fn from_pixel<P: RgbPixel<f32>>(p: &P) -> Self {
        let (r, g, b, a) = p.to_rgba();
        Self {
            alpha: a,
            color: Rgb {
                red: r,
                green: g,
                blue: b,
            },
        }
    }
}

impl Hsla {
    pub fn new(h: f32, s: f32, l: f32, a: f32) -> Self {
        Hsla {
            hue: h,
            saturation: s,
            lightness: l,
            alpha: a,
        }
    }
}

impl From<Hsla> for Rgba<f32> {
    fn from(hsla: Hsla) -> Rgba<f32> {
        let (r, g, b) = hsl_to_rgb(hsla.hue, hsla.saturation, hsla.lightness);
        Rgba::from_pixel(&(r, g, b, hsla.alpha))
    }
}

impl<T> Hsv<T> {
    pub fn new(h: Hue<T>, s: T, v: T) -> Self {
        Hsv {
            hue: h,
            saturation: s,
            value: v,
        }
    }
}

impl From<Rgba<f32>> for Hsv<f32> {
    fn from(c: Rgba<f32>) -> Hsv<f32> {
        let (h, s, v) = rgb_to_hsv(c.color.red, c.color.green, c.color.blue);
        Hsv {
            hue: h.into(),
            saturation: s,
            value: v,
        }
    }
}

impl From<Hsv<f64>> for Rgb<f64> {
    fn from(c: Hsv<f64>) -> Rgb<f64> {
        let (r, g, b) = hsv_to_rgb(c.hue.0, c.saturation, c.value);
        Rgb {
            red: r,
            green: g,
            blue: b,
        }
    }
}
