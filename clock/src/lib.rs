#[derive(Debug, PartialEq)]
pub struct Clock {
    hrs: i32,
    min: i32,
}

pub fn time_format(hours: i32, minutes: i32) -> (i32, i32) {
    let day = 24 * 60;
    let calculation = if hours * 60 + minutes >= 0
    {
        (hours * 60 + minutes) % day
    } else
    {
        (hours * 60 + minutes) % day + day
    };
    (calculation / 60, calculation % 60)
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let (h, m) = time_format(hours, minutes);
        Clock { hrs: h, min: m }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let (h, m) = time_format(self.hrs, self.min + minutes);
        Clock { hrs: h, min: m }
    }

    pub fn to_string(&self) -> String {
        format!("{:02}:{:02}", self.hrs, self.min)
    }
}
