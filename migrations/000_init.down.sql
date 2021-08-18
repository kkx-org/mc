DROP VIEW active_sessions;
DROP TABLE sessions;
DROP FUNCTION delete_expired_sessions();

DROP TABLE skins;
DROP TABLE capes;
ALTER TABLE profiles 
	DROP CONSTRAINT profiles_user_id_fkey;
ALTER TABLE users
	DROP CONSTRAINT users_selected_profile_id_fkey;
DROP TABLE profiles;
DROP TABLE users;
DROP FUNCTION now_utc();
