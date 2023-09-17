drop table if exists course;
create table course(
    course_id int primary key ,
    course_name nvarchar(30) ,
    description varchar(140) ,
    score int,
    time timestamp
);
insert into course values (1, '语文', '语文课，教文言文', 2,'2023-09-12 01:22:33');
insert into course values (2, '英语课', 'English', 3,'2023-09-12 02:12:33');

select * from course;
show tables;

insert into course values (null, 1, 'fourth course for teacher that teacher_id = 1', '2023-09-09 03:15:31');
insert into course values(null,1,'test course for teacher where teacher_id = 1','2023-09-10 16:10:49.771648');

select *
from course ;

update course set course_name='' and course_id=1;
update course set course_id = 1 , course_name='语文'
, description = '期末考试' , score = 2 , time = '2023-02-02 22:12:12' where course_id = 2