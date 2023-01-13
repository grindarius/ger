enum DayOfWeek {
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
}

pub enum Role {
    User,
    Student,
    Professor,
}

pub struct Students {
    pub student_id: String,
    pub student_representative_id: String,
    pub student_profile_image_path: String,
    pub student_nid: String,
    pub student_previous_school_name: String,
    pub student_previous_school_gpa: f64,
    pub professor_id: String,
    pub first_academic_year_id: String,
}

pub struct AcademicYears {
    pub academic_year_id: String,
    pub academic_year_gregorian_year: String,
    pub curriculum_id: String,
    pub academic_year_start_timestamp: time::OffsetDateTime,
    pub academic_year_end_timestamp: time::OffsetDateTime,
}

pub struct Semesters {
    pub semester_id: String,
    pub academic_year_id: String,
    pub semester_start_timestamp: time::OffsetDateTime,
    pub semester_end_timestamp: time::OffsetDateTime,
}

pub struct Faculties {
    pub faculty_id: String,
    pub faculty_name: String,
}

pub struct Users {
    pub user_id: String,
    pub user_username: String,
    pub user_email: String,
    pub user_password: String,
    pub user_role: what,
}

pub struct Majors {
    pub major_id: String,
    pub curriculum_id: String,
    pub major_name: String,
}

pub struct MajorCreditSpecifications {
    pub major_credit_specification_id: String,
    pub major_id: String,
    pub major_credit_specification_minimum_credit: i32,
}

pub struct SubjectSchedules {
    pub subject_schedule_id: String,
    pub subject_id: String,
    pub subject_schedule_day_of_week: DayOfWeek,
    pub subject_schedule_start_time_of_day: time::Time,
    pub subject_schedule_end_time_of_day: time::Time,
}

pub struct OpeningSubjectsInSemesterEligibleMajors {
    pub semester_id: String,
    pub subject_id: String,
    pub major_id: String,
    pub academic_year_id: String,
}

pub struct StudentAssignments {
    pub student_assignment_id: String,
    pub student_assignment_name: String,
    pub subject_id: String,
    pub semester_id: String,
    pub student_assignment_full_score: f64,
}

pub struct StudentNames {
    pub student_name_id: String,
    pub student_id: String,
    pub student_name_language: String,
    pub student_first_name: String,
    pub student_middle_name: String,
    pub student_last_name: String,
}

pub struct Professors {
    pub professor_id: String,
    pub professor_profile_image_path: String,
}

pub struct Curriculums {
    pub curriculum_id: String,
    pub faculty_id: String,
    pub curriculum_name: String,
    pub curriculum_created_timestamp: time::OffsetDateTime,
}

pub struct Buildings {
    pub building_id: String,
    pub building_name: String,
    pub building_cordinates: geo_types::Point<f64>,
}

pub struct Subjects {
    pub subject_id: String,
    pub subject_name: String,
    pub subject_credit: i32,
    pub subject_type: String,
}

pub struct OpeningSubjectsInSemesterSubjectCapacities {
    pub semester_id: String,
    pub subject_id: String,
    pub subject_capacity: i32,
}

pub struct OpeningSubjectsInSemesterAdditionalEligibleStudents {
    pub semester_id: String,
    pub subject_id: String,
    pub additional_student_id: String,
}

pub struct StudentScores {
    pub semester_id: String,
    pub subject_id: String,
    pub student_id: String,
    pub assignment_id: String,
    pub student_score: f64,
}

pub struct ProfessorNames {
    pub professor_name_id: String,
    pub professor_id: String,
    pub professor_name_language: String,
    pub professor_first_name: String,
    pub professor_middle_name: String,
    pub professor_last_name: String,
}

pub struct MajorSubjects {
    pub major_subject_id: String,
    pub major_credit_specification_id: String,
    pub subject_id: String,
}

pub struct Rooms {
    pub room_id: String,
    pub building_id: String,
    pub building_name: String,
    pub room_capacity: i32,
}

pub struct OpeningSubjectsInSemester {
    pub semester_id: String,
    pub subject_schedule_id: String,
    pub room_id: String,
}

pub struct OpeningSubjectsInSemesterProfessors {
    pub semester_id: String,
    pub subject_id: String,
    pub professor_id: String,
}

pub struct StudentEnrollments {
    pub semester_id: String,
    pub subject_id: String,
    pub student_id: String,
}

