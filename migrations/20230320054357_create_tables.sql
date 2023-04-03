DROP TYPE IF EXISTS VALID_QUALITY_TYPES;

CREATE TYPE RoleUser AS ENUM ('Admin', 'User', 'System');

CREATE TABLE users (
  id uuid NOT NULL,
  username VARCHAR(255) NOT NULL UNIQUE,
  password VARCHAR(255) NOT NULL,
  email VARCHAR(255) NOT NULL UNIQUE,
  role_name RoleUser NOT NULL,
  is_active BOOLEAN NOT NULL,
  is_tfa BOOLEAN NOT NULL,
  create_at timestamptz DEFAULT current_timestamp,
  update_at timestamptz DEFAULT current_timestamp,
  PRIMARY KEY(id)
);

CREATE OR REPLACE FUNCTION set_update_at_column()
RETURNS TRIGGER AS $$
BEGIN
   IF row(NEW.*) IS DISTINCT FROM row(OLD.*) THEN
      NEW.update_at= now();
      RETURN NEW;
   ELSE
      RETURN OLD;
   END IF;
END;
$$ language 'plpgsql';

CREATE TRIGGER update_user_date_time BEFORE UPDATE ON users FOR EACH ROW EXECUTE PROCEDURE set_update_at_column();

DO $$
DECLARE
    user_id uuid = gen_random_uuid();
BEGIN
    INSERT INTO users(id,username,password,email,role_name,is_active,is_tfa)
    VALUES (user_id,'test-user','$argon2id$v=19$m=4096,t=3,p=1$xj+gEfx2tF584ugWtZuZpw$t8MR3ns9T5n+0TsmUS3TGVQRmjRaoQVMyuBvvry1SbU','test-user@email.com','User',true,false);
END $$;
