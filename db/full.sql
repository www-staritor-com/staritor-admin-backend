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
    comment '用户信息表';

drop table if exists t_resource;
create table t_resource
(
    id               bigint auto_increment comment '主键'
        primary key,
    title            varchar(256)                       not null comment '标题名',
    category         varchar(32)                        not null comment '资源类别',
    tags             varchar(512)                       null comment '标签，'',''分割',
    url              text                               not null comment '资源路径',
    sort             int      default 0                 null comment '顺序',
    created_datetime datetime default CURRENT_TIMESTAMP not null comment '创建时间',
    created_user     varchar(64)                        null comment '创建人员',
    updated_datetime datetime default CURRENT_TIMESTAMP not null on update CURRENT_TIMESTAMP comment '更新时间',
    updated_user     varchar(64)                        null comment '更新人'
)
    comment '资源表';