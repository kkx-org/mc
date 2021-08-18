CREATE EXTENSION IF NOT EXISTS "pgcrypto";

CREATE OR REPLACE FUNCTION now_utc() RETURNS timestamp AS $$
  SELECT now() at time zone 'utc';
$$ LANGUAGE sql;

CREATE TABLE profiles (
	id uuid DEFAULT gen_random_uuid(),
	user_id uuid NOT NULL,
	name varchar(64) NOT NULL UNIQUE,
	PRIMARY KEY (id)
);

CREATE TABLE users (
	id uuid DEFAULT gen_random_uuid(),
	email varchar(64) NOT NULL UNIQUE,
	hashed_password varchar(512) NOT NULL,
	created_at timestamp DEFAULT now_utc(),
	selected_profile_id uuid,
	PRIMARY KEY (id),
	CONSTRAINT users_selected_profile_id_fkey
		FOREIGN KEY (selected_profile_id)
			REFERENCES profiles(id)
);

ALTER TABLE profiles 
	ADD CONSTRAINT profiles_user_id_fkey
		FOREIGN KEY (user_id)
			REFERENCES users(id);

CREATE TABLE capes (
	id uuid DEFAULT gen_random_uuid(),
	owner_id uuid NOT NULL,
	created_at timestamp DEFAULT now_utc(),
	PRIMARY KEY (id),
	FOREIGN KEY (owner_id)
		REFERENCES users(id)
);

CREATE INDEX ON capes(owner_id);

CREATE TABLE skins (
	id uuid DEFAULT gen_random_uuid(),
	owner_id uuid NOT NULL,
	created_at timestamp DEFAULT now_utc(),
	PRIMARY KEY (id),
	FOREIGN KEY (owner_id)
		REFERENCES users(id)
);

CREATE INDEX ON skins(owner_id);

CREATE TABLE sessions (
	id uuid DEFAULT gen_random_uuid(),
	name varchar(512),
	user_id uuid NOT NULL,
	expired_after timestamp DEFAULT now_utc() + ('1 month')::interval NOT NULL,
	PRIMARY KEY (id),
	FOREIGN KEY (user_id)
		REFERENCES users(id)
);

CREATE INDEX ON sessions(user_id);

CREATE OR REPLACE FUNCTION delete_expired_sessions() RETURNS trigger AS $$
	BEGIN
		DELETE FROM sessions WHERE now_utc() > expired_after;
	  RETURN NULL;
	END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trigger_delete_expired_sessions
	AFTER INSERT ON sessions
	EXECUTE PROCEDURE delete_expired_sessions();

CREATE VIEW active_sessions AS
	SELECT * FROM sessions WHERE now_utc() <= expired_after;

