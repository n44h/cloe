struct Meeting {
    name: String,
    id: String,
    password: Option<String>
}

impl Meeting {
    fn new(name: &str, id: &str, password: Option<&str>) -> Meeting {
        // Cast the password to an Option<String>
        let casted_password = match password {
            Some(p) => Some(p.to_string()),
            None => None
        };

        Meeting {
            name: name.replace(' ', "-").to_lowercase(),
            id: id.replace("-", ""),
            password: casted_password
        }
    }
}

fn main() {
    // Testing Meeting struct
    let new_meeting = Meeting::new("Team Weekly Meeting", "884 955 5968", None);
    println!(
        "Meeting({}, {}, {})", 
        new_meeting.name, 
        new_meeting.id, 
        new_meeting.password.as_deref().unwrap_or("<none>")
    );
}