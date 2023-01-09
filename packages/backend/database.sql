-- create database ger;

-- extensions helping with searches.
create extension if not exists pgroonga;

drop type t_day_of_week cascade;
drop type t_user_role cascade;

-- types for day of week.
create type t_day_of_week as enum ('sunday', 'monday', 'tuesday', 'wednesday', 'thursday', 'friday', 'saturday');
create type t_user_role as enum ('user', 'student', 'professor');

drop table faculties cascade;
drop table curriculums cascade;
drop table majors cascade;
drop table major_credit_specifications cascade;
drop table major_subjects cascade;
drop table academic_years cascade;
drop table semesters cascade;
drop table buildings cascade;
drop table rooms cascade;
drop table students cascade;
drop table subject_schedules cascade;
drop table opening_subjects_in_semester cascade;
drop table opening_subjects_in_semester_professors cascade;
drop table opening_subjects_in_semester_subject_capacities;
drop table opening_subjects_in_semester_eligible_majors cascade;
drop table opening_subjects_in_semester_additional_eligible_students cascade;
drop table student_enrollments cascade;
drop table student_assignments cascade;
drop table student_scores cascade;
drop table professors cascade;
drop table users cascade;

-- available faculties in the university.
create table faculties (
    faculty_id text not null unique,
    faculty_name text not null,
    primary key (faculty_id)
);

-- available curriculums for studying in the university, such as:
-- - normal bachelor
-- - special bachelor (saturdays and sundays)
-- - master degree
--
-- this is directly tied to the faculties.
create table curriculums (
    curriculum_id text not null unique,
    faculty_id text not null,
    curriculum_name text not null,
    curriculum_created_timestamp timestamptz not null default now(),
    primary key (curriculum_id),
    foreign key (faculty_id) references faculties(faculty_id)
);

-- majors in the faculty that you could take in the curriculum.
create table majors (
    major_id text not null unique,
    curriculum_id text not null,
    major_name text not null,
    primary key (major_id),
    foreign key (curriculum_id) references curriculums(curriculum_id)
);

-- credit specifications for a major, this is how many credits you have to take
-- so that you can graduate.
create table major_credit_specifications (
    major_credit_specification_id text not null unique,
    major_id text not null,
    major_credit_specification_minimum_credit int not null,
    primary key (major_credit_specification_id),
    foreign key (major_id) references majors(major_id)
);

-- subjects in the major, this is grouped by the credit specification as a group
-- of subjects that you have to take.
create table major_subjects (
    major_subject_id text not null unique,
    major_credit_specification_id text not null,
    subject_id text not null,
    primary key (major_subject_id),
    foreign key (major_credit_specification_id) references major_credit_specifications(major_credit_specification_id),
    foreign key (subject_id) references subjects(subject_id)
);

-- years in the academic year
create table academic_years (
    academic_year_id text not null unique,
    academic_year_gregorian_year text not null unique default date_part('year', now())::text,
    curriculum_id text not null,
    academic_year_start_timestamp timestamptz not null default now(),
    academic_year_end_timestamp timestamptz not null default now(),
    primary key (academic_year_id)
);

-- available semesters in the year
create table semesters (
    semester_id text not null unique,
    academic_year_id text not null,
    semester_start_timestamp timestamptz not null default now(),
    semester_end_timestamp timestamptz not null default now(),
    primary key (semester_id),
    foreign key (academic_year_id) references academic_years(academic_year_id)
);

-- buildings in the uni
create table buildings (
    building_id text not null unique,
    building_name text not null,
    building_cordinates point not null,
    primary key (building_id)
);

-- rooms in the buildings
create table rooms (
    room_id text not null unique,
    building_id text not null,
    building_name text not null,
    room_capacity int not null default 0,
    primary key (room_id),
    foreign key (building_id) references buildings(building_id)
);

-- subjects opened for studying in the university
create table subjects (
    subject_id text not null unique,
    subject_name text not null,
    subject_credit int not null,
    subject_type text not null,
    primary key (subject_id)
);

-- one subject could span many hours during the week so this is required
create table subject_schedules (
    subject_schedule_id text not null unique,
    subject_id text not null,
    subject_schedule_day_of_week t_day_of_week not null,
    subject_schedule_start_time_of_day time not null,
    subject_schedule_end_time_of_day time not null,
    primary key (subject_schedule_id),
    foreign key (subject_id) references subjects(subject_id)
);

-- which subjects are opened for studying in each semester, this only stores the schedule
-- and the room in which each schedule is stored, these can be tracked back to a subject
-- and get the list of subjects in which they are open in the semester.
create table opening_subjects_in_semester (
    semester_id text not null references semesters(semester_id),
    subject_schedule_id text not null references subject_schedules(subject_schedule_id),
    room_id text not null references rooms(room_id),
    primary key (semester_id, subject_schedule_id, room_id)
);

-- stores which subjects is taught by which professors.
create table opening_subjects_in_semester_professors (
    semester_id text not null references semesters(semester_id),
    subject_id text not null references subjects(subject_id),
    professor_id text not null references professors(professor_id),
    primary key (semester_id, subject_id, professor_id)
);

-- how many students a subject is accepting in each semester and which students can enrol in the class
create table opening_subjects_in_semester_subject_capacities (
    semester_id text not null references semesters(semester_id),
    subject_id text not null references subjects(subject_id),
    subject_capacity int not null,
    primary key (semester_id, subject_id)
);

create table opening_subjects_in_semester_eligible_majors (
    semester_id text not null references semesters(semester_id),
    subject_id text not null references subjects(subject_id),
    major_id text not null references majors(major_id),
    academic_year_id text not null references academic_years(academic_year_id),
    primary key (semester_id, subject_id, major_id, academic_year_id)
);

create table opening_subjects_in_semester_additional_eligible_students (
    semester_id text not null references semesters(semester_id),
    subject_id text not null references subjects(subject_id),
    additional_student_id text not null references students(student_id),
    primary key (semester_id, subject_id, additional_student_id)
);

create table student_enrollments (
    semester_id text not null references semesters(semester_id),
    subject_id text not null references subjects(subject_id),
    student_id text not null references students(student_id),
    primary key (semester_id, subject_id, student_id)
);

create table student_assignments (
    student_assignment_id text not null unique,
    student_assignment_name text not null,
    subject_id text not null,
    semester_id text not null,
    student_assignment_full_score real not null,
    primary key (student_assignment_id)
);

create table student_scores (
    semester_id text not null references semesters(semester_id),
    subject_id text not null references subjects(subject_id),
    student_id text not null references students(student_id),
    assignment_id text not null references student_assignments(student_assignment_id),
    student_score real not null,
    primary key (semester_id, subject_id, student_id, assignment_id)
);

create table professors (
    professor_id text not null references users(user_id) unique,
    professor_profile_image_path text not null default '',
    primary key (professor_id)
);

create table professor_names (
    professor_name_id text not null unique,
    professor_id text not null,
    -- iso 639-1 language code
    professor_name_language text not null,
    professor_first_name text not null,
    professor_middle_name text not null,
    professor_last_name text not null,
    primary key (professor_name_id),
    foreign key (professor_id) references professors(professor_id)
);

create table users (
    user_id text not null unique,
    user_username text not null unique,
    user_email text not null unique,
    user_password text not null,
    user_role t_user_role not null,
    primary key (user_id)
);

-- students data
create table students (
    student_id text not null references users(user_id) unique,
    student_representative_id text not null unique,
    student_profile_image_path text not null default '',
    student_nid text not null,
    student_previous_school_name text not null,
    student_previous_school_gpa real not null,
    professor_id text not null,
    -- what year is the student's first academic year in the university
    first_academic_year_id text not null,
    primary key (user_id),
    foreign key (first_academic_year_id) references academic_years(academic_year_id),
    foreign key (professor_id) references professors(professor_id)
);

create table student_names (
    student_name_id text not null unique,
    student_id text not null,
    -- iso 639-1 language code
    student_name_language text not null,
    student_first_name text not null,
    student_middle_name text not null,
    student_last_name text not null,
    primary key (student_name_id),
    foreign key (student_id) references students(student_id)
);
