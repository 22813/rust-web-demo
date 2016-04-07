CREATE SEQUENCE task_seq
  INCREMENT 1
  MINVALUE 1
  MAXVALUE 10000000
  START 1
  CACHE 1;

CREATE TABLE task
(
      id integer NOT NULL DEFAULT nextval('task_seq'::regclass),
      name character varying(30),
      content character varying(300),
      create_time timestamp with time zone,
      update_time timestamp with time zone,
      status integer NOT NULL DEFAULT 0
);

CREATE SEQUENCE public.account_seq
  INCREMENT 1
  MINVALUE 1
  MAXVALUE 10000000
  START 1
  CACHE 1;


CREATE TABLE account
(
      id integer NOT NULL DEFAULT nextval('account_seq'::regclass),
      name character varying(30),
      password character varying(100)
);

insert into account(name,password) values ('admin','admin');
