pub struct IdBuilder;

impl IdBuilder {
    pub fn leave_id(emp_id: &str, date: String, counter: usize) -> String {
        let date_split: Vec<&str> = date.split(" ").collect();
        let date_split_dash: Vec<&str> = date_split[0].split("-").collect();

        let date_string = date_split_dash.join("");

        return format!("{}{}{}", date_string, emp_id, counter).to_string();
    }
}
