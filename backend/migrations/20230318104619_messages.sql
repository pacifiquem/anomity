-- Add migration script here
CREATE TABLE messages (
  id       uuid primary key default gen_random_uuid(),
  user_id uuid NOT NULL,
  receiver_id uuid NOT NULL,
  content VARCHAR(255) NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

create index on messages(created_at desc);

alter table messages add constraint fk_user_id foreign key (user_id) references users(id);

alter table messages add constraint fk_receiver_id foreign key (receiver_id) references users(id);