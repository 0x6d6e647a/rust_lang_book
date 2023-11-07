use clap::Parser;

trait Temperature {
    fn temperature(&self) -> &f64;
}

struct Fahrenheit {
    temperature: f64,
}

impl Temperature for Fahrenheit {
    fn temperature(&self) -> &f64 {
        &self.temperature
    }
}

impl ToString for Fahrenheit {
    fn to_string(&self) -> String {
        format!("{:.2}°F", self.temperature)
    }
}

struct Celsius {
    temperature: f64,
}

impl Temperature for Celsius {
    fn temperature(&self) -> &f64 {
        &self.temperature
    }
}

impl ToString for Celsius {
    fn to_string(&self) -> String {
        format!("{:.2}°C", self.temperature)
    }
}

fn f2c(f: &Fahrenheit) -> Celsius {
    Celsius {
        temperature: (f.temperature() - 32.0) * 5.0 / 9.0,
    }
}

fn c2f(c: &Celsius) -> Fahrenheit {
    Fahrenheit {
        temperature: (c.temperature() * 9.0 / 5.0) + 32.0,
    }
}

#[derive(Debug, clap::Parser)]
#[command(name = "temperature")]
struct TemperatureConvert {
    #[clap(flatten)]
    temperature: TemperatureArg,
}

#[derive(Debug, clap::Args)]
#[group(required = true, multiple = false)]
struct TemperatureArg {
    #[arg(short, long)]
    celsius: Option<f64>,

    #[arg(short, long)]
    fahrenheit: Option<f64>,
}

fn main() {
    let args = TemperatureConvert::parse();

    let celsius = args.temperature.celsius;
    let fahrenheit = args.temperature.fahrenheit;

    match (celsius, fahrenheit) {
        (Some(c), None) => println!("{}", c2f(&Celsius { temperature: c }).to_string()),
        (None, Some(f)) => println!("{}", f2c(&Fahrenheit { temperature: f }).to_string()),
        (None, None) => panic!("too few arguments"),
        (Some(_), Some(_)) => panic!("too few arguments"),
    }
}
