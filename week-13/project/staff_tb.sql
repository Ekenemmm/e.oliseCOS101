--
-- PostgreSQL database dump
--

\restrict 41z0DIkvaTidAeKv0NYavv11WVatCMY53cZcX6aOI1AXfaVKNvxF9UCXiCGdcM2

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
-- Name: staff employee_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.staff
    ADD CONSTRAINT employee_pkey PRIMARY KEY (staff_id);


--
-- PostgreSQL database dump complete
--

\unrestrict 41z0DIkvaTidAeKv0NYavv11WVatCMY53cZcX6aOI1AXfaVKNvxF9UCXiCGdcM2

