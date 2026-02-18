#[derive(Debug)] // #[derive(Debug, Clone)]
struct Appointment {
    doctor: String,
    start_time: String,
    end_time: String,
}

impl Appointment {
    fn new(doctor: &str, start_time: &str, end_time: &str) -> Self {
        Self {
            doctor: doctor.to_string(),
            start_time: start_time.to_string(),
            end_time: end_time.to_string(),
        }
    }
}

// You can derive `Clone` instead when all fields implement `Clone`.
impl Clone for Appointment {
    fn clone(&self) -> Self {
        Self {
            doctor: self.doctor.clone(),
            start_time: self.start_time.clone(),
            end_time: self.end_time.clone(),
        }
    }
}

pub fn clone_trait() {
    println!("Clone Trait");

    let original_appointment = Appointment::new("Dr. Smith", "10:00 AM", "11:00 AM");
    let cloned_appointment = original_appointment.clone();

    println!(
        "Original Appointment: {}, from {} to {}",
        original_appointment.doctor, original_appointment.start_time, original_appointment.end_time
    );

    println!(
        "Cloned Appointment: {}, from {} to {}",
        cloned_appointment.doctor, cloned_appointment.start_time, cloned_appointment.end_time
    );
}
