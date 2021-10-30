use std::fmt;
use std::fmt::Formatter;

#[derive(Default)]
pub struct StyleSheet {
    pub rules: Vec<Rule>,
}

impl StyleSheet {
    pub fn new(rules: Vec<Rule>) -> Self {
        Self { rules }
    }
}

impl fmt::Debug for StyleSheet {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut rule_result = String::new();
        for rule in &self.rules {
            if rule_result.is_empty() {
                rule_result.push_str("\n\n");
            }
            rule_result.push_str(&format!("{:?}", rule));
        }
        write!(f, "{}", rule_result)
    }
}

#[derive(Default)]
pub struct Rule {
    pub selectors: Vec<Selector>,
    pub declarations: Vec<Declaration>,
}

impl Rule {
    pub fn new(selectors: Vec<Selector>, declarations: Vec<Declaration>) -> Self {
        Self {
            selectors,
            declarations,
        }
    }
}

impl fmt::Debug for Rule {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut sel_result = String::new();
        let mut decl_result = String::new();
        let tab = "     ";

        for selector in &self.selectors {
            if sel_result.is_empty() {
                sel_result.push_str(", ");
            }
            sel_result.push_str(&format!("{:?}", selector));
        }

        for declaration in &self.declarations {
            decl_result.push_str(tab);
            decl_result.push_str(&format!("{:?}", declaration));
            decl_result.push('\n');
        }

        write!(f, "{} {{\n{}}}", sel_result, decl_result)
    }
}

#[derive(PartialEq, Default)]
pub struct Selector {
    pub simple: Vec<SimpleSelector>,
    pub combinations: Vec<char>,
}

impl Selector {
    pub fn new(simple: Vec<SimpleSelector>, combinations: Vec<char>) -> Self {
        Self {
            simple,
            combinations,
        }
    }
}

impl fmt::Debug for Selector {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut result = String::new();
        for sel in &self.simple {
            if result.is_empty() {
                result.push_str(", ");
            }
            result.push_str(&format!("{:?}", sel));
        }
        write!(f, "{}", result)
    }
}

#[derive(PartialEq, Default)]
pub struct SimpleSelector {
    pub tag_name: Option<String>,
    pub id: Option<String>,
    pub classes: Vec<String>,
}

impl SimpleSelector {
    pub fn new(tag_name: Option<String>, id: Option<String>, classes: Vec<String>) -> Self {
        Self {
            tag_name,
            id,
            classes,
        }
    }
}

impl fmt::Debug for SimpleSelector {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut result = String::new();

        match self.tag_name {
            None => {}
            Some(ref t) => result.push_str(t),
        }
        match self.id {
            None => {}
            Some(ref s) => {
                result.push('#');
                result.push_str(s);
            }
        }

        for class in &self.classes {
            result.push('.');
            result.push_str(class);
        }
        write!(f, "{}", result)
    }
}
pub struct Declaration {
    pub property: String,
    pub value: Value,
}

impl Declaration {
    pub fn new(property: String, value: Value) -> Self {
        Self { property, value }
    }
}

impl Default for Declaration {
    fn default() -> Self {
        Declaration {
            property: String::from(""),
            value: Value::Other(String::from("")),
        }
    }
}

impl fmt::Debug for Declaration {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {:?}", self.property, self.value)
    }
}

pub enum Value {
    Color(Color),
    Length(f32, Unit),
    Other(String),
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match *self {
            Value::Color(ref c) => write!(f, "{:?}", c),
            Value::Length(l, _) => write!(f, "{:?}", l),
            Value::Other(ref s) => write!(f, "{:?}", s),
        }
    }
}
#[derive(Copy, Clone)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }
    pub const WHITE: Color = Self {
        r: 1.,
        g: 1.,
        b: 1.,
        a: 1.,
    };
    pub const BLACK: Color = Self {
        r: 0.,
        g: 0.,
        b: 0.,
        a: 1.,
    };
    pub const BLUE: Color = Self {
        r: 0.,
        g: 0.,
        b: 1.,
        a: 1.,
    };
    pub const RED: Color = Self {
        r: 1.,
        g: 0.,
        b: 0.,
        a: 1.,
    };
    pub const ORANGE: Color = Self {
        r: 1.,
        g: 165. / 255.,
        b: 0.,
        a: 1.,
    };
    pub const ORANGERED: Color = Self {
        r: 1.,
        g: 69. / 255.,
        b: 0.,
        a: 1.,
    };
    pub const BRONZE: Color = Self {
        r: 194. / 255.,
        g: 107. / 255.,
        b: 19. / 255.,
        a: 1.,
    };
    pub const GREEN: Color = Self {
        r: 0.,
        g: 1.,
        b: 0.,
        a: 1.,
    };
    pub const TRANSPARENT: Color = Self {
        r: 0.,
        g: 0.,
        b: 0.,
        a: 0.,
    };
}

impl Default for Color {
    fn default() -> Self {
        Color::WHITE
    }
}

impl fmt::Debug for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "r:{}, g:{}, b{}, a:{}", self.r, self.g, self.b, self.a)
    }
}

#[allow(dead_code)]
pub enum Unit {
    Em,   // inherited font size
    Ex,   // height font x char
    Ch,   // width font o char
    Rem,  // font size root element
    Vh,   // 100th height viewport
    Vw,   // 100th width viewport
    Vmin, // 100th smallest side
    Vmax, // 100th largest side
    Px,   // pixel
    Mm,   // millimeter
    Q,    // quarter of millimeter
    Cm,   // centimeter
    In,   // inch
    Pt,   // 1/46th inch
    Pc,   // pica, 12 points
    Pct,  //petcentage
}
