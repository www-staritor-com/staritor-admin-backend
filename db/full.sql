create schema staritor;

use staritor;
drop table if exists t_user_info;
create table t_user_info
(
    id               bigint auto_increment comment '主键'
        primary key,
    code             varchar(64)              not null comment '账号',
    password         varchar(64)              not null comment '密码',
    name             varchar(32)              not null comment '用户名',
    deleted          tinyint(1) default false not null comment '是否删除',
    created_datetime datetime   default now() not null comment '创建时间',
    updated_datetime datetime   default now() not null on update now() comment '更新时间'
)
    comment '用户信息';

