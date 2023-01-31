#[derive(ger_from_row::FromRow)]
pub enum DayOfWeek {
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
}

#[derive(ger_from_row::FromRow)]
pub enum Role {
    User,
    Student,
    Professor,
}

impl Role {
    pub fn as_str<'lt>(self: &Self) -> &'lt str {
        match self {
            Self::User => "user",
            Self::Student => "student",
            Self::Professor => "professor",
        }
    }
}

impl DayOfWeek {
    pub fn as_str<'lt>(self: &Self) -> &'lt str {
        match self {
            Self::Sunday => "sunday",
            Self::Monday => "monday",
            Self::Tuesday => "tuesday",
            Self::Wednesday => "wednesday",
            Self::Thursday => "thursday",
            Self::Friday => "friday",
            Self::Saturday => "saturday",
        }
    }
}

#[derive(ger_from_row::FromRow)]
pub struct AcademicYears {
    pub academic_year_id: String,
    pub academic_year_gregorian_year: String,
    pub academic_year_start_timestamp: time::OffsetDateTime,
    pub academic_year_end_timestamp: time::OffsetDateTime,
    pub academic_year_created_timestamp: time::OffsetDateTime,
}

#[derive(ger_from_row::FromRow)]
pub struct Buildings {
    pub building_id: String,
    pub building_name: String,
    pub building_coordinates: geo_types::Point<f64>,
    pub building_created_timestamp: time::OffsetDateTime,
}

#[derive(ger_from_row::FromRow)]
pub struct Curriculums {
    pub curriculum_id: String,
    pub faculty_id: String,
    pub curriculum_name: String,
    pub curriculum_created_timestamp: time::OffsetDateTime,
}

#[derive(ger_from_row::FromRow)]
pub struct Faculties {
    pub faculty_id: String,
    pub faculty_name: String,
    pub faculty_created_timestamp: time::OffsetDateTime,
}

#[derive(ger_from_row::FromRow)]
pub struct ForumCategories {
    pub forum_category_id: String,
    pub forum_category_name: String,
    pub forum_category_representative_id: String,
    pub user_id: String,
    pub forum_category_color_theme: String,
    pub forum_category_created_timestamp: time::OffsetDateTime,
}

#[derive(ger_from_row::FromRow)]
pub struct ForumGlobalAnnouncements {
    pub forum_global_announcement_id: String,
    pub forum_global_announcement_name: String,
    pub user_id: String,
    pub forum_global_announcement_content: String,
    pub forum_global_announcement_is_active: bool,
    pub forum_global_announcement_created_timestamp: time::OffsetDateTime,
    pub forum_global_announcement_deactivated_timestamp: Option<time::OffsetDateTime>,
}

#[derive(ger_from_row::FromRow)]
pub struct ForumPostReplies {
    pub forum_post_reply_id: String,
    pub user_id: String,
    pub forum_post_reply_content: String,
    pub forum_post_reply_created_timestamp: time::OffsetDateTime,
}

#[derive(ger_from_row::FromRow)]
pub struct ForumPostViews {
    pub forum_post_id: String,
    pub user_id: String,
}

#[derive(ger_from_row::FromRow)]
pub struct ForumPosts {
    pub forum_post_id: String,
    pub forum_post_name: String,
    pub user_id: String,
    pub forum_category_id: String,
    pub forum_post_content: String,
    pub forum_post_created_timestamp: time::OffsetDateTime,
    pub forum_post_is_category_based_announcement: bool,
}

#[derive(ger_from_row::FromRow)]
pub struct GradingCriteriaGrades {
    pub grading_criteria_grade_id: String,
    pub grading_criteria_id: String,
    pub grading_criteria_grade_alphabet: String,
    pub grading_criteria_grade_minimum_score: f64,
}

#[derive(ger_from_row::FromRow)]
pub struct GradingCriterias {
    pub grading_criteria_id: String,
    pub user_id: String,
    pub grading_criteria_name: String,
    pub grading_criteria_created_timestamp: time::OffsetDateTime,
}

#[derive(ger_from_row::FromRow)]
pub struct MajorCreditSpecifications {
    pub major_credit_specification_id: String,
    pub major_id: String,
    pub major_credit_specification_name: String,
    pub major_credit_specification_minimum_credit: i32,
    pub major_credit_specification_created_timestamp: time::OffsetDateTime,
}

#[derive(ger_from_row::FromRow)]
pub struct MajorSubjects {
    pub major_credit_specification_id: String,
    pub subject_id: String,
}

#[derive(ger_from_row::FromRow)]
pub struct Majors {
    pub major_id: String,
    pub major_representative_id: String,
    pub curriculum_id: String,
    pub major_name: String,
    pub major_created_timestamp: time::OffsetDateTime,
}

#[derive(ger_from_row::FromRow)]
pub struct OpeningSubjectsInSemesterAdditionalEligibleStudents {
    pub semester_id: String,
    pub subject_id: String,
    pub additional_student_id: String,
}

#[derive(ger_from_row::FromRow)]
pub struct OpeningSubjectsInSemesterEligibleMajors {
    pub semester_id: String,
    pub subject_id: String,
    pub major_id: String,
    pub academic_year_id: String,
}

#[derive(ger_from_row::FromRow)]
pub struct OpeningSubjectsInSemesterProfessors {
    pub semester_id: String,
    pub subject_id: String,
    pub professor_id: String,
}

#[derive(ger_from_row::FromRow)]
pub struct OpeningSubjectsInSemesterSchedules {
    pub semester_id: String,
    pub subject_id: String,
    pub room_id: String,
    #[fromrow(num)]
    pub day_of_week: DayOfWeek,
    pub start_time_of_day: time::Time,
    pub end_time_of_day: time::Time,
}

#[derive(ger_from_row::FromRow)]
pub struct OpeningSubjectsInSemesterSubjectDescriptions {
    pub semester_id: String,
    pub subject_id: String,
    pub grading_criteria_id: String,
    pub subject_capacity: i32,
    pub is_grade_released: bool,
}

#[derive(ger_from_row::FromRow)]
pub struct ProfessorNames {
    pub professor_name_id: String,
    pub professor_id: String,
    pub professor_name_language: String,
    pub professor_first_name: String,
    pub professor_middle_name: String,
    pub professor_last_name: String,
}

#[derive(ger_from_row::FromRow)]
pub struct Professors {
    pub professor_id: String,
    pub professor_profile_image_path: String,
    pub professor_birthdate: time::Date,
}

#[derive(ger_from_row::FromRow)]
pub struct Rooms {
    pub room_id: String,
    pub building_id: String,
    pub room_name: String,
    pub room_capacity: i32,
    pub room_floor: i16,
}

#[derive(ger_from_row::FromRow)]
pub struct Semesters {
    pub semester_id: String,
    pub academic_year_id: String,
    pub semester_start_timestamp: time::OffsetDateTime,
    pub semester_end_timestamp: time::OffsetDateTime,
    pub semester_created_timestamp: time::OffsetDateTime,
}

#[derive(ger_from_row::FromRow)]
pub struct StudentAssignments {
    pub student_assignment_id: String,
    pub student_assignment_name: String,
    pub subject_id: String,
    pub semester_id: String,
    pub student_assignment_full_score: f64,
}

#[derive(ger_from_row::FromRow)]
pub struct StudentEnrollments {
    pub semester_id: String,
    pub subject_id: String,
    pub student_id: String,
}

#[derive(ger_from_row::FromRow)]
pub struct StudentNames {
    pub student_name_id: String,
    pub student_id: String,
    pub student_name_language: String,
    pub student_first_name: String,
    pub student_middle_name: String,
    pub student_last_name: String,
}

#[derive(ger_from_row::FromRow)]
pub struct StudentScores {
    pub semester_id: String,
    pub subject_id: String,
    pub student_id: String,
    pub assignment_id: String,
    pub student_score: f64,
}

#[derive(ger_from_row::FromRow)]
pub struct StudentSubjectComments {
    pub semester_id: String,
    pub subject_id: String,
    pub student_id: String,
    pub student_comment: String,
}

#[derive(ger_from_row::FromRow)]
pub struct StudentTransactions {
    pub semester_id: String,
    pub student_id: String,
    pub student_transaction_id: String,
    pub student_transaction_is_transaction_successful: bool,
    pub student_transaction_price: f64,
    pub student_transaction_created_timestamp: time::OffsetDateTime,
    pub student_transaction_finished_timestamp: Option<time::OffsetDateTime>,
}

#[derive(ger_from_row::FromRow)]
pub struct Students {
    pub student_id: String,
    pub student_representative_id: String,
    pub student_profile_image_path: String,
    pub student_nid: String,
    pub student_birthdate: time::Date,
    pub student_previous_school_name: String,
    pub student_previous_school_gpa: f64,
    pub major_id: String,
    pub professor_id: String,
    pub first_academic_year_id: String,
}

#[derive(ger_from_row::FromRow)]
pub struct SubjectSchedules {
    pub subject_schedule_id: String,
    pub subject_id: String,
    #[fromrow(num)]
    pub subject_schedule_day_of_week: DayOfWeek,
    pub subject_schedule_start_time_of_day: time::Time,
    pub subject_schedule_end_time_of_day: time::Time,
}

#[derive(ger_from_row::FromRow)]
pub struct Subjects {
    pub subject_id: String,
    pub subject_name: String,
    pub subject_description: String,
    pub subject_credit: i32,
    pub subject_created_timestamp: time::OffsetDateTime,
}

#[derive(ger_from_row::FromRow)]
pub struct UserSessions {
    pub user_session_id: String,
    pub user_session_user_id: String,
    pub user_session_refresh_token: String,
}

#[derive(ger_from_row::FromRow)]
pub struct Users {
    pub user_id: String,
    pub user_username: String,
    pub user_email: String,
    pub user_password: String,
    #[fromrow(num)]
    pub user_role: Role,
    pub user_created_timestamp: time::OffsetDateTime,
}
