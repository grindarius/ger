-- create database ger;

-- extensions helping with searches.
create extension if not exists pgroonga;

set timezone to 'Asia/Bangkok';

drop type t_day_of_week cascade;
drop type t_user_role cascade;

-- types for day of week.
create type t_day_of_week as enum ('sunday', 'monday', 'tuesday', 'wednesday', 'thursday', 'friday', 'saturday');
create type t_user_role as enum ('user', 'student', 'professor');

drop table faculties cascade;
drop table curriculums cascade;
drop table majors cascade;
drop table academic_years cascade;
drop table semesters cascade;
drop table buildings cascade;
drop table rooms cascade;
drop table users cascade;
drop table user_sessions cascade;
drop table grading_criterias cascade;
drop table grading_criteria_grades cascade;
drop table subjects cascade;
drop table subject_schedules cascade;
drop table students cascade;
drop table student_names cascade;
drop table professors cascade;
drop table professor_names cascade;
drop table major_credit_specifications cascade;
drop table major_subjects cascade;
drop table opening_subjects_in_semester_schedules cascade;
drop table opening_subjects_in_semester_professors cascade;
drop table opening_subjects_in_semester_subject_descriptions cascade;
drop table opening_subjects_in_semester_eligible_majors cascade;
drop table opening_subjects_in_semester_additional_eligible_students cascade;
drop table student_enrollments cascade;
drop table student_transactions cascade;
drop table student_subject_comments cascade;
drop table student_assignments cascade;
drop table student_scores cascade;

drop table forum_categories cascade;
drop table forum_posts cascade;
drop table forum_post_replies cascade;
drop table forum_post_views cascade;

-- available faculties in the university.
create table faculties (
    faculty_id text not null unique,
    faculty_name text not null,
    faculty_created_timestamp timestamptz not null default now(),
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
    major_representative_id text not null unique,
    curriculum_id text not null,
    major_name text not null,
    major_created_timestamp timestamptz not null default now(),
    primary key (major_id),
    foreign key (curriculum_id) references curriculums(curriculum_id)
);

-- years in the academic year
create table academic_years (
    academic_year_id text not null unique,
    academic_year_gregorian_year text not null unique default date_part('year', now()),
    academic_year_start_timestamp timestamptz not null default now(),
    academic_year_end_timestamp timestamptz not null default now(),
    academic_year_created_timestamp timestamptz not null default now(),
    primary key (academic_year_id)
);

-- available semesters in the year
create table semesters (
    semester_id text not null unique,
    academic_year_id text not null,
    semester_start_timestamp timestamptz not null default now(),
    semester_end_timestamp timestamptz not null default now(),
    semester_created_timestamp timestamptz not null default now(),
    primary key (semester_id),
    foreign key (academic_year_id) references academic_years(academic_year_id)
);

-- buildings in the uni
create table buildings (
    building_id text not null unique,
    building_name text not null,
    building_coordinates point not null,
    building_created_timestamp timestamptz not null,
    primary key (building_id)
);

-- rooms in the buildings
create table rooms (
    room_id text not null unique,
    building_id text not null,
    room_name text not null,
    room_capacity int not null default 0,
    -- this is 1-indexed
    room_floor smallint not null default 1,
    primary key (room_id),
    foreign key (building_id) references buildings(building_id)
);

create table users (
    user_id text not null unique,
    user_username text not null unique,
    user_email text not null unique,
    user_password text not null,
    user_role t_user_role not null,
    user_created_timestamp timestamptz not null default now(),
    primary key (user_id)
);

create table user_sessions (
    user_session_id text not null unique,
    user_session_user_id text not null unique,
    user_session_refresh_token text not null,
    primary key (user_session_id ),
    foreign key (user_session_user_id) references users(user_id)
);

create index if not exists pgroonga_users_index on users using pgroonga (
    user_username pgroonga_text_full_text_search_ops_v2
);

create table grading_criterias (
    grading_criteria_id text not null unique,
    user_id text not null,
    grading_criteria_name text not null,
    grading_criteria_created_timestamp timestamptz not null,
    primary key (grading_criteria_id),
    foreign key (user_id) references users(user_id)
);

create table grading_criteria_grades (
    grading_criteria_grade_id text not null unique,
    grading_criteria_id text not null,
    grading_criteria_grade_alphabet text not null,
    grading_criteria_grade_minimum_score int not null,
    primary key (grading_criteria_grade_id),
    foreign key (grading_criteria_id) references grading_criterias(grading_criteria_id)
);

-- subjects opened for studying in the university
create table subjects (
    subject_id text not null unique,
    subject_name text not null,
    subject_description text not null,
    subject_credit int not null,
    subject_created_timestamp timestamptz not null,
    primary key (subject_id)
);

create index if not exists pgroonga_subjects_index on subjects using pgroonga (
    subject_name pgroonga_text_full_text_search_ops_v2,
    subject_description pgroonga_text_full_text_search_ops_v2
);

create table subject_schedules (
    subject_schedule_id text not null unique,
    subject_id text not null,
    subject_schedule_day_of_week t_day_of_week not null,
    subject_schedule_start_time_of_day time not null,
    subject_schedule_end_time_of_day time not null,
    primary key (subject_schedule_id),
    foreign key (subject_id) references subjects(subject_id)
);

create table professors (
    professor_id text not null references users(user_id) unique,
    professor_profile_image_path text not null default '',
    professor_birthdate date not null,
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

create index if not exists pgroonga_professor_names_index on professor_names using pgroonga (
    professor_first_name pgroonga_text_full_text_search_ops_v2,
    professor_middle_name pgroonga_text_full_text_search_ops_v2,
    professor_last_name pgroonga_text_full_text_search_ops_v2
);

-- students data
create table students (
    student_id text not null references users(user_id) unique,
    student_representative_id text not null unique,
    student_profile_image_path text not null default '',
    student_nid text not null,
    student_birthdate date not null,
    student_previous_school_name text not null,
    student_previous_school_gpa real not null,
    major_id text not null references majors(major_id),
    professor_id text not null references professors(professor_id),
    -- what year is the student's first academic year in the university
    first_academic_year_id text not null references academic_years(academic_year_id),
    primary key (student_id)
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

create index if not exists pgroonga_student_names_index on student_names using pgroonga (
    student_first_name pgroonga_text_full_text_search_ops_v2,
    student_middle_name pgroonga_text_full_text_search_ops_v2,
    student_last_name pgroonga_text_full_text_search_ops_v2
);

-- credit specifications for a major, this is how many credits you have to take
-- so that you can graduate.
create table major_credit_specifications (
    major_credit_specification_id text not null unique,
    major_id text not null,
    major_credit_specification_name text not null,
    major_credit_specification_minimum_credit int not null,
    major_credit_specification_created_timestamp timestamptz not null,
    primary key (major_credit_specification_id),
    foreign key (major_id) references majors(major_id)
);

-- subjects in the major, this is grouped by the credit specification as a group
-- of subjects that you have to take.
create table major_subjects (
    major_credit_specification_id text not null references major_credit_specifications(major_credit_specification_id),
    subject_id text not null references subjects(subject_id),
    primary key (major_credit_specification_id, subject_id)
);

-- which subjects are opened for studying in each semester, this only stores the schedule
-- and the room in which each schedule is stored, these can be tracked back to a subject
-- and get the list of subjects in which they are open in the semester.
create table opening_subjects_in_semester_schedules (
    semester_id text not null references semesters(semester_id),
    subject_id text not null references subjects(subject_id),
    room_id text not null references rooms(room_id),
    day_of_week t_day_of_week not null,
    start_time_of_day time not null,
    end_time_of_day time not null,
    primary key (semester_id, subject_id, room_id)
);

-- stores which subjects is taught by which professors.
create table opening_subjects_in_semester_professors (
    semester_id text not null references semesters(semester_id),
    subject_id text not null references subjects(subject_id),
    professor_id text not null references professors(professor_id),
    primary key (semester_id, subject_id, professor_id)
);

-- how many students a subject is accepting in each semester and which students can enrol in the class
create table opening_subjects_in_semester_subject_descriptions (
    semester_id text not null references semesters(semester_id),
    subject_id text not null references subjects(subject_id),
    grading_criteria_id text not null references grading_criterias(grading_criteria_id),
    subject_capacity int not null,
    is_grade_released boolean not null default false,
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

create table student_transactions (
    semester_id text not null references semesters(semester_id),
    student_id text not null references students(student_id),
    student_transaction_id text not null unique,
    student_transaction_is_transaction_successful boolean not null default false,
    student_transaction_price real not null,
    student_transaction_created_timestamp timestamptz not null default now(),
    student_transaction_finished_timestamp timestamptz,
    primary key (semester_id, student_id)
);

create table student_subject_comments (
    semester_id text not null references semesters(semester_id),
    subject_id text not null references subjects(subject_id),
    student_id text not null references students(student_id),
    student_comment text not null default '',
    primary key (semester_id, subject_id, student_id)
);

create index if not exists pgroonga_student_subject_comments_index on student_subject_comments using pgroonga (
    student_comment pgroonga_text_full_text_search_ops_v2
);

create table student_assignments (
    student_assignment_id text not null unique,
    student_assignment_name text not null,
    subject_id text not null references subjects(subject_id),
    semester_id text not null references semesters(semester_id),
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

create table forum_categories (
    forum_category_id text not null unique,
    forum_category_name text not null unique,
    forum_category_representative_id text not null unique,
    user_id text not null,
    forum_category_color_theme text not null default '#000000',
    forum_category_created_timestamp timestamptz not null default now(),
    primary key (forum_category_id),
    foreign key (user_id) references users(user_id)
);

alter table forum_categories add constraint color_hex_constraint check (forum_category_color_theme ~* '^#[a-fA-F0-9]{6}$');

create table forum_posts (
    forum_post_id text not null unique,
    forum_post_name text not null,
    forum_category_id text not null,
    forum_post_content text not null,
    forum_post_created_timestamp timestamptz not null default now(),
    forum_post_is_channel_based_announcement boolean not null default false,
    forum_post_is_global_announcement boolean not null default false,
    primary key (forum_post_id),
    foreign key (forum_category_id) references forum_categories(forum_category_id)
);

create table forum_post_replies (
    forum_post_reply_id text not null unique,
    user_id text not null,
    forum_post_reply_content text not null default '',
    forum_post_reply_created_timestamp timestamptz not null default now(),
    primary key (forum_post_reply_id),
    foreign key (user_id) references users(user_id)
);

create table forum_post_views (
    forum_post_id text not null references forum_posts(forum_post_id),
    user_id text not null references users(user_id),
    primary key (forum_post_id, user_id)
);
