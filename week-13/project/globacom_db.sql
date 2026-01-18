--
-- PostgreSQL database dump
--

\restrict ZKUgD77BMbefvFK8kgtXyI6pq5gRr1lxcNsV6Z7lj48mR7GxydyH5zakpRe1DnM

-- Dumped from database version 18.1
-- Dumped by pg_dump version 18.1

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET transaction_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- Name: customer; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.customer (
    c_id integer NOT NULL,
    c_name text,
    c_age integer,
    c_email text,
    c_mobile character varying(15),
    eid integer,
    data_id integer
);


ALTER TABLE public.customer OWNER TO postgres;

--
-- Name: dataplan; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.dataplan (
    data_id integer NOT NULL,
    data_size character varying(20),
    data_duration integer,
    data_price integer
);


ALTER TABLE public.dataplan OWNER TO postgres;

--
-- Name: department; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.department (
    dept_manegrid integer NOT NULL,
    dno integer NOT NULL,
    dname character(15) NOT NULL,
    dlocation character(15) NOT NULL,
    pno integer NOT NULL
);


ALTER TABLE public.department OWNER TO postgres;

--
-- Name: project; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.project (
    proj_manegrid integer NOT NULL,
    pno integer NOT NULL,
    pname character(5) NOT NULL,
    pduration character(15) NOT NULL
);


ALTER TABLE public.project OWNER TO postgres;

--
-- Name: staff; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.staff (
    staff_id integer CONSTRAINT employee_eid_not_null NOT NULL,
    staff_name character(25) CONSTRAINT employee_ename_not_null NOT NULL,
    dno character(5) CONSTRAINT employee_dno_not_null NOT NULL,
    staff_sal real CONSTRAINT employee_esal_not_null NOT NULL,
    age integer CONSTRAINT employee_age_not_null NOT NULL,
    mobile character varying(15) NOT NULL
);


ALTER TABLE public.staff OWNER TO postgres;

--
-- Data for Name: customer; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.customer (c_id, c_name, c_age, c_email, c_mobile, eid, data_id) FROM stdin;
110	Musta Karim	35	m_karim@gmail.com	08055089112	102	5
111	Lilian Jaiya	43	l_jaiye@gmail.com	08055185341	100	3
112	Arthur Musa	50	a_musa@gmail.com	07055282813	107	10
113	Philip Akonjo	41	p_akonjo@gmail.com	09052356772	100	2
114	Marylene Mapa	33	m_mapa@gmail.com	08053333551	120	5
115	Oghenero Agor	50	o_agor@gmail.com	07055566774	117	11
116	Adams Bree	33	a_bree@gmail.com	08056765424	102	1
117	Okafor Mathias	45	o_mathias@gmail.com	08056763367	120	10
118	Samson Adeleke	65	s_adeleke@gmail.com	07056774423	117	11
119	Lawal Tamire	35	l_tamire@gmail.com	09052111101	107	5
120	James Job	44	j_job@gmail.com	08059693919	100	8
121	Matthew Jakande	21	m_jakande@gmail.com	07051232144	120	2
122	Jimila Adegboye	20	j_adegboye@gmail.com	08054921923	107	5
\.


--
-- Data for Name: dataplan; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.dataplan (data_id, data_size, data_duration, data_price) FROM stdin;
1	350MB	2	200
2	1.8GB	14	500
3	3.9GB	30	1000
4	7.5GB	30	1500
5	9.2GB	30	2000
6	10.8GB	30	2500
7	14GB	30	3000
8	18GB	30	4000
9	24GB	30	5000
10	29.9GB	30	8000
11	50GB	30	10000
\.


--
-- Data for Name: department; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.department (dept_manegrid, dno, dname, dlocation, pno) FROM stdin;
108	1	ADMINISTRATION 	IKEJA          	44
101	2	ACCOUNT        	EGBEDA         	11
100	3	PACKAGING      	AJAH           	44
120	4	RESEARCH       	VI             	33
97	5	ACCOUNT        	MAGODO         	22
122	5	OPERATIONS     	MILE 2         	44
107	7	PACKAGING      	KETU           	55
\.


--
-- Data for Name: project; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.project (proj_manegrid, pno, pname, pduration) FROM stdin;
102	11	A    	9 MONTHS       
97	22	B    	14 MONTHS      
120	33	C    	16 MONTHS      
108	44	D    	25 MONTHS      
107	55	E    	9 MONTHS       
\.


--
-- Data for Name: staff; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.staff (staff_id, staff_name, dno, staff_sal, age, mobile) FROM stdin;
101	ALADE JOY                	2    	250000	33	8023089832
107	ALOKWE MARTIN            	7    	380000	48	7090082812
97	DANKANDE AMINAT          	5    	550000	40	9023688832
108	JOSIAH JOSHUA            	1    	120000	30	8053189131
120	ADELEKE JANE             	4    	200000	38	7061045862
122	OSAHON MARK              	6    	320000	44	8022289842
117	SULEIMAN AJAYI           	3    	800000	50	7030089811
104	KUTI LAWAL               	1    	750000	35	9145689842
100	MUSTAPHA ALI             	3    	500000	32	8063285831
\.


--
-- Name: customer customer_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.customer
    ADD CONSTRAINT customer_pkey PRIMARY KEY (c_id);


--
-- Name: dataplan dataplan_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.dataplan
    ADD CONSTRAINT dataplan_pkey PRIMARY KEY (data_id);


--
-- Name: department department_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.department
    ADD CONSTRAINT department_pkey PRIMARY KEY (dept_manegrid);


--
-- Name: staff employee_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.staff
    ADD CONSTRAINT employee_pkey PRIMARY KEY (staff_id);


--
-- Name: project project_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.project
    ADD CONSTRAINT project_pkey PRIMARY KEY (proj_manegrid);


--
-- Name: customer customer_data_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.customer
    ADD CONSTRAINT customer_data_id_fkey FOREIGN KEY (data_id) REFERENCES public.dataplan(data_id);


--
-- PostgreSQL database dump complete
--

\unrestrict ZKUgD77BMbefvFK8kgtXyI6pq5gRr1lxcNsV6Z7lj48mR7GxydyH5zakpRe1DnM

