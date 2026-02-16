trait Investment {
    fn amount(&self) -> f64; // getter
    fn set_amount(&mut self, amount: f64); // setter

    fn double_amount(&mut self) {
        self.set_amount(self.amount() * 2.0);
    }
}

trait Taxable: Investment {
    const TAX_RATE: f64 = 0.25;

    fn tax_bill(&self) -> f64 {
        self.amount() * Self::TAX_RATE
    }
}

#[derive(Debug)]
struct Income {
    amount: f64,
}

impl Investment for Income {
    fn amount(&self) -> f64 {
        self.amount
    }

    fn set_amount(&mut self, amount: f64) {
        self.amount = amount;
    }
}

impl Taxable for Income {}

#[derive(Debug)]
struct Bonus {
    value: f64,
}

impl Investment for Bonus {
    fn amount(&self) -> f64 {
        self.value
    }

    fn set_amount(&mut self, amount: f64) {
        self.value = amount;
    }
}

impl Taxable for Bonus {
    const TAX_RATE: f64 = 0.5; // override default TAX_RATE from Taxable trait
}

#[derive(Debug)]
struct QualityTime {
    minutes: f64,
}

impl Investment for QualityTime {
    fn amount(&self) -> f64 {
        self.minutes
    }

    fn set_amount(&mut self, amount: f64) {
        self.minutes = amount;
    }
}

pub fn associated_constants() {
    let mut income = Income { amount: 1000.0 };
    income.double_amount();
    println!("Tax bill: ${:.2}", income.tax_bill());

    let mut bonus = Bonus { value: 500.0 };
    bonus.double_amount();
    println!("Tax bill: ${:.2}", bonus.tax_bill());

    let weekend = QualityTime { minutes: 300.0 };
    println!("Relaxation time: {:.2} minutes", weekend.amount());
}
