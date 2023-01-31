export interface Point {
  x: number
  y: number
}

export enum DayOfWeek {
  Sunday,
  Monday,
  Tuesday,
  Wednesday,
  Thursday,
  Friday,
  Saturday,
}

export enum Role {
  Admin,
  Student,
  Professor,
}

export interface AcademicYears {
  academic_year_id: string
  academic_year_gregorian_year: string
  academic_year_start_timestamp: string
  academic_year_end_timestamp: string
  academic_year_created_timestamp: string
}

export interface Buildings {
  building_id: string
  building_name: string
  building_coordinates: Point
  building_created_timestamp: string
}

export interface Curriculums {
  curriculum_id: string
  faculty_id: string
  curriculum_name: string
  curriculum_created_timestamp: string
}

export interface Faculties {
  faculty_id: string
  faculty_name: string
  faculty_created_timestamp: string
}

export interface ForumCategories {
  forum_category_id: string
  forum_category_name: string
  forum_category_representative_id: string
  user_id: string
  forum_category_color_theme: string
  forum_category_created_timestamp: string
}

export interface ForumGlobalAnnouncements {
  forum_global_announcement_id: string
  forum_global_announcement_name: string
  user_id: string
  forum_global_announcement_content: string
  forum_global_announcement_is_active: boolean
  forum_global_announcement_created_timestamp: string
  forum_global_announcement_deactivated_timestamp?: string
}

export interface ForumPostReplies {
  forum_post_reply_id: string
  user_id: string
  forum_post_reply_content: string
  forum_post_reply_created_timestamp: string
}

export interface ForumPostViews {
  forum_post_id: string
  user_id: string
}

export interface ForumPosts {
  forum_post_id: string
  forum_post_name: string
  user_id: string
  forum_category_id: string
  forum_post_content: string
  forum_post_created_timestamp: string
  forum_post_is_category_based_announcement: boolean
}

export interface GradingCriteriaGrades {
  grading_criteria_grade_id: string
  grading_criteria_id: string
  grading_criteria_grade_alphabet: string
  grading_criteria_grade_minimum_score: number
}

export interface GradingCriterias {
  grading_criteria_id: string
  user_id: string
  grading_criteria_name: string
  grading_criteria_created_timestamp: string
}

export interface MajorCreditSpecifications {
  major_credit_specification_id: string
  major_id: string
  major_credit_specification_name: string
  major_credit_specification_minimum_credit: number
  major_credit_specification_created_timestamp: string
}

export interface MajorSubjects {
  major_credit_specification_id: string
  subject_id: string
}

export interface Majors {
  major_id: string
  major_representative_id: string
  curriculum_id: string
  major_name: string
  major_created_timestamp: string
}

export interface OpeningSubjectsInSemesterAdditionalEligibleStudents {
  semester_id: string
  subject_id: string
  additional_student_id: string
}

export interface OpeningSubjectsInSemesterEligibleMajors {
  semester_id: string
  subject_id: string
  major_id: string
  academic_year_id: string
}

export interface OpeningSubjectsInSemesterProfessors {
  semester_id: string
  subject_id: string
  professor_id: string
}

export interface OpeningSubjectsInSemesterSchedules {
  semester_id: string
  subject_id: string
  room_id: string
  day_of_week: DayOfWeek
  start_time_of_day: string
  end_time_of_day: string
}

export interface OpeningSubjectsInSemesterSubjectDescriptions {
  semester_id: string
  subject_id: string
  grading_criteria_id: string
  subject_capacity: number
  is_grade_released: boolean
}

export interface ProfessorNames {
  professor_name_id: string
  professor_id: string
  professor_name_language: string
  professor_first_name: string
  professor_middle_name: string
  professor_last_name: string
}

export interface Professors {
  professor_id: string
  professor_profile_image_path: string
  professor_birthdate: string
}

export interface Rooms {
  room_id: string
  building_id: string
  room_name: string
  room_capacity: number
  room_floor: number
}

export interface Semesters {
  semester_id: string
  academic_year_id: string
  semester_start_timestamp: string
  semester_end_timestamp: string
  semester_created_timestamp: string
}

export interface StudentAssignments {
  student_assignment_id: string
  student_assignment_name: string
  subject_id: string
  semester_id: string
  student_assignment_full_score: number
}

export interface StudentEnrollments {
  semester_id: string
  subject_id: string
  student_id: string
}

export interface StudentNames {
  student_name_id: string
  student_id: string
  student_name_language: string
  student_first_name: string
  student_middle_name: string
  student_last_name: string
}

export interface StudentScores {
  semester_id: string
  subject_id: string
  student_id: string
  assignment_id: string
  student_score: number
}

export interface StudentSubjectComments {
  semester_id: string
  subject_id: string
  student_id: string
  student_comment: string
}

export interface StudentTransactions {
  semester_id: string
  student_id: string
  student_transaction_id: string
  student_transaction_is_transaction_successful: boolean
  student_transaction_price: number
  student_transaction_created_timestamp: string
  student_transaction_finished_timestamp?: string
}

export interface Students {
  student_id: string
  student_representative_id: string
  student_profile_image_path: string
  student_nid: string
  student_birthdate: string
  student_previous_school_name: string
  student_previous_school_gpa: number
  major_id: string
  professor_id: string
  first_academic_year_id: string
}

export interface SubjectSchedules {
  subject_schedule_id: string
  subject_id: string
  subject_schedule_day_of_week: DayOfWeek
  subject_schedule_start_time_of_day: string
  subject_schedule_end_time_of_day: string
}

export interface Subjects {
  subject_id: string
  subject_name: string
  subject_description: string
  subject_credit: number
  subject_created_timestamp: string
}

export interface UserSessions {
  user_session_id: string
  user_session_user_id: string
  user_session_refresh_token: string
}

export interface Users {
  user_id: string
  user_username: string
  user_email: string
  user_password: string
  user_role: Role
  user_created_timestamp: string
}
