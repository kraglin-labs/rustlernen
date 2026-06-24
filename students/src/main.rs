mod input_utils;
use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::Write;
use std::fs;


#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Students {
    name: String,
    age: u8,
    matric_number: u32,
    courses: Vec<Courses>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Courses {
    name: String,
    code: String,
    score: Vec<Tests>,
}
#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Tests {
    first_test: i32,
    second_test: i32,
    exam: i32,
}

impl Students {
    fn new(name: &str, age: u8, matric_number: u32) -> Self {
        Self {
            name: name.to_string(),
            age,
            matric_number,
            courses: Vec::new(),
        }
    }

    fn add_course(&mut self, course: Courses) {
        self.courses.push(course);
    }

    fn calculate_cgpa(&self) -> f32 {
        if self.courses.is_empty() {
            return 0.0;
        }
        let mut total_score_sum = 0.0;
        let mut course_count = 0.0;

        for course in &self.courses {
            total_score_sum += course.get_raw_score();
            course_count += 1.0;
        }

        let average = total_score_sum / course_count;
        average / 20.0
    }
}

impl Courses {
    fn new(name: &str, code: &str) -> Self {
        Self {
            name: name.to_string(),
            code: code.to_string(),
            score: Vec::new(),
        }
    }
    
    fn add_test(&mut self, test: Tests) {
        self.score.push(test)
    }

    fn get_raw_score(&self) -> f32 {
        match self.score.first() {
            Some(test) => test.total_score() as f32,
            None => 0.0,
        }
    }

}

impl Tests {
    fn new(first_test: i32, second_test: i32, exam: i32) -> Self {
        Self {
            first_test,
            second_test,
            exam,
        }
    }
    fn total_score(&self) -> i32 {
        self.first_test + self.second_test + self.exam
    }

}
fn save_to_file(students: &Vec<Students>) -> std::io::Result<()> {
    let json_string = serde_json::to_string_pretty(students).unwrap();
    let file = File::create("student_data.json");

    file.expect("REASON").write_all(json_string.as_bytes())?;
    println!("Saved to student_data.json");
    Ok(())
}

fn load_from_file() -> Vec<Students> {
    let data = match fs::read_to_string("student_data.json") {
        Ok(content) => content,
        Err(_) => return Vec::new(),
    };

    serde_json::from_str(&data).unwrap_or_else(|_| Vec::new())
}

#[allow(dead_code)]
fn edit_result(students: &mut Vec<Students>, matric: u32, course_code: &str, new_exam_score: i32) {
    if let Some(student) = students.iter_mut().find(|s| s.matric_number == matric) {
        if let Some(course) = student.courses.iter_mut().find(|c| c.code == course_code) {
            if let Some(test) = course.score.first_mut() {
                test.exam = new_exam_score;
                println!("Updated {}'s {} exam score to {}", student.name, course_code, new_exam_score);
            }
        }
    } else {
        println!("Student not found")
    }
}











fn main() {
    let mut students: Vec<Students> = load_from_file();

    let mth101 = Courses::new("Elementary Mathematics I", "MTH101");
    let phy101 = Courses::new("General Physics I", "PHY101");
    let chm101 = Courses::new("General Chemistry I", "CHM101");
    let bio101 = Courses::new("Biology For Physical Sciences", "BIO101");
    let eng101 = Courses::new("Workshop Practice I", "ENG101");
    let eng103 = Courses::new("Engineering Drawing I", "ENG103");
    let gst101 = Courses::new("Use of English I", "GST101");
    let gst103 = Courses::new("Humanities", "GST103");

    let first_year_courses = vec![mth101, phy101, chm101, bio101, eng101, eng103, gst101, gst103];

    loop {
        println!("Enter student name (or type 'exit' to finish):");
        let mut name = String::new();
        scan!(&mut name);

        if name.trim().eq_ignore_ascii_case("exit") {
            break;
        }

        let mut age: u8 = 0;
        loop {
            println!("Enter student age:");
            scan!(&mut age);
            if age >= 16 && age <= 100 {
                break; 
            }
            println!("Age must be between 16 and 100. Please try again:");
        }

        let mut matric_number: u32 = 0;
        loop {
            println!("Enter student's matriculation number:");
            scan!(&mut matric_number);
            if matric_number <= 999999 && matric_number >= 100000 {
                break; 
            }
            println!("Matric number must be exactly 6 digits. Please try again:");
        }

        let mut student = Students::new(&name, age, matric_number);
        
        for course in &first_year_courses {
            let mut subject = course.clone();
            let mut first_test: i32;
            let mut second_test: i32;
            let mut exam: i32;

            println!("Enter scores for {} ({}):", subject.name, subject.code);
            loop {
                first_test = -1;
                second_test = -1;
                exam = -1;

                scan!(&mut first_test, &mut second_test, &mut exam);

                if first_test < 0 || first_test > 20 || second_test < 0 || second_test > 20 || exam < 0 || exam > 60 {
                    println!("Invalid scores entered. Please re-enter:");
                    continue;
                }
                
                break;
            }

            let test = Tests::new(first_test, second_test, exam);
            subject.add_test(test);
            student.add_course(subject);

        }
        students.push(student.clone());

    }

    
    println!("\nHHHHHHHHHHHHHHH RECORDA HHHHHHHHHHHHHHHHHHHHHHHHHH");
    for student in &students {
        println!("-------------------------------------------------");
        println!("Name:          {}", student.name);
        println!("Age:           {}", student.age);
        println!("Matric Number: {}", student.matric_number);
        println!("Calculated CGPA: {:.2} / 5.0", student.calculate_cgpa());
    }
    println!("HHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHH");
    if let Err(e) = save_to_file(&students) {
        println!("Failed to save data: {}", e);
    }
}