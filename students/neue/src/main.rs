use serde::{Deserialize, Serialize};
use std::fs;

// ========== DATA STRUCTURES ==========
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Student {
    name: String,
    age: u8,
    matric_number: u32,
    courses: Vec<Course>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Course {
    name: String,
    code: String,
    credit_units: u8,
    scores: Scores,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Scores {
    first_test: u8,
    second_test: u8,
    exam: u8,
}

// ========== IMPLEMENTATIONS ==========
impl Scores {
    fn new(first: u8, second: u8, exam: u8) -> Result<Self, String> {
        if first > 20 || second > 20 || exam > 60 {
            return Err(format!(
                "Invalid scores: first_test={}, second_test={}, exam={}. Must be <= 20, <=20, <=60",
                first, second, exam
            ));
        }
        Ok(Self { first_test: first, second_test: second, exam })
    }
    
    fn total(&self) -> u8 {
        self.first_test + self.second_test + self.exam
    }
    
    fn percentage(&self) -> f32 {
        (self.total() as f32 / 100.0) * 100.0
    }
}

impl Course {
    fn new(name: &str, code: &str, credit_units: u8, scores: Scores) -> Self {
        Self {
            name: name.to_string(),
            code: code.to_string(),
            credit_units,
            scores,
        }
    }
    
    fn grade_point(&self) -> f32 {
        let pct = self.scores.percentage();
        match pct {
            70.0..=100.0 => 5.0,
            60.0..=69.0 => 4.0,
            50.0..=59.0 => 3.0,
            45.0..=49.0 => 2.0,
            40.0..=44.0 => 1.0,
            _ => 0.0,
        }
    }
    
    fn letter_grade(&self) -> char {
        let pct = self.scores.percentage();
        match pct {
            70.0..=100.0 => 'A',
            60.0..=69.0 => 'B',
            50.0..=59.0 => 'C',
            45.0..=49.0 => 'D',
            40.0..=44.0 => 'E',
            _ => 'F',
        }
    }
}

impl Student {
    fn new(name: &str, age: u8, matric_number: u32) -> Self {
        Self {
            name: name.to_string(),
            age,
            matric_number,
            courses: Vec::new(),
        }
    }
    
    fn add_course(&mut self, course: Course) {
        self.courses.push(course);
    }
    
    fn calculate_cgpa(&self) -> f32 {
        if self.courses.is_empty() {
            return 0.0;
        }
        
        let (total_points, total_credits) = self.courses.iter()
            .map(|c| (c.grade_point(), c.credit_units as f32))
            .fold((0.0, 0.0), |(points, credits), (gp, cu)| {
                (points + gp * cu, credits + cu)
            });
        
        total_points / total_credits
    }
    
    fn classification(&self) -> &'static str {
        let cgpa = self.calculate_cgpa();
        match cgpa {
            4.50..=5.00 => "First Class Honours",
            3.50..=4.49 => "Second Class Honours (Upper)",
            2.50..=3.49 => "Second Class Honours (Lower)",
            2.00..=2.49 => "Third Class Honours",
            _ => "Pass",
        }
    }
}

// ========== MAIN PROGRAM ==========
fn main() {
    // Load data from JSON file (assuming you saved it)
    let students = load_students_from_json("students.json").unwrap_or_else(|e| {
        eprintln!("Error loading data: {}", e);
        Vec::new()
    });
    
    // Display all students
    for student in &students {
        println!("========================");
        println!("Name: {}", student.name);
        println!("Matric: {}", student.matric_number);
        println!("Age: {}", student.age);
        println!("CGPA: {:.2}", student.calculate_cgpa());
        println!("Classification: {}", student.classification());
        println!("--- Course Breakdown ---");
        for course in &student.courses {
            println!(
                "{}: {}% (Grade: {})", 
                course.code,
                course.scores.percentage() as u8,
                course.letter_grade()
            );
        }
    }
    
    // Find top performer
    if let Some(top) = students.iter().max_by(|a, b| {
        a.calculate_cgpa().partial_cmp(&b.calculate_cgpa()).unwrap()
    }) {
        println!("\n🏆 TOP PERFORMER: {} with CGPA {:.2}", top.name, top.calculate_cgpa());
    }
    
    // Calculate class average
    let class_avg = students.iter()
        .map(|s| s.calculate_cgpa())
        .sum::<f32>() / students.len() as f32;
    println!("📊 Class Average CGPA: {:.2}", class_avg);
}

// ========== FILE I/O (Bonus) ==========
fn load_students_from_json(filename: &str) -> Result<Vec<Student>, Box<dyn std::error::Error>> {
    let data = fs::read_to_string(filename)?;
    let students: Vec<Student> = serde_json::from_str(&data)?;
    Ok(students)
}

fn save_students_to_json(students: &[Student], filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    let json = serde_json::to_string_pretty(students)?;
    fs::write(filename, json)?;
    Ok(())
}