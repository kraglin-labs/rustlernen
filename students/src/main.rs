mod input_utils;

#[allow(dead_code)]
#[derive(Debug, Clone)]
struct Students {
    name: String,
    age: u8,
    matric_number: u32,
    courses: Vec<Courses>,
}

#[derive(Debug, Clone)]
struct Courses {
    name: String,
    code: String,
    score: Vec<Tests>,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
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

fn main() {
    let mut students: Vec<Students> = Vec::new();

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

        println!("Enter student age:");
        let mut age: u8 = 0;
        scan!(&mut age);
        if age < 16 || age > 100 {
            println!("Age must be between 16 and 100");
            continue;
        }

        println!("Enter student's matriculation number:");
        let mut matric_number: u32 = 0;
        scan!(&mut matric_number);
        if *&matric_number > 999999 || *&matric_number < 100000 {
            println!("Matric number must be 6 digits");
            continue;
        }

        let mut student = Students::new(&name, age, matric_number);
        
        for course in &first_year_courses {
            let mut subject = course.clone();
            let mut first_test: i32 = 0;
            let mut second_test: i32 = 0;
            let mut exam: i32 = 0;
            println!("Enter scores for {} ({}):", subject.name, subject.code);
            loop {
                scan!(&mut first_test, &mut second_test, &mut exam);
                if first_test < 0 || first_test > 20 {
                    println!("First test score must be between 0 and 20. Please re-enter:");
                    continue;
                }
                if second_test < 0 || second_test > 20 {
                    println!("Second test score must be between 0 and 20. Please re-enter:");
                    continue;
                }
                if exam < 0 || exam > 60 {
                    println!("Exam score must be between 0 and 60. Please re-enter:");
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

    println!("\n================ STUDENT RECORDS ================");
    for student in &students {
        println!("-------------------------------------------------");
        println!("Name:          {}", student.name);
        println!("Age:           {}", student.age);
        println!("Matric Number: {}", student.matric_number);
        println!("Calculated CGPA: {:.2} / 5.0", student.calculate_cgpa());
    }
    println!("=================================================");
}