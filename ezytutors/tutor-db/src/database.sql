drop table if exists ezy_course_c4;
create table ezy_course_c4
(
    course_id serial primary key,
    tutor_id INT not null,
    course_name varchar(140) not null,
    post_time TIMESTAMP default now()
);

insert into ezy_course_c4
(course_id, tutor_id, course_name, post_time)
values(1, 1, 'First course', '2025-06-13 09:53:12');
insert into ezy_course_c4
(course_id, tutor_id, course_name, post_time)
values(2, 1, 'Second course', '2025-06-15 10:13:33');