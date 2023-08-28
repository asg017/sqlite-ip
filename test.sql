.load target/debug/libsqlite_ip sqlite3_ip_init

.mode box
.header on

create table demo as
select value
from json_each('[
  "0.0.0.1",
  "0.0.0.200",
  "0.0.0.29"
]');

select * from demo
order by 1;

select * from demo
order by 1 collate ip;
