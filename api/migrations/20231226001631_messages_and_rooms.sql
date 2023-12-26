-- Add migration script here
CREATE TABLE rooms (
  id INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  name TEXT,
  user_id INT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  created_at timestamptz NOT NULL DEFAULT NOW()
);

CREATE TABLE messages (
  id INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  room_id INTEGER REFERENCES rooms(id) ON DELETE CASCADE,
  user_id INT REFERENCES users(id) ON DELETE CASCADE,
  message TEXT NOT NULL,
  created_at timestamptz NOT NULL DEFAULT NOW()
);

CREATE TABLE rooms_participants (
  room_id INTEGER REFERENCES rooms(id) ON DELETE CASCADE,
  user_id INT REFERENCES users(id) ON DELETE CASCADE,
  PRIMARY KEY (room_id, user_id)
);